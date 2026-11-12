/*
 * File:        output/java/sdk/com/topas/utmc/detector/DetectorCommand.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.detector;

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
 * Detector command payloads.
 *
 * <p>Tagged union over {@code kind}. Exactly one accessor is
 * non-null; {@link #getKind()} says which.
 */
@JsonDeserialize(using = DetectorCommand.Deserializer.class)
@JsonSerialize(using = DetectorCommand.Serializer.class)
public class DetectorCommand {

    private final String kind;
    private final Object value;

    public DetectorCommand(String kind, Object value) {
        this.kind = kind;
        this.value = value;
    }

    /** The wire discriminator that selected the variant. */
    public String getKind() { return kind; }

    /** The variant, as its concrete generated type. */
    public Object getValue() { return value; }

    public CalibrateDetectorCommand asCalibrateDetectorCommand() {
        return value instanceof CalibrateDetectorCommand ? (CalibrateDetectorCommand) value : null;
    }

    public ResetDetectorCommand asResetDetectorCommand() {
        return value instanceof ResetDetectorCommand ? (ResetDetectorCommand) value : null;
    }

    public SetThresholdDetectorCommand asSetThresholdDetectorCommand() {
        return value instanceof SetThresholdDetectorCommand ? (SetThresholdDetectorCommand) value : null;
    }

    public static class Deserializer extends JsonDeserializer<DetectorCommand> {
        private static final ObjectMapper MAPPER = new ObjectMapper()
                .findAndRegisterModules();

        @Override
        public DetectorCommand deserialize(JsonParser p, DeserializationContext ctx)
                throws IOException {
            JsonNode node = p.readValueAsTree();
            JsonNode tagNode = node.get("kind");
            String tag = tagNode == null ? null : tagNode.asText();
            if (tag == null) {
                throw new IOException("DetectorCommand: missing discriminator 'kind'");
            }
            switch (tag) {
                case "calibrate":
                    return new DetectorCommand(tag, MAPPER.treeToValue(node, CalibrateDetectorCommand.class));
                case "reset":
                    return new DetectorCommand(tag, MAPPER.treeToValue(node, ResetDetectorCommand.class));
                case "setThreshold":
                    return new DetectorCommand(tag, MAPPER.treeToValue(node, SetThresholdDetectorCommand.class));
                default:
                    throw new IOException("DetectorCommand: unknown kind  + tag + ");
            }
        }
    }

    public static class Serializer extends JsonSerializer<DetectorCommand> {
        @Override
        public void serialize(DetectorCommand v, JsonGenerator gen, SerializerProvider sp)
                throws IOException {
            // The variant carries its own tag, so writing the concrete
            // value round-trips without special handling.
            gen.writeObject(v.getValue());
        }
    }
}
