/*
 * File:        output/java/sdk/com/topas/utmc/detector/Location.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.detector;

import com.fasterxml.jackson.annotation.*;

/**
 * Physical location of the detector. Optional.
 *
 * A geographic point: a coordinate plus optional altitude.
 */
public class Location {
    private Double altitudeMeters;
    private Coordinates coordinates;

    /**
     * Altitude in metres above mean sea level. Optional. Bounds [-1000, 5000] inherited from
     * utmc_location-v1.0.xsd AltitudeType.
     */
    @JsonProperty("altitudeMeters")
    public Double getAltitudeMeters() { return altitudeMeters; }
    @JsonProperty("altitudeMeters")
    public void setAltitudeMeters(Double value) { this.altitudeMeters = value; }

    /**
     * WGS84 coordinates.
     */
    @JsonProperty("coordinates")
    public Coordinates getCoordinates() { return coordinates; }
    @JsonProperty("coordinates")
    public void setCoordinates(Coordinates value) { this.coordinates = value; }
}
