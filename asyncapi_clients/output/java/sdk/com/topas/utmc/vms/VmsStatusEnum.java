/*
 * File:        output/java/sdk/com/topas/utmc/vms/VmsStatusEnum.java
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
 * Operational status.
 *
 * Operational status of a variable message sign.
 */
public enum VmsStatusEnum {
    BLANK, DISPLAYING, FAULT, OFF, ON, UNKNOWN;

    @JsonValue
    public String toValue() {
        switch (this) {
            case BLANK: return "blank";
            case DISPLAYING: return "displaying";
            case FAULT: return "fault";
            case OFF: return "off";
            case ON: return "on";
            case UNKNOWN: return "unknown";
        }
        return null;
    }

    @JsonCreator
    public static VmsStatusEnum forValue(String value) throws IOException {
        if (value.equals("blank")) return BLANK;
        if (value.equals("displaying")) return DISPLAYING;
        if (value.equals("fault")) return FAULT;
        if (value.equals("off")) return OFF;
        if (value.equals("on")) return ON;
        if (value.equals("unknown")) return UNKNOWN;
        throw new IOException("Cannot deserialize VmsStatusEnum");
    }
}
