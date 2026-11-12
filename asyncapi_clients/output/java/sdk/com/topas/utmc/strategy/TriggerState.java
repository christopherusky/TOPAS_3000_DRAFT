/*
 * File:        output/java/sdk/com/topas/utmc/strategy/TriggerState.java
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
 * Desired trigger state (enabled/disabled).
 *
 * Desired state when triggering a strategy.
 */
public enum TriggerState {
    DISABLED, ENABLED;

    @JsonValue
    public String toValue() {
        switch (this) {
            case DISABLED: return "disabled";
            case ENABLED: return "enabled";
        }
        return null;
    }

    @JsonCreator
    public static TriggerState forValue(String value) throws IOException {
        if (value.equals("disabled")) return DISABLED;
        if (value.equals("enabled")) return ENABLED;
        throw new IOException("Cannot deserialize TriggerState");
    }
}
