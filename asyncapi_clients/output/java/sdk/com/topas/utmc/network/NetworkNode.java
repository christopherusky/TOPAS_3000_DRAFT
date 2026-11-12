/*
 * File:        output/java/sdk/com/topas/utmc/network/NetworkNode.java
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
 * Road network node (junction or endpoint).
 */
public class NetworkNode {
    private String description;
    private Map<String, Object> extensions;
    private Location location;
    private String nodeID;

    /**
     * Human-readable name.
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
     * Geographic position as GeoJSON Point.
     */
    @JsonProperty("location")
    public Location getLocation() { return location; }
    @JsonProperty("location")
    public void setLocation(Location value) { this.location = value; }

    /**
     * Unique identifier of this node.
     */
    @JsonProperty("nodeId")
    public String getNodeID() { return nodeID; }
    @JsonProperty("nodeId")
    public void setNodeID(String value) { this.nodeID = value; }
}
