/*
 * File:        output/java/snippets/S1.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
import java.nio.charset.StandardCharsets;

import javax.jms.BytesMessage;
import javax.jms.Connection;
import javax.jms.Destination;
import javax.jms.Message;
import javax.jms.MessageConsumer;
import javax.jms.Session;
import javax.jms.TextMessage;

import org.apache.qpid.jms.JmsConnectionFactory;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;

/**
 * Scenario S1 — Subscribe to detector.measurement and print each message.
 *
 * Demonstrates: connect over AMQP 1.0 + mTLS, subscribe to a single
 * channel, parse the JSON payload, print, exit after the first N
 * messages.
 *
 * Tip: a typed SDK is not yet generated for Java (deferred
 * to a follow-up sub-project). Parse the JSON to a record/POJO of
 * your choosing — Jackson's {@code ObjectMapper.treeToValue} is the
 * usual bridge.
 */
public class S1 {

    private static final String CHANNEL = "detector.measurement";

    public static void main(String[] args) throws Exception {
        String url = System.getenv().getOrDefault(
                "TOPAS_AMQP_URL", "amqps://{host}:{port}".replace("{host}", "localhost").replace("{port}", "5671"));
        int maxMessages = Integer.parseInt(
                System.getenv().getOrDefault("TOPAS_MAX_MESSAGES", "5"));

        JmsConnectionFactory factory = new JmsConnectionFactory(url);
        ObjectMapper mapper = new ObjectMapper();

        try (Connection connection = factory.createConnection()) {
            connection.start();
            Session session = connection.createSession(false, Session.AUTO_ACKNOWLEDGE);
            Destination dest = session.createTopic(CHANNEL);
            MessageConsumer consumer = session.createConsumer(dest);
            // Readiness handshake: the harness waits for this
            // before starting the publisher. JMS createConsumer returns
            // once the consumer is established, so anything published
            // after this line will be delivered; anything before it
            // would be discarded with no consumer attached.
            System.err.println("[ready]");

            int received = 0;
            while (received < maxMessages) {
                Message msg = consumer.receive(5000);
                if (msg == null) {
                    System.err.println("[S1] receive() timed out — broker idle? aborting.");
                    return;
                }
                JsonNode payload = mapper.readTree(bodyAsString(msg));
                System.out.printf("[%d] %s%n", received + 1, payload);
                received++;
            }
        }
    }

    /** Read the message body as a UTF-8 String, regardless of whether it
     *  arrived as a TextMessage or a BytesMessage. TOPAS 3000A uses
     *  JSON in BytesMessage by default (A1) but tolerant readers are
     *  cheap insurance. */
    static String bodyAsString(Message msg) throws Exception {
        if (msg instanceof TextMessage) {
            return ((TextMessage) msg).getText();
        }
        if (msg instanceof BytesMessage) {
            BytesMessage b = (BytesMessage) msg;
            byte[] buf = new byte[(int) b.getBodyLength()];
            b.readBytes(buf);
            return new String(buf, StandardCharsets.UTF_8);
        }
        return msg.getBody(String.class);
    }
}
