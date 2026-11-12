/*
 * File:        output/java/sdk/com/topas/utmc/transport_link/TransportLinkGpsDynamic.java
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
 * GPS-probe live measurement for a transport link.
 *
 * Field set drawn from the XSD TransportLinkGpsDynamicGroup (Freeflow
 * Extension): CurrentJourneyTime, the three travel-time statistics
 * (Mean/Median/VarianceTravelTime), NumberOfVehicles and the reserved-
 * lane flow count. Shared base fields (transport_link_id, timestamp,
 * quality, average_speed_kph, link_travel_time_seconds) are inherited
 * from _TransportLinkDynamicBase.
 */
public class TransportLinkGpsDynamic {
    private Double averageSpeedKph;
    private Double currentJourneyTimeSeconds;
    private Map<String, Object> extensions;
    private TransportLinkGPSConfigurationKind kind;
    private Double linkTravelTimeSeconds;
    private Double meanTravelTimeSeconds;
    private Double medianTravelTimeSeconds;
    private Long numberOfVehicles;
    private Quality quality;
    private Double reservedLaneFlowVehicles;
    private OffsetDateTime timestamp;
    private String transportLinkID;
    private Double varianceTravelTimeSeconds;

    /**
     * Estimated average speed in kilometres per hour.
     */
    @JsonProperty("averageSpeedKph")
    public Double getAverageSpeedKph() { return averageSpeedKph; }
    @JsonProperty("averageSpeedKph")
    public void setAverageSpeedKph(Double value) { this.averageSpeedKph = value; }

    /**
     * Current journey time in seconds.
     */
    @JsonProperty("currentJourneyTimeSeconds")
    public Double getCurrentJourneyTimeSeconds() { return currentJourneyTimeSeconds; }
    @JsonProperty("currentJourneyTimeSeconds")
    public void setCurrentJourneyTimeSeconds(Double value) { this.currentJourneyTimeSeconds = value; }

    /**
     * Vendor or local extensions. Open-content object.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * Discriminator for the tagged union. The fixed value "gps" selects the
     * TransportLinkGPSDynamic payload; it is not free-text command content.
     */
    @JsonProperty("kind")
    public TransportLinkGPSConfigurationKind getKind() { return kind; }
    @JsonProperty("kind")
    public void setKind(TransportLinkGPSConfigurationKind value) { this.kind = value; }

    /**
     * Current link journey time in seconds.
     */
    @JsonProperty("linkTravelTimeSeconds")
    public Double getLinkTravelTimeSeconds() { return linkTravelTimeSeconds; }
    @JsonProperty("linkTravelTimeSeconds")
    public void setLinkTravelTimeSeconds(Double value) { this.linkTravelTimeSeconds = value; }

    /**
     * Mean travel time along the link in the measured interval, in seconds.
     */
    @JsonProperty("meanTravelTimeSeconds")
    public Double getMeanTravelTimeSeconds() { return meanTravelTimeSeconds; }
    @JsonProperty("meanTravelTimeSeconds")
    public void setMeanTravelTimeSeconds(Double value) { this.meanTravelTimeSeconds = value; }

    /**
     * Median travel time along the link in the measured interval, in seconds.
     */
    @JsonProperty("medianTravelTimeSeconds")
    public Double getMedianTravelTimeSeconds() { return medianTravelTimeSeconds; }
    @JsonProperty("medianTravelTimeSeconds")
    public void setMedianTravelTimeSeconds(Double value) { this.medianTravelTimeSeconds = value; }

    /**
     * Number of vehicles in the measured interval.
     */
    @JsonProperty("numberOfVehicles")
    public Long getNumberOfVehicles() { return numberOfVehicles; }
    @JsonProperty("numberOfVehicles")
    public void setNumberOfVehicles(Long value) { this.numberOfVehicles = value; }

    /**
     * Optional quality statement for this measurement.
     */
    @JsonProperty("quality")
    public Quality getQuality() { return quality; }
    @JsonProperty("quality")
    public void setQuality(Quality value) { this.quality = value; }

    /**
     * Flow in the reserved lane in the measured interval, in vehicles. A rate, so it may be
     * fractional.
     */
    @JsonProperty("reservedLaneFlowVehicles")
    public Double getReservedLaneFlowVehicles() { return reservedLaneFlowVehicles; }
    @JsonProperty("reservedLaneFlowVehicles")
    public void setReservedLaneFlowVehicles(Double value) { this.reservedLaneFlowVehicles = value; }

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

    /**
     * Variance of travel times along the link in the measured interval. The unit is seconds
     * squared (s²), as for any variance of times; the field name keeps 'Seconds' for
     * compatibility with v1.
     */
    @JsonProperty("varianceTravelTimeSeconds")
    public Double getVarianceTravelTimeSeconds() { return varianceTravelTimeSeconds; }
    @JsonProperty("varianceTravelTimeSeconds")
    public void setVarianceTravelTimeSeconds(Double value) { this.varianceTravelTimeSeconds = value; }
}
