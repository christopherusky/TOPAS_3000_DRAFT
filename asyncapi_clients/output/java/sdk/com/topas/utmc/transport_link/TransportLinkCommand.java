/*
 * File:        output/java/sdk/com/topas/utmc/transport_link/TransportLinkCommand.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.transport_link;

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
 * Transport link command payloads.
 *
 * <p>Tagged union over {@code kind}. Exactly one accessor is
 * non-null; {@link #getKind()} says which.
 */
@JsonDeserialize(using = TransportLinkCommand.Deserializer.class)
@JsonSerialize(using = TransportLinkCommand.Serializer.class)
public class TransportLinkCommand {

    private final String kind;
    private final Object value;

    public TransportLinkCommand(String kind, Object value) {
        this.kind = kind;
        this.value = value;
    }

    /** The wire discriminator that selected the variant. */
    public String getKind() { return kind; }

    /** The variant, as its concrete generated type. */
    public Object getValue() { return value; }

    public RecalibrateTransportLinkCommand asRecalibrateTransportLinkCommand() {
        return value instanceof RecalibrateTransportLinkCommand ? (RecalibrateTransportLinkCommand) value : null;
    }

    public RequestTransportLinkStatusCommand asRequestTransportLinkStatusCommand() {
        return value instanceof RequestTransportLinkStatusCommand ? (RequestTransportLinkStatusCommand) value : null;
    }

    public SetStocCommand asSetStocCommand() {
        return value instanceof SetStocCommand ? (SetStocCommand) value : null;
    }

    public static class Deserializer extends JsonDeserializer<TransportLinkCommand> {
        private static final ObjectMapper MAPPER = new ObjectMapper()
                .findAndRegisterModules();

        @Override
        public TransportLinkCommand deserialize(JsonParser p, DeserializationContext ctx)
                throws IOException {
            JsonNode node = p.readValueAsTree();
            JsonNode tagNode = node.get("kind");
            String tag = tagNode == null ? null : tagNode.asText();
            if (tag == null) {
                throw new IOException("TransportLinkCommand: missing discriminator 'kind'");
            }
            switch (tag) {
                case "recalibrate":
                    return new TransportLinkCommand(tag, MAPPER.treeToValue(node, RecalibrateTransportLinkCommand.class));
                case "requestStatus":
                    return new TransportLinkCommand(tag, MAPPER.treeToValue(node, RequestTransportLinkStatusCommand.class));
                case "setStoc":
                    return new TransportLinkCommand(tag, MAPPER.treeToValue(node, SetStocCommand.class));
                default:
                    throw new IOException("TransportLinkCommand: unknown kind  + tag + ");
            }
        }
    }

    public static class Serializer extends JsonSerializer<TransportLinkCommand> {
        @Override
        public void serialize(TransportLinkCommand v, JsonGenerator gen, SerializerProvider sp)
                throws IOException {
            // The variant carries its own tag, so writing the concrete
            // value round-trips without special handling.
            gen.writeObject(v.getValue());
        }
    }
}
