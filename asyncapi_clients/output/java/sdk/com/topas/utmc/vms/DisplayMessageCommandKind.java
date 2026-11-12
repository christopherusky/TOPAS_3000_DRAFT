/*
 * File:        output/java/sdk/com/topas/utmc/vms/DisplayMessageCommandKind.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.vms;

import java.io.IOException;
import com.fasterxml.jackson.annotation.*;

public enum DisplayMessageCommandKind {
    DISPLAY_MESSAGE;

    @JsonValue
    public String toValue() {
        switch (this) {
            case DISPLAY_MESSAGE: return "displayMessage";
        }
        return null;
    }

    @JsonCreator
    public static DisplayMessageCommandKind forValue(String value) throws IOException {
        if (value.equals("displayMessage")) return DISPLAY_MESSAGE;
        throw new IOException("Cannot deserialize DisplayMessageCommandKind");
    }
}
