/*
 * File:        output/java/sdk/com/topas/utmc/vms/VmsCommand.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.vms;

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
 * Variable message sign command payloads.
 *
 * <p>Tagged union over {@code kind}. Exactly one accessor is
 * non-null; {@link #getKind()} says which.
 */
@JsonDeserialize(using = VmsCommand.Deserializer.class)
@JsonSerialize(using = VmsCommand.Serializer.class)
public class VmsCommand {

    private final String kind;
    private final Object value;

    public VmsCommand(String kind, Object value) {
        this.kind = kind;
        this.value = value;
    }

    /** The wire discriminator that selected the variant. */
    public String getKind() { return kind; }

    /** The variant, as its concrete generated type. */
    public Object getValue() { return value; }

    public ClearSignCommand asClearSignCommand() {
        return value instanceof ClearSignCommand ? (ClearSignCommand) value : null;
    }

    public DisplayGraphicCommand asDisplayGraphicCommand() {
        return value instanceof DisplayGraphicCommand ? (DisplayGraphicCommand) value : null;
    }

    public DisplayMessageCommand asDisplayMessageCommand() {
        return value instanceof DisplayMessageCommand ? (DisplayMessageCommand) value : null;
    }

    public SetBrightnessCommand asSetBrightnessCommand() {
        return value instanceof SetBrightnessCommand ? (SetBrightnessCommand) value : null;
    }

    public static class Deserializer extends JsonDeserializer<VmsCommand> {
        private static final ObjectMapper MAPPER = new ObjectMapper()
                .findAndRegisterModules();

        @Override
        public VmsCommand deserialize(JsonParser p, DeserializationContext ctx)
                throws IOException {
            JsonNode node = p.readValueAsTree();
            JsonNode tagNode = node.get("kind");
            String tag = tagNode == null ? null : tagNode.asText();
            if (tag == null) {
                throw new IOException("VmsCommand: missing discriminator 'kind'");
            }
            switch (tag) {
                case "clearSign":
                    return new VmsCommand(tag, MAPPER.treeToValue(node, ClearSignCommand.class));
                case "displayGraphic":
                    return new VmsCommand(tag, MAPPER.treeToValue(node, DisplayGraphicCommand.class));
                case "displayMessage":
                    return new VmsCommand(tag, MAPPER.treeToValue(node, DisplayMessageCommand.class));
                case "setBrightness":
                    return new VmsCommand(tag, MAPPER.treeToValue(node, SetBrightnessCommand.class));
                default:
                    throw new IOException("VmsCommand: unknown kind  + tag + ");
            }
        }
    }

    public static class Serializer extends JsonSerializer<VmsCommand> {
        @Override
        public void serialize(VmsCommand v, JsonGenerator gen, SerializerProvider sp)
                throws IOException {
            // The variant carries its own tag, so writing the concrete
            // value round-trips without special handling.
            gen.writeObject(v.getValue());
        }
    }
}
