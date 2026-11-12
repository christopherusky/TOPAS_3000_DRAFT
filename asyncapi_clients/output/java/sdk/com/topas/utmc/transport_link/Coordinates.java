/*
 * File:        output/java/sdk/com/topas/utmc/transport_link/Coordinates.java
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
 * WGS84 coordinates.
 *
 * A WGS84 geographic coordinate pair (longitude, latitude).
 */
public class Coordinates {
    private double latitude;
    private double longitude;

    /**
     * Latitude in decimal degrees (WGS84).
     */
    @JsonProperty("latitude")
    public double getLatitude() { return latitude; }
    @JsonProperty("latitude")
    public void setLatitude(double value) { this.latitude = value; }

    /**
     * Longitude in decimal degrees (WGS84).
     */
    @JsonProperty("longitude")
    public double getLongitude() { return longitude; }
    @JsonProperty("longitude")
    public void setLongitude(double value) { this.longitude = value; }
}
