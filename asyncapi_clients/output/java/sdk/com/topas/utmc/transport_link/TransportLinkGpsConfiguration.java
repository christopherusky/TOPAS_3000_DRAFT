/*
 * File:        output/java/sdk/com/topas/utmc/transport_link/TransportLinkGpsConfiguration.java
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
 * Operational configuration for a GPS-monitored transport link.
 */
public class TransportLinkGpsConfiguration {
    private Double capacityVehiclesPerHour;
    private OffsetDateTime configurationDate;
    private Map<String, Object> extensions;
    private Double fullLengthMetres;
    private TransportLinkGPSConfigurationKind kind;
    private Double lengthMetres;
    private Double maximumSpeedKph;
    private Long numberOfLanes;
    private String transportLinkID;

    /**
     * Maximum flow in vehicles per hour.
     */
    @JsonProperty("capacityVehiclesPerHour")
    public Double getCapacityVehiclesPerHour() { return capacityVehiclesPerHour; }
    @JsonProperty("capacityVehiclesPerHour")
    public void setCapacityVehiclesPerHour(Double value) { this.capacityVehiclesPerHour = value; }

    /**
     * When this configuration record became effective.
     */
    @JsonProperty("configurationDate")
    public OffsetDateTime getConfigurationDate() { return configurationDate; }
    @JsonProperty("configurationDate")
    public void setConfigurationDate(OffsetDateTime value) { this.configurationDate = value; }

    /**
     * Vendor or local extensions. Open-content object.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * Distance from stop line to stop line in metres.
     */
    @JsonProperty("fullLengthMetres")
    public Double getFullLengthMetres() { return fullLengthMetres; }
    @JsonProperty("fullLengthMetres")
    public void setFullLengthMetres(Double value) { this.fullLengthMetres = value; }

    /**
     * Discriminator for the tagged union. The fixed value "gps" selects the
     * TransportLinkGPSConfiguration payload; it is not free-text command content.
     */
    @JsonProperty("kind")
    public TransportLinkGPSConfigurationKind getKind() { return kind; }
    @JsonProperty("kind")
    public void setKind(TransportLinkGPSConfigurationKind value) { this.kind = value; }

    /**
     * Distance from detector to stop line in metres.
     */
    @JsonProperty("lengthMetres")
    public Double getLengthMetres() { return lengthMetres; }
    @JsonProperty("lengthMetres")
    public void setLengthMetres(Double value) { this.lengthMetres = value; }

    /**
     * Maximum speed associated with the link in kilometres per hour.
     */
    @JsonProperty("maximumSpeedKph")
    public Double getMaximumSpeedKph() { return maximumSpeedKph; }
    @JsonProperty("maximumSpeedKph")
    public void setMaximumSpeedKph(Double value) { this.maximumSpeedKph = value; }

    /**
     * Number of lanes on the link.
     */
    @JsonProperty("numberOfLanes")
    public Long getNumberOfLanes() { return numberOfLanes; }
    @JsonProperty("numberOfLanes")
    public void setNumberOfLanes(Long value) { this.numberOfLanes = value; }

    /**
     * Identifier of the transport link this configuration applies to.
     */
    @JsonProperty("transportLinkId")
    public String getTransportLinkID() { return transportLinkID; }
    @JsonProperty("transportLinkId")
    public void setTransportLinkID(String value) { this.transportLinkID = value; }
}
