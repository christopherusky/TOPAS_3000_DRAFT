/*
 * File:        output/java/sdk/com/topas/utmc/vms/DisplayGraphicCommand.java
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
 * Display a graphic on the sign.
 */
public class DisplayGraphicCommand {
    private long commandCorrelationID;
    private Long displayDurationSeconds;
    private Map<String, Object> extensions;
    private String graphicURL;
    private DisplayGraphicCommandKind kind;
    private OffsetDateTime requestTime;
    private String vmsID;

    /**
     * Caller-assigned identifier.
     */
    @JsonProperty("commandCorrelationId")
    public long getCommandCorrelationID() { return commandCorrelationID; }
    @JsonProperty("commandCorrelationId")
    public void setCommandCorrelationID(long value) { this.commandCorrelationID = value; }

    /**
     * How long to display the graphic.
     */
    @JsonProperty("displayDurationSeconds")
    public Long getDisplayDurationSeconds() { return displayDurationSeconds; }
    @JsonProperty("displayDurationSeconds")
    public void setDisplayDurationSeconds(Long value) { this.displayDurationSeconds = value; }

    /**
     * Vendor or local extensions.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * URL of the graphic to display.
     */
    @JsonProperty("graphicUrl")
    public String getGraphicURL() { return graphicURL; }
    @JsonProperty("graphicUrl")
    public void setGraphicURL(String value) { this.graphicURL = value; }

    /**
     * Discriminator for the tagged union. The fixed value "displayGraphic" selects the
     * DisplayGraphicCommand payload; it is not free-text command content.
     */
    @JsonProperty("kind")
    public DisplayGraphicCommandKind getKind() { return kind; }
    @JsonProperty("kind")
    public void setKind(DisplayGraphicCommandKind value) { this.kind = value; }

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
