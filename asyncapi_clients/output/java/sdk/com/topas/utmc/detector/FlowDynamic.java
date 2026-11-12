/*
 * File:        output/java/sdk/com/topas/utmc/detector/FlowDynamic.java
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
 * Vehicle flow count over a measurement period.
 */
public class FlowDynamic {
    private Long[] classCounts;
    private String detectorID;
    private Map<String, Object> extensions;
    private FlowDynamicKind kind;
    private Long periodSeconds;
    private Quality quality;
    private OffsetDateTime timestamp;
    private Long totalFlow;

    /**
     * Per-class vehicle counts in positional order — index 0 is Class1, index 7 is Class8. May
     * be shorter if trailing classes are absent. A null entry means that class was not
     * reported, as distinct from a count of zero.
     */
    @JsonProperty("classCounts")
    public Long[] getClassCounts() { return classCounts; }
    @JsonProperty("classCounts")
    public void setClassCounts(Long[] value) { this.classCounts = value; }

    /**
     * Identifier of the detector this reading is about.
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
     * Discriminator for the tagged union. The fixed value "flow" selects the FlowDynamic
     * payload; it is not free-text command content.
     */
    @JsonProperty("kind")
    public FlowDynamicKind getKind() { return kind; }
    @JsonProperty("kind")
    public void setKind(FlowDynamicKind value) { this.kind = value; }

    /**
     * Duration of the measurement period in seconds.
     */
    @JsonProperty("periodSeconds")
    public Long getPeriodSeconds() { return periodSeconds; }
    @JsonProperty("periodSeconds")
    public void setPeriodSeconds(Long value) { this.periodSeconds = value; }

    /**
     * Optional quality statement for this measurement.
     */
    @JsonProperty("quality")
    public Quality getQuality() { return quality; }
    @JsonProperty("quality")
    public void setQuality(Quality value) { this.quality = value; }

    /**
     * Time the measurement was taken.
     */
    @JsonProperty("timestamp")
    public OffsetDateTime getTimestamp() { return timestamp; }
    @JsonProperty("timestamp")
    public void setTimestamp(OffsetDateTime value) { this.timestamp = value; }

    /**
     * Total vehicle count over the period. Omitted when only class counts are reported.
     */
    @JsonProperty("totalFlow")
    public Long getTotalFlow() { return totalFlow; }
    @JsonProperty("totalFlow")
    public void setTotalFlow(Long value) { this.totalFlow = value; }
}
