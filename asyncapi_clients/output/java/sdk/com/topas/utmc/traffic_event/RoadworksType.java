/*
 * File:        output/java/sdk/com/topas/utmc/traffic_event/RoadworksType.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.traffic_event;

import java.io.IOException;
import com.fasterxml.jackson.annotation.*;

/**
 * Classification of the roadworks.
 *
 * Classification of roadworks (7 values from XSD).
 */
public enum RoadworksType {
    CENTRAL_RESERVATION, GENERAL, OTHER, OVERHEAD, SURFACE, VERGE, WATER_MAIN_WORK;

    @JsonValue
    public String toValue() {
        switch (this) {
            case CENTRAL_RESERVATION: return "centralReservation";
            case GENERAL: return "general";
            case OTHER: return "other";
            case OVERHEAD: return "overhead";
            case SURFACE: return "surface";
            case VERGE: return "verge";
            case WATER_MAIN_WORK: return "waterMainWork";
        }
        return null;
    }

    @JsonCreator
    public static RoadworksType forValue(String value) throws IOException {
        if (value.equals("centralReservation")) return CENTRAL_RESERVATION;
        if (value.equals("general")) return GENERAL;
        if (value.equals("other")) return OTHER;
        if (value.equals("overhead")) return OVERHEAD;
        if (value.equals("surface")) return SURFACE;
        if (value.equals("verge")) return VERGE;
        if (value.equals("waterMainWork")) return WATER_MAIN_WORK;
        throw new IOException("Cannot deserialize RoadworksType");
    }
}
