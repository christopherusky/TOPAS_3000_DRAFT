/*
 * File:        output/java/sdk/com/topas/utmc/network/NetworkPath.java
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
 * Road network path (ordered sequence of links).
 */
public class NetworkPath {
    private String description;
    private Map<String, Object> extensions;
    private Double finalTrimMeters;
    private Double initialTrimMeters;
    private String[] linkIDS;
    private String pathID;
    private Boolean permanent;
    private String typeID;

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
     * Distance to trim from the end of the last link.
     */
    @JsonProperty("finalTrimMeters")
    public Double getFinalTrimMeters() { return finalTrimMeters; }
    @JsonProperty("finalTrimMeters")
    public void setFinalTrimMeters(Double value) { this.finalTrimMeters = value; }

    /**
     * Distance to trim from the start of the first link.
     */
    @JsonProperty("initialTrimMeters")
    public Double getInitialTrimMeters() { return initialTrimMeters; }
    @JsonProperty("initialTrimMeters")
    public void setInitialTrimMeters(Double value) { this.initialTrimMeters = value; }

    /**
     * Ordered links defining the path. List order is the route sequence. Omit the field rather
     * than send an empty list. A link may appear more than once, since a route can traverse the
     * same link twice.
     */
    @JsonProperty("linkIds")
    public String[] getLinkIDS() { return linkIDS; }
    @JsonProperty("linkIds")
    public void setLinkIDS(String[] value) { this.linkIDS = value; }

    /**
     * Unique identifier of this path.
     */
    @JsonProperty("pathId")
    public String getPathID() { return pathID; }
    @JsonProperty("pathId")
    public void setPathID(String value) { this.pathID = value; }

    /**
     * Whether this is a permanent or temporary path.
     */
    @JsonProperty("permanent")
    public Boolean getPermanent() { return permanent; }
    @JsonProperty("permanent")
    public void setPermanent(Boolean value) { this.permanent = value; }

    /**
     * Path type classification (e.g. bus route, diversion).
     */
    @JsonProperty("typeId")
    public String getTypeID() { return typeID; }
    @JsonProperty("typeId")
    public void setTypeID(String value) { this.typeID = value; }
}
