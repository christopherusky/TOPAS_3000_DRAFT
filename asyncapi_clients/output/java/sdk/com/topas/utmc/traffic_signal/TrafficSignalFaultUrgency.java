/*
 * File:        output/java/sdk/com/topas/utmc/traffic_signal/TrafficSignalFaultUrgency.java
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
 * The urgency of this fault.
 *
 * DATEX II UrgencyEnum — inherited by TrafficSignalFault.trafficSignalFaultUrgency (PDF
 * §3.3).
 *
 * ``UNKNOWN`` is a deliberate addition to the DATEX II baseline: a
 * publisher that cannot determine urgency must be able to say so rather
 * than defaulting to ``normalUrgency`` and understating a fault. This
 * mirrors ``FaultSeverityEnum.UNKNOWN``, which already carries the value.
 */
public enum TrafficSignalFaultUrgency {
    EXTREMELY_URGENT, NORMAL_URGENCY, UNKNOWN, URGENT;

    @JsonValue
    public String toValue() {
        switch (this) {
            case EXTREMELY_URGENT: return "extremelyUrgent";
            case NORMAL_URGENCY: return "normalUrgency";
            case UNKNOWN: return "unknown";
            case URGENT: return "urgent";
        }
        return null;
    }

    @JsonCreator
    public static TrafficSignalFaultUrgency forValue(String value) throws IOException {
        if (value.equals("extremelyUrgent")) return EXTREMELY_URGENT;
        if (value.equals("normalUrgency")) return NORMAL_URGENCY;
        if (value.equals("unknown")) return UNKNOWN;
        if (value.equals("urgent")) return URGENT;
        throw new IOException("Cannot deserialize TrafficSignalFaultUrgency");
    }
}
