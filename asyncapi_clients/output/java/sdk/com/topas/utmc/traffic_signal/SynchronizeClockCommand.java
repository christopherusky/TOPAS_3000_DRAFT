/*
 * File:        output/java/sdk/com/topas/utmc/traffic_signal/SynchronizeClockCommand.java
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
 * Synchronize the traffic signal controller's clock.
 */
public class SynchronizeClockCommand {
    private long commandCorrelationID;
    private Map<String, Object> extensions;
    private SynchronizeClockCommandKind kind;
    private OffsetDateTime requestTime;
    private OffsetDateTime targetTime;
    private String trafficSignalID;

    /**
     * Caller-assigned identifier correlating a command request with its response. Not an
     * identifier of the command itself. Named to avoid collision with the AMQP 1.0
     * correlation-id message property.
     */
    @JsonProperty("commandCorrelationId")
    public long getCommandCorrelationID() { return commandCorrelationID; }
    @JsonProperty("commandCorrelationId")
    public void setCommandCorrelationID(long value) { this.commandCorrelationID = value; }

    /**
     * Vendor or local extensions.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * Discriminator for the tagged union. The fixed value "synchronizeClock" selects the
     * SynchronizeClockCommand payload; it is not free-text command content.
     */
    @JsonProperty("kind")
    public SynchronizeClockCommandKind getKind() { return kind; }
    @JsonProperty("kind")
    public void setKind(SynchronizeClockCommandKind value) { this.kind = value; }

    /**
     * When the command was issued.
     */
    @JsonProperty("requestTime")
    public OffsetDateTime getRequestTime() { return requestTime; }
    @JsonProperty("requestTime")
    public void setRequestTime(OffsetDateTime value) { this.requestTime = value; }

    /**
     * Target time to synchronize the controller clock to.
     */
    @JsonProperty("targetTime")
    public OffsetDateTime getTargetTime() { return targetTime; }
    @JsonProperty("targetTime")
    public void setTargetTime(OffsetDateTime value) { this.targetTime = value; }

    /**
     * Identifier of the traffic signal receiving the command.
     */
    @JsonProperty("trafficSignalId")
    public String getTrafficSignalID() { return trafficSignalID; }
    @JsonProperty("trafficSignalId")
    public void setTrafficSignalID(String value) { this.trafficSignalID = value; }
}
