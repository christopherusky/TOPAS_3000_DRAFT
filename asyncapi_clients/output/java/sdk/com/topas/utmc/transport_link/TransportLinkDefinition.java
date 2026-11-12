/*
 * File:        output/java/sdk/com/topas/utmc/transport_link/TransportLinkDefinition.java
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
 * Transport link equipment details.
 */
public class TransportLinkDefinition {
    private OffsetDateTime creationDate;
    private EndPoint endPoint;
    private Map<String, Object> extensions;
    private EndPoint location;
    private String longDescription;
    private String originatingObjectID;
    private String originatingParticipantID;
    private String participantID;
    private String shortDescription;
    private String transportLinkID;
    private String typeID;

    /**
     * When this definition record was created.
     */
    @JsonProperty("creationDate")
    public OffsetDateTime getCreationDate() { return creationDate; }
    @JsonProperty("creationDate")
    public void setCreationDate(OffsetDateTime value) { this.creationDate = value; }

    /**
     * End of the transport link as a WGS84 point. Optional here although the v1 XSD requires
     * it, matching transport_route.
     */
    @JsonProperty("endPoint")
    public EndPoint getEndPoint() { return endPoint; }
    @JsonProperty("endPoint")
    public void setEndPoint(EndPoint value) { this.endPoint = value; }

    /**
     * Vendor or local extensions. Open-content object.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * Start of the transport link as a WGS84 point. Optional.
     */
    @JsonProperty("location")
    public EndPoint getLocation() { return location; }
    @JsonProperty("location")
    public void setLocation(EndPoint value) { this.location = value; }

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
     * System that owns this transport link definition. Often implicit from context.
     */
    @JsonProperty("participantId")
    public String getParticipantID() { return participantID; }
    @JsonProperty("participantId")
    public void setParticipantID(String value) { this.participantID = value; }

    /**
     * Short human-readable name (e.g. 'A1 northbound, Jct 5 -> Jct 6').
     */
    @JsonProperty("shortDescription")
    public String getShortDescription() { return shortDescription; }
    @JsonProperty("shortDescription")
    public void setShortDescription(String value) { this.shortDescription = value; }

    /**
     * Unique identifier of this transport link within its owning system.
     */
    @JsonProperty("transportLinkId")
    public String getTransportLinkID() { return transportLinkID; }
    @JsonProperty("transportLinkId")
    public void setTransportLinkID(String value) { this.transportLinkID = value; }

    /**
     * Unique identifier for a transport link type (SCOOT, ANPR, SCOOT Entry, ...).
     */
    @JsonProperty("typeId")
    public String getTypeID() { return typeID; }
    @JsonProperty("typeId")
    public void setTypeID(String value) { this.typeID = value; }
}
