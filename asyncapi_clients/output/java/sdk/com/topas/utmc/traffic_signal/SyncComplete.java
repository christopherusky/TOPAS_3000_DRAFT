/*
 * File:        output/java/sdk/com/topas/utmc/traffic_signal/SyncComplete.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.traffic_signal;

import com.fasterxml.jackson.annotation.*;
import java.time.OffsetDateTime;
import java.util.Map;

/**
 * Terminal message ending a traffic signal sync batch. Sent to the requester's reply node
 * after the last object, or instead of any objects when the filter matched none.
 */
public class SyncComplete {
    private OffsetDateTime completedTime;
    private Map<String, Object> extensions;
    private String message;
    private long objectCount;
    private SyncStatusEnum status;
    private long syncCorrelationID;
    private String syncID;

    /**
     * When the publisher finished the sync.
     */
    @JsonProperty("completedTime")
    public OffsetDateTime getCompletedTime() { return completedTime; }
    @JsonProperty("completedTime")
    public void setCompletedTime(OffsetDateTime value) { this.completedTime = value; }

    /**
     * Vendor or forward-compatibility extensions.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * Human-readable explanation, for rejected or failed.
     */
    @JsonProperty("message")
    public String getMessage() { return message; }
    @JsonProperty("message")
    public void setMessage(String value) { this.message = value; }

    /**
     * Number of objects delivered in this batch. Zero is a valid answer and is the reason this
     * message exists.
     */
    @JsonProperty("objectCount")
    public long getObjectCount() { return objectCount; }
    @JsonProperty("objectCount")
    public void setObjectCount(long value) { this.objectCount = value; }

    /**
     * Outcome of the sync.
     */
    @JsonProperty("status")
    public SyncStatusEnum getStatus() { return status; }
    @JsonProperty("status")
    public void setStatus(SyncStatusEnum value) { this.status = value; }

    /**
     * Copied unchanged from the SyncRequest this completes, so a requester with several syncs
     * in flight can tell them apart.
     */
    @JsonProperty("syncCorrelationId")
    public long getSyncCorrelationID() { return syncCorrelationID; }
    @JsonProperty("syncCorrelationId")
    public void setSyncCorrelationID(long value) { this.syncCorrelationID = value; }

    /**
     * The x-sync-id shared by the messages of this batch. Normally absent when no objects
     * matched, since there was no batch to identify. Only a rejected sync is required to omit
     * it; a success or a failed sync may carry one (a failed sync began a batch).
     */
    @JsonProperty("syncId")
    public String getSyncID() { return syncID; }
    @JsonProperty("syncId")
    public void setSyncID(String value) { this.syncID = value; }
}
