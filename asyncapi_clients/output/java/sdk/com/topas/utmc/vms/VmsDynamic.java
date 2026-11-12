/*
 * File:        output/java/sdk/com/topas/utmc/vms/VmsDynamic.java
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
 * Variable message sign dynamic measurement data.
 */
public class VmsDynamic {
    private Double brightnessPercent;
    private String currentMessage;
    private Map<String, Object> extensions;
    private String graphicURL;
    private OffsetDateTime messageExpiryTime;
    private String[] messageLines;
    private OffsetDateTime messageSetTime;
    private Quality quality;
    private VmsStatusEnum status;
    private OffsetDateTime timestamp;
    private String vmsID;

    /**
     * Current brightness percentage.
     */
    @JsonProperty("brightnessPercent")
    public Double getBrightnessPercent() { return brightnessPercent; }
    @JsonProperty("brightnessPercent")
    public void setBrightnessPercent(Double value) { this.brightnessPercent = value; }

    /**
     * Currently displayed message text.
     */
    @JsonProperty("currentMessage")
    public String getCurrentMessage() { return currentMessage; }
    @JsonProperty("currentMessage")
    public void setCurrentMessage(String value) { this.currentMessage = value; }

    /**
     * Vendor or local extensions.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * URL of the currently displayed graphic.
     */
    @JsonProperty("graphicUrl")
    public String getGraphicURL() { return graphicURL; }
    @JsonProperty("graphicUrl")
    public void setGraphicURL(String value) { this.graphicURL = value; }

    /**
     * When the message expires.
     */
    @JsonProperty("messageExpiryTime")
    public OffsetDateTime getMessageExpiryTime() { return messageExpiryTime; }
    @JsonProperty("messageExpiryTime")
    public void setMessageExpiryTime(OffsetDateTime value) { this.messageExpiryTime = value; }

    /**
     * Message split by lines.
     */
    @JsonProperty("messageLines")
    public String[] getMessageLines() { return messageLines; }
    @JsonProperty("messageLines")
    public void setMessageLines(String[] value) { this.messageLines = value; }

    /**
     * When the current message was set.
     */
    @JsonProperty("messageSetTime")
    public OffsetDateTime getMessageSetTime() { return messageSetTime; }
    @JsonProperty("messageSetTime")
    public void setMessageSetTime(OffsetDateTime value) { this.messageSetTime = value; }

    /**
     * Optional quality statement.
     */
    @JsonProperty("quality")
    public Quality getQuality() { return quality; }
    @JsonProperty("quality")
    public void setQuality(Quality value) { this.quality = value; }

    /**
     * Operational status.
     */
    @JsonProperty("status")
    public VmsStatusEnum getStatus() { return status; }
    @JsonProperty("status")
    public void setStatus(VmsStatusEnum value) { this.status = value; }

    /**
     * Time of the observation.
     */
    @JsonProperty("timestamp")
    public OffsetDateTime getTimestamp() { return timestamp; }
    @JsonProperty("timestamp")
    public void setTimestamp(OffsetDateTime value) { this.timestamp = value; }

    /**
     * Identifier of the VMS.
     */
    @JsonProperty("vmsId")
    public String getVmsID() { return vmsID; }
    @JsonProperty("vmsId")
    public void setVmsID(String value) { this.vmsID = value; }
}
