/*
 * File:        output/java/snippets/S6.java
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
 * Scenario S6 — Discriminated-union consumer for transport-link.measurement.
 *
 * Demonstrates: client-side dispatch on the {@code kind} discriminator
 *. The TransportLinkMeasurement payload is one of three
 * variants — SCOOT, GPS, or ANPR — distinguished by the {@code kind}
 * field. The snippet reads the JSON, switches on the discriminator,
 * and dispatches to a per-variant handler.
 *
 * This is the idiom for handling any of the discriminated unions in
 * TOPAS 3000A (F6 in the cross-cutting design log).
 *
 * Tip: a typed SDK is not yet generated for Java. When it ships,
 * replace the {@code switch} below with a sealed-interface +
 * {@code instanceof} pattern over the generated variant classes.
 */
public class S6 {

    private static final String CHANNEL = "transport-link.measurement";

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
                    System.err.println("[S6] receive() timed out — broker idle? aborting.");
                    return;
                }
                JsonNode payload = mapper.readTree(bodyAsString(msg));
                System.out.printf("[%d] %s%n", received + 1, handle(payload));
                received++;
            }
        }
    }

    /** Dispatch on the {@code kind} JSON discriminator. */
    private static String handle(JsonNode variant) {
        String kind = variant.path("kind").asText("");
        switch (kind) {
            case "scoot":
                return String.format("SCOOT flow=%s occupancy%%=%s",
                        variant.path("currentFlowVehicles").asText("n/a"),
                        variant.path("occupancyPercent").asText("n/a"));
            case "gps":
                return String.format("GPS meanTravelTime=%ss",
                        variant.path("meanTravelTimeSeconds").asText("n/a"));
            case "anpr":
                return String.format("ANPR in=%s out=%s matches=%s",
                        variant.path("numberPlatesIn").asText("n/a"),
                        variant.path("numberPlatesOut").asText("n/a"),
                        variant.path("numberPlatesMatches").asText("n/a"));
            default:
                return "unknown variant kind=" + (kind.isEmpty() ? "<missing>" : kind);
        }
    }

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
