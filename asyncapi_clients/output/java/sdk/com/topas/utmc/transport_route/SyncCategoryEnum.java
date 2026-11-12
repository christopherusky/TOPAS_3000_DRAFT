/*
 * File:        output/java/sdk/com/topas/utmc/transport_route/SyncCategoryEnum.java
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
 * Data categories a sync may be restricted to.
 *
 * Mirrors the schema-per-category pattern: a subscriber that only holds
 * a fault dashboard asks for ``fault`` alone rather than provoking a
 * full republication of every definition and configuration it already
 * has.
 */
public enum SyncCategoryEnum {
    CONFIGURATION, DEFINITION, FAULT, MEASUREMENT, STATUS, TYPE;

    @JsonValue
    public String toValue() {
        switch (this) {
            case CONFIGURATION: return "configuration";
            case DEFINITION: return "definition";
            case FAULT: return "fault";
            case MEASUREMENT: return "measurement";
            case STATUS: return "status";
            case TYPE: return "type";
        }
        return null;
    }

    @JsonCreator
    public static SyncCategoryEnum forValue(String value) throws IOException {
        if (value.equals("configuration")) return CONFIGURATION;
        if (value.equals("definition")) return DEFINITION;
        if (value.equals("fault")) return FAULT;
        if (value.equals("measurement")) return MEASUREMENT;
        if (value.equals("status")) return STATUS;
        if (value.equals("type")) return TYPE;
        throw new IOException("Cannot deserialize SyncCategoryEnum");
    }
}
