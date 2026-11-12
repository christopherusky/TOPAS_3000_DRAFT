/*
 * File:        output/java/sdk/com/topas/utmc/traffic_signal/TrafficSignalMode.java
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
 * The high level mode of the traffic signal.
 *
 * PDF §4.3 — high-level traffic-signal modes.
 */
public enum TrafficSignalMode {
    CENTRAL_MODE, LOCAL_MODE, SIGNALS_OFF_MODE, START_UP_MODE, UNKNOWN_MODE;

    @JsonValue
    public String toValue() {
        switch (this) {
            case CENTRAL_MODE: return "centralMode";
            case LOCAL_MODE: return "localMode";
            case SIGNALS_OFF_MODE: return "signalsOffMode";
            case START_UP_MODE: return "startUpMode";
            case UNKNOWN_MODE: return "unknownMode";
        }
        return null;
    }

    @JsonCreator
    public static TrafficSignalMode forValue(String value) throws IOException {
        if (value.equals("centralMode")) return CENTRAL_MODE;
        if (value.equals("localMode")) return LOCAL_MODE;
        if (value.equals("signalsOffMode")) return SIGNALS_OFF_MODE;
        if (value.equals("startUpMode")) return START_UP_MODE;
        if (value.equals("unknownMode")) return UNKNOWN_MODE;
        throw new IOException("Cannot deserialize TrafficSignalMode");
    }
}
