/*
 * File:        output/java/sdk/com/topas/utmc/detector/QueueDynamic.java
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
 * Queue presence and severity reported by the detector.
 */
public class QueueDynamic {
    private String detectorID;
    private Map<String, Object> extensions;
    private QueueDynamicKind kind;
    private Quality quality;
    private boolean queuePresent;
    private Severity severity;
    private OffsetDateTime timestamp;

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
     * Discriminator for the tagged union. The fixed value "queue" selects the QueueDynamic
     * payload; it is not free-text command content.
     */
    @JsonProperty("kind")
    public QueueDynamicKind getKind() { return kind; }
    @JsonProperty("kind")
    public void setKind(QueueDynamicKind value) { this.kind = value; }

    /**
     * Optional quality statement for this measurement.
     */
    @JsonProperty("quality")
    public Quality getQuality() { return quality; }
    @JsonProperty("quality")
    public void setQuality(Quality value) { this.quality = value; }

    /**
     * Whether a queue is currently detected.
     */
    @JsonProperty("queuePresent")
    public boolean getQueuePresent() { return queuePresent; }
    @JsonProperty("queuePresent")
    public void setQueuePresent(boolean value) { this.queuePresent = value; }

    /**
     * Queue severity when a queue is present.
     */
    @JsonProperty("severity")
    public Severity getSeverity() { return severity; }
    @JsonProperty("severity")
    public void setSeverity(Severity value) { this.severity = value; }

    /**
     * Time the measurement was taken.
     */
    @JsonProperty("timestamp")
    public OffsetDateTime getTimestamp() { return timestamp; }
    @JsonProperty("timestamp")
    public void setTimestamp(OffsetDateTime value) { this.timestamp = value; }
}
