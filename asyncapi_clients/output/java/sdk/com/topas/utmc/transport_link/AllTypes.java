/*
 * File:        output/java/sdk/com/topas/utmc/transport_link/AllTypes.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.transport_link;

import com.fasterxml.jackson.annotation.*;

public class AllTypes {
    private LinkStatusType linkStatusType;
    private RecalibrateTransportLinkCommand recalibrateTransportLinkCommand;
    private RequestTransportLinkStatusCommand requestTransportLinkStatusCommand;
    private SetStocCommand setStocCommand;
    private SyncCategoryEnum syncCategoryEnum;
    private SyncComplete syncComplete;
    private SyncRequest syncRequest;
    private SyncStatusEnum syncStatus;
    private TransportLinkAnprConfiguration transportLinkANPRConfiguration;
    private TransportLinkAnprDynamic transportLinkANPRDynamic;
    private TransportLinkCommandResponse transportLinkCommandResponse;
    private TransportLinkDefinition transportLinkDefinition;
    private TransportLinkFault transportLinkFault;
    private TransportLinkGpsConfiguration transportLinkGPSConfiguration;
    private TransportLinkGpsDynamic transportLinkGPSDynamic;
    private TransportLinkHistoryEntry transportLinkHistoryEntry;
    private TransportLinkScootConfiguration transportLinkSCOOTConfiguration;
    private TransportLinkScootDynamic transportLinkSCOOTDynamic;
    private TransportLinkType transportLinkType;

    @JsonProperty("LinkStatusType")
    public LinkStatusType getLinkStatusType() { return linkStatusType; }
    @JsonProperty("LinkStatusType")
    public void setLinkStatusType(LinkStatusType value) { this.linkStatusType = value; }

    @JsonProperty("RecalibrateTransportLinkCommand")
    public RecalibrateTransportLinkCommand getRecalibrateTransportLinkCommand() { return recalibrateTransportLinkCommand; }
    @JsonProperty("RecalibrateTransportLinkCommand")
    public void setRecalibrateTransportLinkCommand(RecalibrateTransportLinkCommand value) { this.recalibrateTransportLinkCommand = value; }

    @JsonProperty("RequestTransportLinkStatusCommand")
    public RequestTransportLinkStatusCommand getRequestTransportLinkStatusCommand() { return requestTransportLinkStatusCommand; }
    @JsonProperty("RequestTransportLinkStatusCommand")
    public void setRequestTransportLinkStatusCommand(RequestTransportLinkStatusCommand value) { this.requestTransportLinkStatusCommand = value; }

    @JsonProperty("SetStocCommand")
    public SetStocCommand getSetStocCommand() { return setStocCommand; }
    @JsonProperty("SetStocCommand")
    public void setSetStocCommand(SetStocCommand value) { this.setStocCommand = value; }

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

    @JsonProperty("TransportLinkANPRConfiguration")
    public TransportLinkAnprConfiguration getTransportLinkANPRConfiguration() { return transportLinkANPRConfiguration; }
    @JsonProperty("TransportLinkANPRConfiguration")
    public void setTransportLinkANPRConfiguration(TransportLinkAnprConfiguration value) { this.transportLinkANPRConfiguration = value; }

    @JsonProperty("TransportLinkANPRDynamic")
    public TransportLinkAnprDynamic getTransportLinkANPRDynamic() { return transportLinkANPRDynamic; }
    @JsonProperty("TransportLinkANPRDynamic")
    public void setTransportLinkANPRDynamic(TransportLinkAnprDynamic value) { this.transportLinkANPRDynamic = value; }

    @JsonProperty("TransportLinkCommandResponse")
    public TransportLinkCommandResponse getTransportLinkCommandResponse() { return transportLinkCommandResponse; }
    @JsonProperty("TransportLinkCommandResponse")
    public void setTransportLinkCommandResponse(TransportLinkCommandResponse value) { this.transportLinkCommandResponse = value; }

    @JsonProperty("TransportLinkDefinition")
    public TransportLinkDefinition getTransportLinkDefinition() { return transportLinkDefinition; }
    @JsonProperty("TransportLinkDefinition")
    public void setTransportLinkDefinition(TransportLinkDefinition value) { this.transportLinkDefinition = value; }

    @JsonProperty("TransportLinkFault")
    public TransportLinkFault getTransportLinkFault() { return transportLinkFault; }
    @JsonProperty("TransportLinkFault")
    public void setTransportLinkFault(TransportLinkFault value) { this.transportLinkFault = value; }

    @JsonProperty("TransportLinkGPSConfiguration")
    public TransportLinkGpsConfiguration getTransportLinkGPSConfiguration() { return transportLinkGPSConfiguration; }
    @JsonProperty("TransportLinkGPSConfiguration")
    public void setTransportLinkGPSConfiguration(TransportLinkGpsConfiguration value) { this.transportLinkGPSConfiguration = value; }

    @JsonProperty("TransportLinkGPSDynamic")
    public TransportLinkGpsDynamic getTransportLinkGPSDynamic() { return transportLinkGPSDynamic; }
    @JsonProperty("TransportLinkGPSDynamic")
    public void setTransportLinkGPSDynamic(TransportLinkGpsDynamic value) { this.transportLinkGPSDynamic = value; }

    @JsonProperty("TransportLinkHistoryEntry")
    public TransportLinkHistoryEntry getTransportLinkHistoryEntry() { return transportLinkHistoryEntry; }
    @JsonProperty("TransportLinkHistoryEntry")
    public void setTransportLinkHistoryEntry(TransportLinkHistoryEntry value) { this.transportLinkHistoryEntry = value; }

    @JsonProperty("TransportLinkSCOOTConfiguration")
    public TransportLinkScootConfiguration getTransportLinkSCOOTConfiguration() { return transportLinkSCOOTConfiguration; }
    @JsonProperty("TransportLinkSCOOTConfiguration")
    public void setTransportLinkSCOOTConfiguration(TransportLinkScootConfiguration value) { this.transportLinkSCOOTConfiguration = value; }

    @JsonProperty("TransportLinkSCOOTDynamic")
    public TransportLinkScootDynamic getTransportLinkSCOOTDynamic() { return transportLinkSCOOTDynamic; }
    @JsonProperty("TransportLinkSCOOTDynamic")
    public void setTransportLinkSCOOTDynamic(TransportLinkScootDynamic value) { this.transportLinkSCOOTDynamic = value; }

    @JsonProperty("TransportLinkType")
    public TransportLinkType getTransportLinkType() { return transportLinkType; }
    @JsonProperty("TransportLinkType")
    public void setTransportLinkType(TransportLinkType value) { this.transportLinkType = value; }
}
