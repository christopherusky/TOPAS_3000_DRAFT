/*
 * File:        output/java/sdk/com/topas/utmc/vms/VmsFaultStatus.java
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
 * Whether this fault is still active or has been cleared.
 *
 * Whether a fault is still active or has been cleared.
 *
 * This is the only fault lifecycle the payload carries. Record archival
 * — distinguishing a live record from a retired one — is not modelled
 * here and is a matter for the publishing system.
 */
public enum VmsFaultStatus {
    ACTIVE, CLEARED;

    @JsonValue
    public String toValue() {
        switch (this) {
            case ACTIVE: return "active";
            case CLEARED: return "cleared";
        }
        return null;
    }

    @JsonCreator
    public static VmsFaultStatus forValue(String value) throws IOException {
        if (value.equals("active")) return ACTIVE;
        if (value.equals("cleared")) return CLEARED;
        throw new IOException("Cannot deserialize VmsFaultStatus");
    }
}
