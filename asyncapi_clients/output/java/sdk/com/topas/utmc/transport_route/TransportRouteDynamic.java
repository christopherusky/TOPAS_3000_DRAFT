/*
 * File:        output/java/sdk/com/topas/utmc/transport_route/TransportRouteDynamic.java
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
 * Transport route dynamic measurement data.
 */
public class TransportRouteDynamic {
    private Double currentJourneyTimeSeconds;
    private Double estimatedSpeedKph;
    private Map<String, Object> extensions;
    private Quality quality;
    private RouteStatus routeStatus;
    private OffsetDateTime timestamp;
    private String transportRouteID;

    /**
     * Current journey time in seconds.
     */
    @JsonProperty("currentJourneyTimeSeconds")
    public Double getCurrentJourneyTimeSeconds() { return currentJourneyTimeSeconds; }
    @JsonProperty("currentJourneyTimeSeconds")
    public void setCurrentJourneyTimeSeconds(Double value) { this.currentJourneyTimeSeconds = value; }

    /**
     * Estimated speed along the route in km/h.
     */
    @JsonProperty("estimatedSpeedKph")
    public Double getEstimatedSpeedKph() { return estimatedSpeedKph; }
    @JsonProperty("estimatedSpeedKph")
    public void setEstimatedSpeedKph(Double value) { this.estimatedSpeedKph = value; }

    /**
     * Vendor or local extensions.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * Optional quality statement.
     */
    @JsonProperty("quality")
    public Quality getQuality() { return quality; }
    @JsonProperty("quality")
    public void setQuality(Quality value) { this.quality = value; }

    /**
     * Route status (inRange/outOfRange).
     */
    @JsonProperty("routeStatus")
    public RouteStatus getRouteStatus() { return routeStatus; }
    @JsonProperty("routeStatus")
    public void setRouteStatus(RouteStatus value) { this.routeStatus = value; }

    /**
     * Time of the observation.
     */
    @JsonProperty("timestamp")
    public OffsetDateTime getTimestamp() { return timestamp; }
    @JsonProperty("timestamp")
    public void setTimestamp(OffsetDateTime value) { this.timestamp = value; }

    /**
     * Identifier of the route.
     */
    @JsonProperty("transportRouteId")
    public String getTransportRouteID() { return transportRouteID; }
    @JsonProperty("transportRouteId")
    public void setTransportRouteID(String value) { this.transportRouteID = value; }
}
