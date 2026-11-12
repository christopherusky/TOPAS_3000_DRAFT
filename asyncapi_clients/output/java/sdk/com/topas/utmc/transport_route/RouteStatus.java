/*
 * File:        output/java/sdk/com/topas/utmc/transport_route/RouteStatus.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.transport_route;

import java.io.IOException;
import com.fasterxml.jackson.annotation.*;

/**
 * Route status (inRange/outOfRange).
 *
 * Route status (a closed string enum, not an extensible TypeId catalogue).
 */
public enum RouteStatus {
    IN_RANGE, OUT_OF_RANGE;

    @JsonValue
    public String toValue() {
        switch (this) {
            case IN_RANGE: return "inRange";
            case OUT_OF_RANGE: return "outOfRange";
        }
        return null;
    }

    @JsonCreator
    public static RouteStatus forValue(String value) throws IOException {
        if (value.equals("inRange")) return IN_RANGE;
        if (value.equals("outOfRange")) return OUT_OF_RANGE;
        throw new IOException("Cannot deserialize RouteStatus");
    }
}
