/*
 * File:        output/java/sdk/com/topas/utmc/strategy/StrategyDefinition.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.strategy;

import com.fasterxml.jackson.annotation.*;
import java.util.Map;

/**
 * Traffic management strategy static metadata.
 */
public class StrategyDefinition {
    private Map<String, Object> extensions;
    private Location location;
    private String strategyDescription;
    private String strategyID;
    private String strategyName;

    /**
     * Vendor or local extensions.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * Geographic centre of the strategy area as WGS84 GeoJSON Point.
     */
    @JsonProperty("location")
    public Location getLocation() { return location; }
    @JsonProperty("location")
    public void setLocation(Location value) { this.location = value; }

    /**
     * Longer description of what the strategy does.
     */
    @JsonProperty("strategyDescription")
    public String getStrategyDescription() { return strategyDescription; }
    @JsonProperty("strategyDescription")
    public void setStrategyDescription(String value) { this.strategyDescription = value; }

    /**
     * Strategy identifier (e.g. 'STR00000001').
     */
    @JsonProperty("strategyId")
    public String getStrategyID() { return strategyID; }
    @JsonProperty("strategyId")
    public void setStrategyID(String value) { this.strategyID = value; }

    /**
     * Human-readable name of the strategy.
     */
    @JsonProperty("strategyName")
    public String getStrategyName() { return strategyName; }
    @JsonProperty("strategyName")
    public void setStrategyName(String value) { this.strategyName = value; }
}
