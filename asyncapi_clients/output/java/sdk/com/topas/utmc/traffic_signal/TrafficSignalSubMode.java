/*
 * File:        output/java/sdk/com/topas/utmc/traffic_signal/TrafficSignalSubMode.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.traffic_signal;

import java.io.IOException;
import com.fasterxml.jackson.annotation.*;

/**
 * The detailed sub-mode of the traffic signal.
 *
 * PDF §4.4 — detailed traffic-signal sub-modes.
 *
 * Note: §5.1's mode/subMode combinations table references additional values
 * (hurryCall, cablelessLinkingBaseTime, cablelessLinkingNonBaseTime,
 * publicServiceVehiclePriority, lightRailTransitMode, pre-emption) that are
 * NOT defined in §4.4 or the XSD. This enum follows §4.4 + XSD.
 */
public enum TrafficSignalSubMode {
    CABLELESS_LINKING, FIXED_MODE_SIMPLE_OR_LINKED, FIXED_MODE_TO_CURRENT_MAX, MANUAL_MODE, MANUAL_STEP_ON, MOVA_MODE, OTHER_DISTRIBUTED_MODE, PRE_EMPT_LEVEL1, PRE_EMPT_LEVEL2, PRE_EMPT_LEVEL3, UTC_FIXED_TIME, UTC_OTHER_CENTRAL_CONTROL_METHOD, UTC_SCOOT, VEHICLE_ACTUATED, WARDEN_CONTROL;

    @JsonValue
    public String toValue() {
        switch (this) {
            case CABLELESS_LINKING: return "cablelessLinking";
            case FIXED_MODE_SIMPLE_OR_LINKED: return "fixedModeSimpleOrLinked";
            case FIXED_MODE_TO_CURRENT_MAX: return "fixedModeToCurrentMax";
            case MANUAL_MODE: return "manualMode";
            case MANUAL_STEP_ON: return "manualStepOn";
            case MOVA_MODE: return "movaMode";
            case OTHER_DISTRIBUTED_MODE: return "otherDistributedMode";
            case PRE_EMPT_LEVEL1: return "preEmptLevel1";
            case PRE_EMPT_LEVEL2: return "preEmptLevel2";
            case PRE_EMPT_LEVEL3: return "preEmptLevel3";
            case UTC_FIXED_TIME: return "utcFixedTime";
            case UTC_OTHER_CENTRAL_CONTROL_METHOD: return "utcOtherCentralControlMethod";
            case UTC_SCOOT: return "utcScoot";
            case VEHICLE_ACTUATED: return "vehicleActuated";
            case WARDEN_CONTROL: return "wardenControl";
        }
        return null;
    }

    @JsonCreator
    public static TrafficSignalSubMode forValue(String value) throws IOException {
        if (value.equals("cablelessLinking")) return CABLELESS_LINKING;
        if (value.equals("fixedModeSimpleOrLinked")) return FIXED_MODE_SIMPLE_OR_LINKED;
        if (value.equals("fixedModeToCurrentMax")) return FIXED_MODE_TO_CURRENT_MAX;
        if (value.equals("manualMode")) return MANUAL_MODE;
        if (value.equals("manualStepOn")) return MANUAL_STEP_ON;
        if (value.equals("movaMode")) return MOVA_MODE;
        if (value.equals("otherDistributedMode")) return OTHER_DISTRIBUTED_MODE;
        if (value.equals("preEmptLevel1")) return PRE_EMPT_LEVEL1;
        if (value.equals("preEmptLevel2")) return PRE_EMPT_LEVEL2;
        if (value.equals("preEmptLevel3")) return PRE_EMPT_LEVEL3;
        if (value.equals("utcFixedTime")) return UTC_FIXED_TIME;
        if (value.equals("utcOtherCentralControlMethod")) return UTC_OTHER_CENTRAL_CONTROL_METHOD;
        if (value.equals("utcScoot")) return UTC_SCOOT;
        if (value.equals("vehicleActuated")) return VEHICLE_ACTUATED;
        if (value.equals("wardenControl")) return WARDEN_CONTROL;
        throw new IOException("Cannot deserialize TrafficSignalSubMode");
    }
}
