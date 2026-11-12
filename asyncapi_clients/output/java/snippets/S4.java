/*
 * File:        output/java/snippets/S4.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
import java.nio.charset.StandardCharsets;
import java.util.BitSet;
import java.util.HashMap;
import java.util.Map;

import javax.jms.BytesMessage;
import javax.jms.Connection;
import javax.jms.Destination;
import javax.jms.Message;
import javax.jms.MessageConsumer;
import javax.jms.Session;
import javax.jms.TextMessage;

import org.apache.qpid.jms.JmsConnectionFactory;
import org.apache.qpid.jms.message.JmsMessage;
import org.apache.qpid.jms.provider.amqp.message.AmqpJmsMessageFacade;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;

/**
 * Scenario S4 — Distinguish sync vs delta messages and reassemble a sync batch.
 *
 * Demonstrates: reading the x-message-mode AMQP application
 * property (A3) to tell snapshot data from change events; using the
 * x-sync-id / x-sync-sequence /
 * x-sync-total triple (A5) to detect when a sync batch is
 * complete.
 *
 * Sync progress is tracked in a {@code Map<String, BitSet>} keyed by
 * x-sync-id; each bit corresponds to one
 * x-sync-sequence value seen on the wire.
 */
public class S4 {

    private static final String CHANNEL = "detector.measurement";

    public static void main(String[] args) throws Exception {
        String url = System.getenv().getOrDefault(
                "TOPAS_AMQP_URL", "amqps://{host}:{port}".replace("{host}", "localhost").replace("{port}", "5671"));
        int maxMessages = Integer.parseInt(
                System.getenv().getOrDefault("TOPAS_MAX_MESSAGES", "20"));

        JmsConnectionFactory factory = new JmsConnectionFactory(url);
        ObjectMapper mapper = new ObjectMapper();
        // sync-id -> set of sequence numbers received so far.
        Map<String, BitSet> syncProgress = new HashMap<>();

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
                    System.err.println("[S4] receive() timed out — broker idle? aborting.");
                    return;
                }
                // x-message-mode, x-sync-id,
                // x-sync-sequence and x-sync-total
                // all contain hyphens which the JMS API rejects as invalid
                // identifiers. Read them directly from the AMQP
                // application-properties map via the Qpid JMS facade.
                AmqpJmsMessageFacade facade =
                        (AmqpJmsMessageFacade) ((JmsMessage) msg).getFacade();
                Object rawMode = facade.getApplicationProperty("x-message-mode");
                String mode = rawMode != null ? rawMode.toString() : "delta";
                JsonNode body = mapper.readTree(bodyAsString(msg));
                received++;

                if ("delta".equals(mode)) {
                    System.out.printf("[delta] %s%n", body);
                    continue;
                }

                // mode == "sync" — track batch completion.
                Object rawSyncId = facade.getApplicationProperty("x-sync-id");
                String syncId = rawSyncId != null ? rawSyncId.toString() : null;
                Object rawSeq = facade.getApplicationProperty("x-sync-sequence");
                int seq = rawSeq != null ? ((Number) rawSeq).intValue() : 0;
                Object rawTotal = facade.getApplicationProperty("x-sync-total");
                int total = rawTotal != null ? ((Number) rawTotal).intValue() : 0;
                if (syncId == null || total == 0) {
                    System.out.printf("[sync*] %s  (incomplete sync metadata)%n", body);
                    continue;
                }
                BitSet seen = syncProgress.computeIfAbsent(syncId, k -> new BitSet());
                seen.set(seq);
                int progress = seen.cardinality();
                System.out.printf("[sync %s %d/%d] %s%n", syncId, progress, total, body);
                if (progress == total) {
                    System.out.printf("[sync %s] complete%n", syncId);
                    syncProgress.remove(syncId);
                }
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
