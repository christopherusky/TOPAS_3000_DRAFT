/*
 * File:        output/java/sdk/com/topas/utmc/vms/AllTypes.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.vms;

import com.fasterxml.jackson.annotation.*;

public class AllTypes {
    private ClearSignCommand clearSignCommand;
    private DisplayGraphicCommand displayGraphicCommand;
    private DisplayMessageCommand displayMessageCommand;
    private SetBrightnessCommand setBrightnessCommand;
    private SyncCategoryEnum syncCategoryEnum;
    private SyncComplete syncComplete;
    private SyncRequest syncRequest;
    private SyncStatusEnum syncStatus;
    private VmsColourCapability vmsColourCapability;
    private VmsCommandResponse vmsCommandResponse;
    private VmsConfiguration vmsConfiguration;
    private VmsDefinition vmsDefinition;
    private VmsDynamic vmsDynamic;
    private VmsFault vmsFault;
    private VmsHistoryEntry vmsHistoryEntry;
    private VmsStatusEnum vmsStatus;
    private VmsType vmsType;

    @JsonProperty("ClearSignCommand")
    public ClearSignCommand getClearSignCommand() { return clearSignCommand; }
    @JsonProperty("ClearSignCommand")
    public void setClearSignCommand(ClearSignCommand value) { this.clearSignCommand = value; }

    @JsonProperty("DisplayGraphicCommand")
    public DisplayGraphicCommand getDisplayGraphicCommand() { return displayGraphicCommand; }
    @JsonProperty("DisplayGraphicCommand")
    public void setDisplayGraphicCommand(DisplayGraphicCommand value) { this.displayGraphicCommand = value; }

    @JsonProperty("DisplayMessageCommand")
    public DisplayMessageCommand getDisplayMessageCommand() { return displayMessageCommand; }
    @JsonProperty("DisplayMessageCommand")
    public void setDisplayMessageCommand(DisplayMessageCommand value) { this.displayMessageCommand = value; }

    @JsonProperty("SetBrightnessCommand")
    public SetBrightnessCommand getSetBrightnessCommand() { return setBrightnessCommand; }
    @JsonProperty("SetBrightnessCommand")
    public void setSetBrightnessCommand(SetBrightnessCommand value) { this.setBrightnessCommand = value; }

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

    @JsonProperty("VmsColourCapability")
    public VmsColourCapability getVmsColourCapability() { return vmsColourCapability; }
    @JsonProperty("VmsColourCapability")
    public void setVmsColourCapability(VmsColourCapability value) { this.vmsColourCapability = value; }

    @JsonProperty("VmsCommandResponse")
    public VmsCommandResponse getVmsCommandResponse() { return vmsCommandResponse; }
    @JsonProperty("VmsCommandResponse")
    public void setVmsCommandResponse(VmsCommandResponse value) { this.vmsCommandResponse = value; }

    @JsonProperty("VmsConfiguration")
    public VmsConfiguration getVmsConfiguration() { return vmsConfiguration; }
    @JsonProperty("VmsConfiguration")
    public void setVmsConfiguration(VmsConfiguration value) { this.vmsConfiguration = value; }

    @JsonProperty("VmsDefinition")
    public VmsDefinition getVmsDefinition() { return vmsDefinition; }
    @JsonProperty("VmsDefinition")
    public void setVmsDefinition(VmsDefinition value) { this.vmsDefinition = value; }

    @JsonProperty("VmsDynamic")
    public VmsDynamic getVmsDynamic() { return vmsDynamic; }
    @JsonProperty("VmsDynamic")
    public void setVmsDynamic(VmsDynamic value) { this.vmsDynamic = value; }

    @JsonProperty("VmsFault")
    public VmsFault getVmsFault() { return vmsFault; }
    @JsonProperty("VmsFault")
    public void setVmsFault(VmsFault value) { this.vmsFault = value; }

    @JsonProperty("VmsHistoryEntry")
    public VmsHistoryEntry getVmsHistoryEntry() { return vmsHistoryEntry; }
    @JsonProperty("VmsHistoryEntry")
    public void setVmsHistoryEntry(VmsHistoryEntry value) { this.vmsHistoryEntry = value; }

    @JsonProperty("VmsStatus")
    public VmsStatusEnum getVmsStatus() { return vmsStatus; }
    @JsonProperty("VmsStatus")
    public void setVmsStatus(VmsStatusEnum value) { this.vmsStatus = value; }

    @JsonProperty("VmsType")
    public VmsType getVmsType() { return vmsType; }
    @JsonProperty("VmsType")
    public void setVmsType(VmsType value) { this.vmsType = value; }
}
