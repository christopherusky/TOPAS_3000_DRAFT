/*
 * File:        output/java/sdk/com/topas/utmc/strategy/StrategyCommandResponse.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.strategy;

import com.fasterxml.jackson.annotation.*;
import java.time.OffsetDateTime;
import java.util.Map;

/**
 * Strategy command response payloads.
 */
public class StrategyCommandResponse {
    private String code;
    private long commandCorrelationID;
    private ErrorCode errorCode;
    private Map<String, Object> extensions;
    private String message;
    private OffsetDateTime responseTime;
    private String serviceRequester;
    private StrategyCommandResponseStatus status;
    private String strategyID;

    /**
     * Machine-readable response code for programmatic handling.
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
     * Strategy-specific rejection category. Only permitted when the outcome is not success or
     * pending.
     */
    @JsonProperty("errorCode")
    public ErrorCode getErrorCode() { return errorCode; }
    @JsonProperty("errorCode")
    public void setErrorCode(ErrorCode value) { this.errorCode = value; }

    /**
     * Vendor or local extensions.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * Human-readable detail.
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
     * Audit label copied from the command, if it carried one. Never used for authorisation or
     * idempotency.
     */
    @JsonProperty("serviceRequester")
    public String getServiceRequester() { return serviceRequester; }
    @JsonProperty("serviceRequester")
    public void setServiceRequester(String value) { this.serviceRequester = value; }

    /**
     * Outcome of the command execution.
     */
    @JsonProperty("status")
    public StrategyCommandResponseStatus getStatus() { return status; }
    @JsonProperty("status")
    public void setStatus(StrategyCommandResponseStatus value) { this.status = value; }

    /**
     * Identifier of the strategy the command targeted.
     */
    @JsonProperty("strategyId")
    public String getStrategyID() { return strategyID; }
    @JsonProperty("strategyId")
    public void setStrategyID(String value) { this.strategyID = value; }
}
