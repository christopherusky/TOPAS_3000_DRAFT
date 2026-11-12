/*
 * File:        output/java/sdk/com/topas/utmc/fault_support/FaultCategory.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.fault_support;

import java.io.IOException;
import com.fasterxml.jackson.annotation.*;

/**
 * Coarse grouping for catalogue entries.
 *
 * Deliberately short. It exists so an operator can filter "show me the
 * communications faults" without the catalogue having to agree on a
 * taxonomy first; the precise meaning of a fault is its catalogue entry,
 * not its category.
 *
 * Coarse grouping, for filtering rather than semantics.
 */
public enum FaultCategory {
    COMMUNICATIONS, ENVIRONMENTAL, HARDWARE, OTHER, SIGNAL_TIMINGS;

    @JsonValue
    public String toValue() {
        switch (this) {
            case COMMUNICATIONS: return "communications";
            case ENVIRONMENTAL: return "environmental";
            case HARDWARE: return "hardware";
            case OTHER: return "other";
            case SIGNAL_TIMINGS: return "signalTimings";
        }
        return null;
    }

    @JsonCreator
    public static FaultCategory forValue(String value) throws IOException {
        if (value.equals("communications")) return COMMUNICATIONS;
        if (value.equals("environmental")) return ENVIRONMENTAL;
        if (value.equals("hardware")) return HARDWARE;
        if (value.equals("other")) return OTHER;
        if (value.equals("signalTimings")) return SIGNAL_TIMINGS;
        throw new IOException("Cannot deserialize FaultCategory");
    }
}
