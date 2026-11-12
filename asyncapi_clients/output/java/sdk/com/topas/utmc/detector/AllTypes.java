/*
 * File:        output/java/sdk/com/topas/utmc/detector/AllTypes.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.detector;

import com.fasterxml.jackson.annotation.*;

public class AllTypes {
    private CalibrateDetectorCommand calibrateDetectorCommand;
    private DetectorCommandResponse detectorCommandResponse;
    private DetectorConfiguration detectorConfiguration;
    private DetectorDefinition detectorDefinition;
    private DetectorFault detectorFault;
    private DetectorHistoryEntry detectorHistoryEntry;
    private DetectorType detectorType;
    private FlowDynamic flowDynamic;
    private HeadwayDynamic headwayDynamic;
    private OccupancyDynamic occupancyDynamic;
    private QueueDynamic queueDynamic;
    private ResetDetectorCommand resetDetectorCommand;
    private SetThresholdDetectorCommand setThresholdDetectorCommand;
    private SpeedDynamic speedDynamic;
    private SyncCategoryEnum syncCategoryEnum;
    private SyncComplete syncComplete;
    private SyncRequest syncRequest;
    private SyncStatusEnum syncStatus;

    @JsonProperty("CalibrateDetectorCommand")
    public CalibrateDetectorCommand getCalibrateDetectorCommand() { return calibrateDetectorCommand; }
    @JsonProperty("CalibrateDetectorCommand")
    public void setCalibrateDetectorCommand(CalibrateDetectorCommand value) { this.calibrateDetectorCommand = value; }

    @JsonProperty("DetectorCommandResponse")
    public DetectorCommandResponse getDetectorCommandResponse() { return detectorCommandResponse; }
    @JsonProperty("DetectorCommandResponse")
    public void setDetectorCommandResponse(DetectorCommandResponse value) { this.detectorCommandResponse = value; }

    @JsonProperty("DetectorConfiguration")
    public DetectorConfiguration getDetectorConfiguration() { return detectorConfiguration; }
    @JsonProperty("DetectorConfiguration")
    public void setDetectorConfiguration(DetectorConfiguration value) { this.detectorConfiguration = value; }

    @JsonProperty("DetectorDefinition")
    public DetectorDefinition getDetectorDefinition() { return detectorDefinition; }
    @JsonProperty("DetectorDefinition")
    public void setDetectorDefinition(DetectorDefinition value) { this.detectorDefinition = value; }

    @JsonProperty("DetectorFault")
    public DetectorFault getDetectorFault() { return detectorFault; }
    @JsonProperty("DetectorFault")
    public void setDetectorFault(DetectorFault value) { this.detectorFault = value; }

    @JsonProperty("DetectorHistoryEntry")
    public DetectorHistoryEntry getDetectorHistoryEntry() { return detectorHistoryEntry; }
    @JsonProperty("DetectorHistoryEntry")
    public void setDetectorHistoryEntry(DetectorHistoryEntry value) { this.detectorHistoryEntry = value; }

    @JsonProperty("DetectorType")
    public DetectorType getDetectorType() { return detectorType; }
    @JsonProperty("DetectorType")
    public void setDetectorType(DetectorType value) { this.detectorType = value; }

    @JsonProperty("FlowDynamic")
    public FlowDynamic getFlowDynamic() { return flowDynamic; }
    @JsonProperty("FlowDynamic")
    public void setFlowDynamic(FlowDynamic value) { this.flowDynamic = value; }

    @JsonProperty("HeadwayDynamic")
    public HeadwayDynamic getHeadwayDynamic() { return headwayDynamic; }
    @JsonProperty("HeadwayDynamic")
    public void setHeadwayDynamic(HeadwayDynamic value) { this.headwayDynamic = value; }

    @JsonProperty("OccupancyDynamic")
    public OccupancyDynamic getOccupancyDynamic() { return occupancyDynamic; }
    @JsonProperty("OccupancyDynamic")
    public void setOccupancyDynamic(OccupancyDynamic value) { this.occupancyDynamic = value; }

    @JsonProperty("QueueDynamic")
    public QueueDynamic getQueueDynamic() { return queueDynamic; }
    @JsonProperty("QueueDynamic")
    public void setQueueDynamic(QueueDynamic value) { this.queueDynamic = value; }

    @JsonProperty("ResetDetectorCommand")
    public ResetDetectorCommand getResetDetectorCommand() { return resetDetectorCommand; }
    @JsonProperty("ResetDetectorCommand")
    public void setResetDetectorCommand(ResetDetectorCommand value) { this.resetDetectorCommand = value; }

    @JsonProperty("SetThresholdDetectorCommand")
    public SetThresholdDetectorCommand getSetThresholdDetectorCommand() { return setThresholdDetectorCommand; }
    @JsonProperty("SetThresholdDetectorCommand")
    public void setSetThresholdDetectorCommand(SetThresholdDetectorCommand value) { this.setThresholdDetectorCommand = value; }

    @JsonProperty("SpeedDynamic")
    public SpeedDynamic getSpeedDynamic() { return speedDynamic; }
    @JsonProperty("SpeedDynamic")
    public void setSpeedDynamic(SpeedDynamic value) { this.speedDynamic = value; }

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
}
