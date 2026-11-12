/*
 * File:        output/java/snippets/S7.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
import java.nio.charset.StandardCharsets;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedHashMap;
import java.util.Map;
import java.util.Set;

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
 * Scenario S7 — Static reference data: consume the network topology.
 *
 * Demonstrates: the snapshot-then-disconnect idiom for static
 * reference data. Network topology is published across five
 * channels (one per object type: node, link, zone, path, turn). A
 * consumer that needs the topology opens five receivers, accumulates
 * each object into a typed {@code Map} keyed by the per-type id field,
 * and disconnects once all five streams have produced their first
 * message — different from S1/S5 which are continuous-telemetry
 * consumers.
 *
 * The resulting in-memory {@code Map} is what a route-planning or
 * incident-location component would build before serving its first
 * request.
 */
public class S7 {

    /** Channel name -> per-channel object store, keyed by the per-type id field. */
    private static final Map<String, Map<String, JsonNode>> TOPOLOGY = new LinkedHashMap<>();

    /** Each network object has a type-specific id field (no shared objectId). */
    private static final Map<String, String> ID_FIELDS = new LinkedHashMap<>();

    static {
        TOPOLOGY.put("network.node", new HashMap<>());
        TOPOLOGY.put("network.link", new HashMap<>());
        TOPOLOGY.put("network.zone", new HashMap<>());
        TOPOLOGY.put("network.path", new HashMap<>());
        TOPOLOGY.put("network.turn", new HashMap<>());

        ID_FIELDS.put("network.node", "nodeId");
        ID_FIELDS.put("network.link", "linkId");
        ID_FIELDS.put("network.zone", "zoneId");
        ID_FIELDS.put("network.path", "pathId");
        ID_FIELDS.put("network.turn", "turnId");
    }

    public static void main(String[] args) throws Exception {
        String url = System.getenv().getOrDefault(
                "TOPAS_AMQP_URL", "amqps://{host}:{port}".replace("{host}", "localhost").replace("{port}", "5671"));

        JmsConnectionFactory factory = new JmsConnectionFactory(url);
        ObjectMapper mapper = new ObjectMapper();

        try (Connection connection = factory.createConnection()) {
            connection.start();
            Session session = connection.createSession(false, Session.AUTO_ACKNOWLEDGE);

            Map<String, MessageConsumer> consumers = new LinkedHashMap<>();
            for (String channel : TOPOLOGY.keySet()) {
                Destination dest = session.createTopic(channel);
                consumers.put(channel, session.createConsumer(dest));
            }
            // Readiness handshake: the harness waits for this
            // before starting the publisher. JMS createConsumer returns
            // once the consumer is established, so anything published
            // after this line will be delivered; anything before it
            // would be discarded with no consumer attached.
            System.err.println("[ready]");

            Set<String> seenFirstOn = new HashSet<>();
            // Round-robin poll all five consumers; in production you
            // would tune this differently (drain each stream, or wait
            // for a sync-batch completion per A5). This getting-started
            // snippet just exits as soon as every channel has produced
            // at least one message.
            long deadline = System.currentTimeMillis() + 30_000;
            while (seenFirstOn.size() < TOPOLOGY.size()
                    && System.currentTimeMillis() < deadline) {
                for (Map.Entry<String, MessageConsumer> entry : consumers.entrySet()) {
                    Message msg = entry.getValue().receive(500);
                    if (msg == null) {
                        continue;
                    }
                    String channel = entry.getKey();
                    JsonNode obj = mapper.readTree(bodyAsString(msg));
                    String idField = ID_FIELDS.get(channel);
                    String id = obj.path(idField).asText(null);
                    if (id == null) {
                        // Safety net — every network object should
                        // carry its type-specific id (nodeId, linkId, ...).
                        continue;
                    }
                    TOPOLOGY.get(channel).put(id, obj);
                    seenFirstOn.add(channel);
                }
            }

            if (seenFirstOn.size() < TOPOLOGY.size()) {
                System.err.printf("[S7] timed out waiting for all five channels; saw: %s%n",
                        seenFirstOn);
                return;
            }

            StringBuilder summary = new StringBuilder("[topology] ");
            boolean first = true;
            for (Map.Entry<String, Map<String, JsonNode>> entry : TOPOLOGY.entrySet()) {
                if (!first) {
                    summary.append(", ");
                }
                String channel = entry.getKey();
                String shortName = channel.substring(channel.lastIndexOf('.') + 1);
                summary.append(shortName).append('=').append(entry.getValue().size());
                first = false;
            }
            System.out.println(summary);
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
