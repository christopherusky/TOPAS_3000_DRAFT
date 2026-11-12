/*
 * File:        output/java/sdk/com/topas/utmc/traffic_event/LanesAffected.java
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
 * Lane closure pattern (I=open, T=closed).
 *
 * Lane closure pattern.
 *
 * I = open lane, T = closed lane, read nearside to offside.
 *
 * Lane closure pattern.
 */
public enum LanesAffected {
    ALL_OPEN, I, II, III, IIII, IIIT, IIT, IITT, IT, ITI, ITT, ITTT, T, TI, TII, TIII, TIT, TT, TTI, TTT, TTTT;

    @JsonValue
    public String toValue() {
        switch (this) {
            case ALL_OPEN: return "allOpen";
            case I: return "I";
            case II: return "II";
            case III: return "III";
            case IIII: return "IIII";
            case IIIT: return "IIIT";
            case IIT: return "IIT";
            case IITT: return "IITT";
            case IT: return "IT";
            case ITI: return "ITI";
            case ITT: return "ITT";
            case ITTT: return "ITTT";
            case T: return "T";
            case TI: return "TI";
            case TII: return "TII";
            case TIII: return "TIII";
            case TIT: return "TIT";
            case TT: return "TT";
            case TTI: return "TTI";
            case TTT: return "TTT";
            case TTTT: return "TTTT";
        }
        return null;
    }

    @JsonCreator
    public static LanesAffected forValue(String value) throws IOException {
        if (value.equals("allOpen")) return ALL_OPEN;
        if (value.equals("I")) return I;
        if (value.equals("II")) return II;
        if (value.equals("III")) return III;
        if (value.equals("IIII")) return IIII;
        if (value.equals("IIIT")) return IIIT;
        if (value.equals("IIT")) return IIT;
        if (value.equals("IITT")) return IITT;
        if (value.equals("IT")) return IT;
        if (value.equals("ITI")) return ITI;
        if (value.equals("ITT")) return ITT;
        if (value.equals("ITTT")) return ITTT;
        if (value.equals("T")) return T;
        if (value.equals("TI")) return TI;
        if (value.equals("TII")) return TII;
        if (value.equals("TIII")) return TIII;
        if (value.equals("TIT")) return TIT;
        if (value.equals("TT")) return TT;
        if (value.equals("TTI")) return TTI;
        if (value.equals("TTT")) return TTT;
        if (value.equals("TTTT")) return TTTT;
        throw new IOException("Cannot deserialize LanesAffected");
    }
}
