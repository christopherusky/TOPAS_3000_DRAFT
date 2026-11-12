/*
 * File:        output/java/sdk/com/topas/utmc/detector/DetectorMeasurement.java
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
 * Detector dynamic measurement data.
 *
 * <p>Tagged union over {@code kind}. Exactly one accessor is
 * non-null; {@link #getKind()} says which.
 */
@JsonDeserialize(using = DetectorMeasurement.Deserializer.class)
@JsonSerialize(using = DetectorMeasurement.Serializer.class)
public class DetectorMeasurement {

    private final String kind;
    private final Object value;

    public DetectorMeasurement(String kind, Object value) {
        this.kind = kind;
        this.value = value;
    }

    /** The wire discriminator that selected the variant. */
    public String getKind() { return kind; }

    /** The variant, as its concrete generated type. */
    public Object getValue() { return value; }

    public FlowDynamic asFlowDynamic() {
        return value instanceof FlowDynamic ? (FlowDynamic) value : null;
    }

    public HeadwayDynamic asHeadwayDynamic() {
        return value instanceof HeadwayDynamic ? (HeadwayDynamic) value : null;
    }

    public OccupancyDynamic asOccupancyDynamic() {
        return value instanceof OccupancyDynamic ? (OccupancyDynamic) value : null;
    }

    public QueueDynamic asQueueDynamic() {
        return value instanceof QueueDynamic ? (QueueDynamic) value : null;
    }

    public SpeedDynamic asSpeedDynamic() {
        return value instanceof SpeedDynamic ? (SpeedDynamic) value : null;
    }

    public static class Deserializer extends JsonDeserializer<DetectorMeasurement> {
        private static final ObjectMapper MAPPER = new ObjectMapper()
                .findAndRegisterModules();

        @Override
        public DetectorMeasurement deserialize(JsonParser p, DeserializationContext ctx)
                throws IOException {
            JsonNode node = p.readValueAsTree();
            JsonNode tagNode = node.get("kind");
            String tag = tagNode == null ? null : tagNode.asText();
            if (tag == null) {
                throw new IOException("DetectorMeasurement: missing discriminator 'kind'");
            }
            switch (tag) {
                case "flow":
                    return new DetectorMeasurement(tag, MAPPER.treeToValue(node, FlowDynamic.class));
                case "headway":
                    return new DetectorMeasurement(tag, MAPPER.treeToValue(node, HeadwayDynamic.class));
                case "occupancy":
                    return new DetectorMeasurement(tag, MAPPER.treeToValue(node, OccupancyDynamic.class));
                case "queue":
                    return new DetectorMeasurement(tag, MAPPER.treeToValue(node, QueueDynamic.class));
                case "speed":
                    return new DetectorMeasurement(tag, MAPPER.treeToValue(node, SpeedDynamic.class));
                default:
                    throw new IOException("DetectorMeasurement: unknown kind  + tag + ");
            }
        }
    }

    public static class Serializer extends JsonSerializer<DetectorMeasurement> {
        @Override
        public void serialize(DetectorMeasurement v, JsonGenerator gen, SerializerProvider sp)
                throws IOException {
            // The variant carries its own tag, so writing the concrete
            // value round-trips without special handling.
            gen.writeObject(v.getValue());
        }
    }
}
