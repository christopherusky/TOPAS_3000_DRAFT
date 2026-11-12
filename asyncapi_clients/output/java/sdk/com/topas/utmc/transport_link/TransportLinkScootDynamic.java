/*
 * File:        output/java/sdk/com/topas/utmc/transport_link/TransportLinkScootDynamic.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.transport_link;

import com.fasterxml.jackson.annotation.*;
import java.time.OffsetDateTime;
import java.util.Map;

/**
 * SCOOT-reported live measurement for a transport link.
 */
public class TransportLinkScootDynamic {
    private Double averageSpeedKph;
    private Double congestionPercent;
    private Double currentFlowVehicles;
    private Map<String, Object> extensions;
    private TransportLinkSCOOTConfigurationKind kind;
    private Double linkTravelTimeSeconds;
    private Double occupancyPercent;
    private Quality quality;
    private OffsetDateTime timestamp;
    private String transportLinkID;

    /**
     * Estimated average speed in kilometres per hour.
     */
    @JsonProperty("averageSpeedKph")
    public Double getAverageSpeedKph() { return averageSpeedKph; }
    @JsonProperty("averageSpeedKph")
    public void setAverageSpeedKph(Double value) { this.averageSpeedKph = value; }

    /**
     * Current percentage congestion as reported by SCOOT.
     */
    @JsonProperty("congestionPercent")
    public Double getCongestionPercent() { return congestionPercent; }
    @JsonProperty("congestionPercent")
    public void setCongestionPercent(Double value) { this.congestionPercent = value; }

    /**
     * Current flow in vehicles over the SCOOT measurement interval. A rate, so it may be
     * fractional (e.g. a smoothed or averaged flow).
     */
    @JsonProperty("currentFlowVehicles")
    public Double getCurrentFlowVehicles() { return currentFlowVehicles; }
    @JsonProperty("currentFlowVehicles")
    public void setCurrentFlowVehicles(Double value) { this.currentFlowVehicles = value; }

    /**
     * Vendor or local extensions. Open-content object.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * Discriminator for the tagged union. The fixed value "scoot" selects the
     * TransportLinkSCOOTDynamic payload; it is not free-text command content.
     */
    @JsonProperty("kind")
    public TransportLinkSCOOTConfigurationKind getKind() { return kind; }
    @JsonProperty("kind")
    public void setKind(TransportLinkSCOOTConfigurationKind value) { this.kind = value; }

    /**
     * Current link journey time in seconds.
     */
    @JsonProperty("linkTravelTimeSeconds")
    public Double getLinkTravelTimeSeconds() { return linkTravelTimeSeconds; }
    @JsonProperty("linkTravelTimeSeconds")
    public void setLinkTravelTimeSeconds(Double value) { this.linkTravelTimeSeconds = value; }

    /**
     * Average link occupancy percentage across all detectors on the link.
     */
    @JsonProperty("occupancyPercent")
    public Double getOccupancyPercent() { return occupancyPercent; }
    @JsonProperty("occupancyPercent")
    public void setOccupancyPercent(Double value) { this.occupancyPercent = value; }

    /**
     * Optional quality statement for this measurement.
     */
    @JsonProperty("quality")
    public Quality getQuality() { return quality; }
    @JsonProperty("quality")
    public void setQuality(Quality value) { this.quality = value; }

    /**
     * Time the measurement was taken.
     */
    @JsonProperty("timestamp")
    public OffsetDateTime getTimestamp() { return timestamp; }
    @JsonProperty("timestamp")
    public void setTimestamp(OffsetDateTime value) { this.timestamp = value; }

    /**
     * Identifier of the transport link this reading is about.
     */
    @JsonProperty("transportLinkId")
    public String getTransportLinkID() { return transportLinkID; }
    @JsonProperty("transportLinkId")
    public void setTransportLinkID(String value) { this.transportLinkID = value; }
}
