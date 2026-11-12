/*
 * File:        output/java/sdk/com/topas/utmc/traffic_signal/TrafficSignalDefinition.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.traffic_signal;

import com.fasterxml.jackson.annotation.*;
import java.time.OffsetDateTime;
import java.util.Map;

/**
 * Traffic signal equipment details.
 */
public class TrafficSignalDefinition {
    private String controlMethodID;
    private OffsetDateTime creationDate;
    private Map<String, Object> extensions;
    private Location location;
    private String longDescription;
    private String originatingObjectID;
    private String originatingParticipantID;
    private String participantID;
    private String shortDescription;
    private String trafficSignalID;

    /**
     * Optional reference into the TrafficSignalType catalogue identifying the operational
     * control method (MOVA, SCOOT, fixed-time, ...). Orthogonal to
     * TrafficSignalConfiguration.trafficSignalType, which classifies the junction kind
     * (intersection, pelican, puffin, ...).
     */
    @JsonProperty("controlMethodId")
    public String getControlMethodID() { return controlMethodID; }
    @JsonProperty("controlMethodId")
    public void setControlMethodID(String value) { this.controlMethodID = value; }

    /**
     * When this definition record was created.
     */
    @JsonProperty("creationDate")
    public OffsetDateTime getCreationDate() { return creationDate; }
    @JsonProperty("creationDate")
    public void setCreationDate(OffsetDateTime value) { this.creationDate = value; }

    /**
     * Vendor or forward-compatibility extensions. Open-content object.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * Location of traffic signal.
     */
    @JsonProperty("location")
    public Location getLocation() { return location; }
    @JsonProperty("location")
    public void setLocation(Location value) { this.location = value; }

    /**
     * Longer description (context, commissioning notes, etc.).
     */
    @JsonProperty("longDescription")
    public String getLongDescription() { return longDescription; }
    @JsonProperty("longDescription")
    public void setLongDescription(String value) { this.longDescription = value; }

    /**
     * Identifier of this object on the originating (upstream) system. Enables cross-system
     * traceability.
     */
    @JsonProperty("originatingObjectId")
    public String getOriginatingObjectID() { return originatingObjectID; }
    @JsonProperty("originatingObjectId")
    public void setOriginatingObjectID(String value) { this.originatingObjectID = value; }

    /**
     * Identifier of the upstream system that originally created this object (comparable to the
     * UML DataSourceTypeId).
     */
    @JsonProperty("originatingParticipantId")
    public String getOriginatingParticipantID() { return originatingParticipantID; }
    @JsonProperty("originatingParticipantId")
    public void setOriginatingParticipantID(String value) { this.originatingParticipantID = value; }

    /**
     * System that owns this traffic signal definition. Often implicit from context.
     */
    @JsonProperty("participantId")
    public String getParticipantID() { return participantID; }
    @JsonProperty("participantId")
    public void setParticipantID(String value) { this.participantID = value; }

    /**
     * Short human-readable name (e.g. 'Main St / High St junction').
     */
    @JsonProperty("shortDescription")
    public String getShortDescription() { return shortDescription; }
    @JsonProperty("shortDescription")
    public void setShortDescription(String value) { this.shortDescription = value; }

    /**
     * Human readable identifier for the Traffic Signal.
     */
    @JsonProperty("trafficSignalId")
    public String getTrafficSignalID() { return trafficSignalID; }
    @JsonProperty("trafficSignalId")
    public void setTrafficSignalID(String value) { this.trafficSignalID = value; }
}
