/*
 * File:        output/java/sdk/com/topas/utmc/transport_link/SetStocCommand.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.transport_link;

import com.fasterxml.jackson.annotation.*;
import java.time.OffsetDateTime;
import java.util.Map;

/**
 * Set the SCOOT STOC threshold value on a transport link.
 */
public class SetStocCommand {
    private long commandCorrelationID;
    private Map<String, Object> extensions;
    private SetStocCommandKind kind;
    private OffsetDateTime requestTime;
    private String stocValue;
    private String transportLinkID;

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
     * Vendor or local extensions.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * Discriminator for the tagged union. The fixed value "setStoc" selects the SetStocCommand
     * payload; it is not free-text command content.
     */
    @JsonProperty("kind")
    public SetStocCommandKind getKind() { return kind; }
    @JsonProperty("kind")
    public void setKind(SetStocCommandKind value) { this.kind = value; }

    /**
     * When the command was issued.
     */
    @JsonProperty("requestTime")
    public OffsetDateTime getRequestTime() { return requestTime; }
    @JsonProperty("requestTime")
    public void setRequestTime(OffsetDateTime value) { this.requestTime = value; }

    /**
     * STOC value to set.
     */
    @JsonProperty("stocValue")
    public String getStocValue() { return stocValue; }
    @JsonProperty("stocValue")
    public void setStocValue(String value) { this.stocValue = value; }

    /**
     * Identifier of the transport link receiving the command.
     */
    @JsonProperty("transportLinkId")
    public String getTransportLinkID() { return transportLinkID; }
    @JsonProperty("transportLinkId")
    public void setTransportLinkID(String value) { this.transportLinkID = value; }
}
