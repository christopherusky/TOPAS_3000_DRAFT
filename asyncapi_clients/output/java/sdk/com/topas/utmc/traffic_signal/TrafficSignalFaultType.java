/*
 * File:        output/java/sdk/com/topas/utmc/traffic_signal/TrafficSignalFaultType.java
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
 * Coarse DATEX II fault bucket, as in TS004.0064bis:2017. Required unless faultTypeId is
 * given; when both are present faultTypeId is the authoritative classification.
 *
 * PDF §4.1 — traffic-signal fault types.
 */
public enum TrafficSignalFaultType {
    DETECTOR_FAULT, ELECTRONICS_FAULT, INDICATOR_FAULT, LAMP_FAULT, MECHANICAL_FAULT, OTHER, UTILITY_FAULT;

    @JsonValue
    public String toValue() {
        switch (this) {
            case DETECTOR_FAULT: return "detectorFault";
            case ELECTRONICS_FAULT: return "electronicsFault";
            case INDICATOR_FAULT: return "indicatorFault";
            case LAMP_FAULT: return "lampFault";
            case MECHANICAL_FAULT: return "mechanicalFault";
            case OTHER: return "other";
            case UTILITY_FAULT: return "utilityFault";
        }
        return null;
    }

    @JsonCreator
    public static TrafficSignalFaultType forValue(String value) throws IOException {
        if (value.equals("detectorFault")) return DETECTOR_FAULT;
        if (value.equals("electronicsFault")) return ELECTRONICS_FAULT;
        if (value.equals("indicatorFault")) return INDICATOR_FAULT;
        if (value.equals("lampFault")) return LAMP_FAULT;
        if (value.equals("mechanicalFault")) return MECHANICAL_FAULT;
        if (value.equals("other")) return OTHER;
        if (value.equals("utilityFault")) return UTILITY_FAULT;
        throw new IOException("Cannot deserialize TrafficSignalFaultType");
    }
}
