/*
 * File:        output/java/sdk/com/topas/utmc/transport_link/TransportLinkAnprDynamic.java
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
 * ANPR-reported live measurement for a transport link.
 */
public class TransportLinkAnprDynamic {
    private Double averageSpeedKph;
    private Map<String, Object> extensions;
    private TransportLinkANPRConfigurationKind kind;
    private Double linkTravelTimeSeconds;
    private Long numberPlatesIn;
    private Long numberPlatesMatches;
    private Long numberPlatesMatchesUsed;
    private Long numberPlatesOut;
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
     * Vendor or local extensions. Open-content object.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * Discriminator for the tagged union. The fixed value "anpr" selects the
     * TransportLinkANPRDynamic payload; it is not free-text command content.
     */
    @JsonProperty("kind")
    public TransportLinkANPRConfigurationKind getKind() { return kind; }
    @JsonProperty("kind")
    public void setKind(TransportLinkANPRConfigurationKind value) { this.kind = value; }

    /**
     * Current link journey time in seconds.
     */
    @JsonProperty("linkTravelTimeSeconds")
    public Double getLinkTravelTimeSeconds() { return linkTravelTimeSeconds; }
    @JsonProperty("linkTravelTimeSeconds")
    public void setLinkTravelTimeSeconds(Double value) { this.linkTravelTimeSeconds = value; }

    /**
     * Number of successful plate reads at the start of the link.
     */
    @JsonProperty("numberPlatesIn")
    public Long getNumberPlatesIn() { return numberPlatesIn; }
    @JsonProperty("numberPlatesIn")
    public void setNumberPlatesIn(Long value) { this.numberPlatesIn = value; }

    /**
     * Count of number-plate matches in the measurement period.
     */
    @JsonProperty("numberPlatesMatches")
    public Long getNumberPlatesMatches() { return numberPlatesMatches; }
    @JsonProperty("numberPlatesMatches")
    public void setNumberPlatesMatches(Long value) { this.numberPlatesMatches = value; }

    /**
     * Number of successful matches actually used in the travel-time calculation.
     */
    @JsonProperty("numberPlatesMatchesUsed")
    public Long getNumberPlatesMatchesUsed() { return numberPlatesMatchesUsed; }
    @JsonProperty("numberPlatesMatchesUsed")
    public void setNumberPlatesMatchesUsed(Long value) { this.numberPlatesMatchesUsed = value; }

    /**
     * Number of successful plate reads at the end of the link.
     */
    @JsonProperty("numberPlatesOut")
    public Long getNumberPlatesOut() { return numberPlatesOut; }
    @JsonProperty("numberPlatesOut")
    public void setNumberPlatesOut(Long value) { this.numberPlatesOut = value; }

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
