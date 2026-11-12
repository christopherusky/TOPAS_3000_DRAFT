/*
 * File:        output/java/sdk/com/topas/utmc/transport_route/TransportRouteCommand.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.transport_route;

import java.io.IOException;

import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.databind.DeserializationContext;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.JsonDeserializer;
import com.fasterxml.jackson.databind.JsonSerializer;
import com.fasterxml.jackson.databind.SerializerProvider;
import com.fasterxml.jackson.core.JsonGenerator;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;

/**
 * Transport route command payloads.
 *
 * <p>Tagged union over {@code kind}. Exactly one accessor is
 * non-null; {@link #getKind()} says which.
 */
@JsonDeserialize(using = TransportRouteCommand.Deserializer.class)
@JsonSerialize(using = TransportRouteCommand.Serializer.class)
public class TransportRouteCommand {

    private final String kind;
    private final Object value;

    public TransportRouteCommand(String kind, Object value) {
        this.kind = kind;
        this.value = value;
    }

    /** The wire discriminator that selected the variant. */
    public String getKind() { return kind; }

    /** The variant, as its concrete generated type. */
    public Object getValue() { return value; }

    public RequestTransportRouteStatusCommand asRequestTransportRouteStatusCommand() {
        return value instanceof RequestTransportRouteStatusCommand ? (RequestTransportRouteStatusCommand) value : null;
    }

    public static class Deserializer extends JsonDeserializer<TransportRouteCommand> {
        private static final ObjectMapper MAPPER = new ObjectMapper()
                .findAndRegisterModules();

        @Override
        public TransportRouteCommand deserialize(JsonParser p, DeserializationContext ctx)
                throws IOException {
            JsonNode node = p.readValueAsTree();
            JsonNode tagNode = node.get("kind");
            String tag = tagNode == null ? null : tagNode.asText();
            if (tag == null) {
                throw new IOException("TransportRouteCommand: missing discriminator 'kind'");
            }
            switch (tag) {
                case "requestStatus":
                    return new TransportRouteCommand(tag, MAPPER.treeToValue(node, RequestTransportRouteStatusCommand.class));
                default:
                    throw new IOException("TransportRouteCommand: unknown kind  + tag + ");
            }
        }
    }

    public static class Serializer extends JsonSerializer<TransportRouteCommand> {
        @Override
        public void serialize(TransportRouteCommand v, JsonGenerator gen, SerializerProvider sp)
                throws IOException {
            // The variant carries its own tag, so writing the concrete
            // value round-trips without special handling.
            gen.writeObject(v.getValue());
        }
    }
}
