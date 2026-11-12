/*
 * File:        output/java/sdk/com/topas/utmc/transport_link/TransportLinkGPSConfigurationKind.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.transport_link;

import java.io.IOException;
import com.fasterxml.jackson.annotation.*;

public enum TransportLinkGPSConfigurationKind {
    GPS;

    @JsonValue
    public String toValue() {
        switch (this) {
            case GPS: return "gps";
        }
        return null;
    }

    @JsonCreator
    public static TransportLinkGPSConfigurationKind forValue(String value) throws IOException {
        if (value.equals("gps")) return GPS;
        throw new IOException("Cannot deserialize TransportLinkGPSConfigurationKind");
    }
}
