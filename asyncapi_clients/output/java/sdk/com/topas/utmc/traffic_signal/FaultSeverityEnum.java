/*
 * File:        output/java/sdk/com/topas/utmc/traffic_signal/FaultSeverityEnum.java
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
 * DATEX II FaultSeverityEnum — inherited via Fault base (PDF §3.3).
 *
 * Severity of the fault.
 */
public enum FaultSeverityEnum {
    HIGH, LOW, MEDIUM, UNKNOWN;

    @JsonValue
    public String toValue() {
        switch (this) {
            case HIGH: return "high";
            case LOW: return "low";
            case MEDIUM: return "medium";
            case UNKNOWN: return "unknown";
        }
        return null;
    }

    @JsonCreator
    public static FaultSeverityEnum forValue(String value) throws IOException {
        if (value.equals("high")) return HIGH;
        if (value.equals("low")) return LOW;
        if (value.equals("medium")) return MEDIUM;
        if (value.equals("unknown")) return UNKNOWN;
        throw new IOException("Cannot deserialize FaultSeverityEnum");
    }
}
