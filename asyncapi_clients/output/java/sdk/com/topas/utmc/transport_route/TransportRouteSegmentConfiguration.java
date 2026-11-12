/*
 * File:        output/java/sdk/com/topas/utmc/transport_route/TransportRouteSegmentConfiguration.java
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
 * Segment configuration — links a route segment to a transport link.
 *
 * Sibling type in the configuration file.
 */
public class TransportRouteSegmentConfiguration {
    private OffsetDateTime configurationDate;
    private Map<String, Object> extensions;
    private long routeSegmentID;
    private String transportLinkID;
    private String transportRouteID;

    /**
     * When effective.
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
     * Ordered segment identifier within the route.
     */
    @JsonProperty("routeSegmentId")
    public long getRouteSegmentID() { return routeSegmentID; }
    @JsonProperty("routeSegmentId")
    public void setRouteSegmentID(long value) { this.routeSegmentID = value; }

    /**
     * Cross-domain reference to a transport link.
     */
    @JsonProperty("transportLinkId")
    public String getTransportLinkID() { return transportLinkID; }
    @JsonProperty("transportLinkId")
    public void setTransportLinkID(String value) { this.transportLinkID = value; }

    /**
     * Route this segment belongs to.
     */
    @JsonProperty("transportRouteId")
    public String getTransportRouteID() { return transportRouteID; }
    @JsonProperty("transportRouteId")
    public void setTransportRouteID(String value) { this.transportRouteID = value; }
}
