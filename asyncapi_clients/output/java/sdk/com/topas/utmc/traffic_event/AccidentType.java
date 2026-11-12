/*
 * File:        output/java/sdk/com/topas/utmc/traffic_event/AccidentType.java
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
 * Classification of the accident.
 *
 * Classification of accident (20 values).
 *
 * 18 of the 20 named XSD values, plus ``other``, plus ``overturnedTrailer``,
 * which has no XSD code. XSD codes 5 ("Accident History") and 20 ("Vehicle
 * spun around") are not carried.
 */
public enum AccidentType {
    ACCIDENT, BUS_ACCIDENT, CHEMICAL_SPILLAGE, COLLISION, FUEL_SPILLAGE, HAZARDOUS_MATERIALS, HEAVY_LORRY, INVESTIGATION_WORK, JACK_KNIFED_ARTICULATED_LORRY, JACK_KNIFED_CARAVAN, JACK_KNIFED_TRAILER, MULTI_VEHICLE, OIL_SPILLAGE, OTHER, OVERTURNED_HEAVY_LORRY, OVERTURNED_TRAILER, OVERTURNED_VEHICLE, SECONDARY_ACCIDENT, SERIOUS_ACCIDENT, SHED_LOAD;

    @JsonValue
    public String toValue() {
        switch (this) {
            case ACCIDENT: return "accident";
            case BUS_ACCIDENT: return "busAccident";
            case CHEMICAL_SPILLAGE: return "chemicalSpillage";
            case COLLISION: return "collision";
            case FUEL_SPILLAGE: return "fuelSpillage";
            case HAZARDOUS_MATERIALS: return "hazardousMaterials";
            case HEAVY_LORRY: return "heavyLorry";
            case INVESTIGATION_WORK: return "investigationWork";
            case JACK_KNIFED_ARTICULATED_LORRY: return "jackKnifedArticulatedLorry";
            case JACK_KNIFED_CARAVAN: return "jackKnifedCaravan";
            case JACK_KNIFED_TRAILER: return "jackKnifedTrailer";
            case MULTI_VEHICLE: return "multiVehicle";
            case OIL_SPILLAGE: return "oilSpillage";
            case OTHER: return "other";
            case OVERTURNED_HEAVY_LORRY: return "overturnedHeavyLorry";
            case OVERTURNED_TRAILER: return "overturnedTrailer";
            case OVERTURNED_VEHICLE: return "overturnedVehicle";
            case SECONDARY_ACCIDENT: return "secondaryAccident";
            case SERIOUS_ACCIDENT: return "seriousAccident";
            case SHED_LOAD: return "shedLoad";
        }
        return null;
    }

    @JsonCreator
    public static AccidentType forValue(String value) throws IOException {
        if (value.equals("accident")) return ACCIDENT;
        if (value.equals("busAccident")) return BUS_ACCIDENT;
        if (value.equals("chemicalSpillage")) return CHEMICAL_SPILLAGE;
        if (value.equals("collision")) return COLLISION;
        if (value.equals("fuelSpillage")) return FUEL_SPILLAGE;
        if (value.equals("hazardousMaterials")) return HAZARDOUS_MATERIALS;
        if (value.equals("heavyLorry")) return HEAVY_LORRY;
        if (value.equals("investigationWork")) return INVESTIGATION_WORK;
        if (value.equals("jackKnifedArticulatedLorry")) return JACK_KNIFED_ARTICULATED_LORRY;
        if (value.equals("jackKnifedCaravan")) return JACK_KNIFED_CARAVAN;
        if (value.equals("jackKnifedTrailer")) return JACK_KNIFED_TRAILER;
        if (value.equals("multiVehicle")) return MULTI_VEHICLE;
        if (value.equals("oilSpillage")) return OIL_SPILLAGE;
        if (value.equals("other")) return OTHER;
        if (value.equals("overturnedHeavyLorry")) return OVERTURNED_HEAVY_LORRY;
        if (value.equals("overturnedTrailer")) return OVERTURNED_TRAILER;
        if (value.equals("overturnedVehicle")) return OVERTURNED_VEHICLE;
        if (value.equals("secondaryAccident")) return SECONDARY_ACCIDENT;
        if (value.equals("seriousAccident")) return SERIOUS_ACCIDENT;
        if (value.equals("shedLoad")) return SHED_LOAD;
        throw new IOException("Cannot deserialize AccidentType");
    }
}
