/*
 * File:        output/java/sdk/com/topas/utmc/network/NetworkZone.java
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
 * Road network zone (area grouping of links/nodes).
 */
public class NetworkZone {
    private String description;
    private Map<String, Object> extensions;
    private Geometry geometry;
    private String[] linkIDS;
    private Boolean permanent;
    private String typeID;
    private String zoneID;

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
     * Zone boundary geometry as GeoJSON LineString.
     */
    @JsonProperty("geometry")
    public Geometry getGeometry() { return geometry; }
    @JsonProperty("geometry")
    public void setGeometry(Geometry value) { this.geometry = value; }

    /**
     * The links that make up the zone, as a set: the order of the array carries no meaning (v1
     * describes a zone as an unordered sequence of links). Omit the field rather than send an
     * empty list. Each link appears at most once.
     */
    @JsonProperty("linkIds")
    public String[] getLinkIDS() { return linkIDS; }
    @JsonProperty("linkIds")
    public void setLinkIDS(String[] value) { this.linkIDS = value; }

    /**
     * Whether this is a permanent or temporary zone. Absent means permanent (the v1 default for
     * a zone).
     */
    @JsonProperty("permanent")
    public Boolean getPermanent() { return permanent; }
    @JsonProperty("permanent")
    public void setPermanent(Boolean value) { this.permanent = value; }

    /**
     * Zone type classification (e.g. electoral ward, incident area).
     */
    @JsonProperty("typeId")
    public String getTypeID() { return typeID; }
    @JsonProperty("typeId")
    public void setTypeID(String value) { this.typeID = value; }

    /**
     * Unique identifier of this zone.
     */
    @JsonProperty("zoneId")
    public String getZoneID() { return zoneID; }
    @JsonProperty("zoneId")
    public void setZoneID(String value) { this.zoneID = value; }
}
