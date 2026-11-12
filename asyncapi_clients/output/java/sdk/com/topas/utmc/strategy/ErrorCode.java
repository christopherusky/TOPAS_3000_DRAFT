/*
 * File:        output/java/sdk/com/topas/utmc/strategy/ErrorCode.java
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
 * Strategy-specific rejection category. Only permitted when the outcome is not success or
 * pending.
 *
 * Categorisation of strategy trigger rejection reason.
 */
public enum ErrorCode {
    ACCESS_DENIED, NOT_AUTHENTICATED, OTHER, STRATEGY_ID_DOES_NOT_EXIST;

    @JsonValue
    public String toValue() {
        switch (this) {
            case ACCESS_DENIED: return "accessDenied";
            case NOT_AUTHENTICATED: return "notAuthenticated";
            case OTHER: return "other";
            case STRATEGY_ID_DOES_NOT_EXIST: return "strategyIdDoesNotExist";
        }
        return null;
    }

    @JsonCreator
    public static ErrorCode forValue(String value) throws IOException {
        if (value.equals("accessDenied")) return ACCESS_DENIED;
        if (value.equals("notAuthenticated")) return NOT_AUTHENTICATED;
        if (value.equals("other")) return OTHER;
        if (value.equals("strategyIdDoesNotExist")) return STRATEGY_ID_DOES_NOT_EXIST;
        throw new IOException("Cannot deserialize ErrorCode");
    }
}
