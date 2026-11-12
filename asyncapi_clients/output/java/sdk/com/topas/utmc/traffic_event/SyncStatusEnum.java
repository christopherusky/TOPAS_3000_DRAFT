/*
 * File:        output/java/sdk/com/topas/utmc/traffic_event/SyncStatusEnum.java
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
 * Outcome of the sync.
 *
 * Outcome of a sync request (A12).
 */
public enum SyncStatusEnum {
    FAILED, REJECTED, SUCCESS;

    @JsonValue
    public String toValue() {
        switch (this) {
            case FAILED: return "failed";
            case REJECTED: return "rejected";
            case SUCCESS: return "success";
        }
        return null;
    }

    @JsonCreator
    public static SyncStatusEnum forValue(String value) throws IOException {
        if (value.equals("failed")) return FAILED;
        if (value.equals("rejected")) return REJECTED;
        if (value.equals("success")) return SUCCESS;
        throw new IOException("Cannot deserialize SyncStatusEnum");
    }
}
