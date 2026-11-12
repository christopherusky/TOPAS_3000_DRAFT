/*
 * File:        output/java/sdk/com/topas/utmc/strategy/StrategyTrigger.java
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
 * Strategy trigger command payloads.
 */
public class StrategyTrigger {
    private long commandCorrelationID;
    private Map<String, Object> extensions;
    private OffsetDateTime requestTime;
    private String serviceRequester;
    private String strategyID;
    private TriggerState triggerState;

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
     * When the command was issued.
     */
    @JsonProperty("requestTime")
    public OffsetDateTime getRequestTime() { return requestTime; }
    @JsonProperty("requestTime")
    public void setRequestTime(OffsetDateTime value) { this.requestTime = value; }

    /**
     * Audit label naming the requesting system. Sender-supplied, so it is never used for
     * authorisation or idempotency.
     */
    @JsonProperty("serviceRequester")
    public String getServiceRequester() { return serviceRequester; }
    @JsonProperty("serviceRequester")
    public void setServiceRequester(String value) { this.serviceRequester = value; }

    /**
     * Identifier of the strategy to trigger.
     */
    @JsonProperty("strategyId")
    public String getStrategyID() { return strategyID; }
    @JsonProperty("strategyId")
    public void setStrategyID(String value) { this.strategyID = value; }

    /**
     * Desired trigger state (enabled/disabled).
     */
    @JsonProperty("triggerState")
    public TriggerState getTriggerState() { return triggerState; }
    @JsonProperty("triggerState")
    public void setTriggerState(TriggerState value) { this.triggerState = value; }
}
