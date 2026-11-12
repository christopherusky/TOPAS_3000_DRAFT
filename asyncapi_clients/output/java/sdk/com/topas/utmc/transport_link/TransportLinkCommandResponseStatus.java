/*
 * File:        output/java/sdk/com/topas/utmc/transport_link/TransportLinkCommandResponseStatus.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.transport_link;

import java.io.IOException;
import com.fasterxml.jackson.annotation.*;

/**
 * Outcome of the command execution.
 *
 * Outcome of a command execution.
 *
 * Every value except ``PENDING`` is terminal. Consumers SHOULD test for
 * "not pending" rather than enumerating the terminal values, so that a
 * future profile adding one does not strand them.
 */
public enum TransportLinkCommandResponseStatus {
    FAILED, INVALID, NOT_SUPPORTED, PENDING, SUCCESS, TIMED_OUT;

    @JsonValue
    public String toValue() {
        switch (this) {
            case FAILED: return "failed";
            case INVALID: return "invalid";
            case NOT_SUPPORTED: return "notSupported";
            case PENDING: return "pending";
            case SUCCESS: return "success";
            case TIMED_OUT: return "timedOut";
        }
        return null;
    }

    @JsonCreator
    public static TransportLinkCommandResponseStatus forValue(String value) throws IOException {
        if (value.equals("failed")) return FAILED;
        if (value.equals("invalid")) return INVALID;
        if (value.equals("notSupported")) return NOT_SUPPORTED;
        if (value.equals("pending")) return PENDING;
        if (value.equals("success")) return SUCCESS;
        if (value.equals("timedOut")) return TIMED_OUT;
        throw new IOException("Cannot deserialize TransportLinkCommandResponseStatus");
    }
}
