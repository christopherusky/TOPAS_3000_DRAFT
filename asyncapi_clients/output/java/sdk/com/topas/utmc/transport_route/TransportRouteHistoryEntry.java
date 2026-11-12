/*
 * File:        output/java/sdk/com/topas/utmc/transport_route/TransportRouteHistoryEntry.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.transport_route;

import com.fasterxml.jackson.annotation.*;
import java.time.OffsetDateTime;
import java.util.Map;

/**
 * Transport route history entries.
 */
public class TransportRouteHistoryEntry {
    private String eventType;
    private Map<String, Object> extensions;
    private OffsetDateTime logDate;
    private String logEntry;
    private long logID;
    private String logUser;
    private String transportRouteID;

    /**
     * Category of the history event.
     */
    @JsonProperty("eventType")
    public String getEventType() { return eventType; }
    @JsonProperty("eventType")
    public void setEventType(String value) { this.eventType = value; }

    /**
     * Vendor or local extensions.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * When the log entry was recorded.
     */
    @JsonProperty("logDate")
    public OffsetDateTime getLogDate() { return logDate; }
    @JsonProperty("logDate")
    public void setLogDate(OffsetDateTime value) { this.logDate = value; }

    /**
     * Equipment log data.
     */
    @JsonProperty("logEntry")
    public String getLogEntry() { return logEntry; }
    @JsonProperty("logEntry")
    public void setLogEntry(String value) { this.logEntry = value; }

    /**
     * Unique identifier of this log entry.
     */
    @JsonProperty("logId")
    public long getLogID() { return logID; }
    @JsonProperty("logId")
    public void setLogID(long value) { this.logID = value; }

    /**
     * User who created the entry.
     */
    @JsonProperty("logUser")
    public String getLogUser() { return logUser; }
    @JsonProperty("logUser")
    public void setLogUser(String value) { this.logUser = value; }

    /**
     * Identifier of the route.
     */
    @JsonProperty("transportRouteId")
    public String getTransportRouteID() { return transportRouteID; }
    @JsonProperty("transportRouteId")
    public void setTransportRouteID(String value) { this.transportRouteID = value; }
}
