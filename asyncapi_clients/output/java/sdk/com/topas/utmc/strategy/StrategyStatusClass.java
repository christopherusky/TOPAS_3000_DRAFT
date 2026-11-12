/*
 * File:        output/java/sdk/com/topas/utmc/strategy/StrategyStatusClass.java
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
 * Traffic management strategy current status.
 */
public class StrategyStatusClass {
    private String errorMessage;
    private Map<String, Object> extensions;
    private String statusMessage;
    private OffsetDateTime strategyChangeStateTime;
    private String strategyID;
    private StrategyStatusEnumEnum strategyStatus;

    /**
     * Error information (e.g. 'Strategy will never become active').
     */
    @JsonProperty("errorMessage")
    public String getErrorMessage() { return errorMessage; }
    @JsonProperty("errorMessage")
    public void setErrorMessage(String value) { this.errorMessage = value; }

    /**
     * Vendor or local extensions.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * Non-error status information (e.g. 'Strategy applied 6 of 10 measures').
     */
    @JsonProperty("statusMessage")
    public String getStatusMessage() { return statusMessage; }
    @JsonProperty("statusMessage")
    public void setStatusMessage(String value) { this.statusMessage = value; }

    /**
     * When the strategy status changed.
     */
    @JsonProperty("strategyChangeStateTime")
    public OffsetDateTime getStrategyChangeStateTime() { return strategyChangeStateTime; }
    @JsonProperty("strategyChangeStateTime")
    public void setStrategyChangeStateTime(OffsetDateTime value) { this.strategyChangeStateTime = value; }

    /**
     * Identifier of the strategy this status is about.
     */
    @JsonProperty("strategyId")
    public String getStrategyID() { return strategyID; }
    @JsonProperty("strategyId")
    public void setStrategyID(String value) { this.strategyID = value; }

    /**
     * Current status (active/inactive).
     */
    @JsonProperty("strategyStatus")
    public StrategyStatusEnumEnum getStrategyStatus() { return strategyStatus; }
    @JsonProperty("strategyStatus")
    public void setStrategyStatus(StrategyStatusEnumEnum value) { this.strategyStatus = value; }
}
