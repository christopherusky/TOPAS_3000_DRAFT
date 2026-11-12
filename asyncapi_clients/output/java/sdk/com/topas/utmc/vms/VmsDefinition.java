/*
 * File:        output/java/sdk/com/topas/utmc/vms/VmsDefinition.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.vms;

import com.fasterxml.jackson.annotation.*;
import java.time.OffsetDateTime;
import java.util.Map;

/**
 * Variable message sign equipment details.
 */
public class VmsDefinition {
    private Long charactersPerLine;
    private VmsColourCapability colourCapability;
    private OffsetDateTime creationDate;
    private Map<String, Object> extensions;
    private Double linkDistanceMetres;
    private Location location;
    private String longDescription;
    private String networkLinkID;
    private String networkPathID;
    private Long numberOfLines;
    private String originatingObjectID;
    private String originatingParticipantID;
    private String ownerOperator;
    private String participantID;
    private String shortDescription;
    private Long signHeight;
    private Long signWidth;
    private Boolean supportsGraphics;
    private String transportLinkID;
    private String typeID;
    private String vmsID;

    /**
     * Characters per line.
     */
    @JsonProperty("charactersPerLine")
    public Long getCharactersPerLine() { return charactersPerLine; }
    @JsonProperty("charactersPerLine")
    public void setCharactersPerLine(Long value) { this.charactersPerLine = value; }

    /**
     * Colour capability.
     */
    @JsonProperty("colourCapability")
    public VmsColourCapability getColourCapability() { return colourCapability; }
    @JsonProperty("colourCapability")
    public void setColourCapability(VmsColourCapability value) { this.colourCapability = value; }

    /**
     * When this definition was created.
     */
    @JsonProperty("creationDate")
    public OffsetDateTime getCreationDate() { return creationDate; }
    @JsonProperty("creationDate")
    public void setCreationDate(OffsetDateTime value) { this.creationDate = value; }

    /**
     * Vendor or local extensions.
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
     * Physical location.
     */
    @JsonProperty("location")
    public Location getLocation() { return location; }
    @JsonProperty("location")
    public void setLocation(Location value) { this.location = value; }

    /**
     * Longer description.
     */
    @JsonProperty("longDescription")
    public String getLongDescription() { return longDescription; }
    @JsonProperty("longDescription")
    public void setLongDescription(String value) { this.longDescription = value; }

    /**
     * Network link the sign stands on. At most one of this and networkPathId. Optional.
     */
    @JsonProperty("networkLinkId")
    public String getNetworkLinkID() { return networkLinkID; }
    @JsonProperty("networkLinkId")
    public void setNetworkLinkID(String value) { this.networkLinkID = value; }

    /**
     * Network path the sign stands on. At most one of this and networkLinkId. Optional.
     */
    @JsonProperty("networkPathId")
    public String getNetworkPathID() { return networkPathID; }
    @JsonProperty("networkPathId")
    public void setNetworkPathID(String value) { this.networkPathID = value; }

    /**
     * Number of text lines.
     */
    @JsonProperty("numberOfLines")
    public Long getNumberOfLines() { return numberOfLines; }
    @JsonProperty("numberOfLines")
    public void setNumberOfLines(Long value) { this.numberOfLines = value; }

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
     * Sign height in pixels/units.
     */
    @JsonProperty("signHeight")
    public Long getSignHeight() { return signHeight; }
    @JsonProperty("signHeight")
    public void setSignHeight(Long value) { this.signHeight = value; }

    /**
     * Sign width in pixels/units.
     */
    @JsonProperty("signWidth")
    public Long getSignWidth() { return signWidth; }
    @JsonProperty("signWidth")
    public void setSignWidth(Long value) { this.signWidth = value; }

    /**
     * Whether the sign supports graphics.
     */
    @JsonProperty("supportsGraphics")
    public Boolean getSupportsGraphics() { return supportsGraphics; }
    @JsonProperty("supportsGraphics")
    public void setSupportsGraphics(Boolean value) { this.supportsGraphics = value; }

    /**
     * Transport link this sign monitors or sits on. Optional.
     */
    @JsonProperty("transportLinkId")
    public String getTransportLinkID() { return transportLinkID; }
    @JsonProperty("transportLinkId")
    public void setTransportLinkID(String value) { this.transportLinkID = value; }

    /**
     * Unique identifier for a VMS type (sign type catalogue entry).
     */
    @JsonProperty("typeId")
    public String getTypeID() { return typeID; }
    @JsonProperty("typeId")
    public void setTypeID(String value) { this.typeID = value; }

    /**
     * Unique identifier of this VMS.
     */
    @JsonProperty("vmsId")
    public String getVmsID() { return vmsID; }
    @JsonProperty("vmsId")
    public void setVmsID(String value) { this.vmsID = value; }
}
