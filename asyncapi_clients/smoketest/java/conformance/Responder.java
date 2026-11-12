/*
 * Origin:     Hand-written for the asyncapi-clients conformance suite
 * Changes:    Hand-edit allowed; this is a test harness, not generated output
 *
 * Author:     Christopher Kettell, TRL Software
 * Date:       2026-08-28
 * Version:    1.0
 * License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
 */
import java.nio.charset.StandardCharsets;
import java.time.Instant;
import java.util.ArrayList;
import java.util.List;

import javax.jms.BytesMessage;
import javax.jms.Connection;
import javax.jms.Destination;
import javax.jms.Message;
import javax.jms.MessageConsumer;
import javax.jms.MessageProducer;
import javax.jms.Session;
import javax.jms.TextMessage;

import org.apache.qpid.jms.JmsConnectionFactory;
import org.apache.qpid.jms.JmsDestination;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.node.ObjectNode;

/**
 * A11 command responder — Java half of the cross-language matrix.
 *
 * The mirror image of S3: it consumes vms.command, reads the requester's
 * `reply-to`, and answers *that node* with `correlation-id` mirroring the
 * request's `message-id` and `commandCorrelationId` carried through
 * unchanged. Every requester in the suite asserts exactly those three
 * things, so this is the responder side of the same contract.
 *
 * Anonymous sending is JMS's `createProducer(null)` plus
 * `send(destination, message)`. Qpid JMS maps that onto an AMQP anonymous
 * relay link, so one producer answers every requester's reply node — the
 * A11.4 preferred mechanism.
 *
 * Configuration mirrors vms_responder.py exactly, so the harness can swap
 * responders without knowing which language it is driving:
 *   TOPAS_RESPONDER_PROGRESSION   comma-separated statuses per command
 *   TOPAS_RESPONDER_DELAY         seconds between frames in a progression
 *   TOPAS_RESPONDER_MAX_REPLIES   exit after N whole progressions
 */
public class Responder {

    private static final String COMMAND_CHANNEL = "vms.command";

    public static void main(String[] args) throws Exception {
        String url = System.getenv().getOrDefault(
                "TOPAS_AMQP_URL", "amqp://localhost:5672");
        List<String> progression = new ArrayList<>();
        for (String s : System.getenv().getOrDefault(
                "TOPAS_RESPONDER_PROGRESSION", "pending,pending,success").split(",")) {
            if (!s.trim().isEmpty()) {
                progression.add(s.trim());
            }
        }
        double delay = Double.parseDouble(
                System.getenv().getOrDefault("TOPAS_RESPONDER_DELAY", "0.15"));
        String maxRepliesEnv = System.getenv("TOPAS_RESPONDER_MAX_REPLIES");
        Integer maxReplies = maxRepliesEnv == null ? null : Integer.valueOf(maxRepliesEnv);

        ObjectMapper mapper = new ObjectMapper();
        JmsConnectionFactory factory = new JmsConnectionFactory(url);

        try (Connection connection = factory.createConnection()) {
            connection.start();
            Session session = connection.createSession(false, Session.AUTO_ACKNOWLEDGE);
            MessageConsumer consumer = session.createConsumer(
                    session.createTopic(COMMAND_CHANNEL));
            // Anonymous producer: the destination travels per-send rather
            // than being fixed on the link, so one producer serves every
            // reply node (A11.4).
            MessageProducer producer = session.createProducer(null);

            // Readiness handshake — the harness waits for this rather than
            // sleeping, so a command published before the consumer attached
            // cannot be silently discarded.
            System.out.println("[responder-ready] java (anonymous producer)");
            System.out.flush();

            int replied = 0;
            while (maxReplies == null || replied < maxReplies) {
                Message command = consumer.receive(30_000);
                if (command == null) {
                    System.err.println("[responder] no command within 30s — exiting");
                    break;
                }

                Destination replyTo = command.getJMSReplyTo();
                if (replyTo == null) {
                    // Under A11 there is nowhere to address the response.
                    // Loud, and does not answer.
                    System.err.println("[responder] FATAL: command carried no reply-to; "
                            + "under A11 there is nowhere to address the response");
                    break;
                }
                // AMQP's `reply-to` carries an address; JMS requires a
                // destination *type*, which the wire does not supply. With
                // no x-opt-jms-reply-to annotation, Qpid JMS infers one from
                // the consumer's own destination — and this consumer is a
                // topic, so getJMSReplyTo() returns a JmsTopic. Answering
                // that sends the response to a MULTICAST address with no
                // subscribers, and the broker discards it: the requester
                // times out having received nothing at all, with every
                // correlation counter still at zero.
                //
                // A dynamic reply node is point-to-point by definition, so
                // the address is re-bound as a queue. Note this is the
                // responder compensating for a JMS type distinction that
                // A11 does not make.
                String replyAddress = ((JmsDestination) replyTo).getAddress();
                Destination replyNode = session.createQueue(replyAddress);

                // A11: correlation-id mirrors the request's message-id.
                String requestMessageId = command.getJMSMessageID();

                // Decoding does not degrade: an unrecognised body is a
                // visible failure, not a placeholder object.
                JsonNode body = mapper.readTree(bodyAsString(command));
                JsonNode appId = body.path("commandCorrelationId");

                for (int i = 0; i < progression.size(); i++) {
                    ObjectNode frame = mapper.createObjectNode();
                    frame.set("vmsId", body.path("vmsId").isMissingNode()
                            ? mapper.nullNode() : body.get("vmsId"));
                    // Carried through unchanged, at the top level rather
                    // than buried in an echo.
                    if (appId.isMissingNode() || appId.isNull()) {
                        frame.putNull("commandCorrelationId");
                    } else {
                        frame.set("commandCorrelationId", appId);
                    }
                    frame.put("status", progression.get(i));
                    frame.put("responseTime", Instant.now().toString());
                    frame.set("echo", body);

                    TextMessage reply = session.createTextMessage(
                            mapper.writeValueAsString(frame));
                    reply.setJMSCorrelationID(requestMessageId);
                    // Sending to an explicit destination on an anonymous
                    // producer is what sets the AMQP `to` property.
                    producer.send(replyNode, reply);
                    System.out.printf("[responder] %s -> %s corr=%s%n",
                            progression.get(i), replyAddress, requestMessageId);
                    System.out.flush();

                    if (i < progression.size() - 1 && delay > 0) {
                        Thread.sleep((long) (delay * 1000));
                    }
                }
                replied++;
            }
        }
    }

    /**
     * An explicit compatibility layer over the body representations clients
     * produce. It throws rather than degrading — a conformance responder
     * that turns an unrecognised body into a placeholder loses the field
     * under test and lets the requester's check pass vacuously.
     */
    private static String bodyAsString(Message msg) throws Exception {
        if (msg instanceof TextMessage) {
            return ((TextMessage) msg).getText();
        }
        if (msg instanceof BytesMessage) {
            BytesMessage bytes = (BytesMessage) msg;
            byte[] buf = new byte[(int) bytes.getBodyLength()];
            bytes.readBytes(buf);
            return new String(buf, StandardCharsets.UTF_8);
        }
        throw new IllegalStateException(
                "unhandled body representation " + msg.getClass().getName()
                + "; add it here rather than degrading to a placeholder");
    }
}
