/*
 * File:        output/java/sdk/com/topas/utmc/network/GeometryType.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.network;

import java.io.IOException;
import com.fasterxml.jackson.annotation.*;

public enum GeometryType {
    LINE_STRING;

    @JsonValue
    public String toValue() {
        switch (this) {
            case LINE_STRING: return "LineString";
        }
        return null;
    }

    @JsonCreator
    public static GeometryType forValue(String value) throws IOException {
        if (value.equals("LineString")) return LINE_STRING;
        throw new IOException("Cannot deserialize GeometryType");
    }
}
