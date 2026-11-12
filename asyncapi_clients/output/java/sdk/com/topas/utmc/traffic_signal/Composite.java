/*
 * File:        output/java/sdk/com/topas/utmc/traffic_signal/Composite.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.traffic_signal;

import com.fasterxml.jackson.annotation.*;
import java.util.Map;

/**
 * Signal-group timing row within a PlanLine (§3.5, §3.6).
 */
public class Composite {
    private String alternateSignalGroup;
    private Map<String, Object> extensions;
    private String primarySignalGroup;
    private long primarySignalGroupDuration;

    /**
     * A text representation of the alternatives to the primary signal group.
     */
    @JsonProperty("alternateSignalGroup")
    public String getAlternateSignalGroup() { return alternateSignalGroup; }
    @JsonProperty("alternateSignalGroup")
    public void setAlternateSignalGroup(String value) { this.alternateSignalGroup = value; }

    /**
     * Vendor or forward-compatibility extensions. Open-content object.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * A text representation of the primary signal group.
     */
    @JsonProperty("primarySignalGroup")
    public String getPrimarySignalGroup() { return primarySignalGroup; }
    @JsonProperty("primarySignalGroup")
    public void setPrimarySignalGroup(String value) { this.primarySignalGroup = value; }

    /**
     * The length of the primary signal group. Note that there is no timing data associated with
     * the alternate signal group.
     */
    @JsonProperty("primarySignalGroupDuration")
    public long getPrimarySignalGroupDuration() { return primarySignalGroupDuration; }
    @JsonProperty("primarySignalGroupDuration")
    public void setPrimarySignalGroupDuration(long value) { this.primarySignalGroupDuration = value; }
}
