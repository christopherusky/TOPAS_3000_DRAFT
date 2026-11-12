/*
 * File:        output/java/sdk/com/topas/utmc/detector/DetectorConfiguration.java
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
 * Detector operational configuration.
 */
public class DetectorConfiguration {
    private OffsetDateTime configurationDate;
    private String detectorID;
    private Map<String, Object> extensions;
    private Long thresholdDown;
    private Long thresholdUp;

    /**
     * When this configuration record became effective.
     */
    @JsonProperty("configurationDate")
    public OffsetDateTime getConfigurationDate() { return configurationDate; }
    @JsonProperty("configurationDate")
    public void setConfigurationDate(OffsetDateTime value) { this.configurationDate = value; }

    /**
     * Identifier of the detector this configuration applies to.
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
     * Downward threshold for alerting.
     */
    @JsonProperty("thresholdDown")
    public Long getThresholdDown() { return thresholdDown; }
    @JsonProperty("thresholdDown")
    public void setThresholdDown(Long value) { this.thresholdDown = value; }

    /**
     * Upward threshold for alerting (e.g. occupancy threshold).
     */
    @JsonProperty("thresholdUp")
    public Long getThresholdUp() { return thresholdUp; }
    @JsonProperty("thresholdUp")
    public void setThresholdUp(Long value) { this.thresholdUp = value; }
}
