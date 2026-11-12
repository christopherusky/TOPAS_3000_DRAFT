/*
 * File:        output/java/sdk/com/topas/utmc/traffic_event/EventStatusEnum.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.traffic_event;

import java.io.IOException;
import com.fasterxml.jackson.annotation.*;

/**
 * Event lifecycle state.
 */
public enum EventStatusEnum {
    ACTIVE, CLEARED, PLANNED, SUSPENDED;

    @JsonValue
    public String toValue() {
        switch (this) {
            case ACTIVE: return "active";
            case CLEARED: return "cleared";
            case PLANNED: return "planned";
            case SUSPENDED: return "suspended";
        }
        return null;
    }

    @JsonCreator
    public static EventStatusEnum forValue(String value) throws IOException {
        if (value.equals("active")) return ACTIVE;
        if (value.equals("cleared")) return CLEARED;
        if (value.equals("planned")) return PLANNED;
        if (value.equals("suspended")) return SUSPENDED;
        throw new IOException("Cannot deserialize EventStatusEnum");
    }
}
