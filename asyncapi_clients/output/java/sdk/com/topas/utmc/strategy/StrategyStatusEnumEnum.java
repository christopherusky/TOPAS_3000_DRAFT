/*
 * File:        output/java/sdk/com/topas/utmc/strategy/StrategyStatusEnumEnum.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.strategy;

import java.io.IOException;
import com.fasterxml.jackson.annotation.*;

/**
 * Current status (active/inactive).
 *
 * Status of a traffic management strategy.
 */
public enum StrategyStatusEnumEnum {
    ACTIVE, INACTIVE;

    @JsonValue
    public String toValue() {
        switch (this) {
            case ACTIVE: return "active";
            case INACTIVE: return "inactive";
        }
        return null;
    }

    @JsonCreator
    public static StrategyStatusEnumEnum forValue(String value) throws IOException {
        if (value.equals("active")) return ACTIVE;
        if (value.equals("inactive")) return INACTIVE;
        throw new IOException("Cannot deserialize StrategyStatusEnumEnum");
    }
}
