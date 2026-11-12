/*
 * File:        output/java/sdk/com/topas/utmc/traffic_signal/Location.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.traffic_signal;

import com.fasterxml.jackson.annotation.*;

/**
 * Location of traffic signal.
 *
 * A GeoJSON Point geometry (RFC 7946).
 *
 * Coordinates are [longitude, latitude] per GeoJSON specification.
 */
public class Location {
    private Object[] coordinates;
    private Type type;

    /**
     * [longitude, latitude] in WGS84 decimal degrees.
     */
    @JsonProperty("coordinates")
    public Object[] getCoordinates() { return coordinates; }
    @JsonProperty("coordinates")
    public void setCoordinates(Object[] value) { this.coordinates = value; }

    @JsonProperty("type")
    public Type getType() { return type; }
    @JsonProperty("type")
    public void setType(Type value) { this.type = value; }
}
