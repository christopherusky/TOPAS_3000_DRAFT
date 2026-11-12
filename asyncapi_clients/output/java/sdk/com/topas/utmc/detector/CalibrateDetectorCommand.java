/*
 * File:        output/java/sdk/com/topas/utmc/detector/CalibrateDetectorCommand.java
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
 * Trigger a calibration cycle on the detector.
 */
public class CalibrateDetectorCommand {
    private long commandCorrelationID;
    private String detectorID;
    private Map<String, Object> extensions;
    private CalibrateDetectorCommandKind kind;
    private OffsetDateTime requestTime;

    /**
     * Caller-assigned identifier correlating a command request with its response. Not an
     * identifier of the command itself. Named to avoid collision with the AMQP 1.0
     * correlation-id message property.
     */
    @JsonProperty("commandCorrelationId")
    public long getCommandCorrelationID() { return commandCorrelationID; }
    @JsonProperty("commandCorrelationId")
    public void setCommandCorrelationID(long value) { this.commandCorrelationID = value; }

    /**
     * Identifier of the detector receiving the command.
     */
    @JsonProperty("detectorId")
    public String getDetectorID() { return detectorID; }
    @JsonProperty("detectorId")
    public void setDetectorID(String value) { this.detectorID = value; }

    /**
     * Vendor or local extensions.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * Discriminator for the tagged union. The fixed value "calibrate" selects the
     * CalibrateDetectorCommand payload; it is not free-text command content.
     */
    @JsonProperty("kind")
    public CalibrateDetectorCommandKind getKind() { return kind; }
    @JsonProperty("kind")
    public void setKind(CalibrateDetectorCommandKind value) { this.kind = value; }

    /**
     * When the command was issued.
     */
    @JsonProperty("requestTime")
    public OffsetDateTime getRequestTime() { return requestTime; }
    @JsonProperty("requestTime")
    public void setRequestTime(OffsetDateTime value) { this.requestTime = value; }
}
