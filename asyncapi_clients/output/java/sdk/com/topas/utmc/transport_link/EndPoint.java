/*
 * File:        output/java/sdk/com/topas/utmc/transport_link/EndPoint.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.transport_link;

import com.fasterxml.jackson.annotation.*;

/**
 * End of the transport link as a WGS84 point. Optional here although the v1 XSD requires
 * it, matching transport_route.
 *
 * A geographic point: a coordinate plus optional altitude.
 *
 * Start of the transport link as a WGS84 point. Optional.
 */
public class EndPoint {
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
