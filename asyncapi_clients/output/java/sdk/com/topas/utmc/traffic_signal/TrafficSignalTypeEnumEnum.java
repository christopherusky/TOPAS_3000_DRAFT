/*
 * File:        output/java/sdk/com/topas/utmc/traffic_signal/TrafficSignalTypeEnumEnum.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.traffic_signal;

import java.io.IOException;
import com.fasterxml.jackson.annotation.*;

/**
 * Type of the traffic signal.
 *
 * PDF §4.2 — traffic-signal types. XSD values (camelCase) are authoritative.
 */
public enum TrafficSignalTypeEnumEnum {
    INTERSECTION, OTHER, PEGASUS, PELICAN, PUFFIN, TOUCAN, TRAM_CROSSING, WIG_WAG;

    @JsonValue
    public String toValue() {
        switch (this) {
            case INTERSECTION: return "intersection";
            case OTHER: return "other";
            case PEGASUS: return "pegasus";
            case PELICAN: return "pelican";
            case PUFFIN: return "puffin";
            case TOUCAN: return "toucan";
            case TRAM_CROSSING: return "tramCrossing";
            case WIG_WAG: return "wigWag";
        }
        return null;
    }

    @JsonCreator
    public static TrafficSignalTypeEnumEnum forValue(String value) throws IOException {
        if (value.equals("intersection")) return INTERSECTION;
        if (value.equals("other")) return OTHER;
        if (value.equals("pegasus")) return PEGASUS;
        if (value.equals("pelican")) return PELICAN;
        if (value.equals("puffin")) return PUFFIN;
        if (value.equals("toucan")) return TOUCAN;
        if (value.equals("tramCrossing")) return TRAM_CROSSING;
        if (value.equals("wigWag")) return WIG_WAG;
        throw new IOException("Cannot deserialize TrafficSignalTypeEnumEnum");
    }
}
