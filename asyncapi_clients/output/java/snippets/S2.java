/*
 * File:        output/java/snippets/S2.java
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
 * Scenario S2 — Wildcard subscription across a domain.
 *
 * Demonstrates: subscribing to *.fault (every
 * fault from every device domain) using AMQP 1.0 source filters.
 * Teaches A8 (channel naming convention).
 *
 * Wildcard syntax is broker-specific; this snippet uses
 * {@code session.createTopic(...)} which Apache Artemis treats as
 * an address-pattern topic. Apache Qpid Dispatch Router uses a
 * different prefix (e.g. {@code mc/*.fault}); adjust to your
 * deployment.
 */
public class S2 {

    private static final String PATTERN = "*.fault";

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
            // Topic (multi-publisher) destination — wildcard semantics
            // depend on the broker. See class Javadoc.
            Destination dest = session.createTopic(PATTERN);
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
                    System.err.println("[S2] receive() timed out — broker idle? aborting.");
                    return;
                }
                JsonNode payload = mapper.readTree(bodyAsString(msg));
                String address = msg.getJMSDestination() != null
                        ? msg.getJMSDestination().toString()
                        : PATTERN;
                String domain = address.contains(".")
                        ? address.substring(0, address.indexOf('.'))
                        : address;
                System.out.printf("[fault from %s] %s%n", domain, payload);
                received++;
            }
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
