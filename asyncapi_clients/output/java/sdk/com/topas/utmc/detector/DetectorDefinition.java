/*
 * File:        output/java/sdk/com/topas/utmc/detector/DetectorDefinition.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.detector;

import com.fasterxml.jackson.annotation.*;
import java.time.OffsetDateTime;
import java.util.Map;

/**
 * Detector equipment details.
 */
public class DetectorDefinition {
    private OffsetDateTime creationDate;
    private String detectorID;
    private Map<String, Object> extensions;
    private Double linkDistanceMetres;
    private Location location;
    private String longDescription;
    private String networkLinkID;
    private String networkPathID;
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
     * Unique identifier of this detector within its owning system.
     */
    @JsonProperty("detectorId")
    public String getDetectorID() { return detectorID; }
    @JsonProperty("detectorId")
    public void setDetectorID(String value) { this.detectorID = value; }

    /**
     * Vendor or local extensions. Open-content object.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * Distance along that transport link, in metres. Optional.
     */
    @JsonProperty("linkDistanceMetres")
    public Double getLinkDistanceMetres() { return linkDistanceMetres; }
    @JsonProperty("linkDistanceMetres")
    public void setLinkDistanceMetres(Double value) { this.linkDistanceMetres = value; }

    /**
     * Physical location of the detector. Optional.
     */
    @JsonProperty("location")
    public Location getLocation() { return location; }
    @JsonProperty("location")
    public void setLocation(Location value) { this.location = value; }

    /**
     * Longer description (context, installation notes, etc.).
     */
    @JsonProperty("longDescription")
    public String getLongDescription() { return longDescription; }
    @JsonProperty("longDescription")
    public void setLongDescription(String value) { this.longDescription = value; }

    /**
     * Network link the detector stands on. At most one of this and networkPathId. Optional.
     */
    @JsonProperty("networkLinkId")
    public String getNetworkLinkID() { return networkLinkID; }
    @JsonProperty("networkLinkId")
    public void setNetworkLinkID(String value) { this.networkLinkID = value; }

    /**
     * Network path the detector stands on. At most one of this and networkLinkId. Optional.
     */
    @JsonProperty("networkPathId")
    public String getNetworkPathID() { return networkPathID; }
    @JsonProperty("networkPathId")
    public void setNetworkPathID(String value) { this.networkPathID = value; }

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
     * System that owns this detector definition. Often implicit from context.
     */
    @JsonProperty("participantId")
    public String getParticipantID() { return participantID; }
    @JsonProperty("participantId")
    public void setParticipantID(String value) { this.participantID = value; }

    /**
     * Short human-readable name (e.g. 'A1 southbound').
     */
    @JsonProperty("shortDescription")
    public String getShortDescription() { return shortDescription; }
    @JsonProperty("shortDescription")
    public void setShortDescription(String value) { this.shortDescription = value; }

    /**
     * Transport link this detector monitors or sits on. Optional.
     */
    @JsonProperty("transportLinkId")
    public String getTransportLinkID() { return transportLinkID; }
    @JsonProperty("transportLinkId")
    public void setTransportLinkID(String value) { this.transportLinkID = value; }

    /**
     * Unique identifier for a detector type (loop, radar, ANPR, ...).
     */
    @JsonProperty("typeId")
    public String getTypeID() { return typeID; }
    @JsonProperty("typeId")
    public void setTypeID(String value) { this.typeID = value; }
}
