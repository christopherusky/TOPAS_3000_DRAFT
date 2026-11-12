/*
 * File:        output/java/sdk/com/topas/utmc/transport_route/TransportRouteDefinition.java
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
 * Transport route equipment details.
 */
public class TransportRouteDefinition {
    private OffsetDateTime creationDate;
    private EndPoint endPoint;
    private Map<String, Object> extensions;
    private EndPoint location;
    private String longDescription;
    private String originatingObjectID;
    private String originatingParticipantID;
    private String ownerOperator;
    private String participantID;
    private String shortDescription;
    private String transportRouteID;
    private String typeID;

    /**
     * When this definition was created.
     */
    @JsonProperty("creationDate")
    public OffsetDateTime getCreationDate() { return creationDate; }
    @JsonProperty("creationDate")
    public void setCreationDate(OffsetDateTime value) { this.creationDate = value; }

    /**
     * Route endpoint location.
     */
    @JsonProperty("endPoint")
    public EndPoint getEndPoint() { return endPoint; }
    @JsonProperty("endPoint")
    public void setEndPoint(EndPoint value) { this.endPoint = value; }

    /**
     * Vendor or local extensions.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * Start point location.
     */
    @JsonProperty("location")
    public EndPoint getLocation() { return location; }
    @JsonProperty("location")
    public void setLocation(EndPoint value) { this.location = value; }

    /**
     * Longer description.
     */
    @JsonProperty("longDescription")
    public String getLongDescription() { return longDescription; }
    @JsonProperty("longDescription")
    public void setLongDescription(String value) { this.longDescription = value; }

    /**
     * ID on originating system.
     */
    @JsonProperty("originatingObjectId")
    public String getOriginatingObjectID() { return originatingObjectID; }
    @JsonProperty("originatingObjectId")
    public void setOriginatingObjectID(String value) { this.originatingObjectID = value; }

    /**
     * Upstream system.
     */
    @JsonProperty("originatingParticipantId")
    public String getOriginatingParticipantID() { return originatingParticipantID; }
    @JsonProperty("originatingParticipantId")
    public void setOriginatingParticipantID(String value) { this.originatingParticipantID = value; }

    /**
     * Owner or operator.
     */
    @JsonProperty("ownerOperator")
    public String getOwnerOperator() { return ownerOperator; }
    @JsonProperty("ownerOperator")
    public void setOwnerOperator(String value) { this.ownerOperator = value; }

    /**
     * Owning system.
     */
    @JsonProperty("participantId")
    public String getParticipantID() { return participantID; }
    @JsonProperty("participantId")
    public void setParticipantID(String value) { this.participantID = value; }

    /**
     * Short name.
     */
    @JsonProperty("shortDescription")
    public String getShortDescription() { return shortDescription; }
    @JsonProperty("shortDescription")
    public void setShortDescription(String value) { this.shortDescription = value; }

    /**
     * Unique identifier of this route.
     */
    @JsonProperty("transportRouteId")
    public String getTransportRouteID() { return transportRouteID; }
    @JsonProperty("transportRouteId")
    public void setTransportRouteID(String value) { this.transportRouteID = value; }

    /**
     * Unique identifier for a transport route type (road, bus, rail, ...).
     */
    @JsonProperty("typeId")
    public String getTypeID() { return typeID; }
    @JsonProperty("typeId")
    public void setTypeID(String value) { this.typeID = value; }
}
