/*
 * File:        output/java/sdk/com/topas/utmc/network/AllTypes.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.network;

import com.fasterxml.jackson.annotation.*;

public class AllTypes {
    private NetworkLink networkLink;
    private NetworkNode networkNode;
    private NetworkPath networkPath;
    private NetworkTurn networkTurn;
    private NetworkZone networkZone;
    private SyncCategoryEnum syncCategoryEnum;
    private SyncComplete syncComplete;
    private SyncRequest syncRequest;
    private Status syncStatus;

    @JsonProperty("NetworkLink")
    public NetworkLink getNetworkLink() { return networkLink; }
    @JsonProperty("NetworkLink")
    public void setNetworkLink(NetworkLink value) { this.networkLink = value; }

    @JsonProperty("NetworkNode")
    public NetworkNode getNetworkNode() { return networkNode; }
    @JsonProperty("NetworkNode")
    public void setNetworkNode(NetworkNode value) { this.networkNode = value; }

    @JsonProperty("NetworkPath")
    public NetworkPath getNetworkPath() { return networkPath; }
    @JsonProperty("NetworkPath")
    public void setNetworkPath(NetworkPath value) { this.networkPath = value; }

    @JsonProperty("NetworkTurn")
    public NetworkTurn getNetworkTurn() { return networkTurn; }
    @JsonProperty("NetworkTurn")
    public void setNetworkTurn(NetworkTurn value) { this.networkTurn = value; }

    @JsonProperty("NetworkZone")
    public NetworkZone getNetworkZone() { return networkZone; }
    @JsonProperty("NetworkZone")
    public void setNetworkZone(NetworkZone value) { this.networkZone = value; }

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
    public Status getSyncStatus() { return syncStatus; }
    @JsonProperty("SyncStatus")
    public void setSyncStatus(Status value) { this.syncStatus = value; }
}
