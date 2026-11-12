/*
 * File:        output/java/sdk/com/topas/utmc/vms/VmsColourCapability.java
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

/**
 * Colour capability of a variable message sign.
 *
 * Colour capability.
 */
public enum VmsColourCapability {
    AMBER, FULL_COLOUR, MONOCHROME, MULTI_COLOUR;

    @JsonValue
    public String toValue() {
        switch (this) {
            case AMBER: return "amber";
            case FULL_COLOUR: return "fullColour";
            case MONOCHROME: return "monochrome";
            case MULTI_COLOUR: return "multiColour";
        }
        return null;
    }

    @JsonCreator
    public static VmsColourCapability forValue(String value) throws IOException {
        if (value.equals("amber")) return AMBER;
        if (value.equals("fullColour")) return FULL_COLOUR;
        if (value.equals("monochrome")) return MONOCHROME;
        if (value.equals("multiColour")) return MULTI_COLOUR;
        throw new IOException("Cannot deserialize VmsColourCapability");
    }
}
