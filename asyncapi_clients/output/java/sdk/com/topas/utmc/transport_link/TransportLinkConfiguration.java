/*
 * File:        output/java/sdk/com/topas/utmc/transport_link/TransportLinkConfiguration.java
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
 * Transport link operational configuration (SCOOT/GPS/ANPR discriminated union).
 *
 * <p>Tagged union over {@code kind}. Exactly one accessor is
 * non-null; {@link #getKind()} says which.
 */
@JsonDeserialize(using = TransportLinkConfiguration.Deserializer.class)
@JsonSerialize(using = TransportLinkConfiguration.Serializer.class)
public class TransportLinkConfiguration {

    private final String kind;
    private final Object value;

    public TransportLinkConfiguration(String kind, Object value) {
        this.kind = kind;
        this.value = value;
    }

    /** The wire discriminator that selected the variant. */
    public String getKind() { return kind; }

    /** The variant, as its concrete generated type. */
    public Object getValue() { return value; }

    public TransportLinkAnprConfiguration asTransportLinkAnprConfiguration() {
        return value instanceof TransportLinkAnprConfiguration ? (TransportLinkAnprConfiguration) value : null;
    }

    public TransportLinkGpsConfiguration asTransportLinkGpsConfiguration() {
        return value instanceof TransportLinkGpsConfiguration ? (TransportLinkGpsConfiguration) value : null;
    }

    public TransportLinkScootConfiguration asTransportLinkScootConfiguration() {
        return value instanceof TransportLinkScootConfiguration ? (TransportLinkScootConfiguration) value : null;
    }

    public static class Deserializer extends JsonDeserializer<TransportLinkConfiguration> {
        private static final ObjectMapper MAPPER = new ObjectMapper()
                .findAndRegisterModules();

        @Override
        public TransportLinkConfiguration deserialize(JsonParser p, DeserializationContext ctx)
                throws IOException {
            JsonNode node = p.readValueAsTree();
            JsonNode tagNode = node.get("kind");
            String tag = tagNode == null ? null : tagNode.asText();
            if (tag == null) {
                throw new IOException("TransportLinkConfiguration: missing discriminator 'kind'");
            }
            switch (tag) {
                case "anpr":
                    return new TransportLinkConfiguration(tag, MAPPER.treeToValue(node, TransportLinkAnprConfiguration.class));
                case "gps":
                    return new TransportLinkConfiguration(tag, MAPPER.treeToValue(node, TransportLinkGpsConfiguration.class));
                case "scoot":
                    return new TransportLinkConfiguration(tag, MAPPER.treeToValue(node, TransportLinkScootConfiguration.class));
                default:
                    throw new IOException("TransportLinkConfiguration: unknown kind  + tag + ");
            }
        }
    }

    public static class Serializer extends JsonSerializer<TransportLinkConfiguration> {
        @Override
        public void serialize(TransportLinkConfiguration v, JsonGenerator gen, SerializerProvider sp)
                throws IOException {
            // The variant carries its own tag, so writing the concrete
            // value round-trips without special handling.
            gen.writeObject(v.getValue());
        }
    }
}
