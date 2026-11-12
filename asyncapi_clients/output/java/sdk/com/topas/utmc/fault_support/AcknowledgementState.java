/*
 * File:        output/java/sdk/com/topas/utmc/fault_support/AcknowledgementState.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.fault_support;

import java.io.IOException;
import com.fasterxml.jackson.annotation.*;

/**
 * Acknowledgement state of a device fault.
 *
 * Maps from legacy XSD integer values:
 * - 0 -> notAcknowledged
 * - 1 -> acknowledged
 * - 2 -> deferred
 * - 3 -> recordedForAction
 */
public enum AcknowledgementState {
    ACKNOWLEDGED, DEFERRED, NOT_ACKNOWLEDGED, RECORDED_FOR_ACTION;

    @JsonValue
    public String toValue() {
        switch (this) {
            case ACKNOWLEDGED: return "acknowledged";
            case DEFERRED: return "deferred";
            case NOT_ACKNOWLEDGED: return "notAcknowledged";
            case RECORDED_FOR_ACTION: return "recordedForAction";
        }
        return null;
    }

    @JsonCreator
    public static AcknowledgementState forValue(String value) throws IOException {
        if (value.equals("acknowledged")) return ACKNOWLEDGED;
        if (value.equals("deferred")) return DEFERRED;
        if (value.equals("notAcknowledged")) return NOT_ACKNOWLEDGED;
        if (value.equals("recordedForAction")) return RECORDED_FOR_ACTION;
        throw new IOException("Cannot deserialize AcknowledgementState");
    }
}
