/*
 * File:        output/java/sdk/com/topas/utmc/transport_route/TransportRouteCommandResponse.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.transport_route;

import com.fasterxml.jackson.annotation.*;
import java.time.OffsetDateTime;
import java.util.Map;

/**
 * Transport route command response payloads.
 */
public class TransportRouteCommandResponse {
    private String code;
    private long commandCorrelationID;
    private Map<String, Object> extensions;
    private String message;
    private OffsetDateTime responseTime;
    private TransportRouteCommandResponseStatus status;
    private String transportRouteID;

    /**
     * Machine-readable response code.
     */
    @JsonProperty("code")
    public String getCode() { return code; }
    @JsonProperty("code")
    public void setCode(String value) { this.code = value; }

    /**
     * Correlation identifier copied verbatim from the originating command request. Not an
     * identifier of the command itself.
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
     * Human-readable response message.
     */
    @JsonProperty("message")
    public String getMessage() { return message; }
    @JsonProperty("message")
    public void setMessage(String value) { this.message = value; }

    /**
     * When the response was generated.
     */
    @JsonProperty("responseTime")
    public OffsetDateTime getResponseTime() { return responseTime; }
    @JsonProperty("responseTime")
    public void setResponseTime(OffsetDateTime value) { this.responseTime = value; }

    /**
     * Outcome of the command execution.
     */
    @JsonProperty("status")
    public TransportRouteCommandResponseStatus getStatus() { return status; }
    @JsonProperty("status")
    public void setStatus(TransportRouteCommandResponseStatus value) { this.status = value; }

    /**
     * Identifier of the route the command targeted.
     */
    @JsonProperty("transportRouteId")
    public String getTransportRouteID() { return transportRouteID; }
    @JsonProperty("transportRouteId")
    public void setTransportRouteID(String value) { this.transportRouteID = value; }
}
