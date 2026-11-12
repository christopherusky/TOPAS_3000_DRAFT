/*
 * File:        output/java/sdk/com/topas/utmc/network/Geometry.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.network;

import com.fasterxml.jackson.annotation.*;

/**
 * Polyline shape of the link as GeoJSON LineString.
 *
 * A GeoJSON LineString geometry (RFC 7946).
 *
 * An ordered sequence of two or more positions forming a polyline.
 * Coordinates are [longitude, latitude] per GeoJSON specification.
 *
 * Turn path geometry as GeoJSON LineString.
 *
 * Zone boundary geometry as GeoJSON LineString.
 */
public class Geometry {
    private Object[][] coordinates;
    private GeometryType type;

    /**
     * Ordered [longitude, latitude] pairs forming a polyline.
     */
    @JsonProperty("coordinates")
    public Object[][] getCoordinates() { return coordinates; }
    @JsonProperty("coordinates")
    public void setCoordinates(Object[][] value) { this.coordinates = value; }

    @JsonProperty("type")
    public GeometryType getType() { return type; }
    @JsonProperty("type")
    public void setType(GeometryType value) { this.type = value; }
}
