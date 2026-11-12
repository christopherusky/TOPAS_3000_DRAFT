/*
 * File:        output/java/sdk/com/topas/utmc/vms/SetBrightnessCommand.java
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
 * Set the brightness of the sign.
 */
public class SetBrightnessCommand {
    private double brightnessPercent;
    private long commandCorrelationID;
    private Map<String, Object> extensions;
    private SetBrightnessCommandKind kind;
    private OffsetDateTime requestTime;
    private String vmsID;

    /**
     * Brightness percentage to set.
     */
    @JsonProperty("brightnessPercent")
    public double getBrightnessPercent() { return brightnessPercent; }
    @JsonProperty("brightnessPercent")
    public void setBrightnessPercent(double value) { this.brightnessPercent = value; }

    /**
     * Caller-assigned identifier.
     */
    @JsonProperty("commandCorrelationId")
    public long getCommandCorrelationID() { return commandCorrelationID; }
    @JsonProperty("commandCorrelationId")
    public void setCommandCorrelationID(long value) { this.commandCorrelationID = value; }

    /**
     * Vendor or local extensions.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * Discriminator for the tagged union. The fixed value "setBrightness" selects the
     * SetBrightnessCommand payload; it is not free-text command content.
     */
    @JsonProperty("kind")
    public SetBrightnessCommandKind getKind() { return kind; }
    @JsonProperty("kind")
    public void setKind(SetBrightnessCommandKind value) { this.kind = value; }

    /**
     * When the command was issued.
     */
    @JsonProperty("requestTime")
    public OffsetDateTime getRequestTime() { return requestTime; }
    @JsonProperty("requestTime")
    public void setRequestTime(OffsetDateTime value) { this.requestTime = value; }

    /**
     * Identifier of the VMS receiving the command.
     */
    @JsonProperty("vmsId")
    public String getVmsID() { return vmsID; }
    @JsonProperty("vmsId")
    public void setVmsID(String value) { this.vmsID = value; }
}
