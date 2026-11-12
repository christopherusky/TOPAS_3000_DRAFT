/*
 * File:        output/java/sdk/com/topas/utmc/strategy/AllTypes.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.strategy;

import com.fasterxml.jackson.annotation.*;

public class AllTypes {
    private StrategyCommandResponse strategyCommandResponse;
    private StrategyDefinition strategyDefinition;
    private StrategyStatusClass strategyStatus;
    private StrategyStatusEnumEnum strategyStatusEnum;
    private StrategyTrigger strategyTrigger;
    private SyncCategoryEnum syncCategoryEnum;
    private SyncComplete syncComplete;
    private SyncRequest syncRequest;
    private SyncStatusEnum syncStatus;
    private TriggerState triggerStateEnum;
    private ErrorCode triggerUpdateErrorEnum;

    @JsonProperty("StrategyCommandResponse")
    public StrategyCommandResponse getStrategyCommandResponse() { return strategyCommandResponse; }
    @JsonProperty("StrategyCommandResponse")
    public void setStrategyCommandResponse(StrategyCommandResponse value) { this.strategyCommandResponse = value; }

    @JsonProperty("StrategyDefinition")
    public StrategyDefinition getStrategyDefinition() { return strategyDefinition; }
    @JsonProperty("StrategyDefinition")
    public void setStrategyDefinition(StrategyDefinition value) { this.strategyDefinition = value; }

    @JsonProperty("StrategyStatus")
    public StrategyStatusClass getStrategyStatus() { return strategyStatus; }
    @JsonProperty("StrategyStatus")
    public void setStrategyStatus(StrategyStatusClass value) { this.strategyStatus = value; }

    @JsonProperty("StrategyStatusEnum")
    public StrategyStatusEnumEnum getStrategyStatusEnum() { return strategyStatusEnum; }
    @JsonProperty("StrategyStatusEnum")
    public void setStrategyStatusEnum(StrategyStatusEnumEnum value) { this.strategyStatusEnum = value; }

    @JsonProperty("StrategyTrigger")
    public StrategyTrigger getStrategyTrigger() { return strategyTrigger; }
    @JsonProperty("StrategyTrigger")
    public void setStrategyTrigger(StrategyTrigger value) { this.strategyTrigger = value; }

    @JsonProperty("SyncCategoryEnum")
    public SyncCategoryEnum getSyncCategoryEnum() { return syncCategoryEnum; }
    @JsonProperty("SyncCategoryEnum")
    public void setSyncCategoryEnum(SyncCategoryEnum value) { this.syncCategoryEnum = value; }

    @JsonProperty("SyncComplete")
    public SyncComplete getSyncComplete() { return syncComplete; }
    @JsonProperty("SyncComplete")
    public void setSyncComplete(SyncComplete value) { this.syncComplete = value; }

    @JsonProperty("SyncRequest")
    public SyncRequest getSyncRequest() { return syncRequest; }
    @JsonProperty("SyncRequest")
    public void setSyncRequest(SyncRequest value) { this.syncRequest = value; }

    @JsonProperty("SyncStatus")
    public SyncStatusEnum getSyncStatus() { return syncStatus; }
    @JsonProperty("SyncStatus")
    public void setSyncStatus(SyncStatusEnum value) { this.syncStatus = value; }

    @JsonProperty("TriggerStateEnum")
    public TriggerState getTriggerStateEnum() { return triggerStateEnum; }
    @JsonProperty("TriggerStateEnum")
    public void setTriggerStateEnum(TriggerState value) { this.triggerStateEnum = value; }

    @JsonProperty("TriggerUpdateErrorEnum")
    public ErrorCode getTriggerUpdateErrorEnum() { return triggerUpdateErrorEnum; }
    @JsonProperty("TriggerUpdateErrorEnum")
    public void setTriggerUpdateErrorEnum(ErrorCode value) { this.triggerUpdateErrorEnum = value; }
}
