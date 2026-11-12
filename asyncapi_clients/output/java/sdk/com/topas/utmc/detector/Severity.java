/*
 * File:        output/java/sdk/com/topas/utmc/detector/Severity.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.detector;

import java.io.IOException;
import com.fasterxml.jackson.annotation.*;

/**
 * Queue severity when a queue is present.
 */
public enum Severity {
    CONGESTED, DELAYED, DENSE, NORMAL, SEVERE, STOP_AND_GO;

    @JsonValue
    public String toValue() {
        switch (this) {
            case CONGESTED: return "congested";
            case DELAYED: return "delayed";
            case DENSE: return "dense";
            case NORMAL: return "normal";
            case SEVERE: return "severe";
            case STOP_AND_GO: return "stopAndGo";
        }
        return null;
    }

    @JsonCreator
    public static Severity forValue(String value) throws IOException {
        if (value.equals("congested")) return CONGESTED;
        if (value.equals("delayed")) return DELAYED;
        if (value.equals("dense")) return DENSE;
        if (value.equals("normal")) return NORMAL;
        if (value.equals("severe")) return SEVERE;
        if (value.equals("stopAndGo")) return STOP_AND_GO;
        throw new IOException("Cannot deserialize Severity");
    }
}
