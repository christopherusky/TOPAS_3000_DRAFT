/*
 * File:        output/java/sdk/com/topas/utmc/transport_route/TransportRouteConfiguration.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.transport_route;

import com.fasterxml.jackson.annotation.*;
import java.time.OffsetDateTime;
import java.util.Map;

/**
 * Transport route configuration and segment mapping.
 */
public class TransportRouteConfiguration {
    private OffsetDateTime configurationDate;
    private Map<String, Object> extensions;
    private String routeEnd;
    private Double routeLengthMeters;
    private String routeName;
    private String routeStart;
    private String routeVia;
    private String transportRouteID;

    /**
     * When this configuration became effective.
     */
    @JsonProperty("configurationDate")
    public OffsetDateTime getConfigurationDate() { return configurationDate; }
    @JsonProperty("configurationDate")
    public void setConfigurationDate(OffsetDateTime value) { this.configurationDate = value; }

    /**
     * Vendor or local extensions.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * Route end description.
     */
    @JsonProperty("routeEnd")
    public String getRouteEnd() { return routeEnd; }
    @JsonProperty("routeEnd")
    public void setRouteEnd(String value) { this.routeEnd = value; }

    /**
     * Route length in meters.
     */
    @JsonProperty("routeLengthMeters")
    public Double getRouteLengthMeters() { return routeLengthMeters; }
    @JsonProperty("routeLengthMeters")
    public void setRouteLengthMeters(Double value) { this.routeLengthMeters = value; }

    /**
     * Human-readable route name.
     */
    @JsonProperty("routeName")
    public String getRouteName() { return routeName; }
    @JsonProperty("routeName")
    public void setRouteName(String value) { this.routeName = value; }

    /**
     * Route start description.
     */
    @JsonProperty("routeStart")
    public String getRouteStart() { return routeStart; }
    @JsonProperty("routeStart")
    public void setRouteStart(String value) { this.routeStart = value; }

    /**
     * Via point description.
     */
    @JsonProperty("routeVia")
    public String getRouteVia() { return routeVia; }
    @JsonProperty("routeVia")
    public void setRouteVia(String value) { this.routeVia = value; }

    /**
     * Route this configuration applies to.
     */
    @JsonProperty("transportRouteId")
    public String getTransportRouteID() { return transportRouteID; }
    @JsonProperty("transportRouteId")
    public void setTransportRouteID(String value) { this.transportRouteID = value; }
}
