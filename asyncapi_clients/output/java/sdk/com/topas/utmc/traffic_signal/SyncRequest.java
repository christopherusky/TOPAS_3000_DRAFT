/*
 * File:        output/java/sdk/com/topas/utmc/traffic_signal/SyncRequest.java
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
 * Request the current state of traffic signal objects. The Data Publisher answers to the
 * request's reply-to address, not the domain's data channels: a sync batch (x-message-mode:
 * sync, x-sync-id/-sequence/-total) followed by a SyncComplete.
 */
public class SyncRequest {
    private Boolean activeOnly;
    private SyncCategoryEnum[] categories;
    private Map<String, Object> extensions;
    private String[] objectIDS;
    private OffsetDateTime requestTime;
    private OffsetDateTime since;
    private String subArea;
    private long syncCorrelationID;
    private String typeID;

    /**
     * Restrict the sync to objects in an active state — for faults, those not yet cleared.
     * Omitted is equivalent to false. This is the filter behind "send me every active fault":
     * without it a subscriber would receive the full fault history it does not want.
     */
    @JsonProperty("activeOnly")
    public Boolean getActiveOnly() { return activeOnly; }
    @JsonProperty("activeOnly")
    public void setActiveOnly(Boolean value) { this.activeOnly = value; }

    /**
     * Restrict the sync to these data categories. Omitted means every category the domain
     * publishes.
     */
    @JsonProperty("categories")
    public SyncCategoryEnum[] getCategories() { return categories; }
    @JsonProperty("categories")
    public void setCategories(SyncCategoryEnum[] value) { this.categories = value; }

    /**
     * Vendor or forward-compatibility extensions.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * Restrict the sync to these objects. Omitted means every object of the requested
     * categories.
     */
    @JsonProperty("objectIds")
    public String[] getObjectIDS() { return objectIDS; }
    @JsonProperty("objectIds")
    public void setObjectIDS(String[] value) { this.objectIDS = value; }

    /**
     * When the request was issued.
     */
    @JsonProperty("requestTime")
    public OffsetDateTime getRequestTime() { return requestTime; }
    @JsonProperty("requestTime")
    public void setRequestTime(OffsetDateTime value) { this.requestTime = value; }

    /**
     * Restrict the sync to objects changed at or after this time. Intended for a subscriber
     * recovering from a known-length outage; omitted means no lower bound.
     */
    @JsonProperty("since")
    public OffsetDateTime getSince() { return since; }
    @JsonProperty("since")
    public void setSince(OffsetDateTime value) { this.since = value; }

    /**
     * Restrict the sync to objects in this sub-area. Omitted means every sub-area.
     */
    @JsonProperty("subArea")
    public String getSubArea() { return subArea; }
    @JsonProperty("subArea")
    public void setSubArea(String value) { this.subArea = value; }

    /**
     * Caller-assigned identifier correlating this request with the sync batch that answers it.
     * The Data Publisher echoes it in the batch's x-sync-request-id header. Not an identifier
     * of the sync itself — that is x-sync-id. Named to avoid collision with the AMQP 1.0
     * correlation-id message property, as for commandCorrelationId.
     */
    @JsonProperty("syncCorrelationId")
    public long getSyncCorrelationID() { return syncCorrelationID; }
    @JsonProperty("syncCorrelationId")
    public void setSyncCorrelationID(long value) { this.syncCorrelationID = value; }

    /**
     * Restrict the sync to objects of this type. Omitted means every type.
     */
    @JsonProperty("typeId")
    public String getTypeID() { return typeID; }
    @JsonProperty("typeId")
    public void setTypeID(String value) { this.typeID = value; }
}
