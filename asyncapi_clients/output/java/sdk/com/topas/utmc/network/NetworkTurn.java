/*
 * File:        output/java/sdk/com/topas/utmc/network/NetworkTurn.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.network;

import com.fasterxml.jackson.annotation.*;
import java.util.Map;

/**
 * Road network turn (movement through a node between links).
 */
public class NetworkTurn {
    private boolean allowed;
    private String description;
    private Map<String, Object> extensions;
    private String fromLinkID;
    private Geometry geometry;
    private String toLinkID;
    private String turnID;

    /**
     * Whether this turn is permitted.
     */
    @JsonProperty("allowed")
    public boolean getAllowed() { return allowed; }
    @JsonProperty("allowed")
    public void setAllowed(boolean value) { this.allowed = value; }

    /**
     * Human-readable description.
     */
    @JsonProperty("description")
    public String getDescription() { return description; }
    @JsonProperty("description")
    public void setDescription(String value) { this.description = value; }

    /**
     * Vendor or local extensions.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * Approach link.
     */
    @JsonProperty("fromLinkId")
    public String getFromLinkID() { return fromLinkID; }
    @JsonProperty("fromLinkId")
    public void setFromLinkID(String value) { this.fromLinkID = value; }

    /**
     * Turn path geometry as GeoJSON LineString.
     */
    @JsonProperty("geometry")
    public Geometry getGeometry() { return geometry; }
    @JsonProperty("geometry")
    public void setGeometry(Geometry value) { this.geometry = value; }

    /**
     * Departure link.
     */
    @JsonProperty("toLinkId")
    public String getToLinkID() { return toLinkID; }
    @JsonProperty("toLinkId")
    public void setToLinkID(String value) { this.toLinkID = value; }

    /**
     * Unique identifier of this turn.
     */
    @JsonProperty("turnId")
    public String getTurnID() { return turnID; }
    @JsonProperty("turnId")
    public void setTurnID(String value) { this.turnID = value; }
}
