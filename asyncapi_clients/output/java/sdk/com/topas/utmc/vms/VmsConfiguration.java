/*
 * File:        output/java/sdk/com/topas/utmc/vms/VmsConfiguration.java
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
 * Variable message sign operational configuration.
 */
public class VmsConfiguration {
    private Boolean autoBrightness;
    private OffsetDateTime configurationDate;
    private Double defaultBrightnessPercent;
    private Long defaultDisplayTimeSeconds;
    private Map<String, Object> extensions;
    private Long scrollSpeed;
    private String vmsID;

    /**
     * Whether auto-brightness is enabled.
     */
    @JsonProperty("autoBrightness")
    public Boolean getAutoBrightness() { return autoBrightness; }
    @JsonProperty("autoBrightness")
    public void setAutoBrightness(Boolean value) { this.autoBrightness = value; }

    /**
     * When effective.
     */
    @JsonProperty("configurationDate")
    public OffsetDateTime getConfigurationDate() { return configurationDate; }
    @JsonProperty("configurationDate")
    public void setConfigurationDate(OffsetDateTime value) { this.configurationDate = value; }

    /**
     * Default brightness percentage.
     */
    @JsonProperty("defaultBrightnessPercent")
    public Double getDefaultBrightnessPercent() { return defaultBrightnessPercent; }
    @JsonProperty("defaultBrightnessPercent")
    public void setDefaultBrightnessPercent(Double value) { this.defaultBrightnessPercent = value; }

    /**
     * Default message display time in seconds.
     */
    @JsonProperty("defaultDisplayTimeSeconds")
    public Long getDefaultDisplayTimeSeconds() { return defaultDisplayTimeSeconds; }
    @JsonProperty("defaultDisplayTimeSeconds")
    public void setDefaultDisplayTimeSeconds(Long value) { this.defaultDisplayTimeSeconds = value; }

    /**
     * Vendor or local extensions.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * Text scroll speed.
     */
    @JsonProperty("scrollSpeed")
    public Long getScrollSpeed() { return scrollSpeed; }
    @JsonProperty("scrollSpeed")
    public void setScrollSpeed(Long value) { this.scrollSpeed = value; }

    /**
     * VMS this configuration applies to.
     */
    @JsonProperty("vmsId")
    public String getVmsID() { return vmsID; }
    @JsonProperty("vmsId")
    public void setVmsID(String value) { this.vmsID = value; }
}
