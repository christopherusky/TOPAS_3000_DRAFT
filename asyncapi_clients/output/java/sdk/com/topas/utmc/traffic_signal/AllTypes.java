/*
 * File:        output/java/sdk/com/topas/utmc/traffic_signal/AllTypes.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.traffic_signal;

import com.fasterxml.jackson.annotation.*;

public class AllTypes {
    private FaultSeverityEnum faultSeverityEnum;
    private ForceStageCommand forceStageCommand;
    private HoldStageCommand holdStageCommand;
    private PlanLine planLine;
    private RequestStatusCommand requestStatusCommand;
    private SetPlanCommand setPlanCommand;
    private Composite signalGroupTimings;
    private SyncCategoryEnum syncCategoryEnum;
    private SyncComplete syncComplete;
    private SynchronizeClockCommand synchronizeClockCommand;
    private SyncRequest syncRequest;
    private SyncStatusEnum syncStatus;
    private TrafficSignalCommandResponse trafficSignalCommandResponse;
    private TrafficSignalConfiguration trafficSignalConfiguration;
    private TrafficSignalDefinition trafficSignalDefinition;
    private TrafficSignalDynamicState trafficSignalDynamicState;
    private TrafficSignalFault trafficSignalFault;
    private TrafficSignalFaultType trafficSignalFaultEnum;
    private TrafficSignalHistoryEntry trafficSignalHistoryEntry;
    private TrafficSignalMode trafficSignalModeEnum;
    private TrafficSignalSubMode trafficSignalSubModeEnum;
    private TrafficSignalTypeClass trafficSignalType;
    private TrafficSignalTypeEnumEnum trafficSignalTypeEnum;
    private TrafficSignalFaultUrgency urgencyEnum;

    @JsonProperty("FaultSeverityEnum")
    public FaultSeverityEnum getFaultSeverityEnum() { return faultSeverityEnum; }
    @JsonProperty("FaultSeverityEnum")
    public void setFaultSeverityEnum(FaultSeverityEnum value) { this.faultSeverityEnum = value; }

    @JsonProperty("ForceStageCommand")
    public ForceStageCommand getForceStageCommand() { return forceStageCommand; }
    @JsonProperty("ForceStageCommand")
    public void setForceStageCommand(ForceStageCommand value) { this.forceStageCommand = value; }

    @JsonProperty("HoldStageCommand")
    public HoldStageCommand getHoldStageCommand() { return holdStageCommand; }
    @JsonProperty("HoldStageCommand")
    public void setHoldStageCommand(HoldStageCommand value) { this.holdStageCommand = value; }

    @JsonProperty("PlanLine")
    public PlanLine getPlanLine() { return planLine; }
    @JsonProperty("PlanLine")
    public void setPlanLine(PlanLine value) { this.planLine = value; }

    @JsonProperty("RequestStatusCommand")
    public RequestStatusCommand getRequestStatusCommand() { return requestStatusCommand; }
    @JsonProperty("RequestStatusCommand")
    public void setRequestStatusCommand(RequestStatusCommand value) { this.requestStatusCommand = value; }

    @JsonProperty("SetPlanCommand")
    public SetPlanCommand getSetPlanCommand() { return setPlanCommand; }
    @JsonProperty("SetPlanCommand")
    public void setSetPlanCommand(SetPlanCommand value) { this.setPlanCommand = value; }

    @JsonProperty("SignalGroupTimings")
    public Composite getSignalGroupTimings() { return signalGroupTimings; }
    @JsonProperty("SignalGroupTimings")
    public void setSignalGroupTimings(Composite value) { this.signalGroupTimings = value; }

    @JsonProperty("SyncCategoryEnum")
    public SyncCategoryEnum getSyncCategoryEnum() { return syncCategoryEnum; }
    @JsonProperty("SyncCategoryEnum")
    public void setSyncCategoryEnum(SyncCategoryEnum value) { this.syncCategoryEnum = value; }

    @JsonProperty("SyncComplete")
    public SyncComplete getSyncComplete() { return syncComplete; }
    @JsonProperty("SyncComplete")
    public void setSyncComplete(SyncComplete value) { this.syncComplete = value; }

    @JsonProperty("SynchronizeClockCommand")
    public SynchronizeClockCommand getSynchronizeClockCommand() { return synchronizeClockCommand; }
    @JsonProperty("SynchronizeClockCommand")
    public void setSynchronizeClockCommand(SynchronizeClockCommand value) { this.synchronizeClockCommand = value; }

    @JsonProperty("SyncRequest")
    public SyncRequest getSyncRequest() { return syncRequest; }
    @JsonProperty("SyncRequest")
    public void setSyncRequest(SyncRequest value) { this.syncRequest = value; }

    @JsonProperty("SyncStatus")
    public SyncStatusEnum getSyncStatus() { return syncStatus; }
    @JsonProperty("SyncStatus")
    public void setSyncStatus(SyncStatusEnum value) { this.syncStatus = value; }

    @JsonProperty("TrafficSignalCommandResponse")
    public TrafficSignalCommandResponse getTrafficSignalCommandResponse() { return trafficSignalCommandResponse; }
    @JsonProperty("TrafficSignalCommandResponse")
    public void setTrafficSignalCommandResponse(TrafficSignalCommandResponse value) { this.trafficSignalCommandResponse = value; }

    @JsonProperty("TrafficSignalConfiguration")
    public TrafficSignalConfiguration getTrafficSignalConfiguration() { return trafficSignalConfiguration; }
    @JsonProperty("TrafficSignalConfiguration")
    public void setTrafficSignalConfiguration(TrafficSignalConfiguration value) { this.trafficSignalConfiguration = value; }

    @JsonProperty("TrafficSignalDefinition")
    public TrafficSignalDefinition getTrafficSignalDefinition() { return trafficSignalDefinition; }
    @JsonProperty("TrafficSignalDefinition")
    public void setTrafficSignalDefinition(TrafficSignalDefinition value) { this.trafficSignalDefinition = value; }

    @JsonProperty("TrafficSignalDynamicState")
    public TrafficSignalDynamicState getTrafficSignalDynamicState() { return trafficSignalDynamicState; }
    @JsonProperty("TrafficSignalDynamicState")
    public void setTrafficSignalDynamicState(TrafficSignalDynamicState value) { this.trafficSignalDynamicState = value; }

    @JsonProperty("TrafficSignalFault")
    public TrafficSignalFault getTrafficSignalFault() { return trafficSignalFault; }
    @JsonProperty("TrafficSignalFault")
    public void setTrafficSignalFault(TrafficSignalFault value) { this.trafficSignalFault = value; }

    @JsonProperty("TrafficSignalFaultEnum")
    public TrafficSignalFaultType getTrafficSignalFaultEnum() { return trafficSignalFaultEnum; }
    @JsonProperty("TrafficSignalFaultEnum")
    public void setTrafficSignalFaultEnum(TrafficSignalFaultType value) { this.trafficSignalFaultEnum = value; }

    @JsonProperty("TrafficSignalHistoryEntry")
    public TrafficSignalHistoryEntry getTrafficSignalHistoryEntry() { return trafficSignalHistoryEntry; }
    @JsonProperty("TrafficSignalHistoryEntry")
    public void setTrafficSignalHistoryEntry(TrafficSignalHistoryEntry value) { this.trafficSignalHistoryEntry = value; }

    @JsonProperty("TrafficSignalModeEnum")
    public TrafficSignalMode getTrafficSignalModeEnum() { return trafficSignalModeEnum; }
    @JsonProperty("TrafficSignalModeEnum")
    public void setTrafficSignalModeEnum(TrafficSignalMode value) { this.trafficSignalModeEnum = value; }

    @JsonProperty("TrafficSignalSubModeEnum")
    public TrafficSignalSubMode getTrafficSignalSubModeEnum() { return trafficSignalSubModeEnum; }
    @JsonProperty("TrafficSignalSubModeEnum")
    public void setTrafficSignalSubModeEnum(TrafficSignalSubMode value) { this.trafficSignalSubModeEnum = value; }

    @JsonProperty("TrafficSignalType")
    public TrafficSignalTypeClass getTrafficSignalType() { return trafficSignalType; }
    @JsonProperty("TrafficSignalType")
    public void setTrafficSignalType(TrafficSignalTypeClass value) { this.trafficSignalType = value; }

    @JsonProperty("TrafficSignalTypeEnum")
    public TrafficSignalTypeEnumEnum getTrafficSignalTypeEnum() { return trafficSignalTypeEnum; }
    @JsonProperty("TrafficSignalTypeEnum")
    public void setTrafficSignalTypeEnum(TrafficSignalTypeEnumEnum value) { this.trafficSignalTypeEnum = value; }

    @JsonProperty("UrgencyEnum")
    public TrafficSignalFaultUrgency getUrgencyEnum() { return urgencyEnum; }
    @JsonProperty("UrgencyEnum")
    public void setUrgencyEnum(TrafficSignalFaultUrgency value) { this.urgencyEnum = value; }
}
