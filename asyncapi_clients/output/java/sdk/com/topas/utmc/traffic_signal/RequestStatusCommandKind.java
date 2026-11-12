/*
 * File:        output/java/sdk/com/topas/utmc/traffic_signal/RequestStatusCommandKind.java
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

public enum RequestStatusCommandKind {
    REQUEST_STATUS;

    @JsonValue
    public String toValue() {
        switch (this) {
            case REQUEST_STATUS: return "requestStatus";
        }
        return null;
    }

    @JsonCreator
    public static RequestStatusCommandKind forValue(String value) throws IOException {
        if (value.equals("requestStatus")) return REQUEST_STATUS;
        throw new IOException("Cannot deserialize RequestStatusCommandKind");
    }
}
