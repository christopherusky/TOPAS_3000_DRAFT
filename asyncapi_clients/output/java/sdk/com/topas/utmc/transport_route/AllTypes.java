/*
 * File:        output/java/sdk/com/topas/utmc/transport_route/AllTypes.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.transport_route;

import com.fasterxml.jackson.annotation.*;

public class AllTypes {
    private RequestTransportRouteStatusCommand requestTransportRouteStatusCommand;
    private SyncCategoryEnum syncCategoryEnum;
    private SyncComplete syncComplete;
    private SyncRequest syncRequest;
    private SyncStatusEnum syncStatus;
    private TransportRouteCommandResponse transportRouteCommandResponse;
    private TransportRouteConfiguration transportRouteConfiguration;
    private TransportRouteDefinition transportRouteDefinition;
    private TransportRouteDynamic transportRouteDynamic;
    private TransportRouteFault transportRouteFault;
    private TransportRouteHistoryEntry transportRouteHistoryEntry;
    private TransportRouteSegmentConfiguration transportRouteSegmentConfiguration;
    private RouteStatus transportRouteStatus;
    private TransportRouteType transportRouteType;

    @JsonProperty("RequestTransportRouteStatusCommand")
    public RequestTransportRouteStatusCommand getRequestTransportRouteStatusCommand() { return requestTransportRouteStatusCommand; }
    @JsonProperty("RequestTransportRouteStatusCommand")
    public void setRequestTransportRouteStatusCommand(RequestTransportRouteStatusCommand value) { this.requestTransportRouteStatusCommand = value; }

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

    @JsonProperty("TransportRouteCommandResponse")
    public TransportRouteCommandResponse getTransportRouteCommandResponse() { return transportRouteCommandResponse; }
    @JsonProperty("TransportRouteCommandResponse")
    public void setTransportRouteCommandResponse(TransportRouteCommandResponse value) { this.transportRouteCommandResponse = value; }

    @JsonProperty("TransportRouteConfiguration")
    public TransportRouteConfiguration getTransportRouteConfiguration() { return transportRouteConfiguration; }
    @JsonProperty("TransportRouteConfiguration")
    public void setTransportRouteConfiguration(TransportRouteConfiguration value) { this.transportRouteConfiguration = value; }

    @JsonProperty("TransportRouteDefinition")
    public TransportRouteDefinition getTransportRouteDefinition() { return transportRouteDefinition; }
    @JsonProperty("TransportRouteDefinition")
    public void setTransportRouteDefinition(TransportRouteDefinition value) { this.transportRouteDefinition = value; }

    @JsonProperty("TransportRouteDynamic")
    public TransportRouteDynamic getTransportRouteDynamic() { return transportRouteDynamic; }
    @JsonProperty("TransportRouteDynamic")
    public void setTransportRouteDynamic(TransportRouteDynamic value) { this.transportRouteDynamic = value; }

    @JsonProperty("TransportRouteFault")
    public TransportRouteFault getTransportRouteFault() { return transportRouteFault; }
    @JsonProperty("TransportRouteFault")
    public void setTransportRouteFault(TransportRouteFault value) { this.transportRouteFault = value; }

    @JsonProperty("TransportRouteHistoryEntry")
    public TransportRouteHistoryEntry getTransportRouteHistoryEntry() { return transportRouteHistoryEntry; }
    @JsonProperty("TransportRouteHistoryEntry")
    public void setTransportRouteHistoryEntry(TransportRouteHistoryEntry value) { this.transportRouteHistoryEntry = value; }

    @JsonProperty("TransportRouteSegmentConfiguration")
    public TransportRouteSegmentConfiguration getTransportRouteSegmentConfiguration() { return transportRouteSegmentConfiguration; }
    @JsonProperty("TransportRouteSegmentConfiguration")
    public void setTransportRouteSegmentConfiguration(TransportRouteSegmentConfiguration value) { this.transportRouteSegmentConfiguration = value; }

    @JsonProperty("TransportRouteStatus")
    public RouteStatus getTransportRouteStatus() { return transportRouteStatus; }
    @JsonProperty("TransportRouteStatus")
    public void setTransportRouteStatus(RouteStatus value) { this.transportRouteStatus = value; }

    @JsonProperty("TransportRouteType")
    public TransportRouteType getTransportRouteType() { return transportRouteType; }
    @JsonProperty("TransportRouteType")
    public void setTransportRouteType(TransportRouteType value) { this.transportRouteType = value; }
}
