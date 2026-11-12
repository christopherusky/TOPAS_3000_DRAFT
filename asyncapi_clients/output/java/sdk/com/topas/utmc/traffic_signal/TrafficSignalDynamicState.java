/*
 * File:        output/java/sdk/com/topas/utmc/traffic_signal/TrafficSignalDynamicState.java
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
 * Traffic signal dynamic state with PlanLine/SignalGroupTimings (TS004.0064bis:2017 §3.5).
 */
public class TrafficSignalDynamicState {
    private Map<String, Object> extensions;
    private PlanLine planLine;
    private String subArea;
    private OffsetDateTime timestamp;
    private String trafficSignalID;
    private TrafficSignalMode trafficSignalMode;
    private TrafficSignalSubMode trafficSignalSubMode;

    /**
     * Vendor or forward-compatibility extensions. Open-content object.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * A plan line for this traffic signal. This is intended to provide information allowing the
     * plan data to be displayed to an operator. It contains the current plan data and would be
     * updated when the plan is changed. For adaptive central control, such as SCOOT, the
     * information should be available periodically and contain the plan at that instant.
     */
    @JsonProperty("planLine")
    public PlanLine getPlanLine() { return planLine; }
    @JsonProperty("planLine")
    public void setPlanLine(PlanLine value) { this.planLine = value; }

    /**
     * Name for the sub-area that identifies the group of signals to which the associated
     * traffic signal equipment belongs.
     */
    @JsonProperty("subArea")
    public String getSubArea() { return subArea; }
    @JsonProperty("subArea")
    public void setSubArea(String value) { this.subArea = value; }

    /**
     * Time the state was observed.
     */
    @JsonProperty("timestamp")
    public OffsetDateTime getTimestamp() { return timestamp; }
    @JsonProperty("timestamp")
    public void setTimestamp(OffsetDateTime value) { this.timestamp = value; }

    /**
     * Identifier of the traffic signal this reading is about.
     */
    @JsonProperty("trafficSignalId")
    public String getTrafficSignalID() { return trafficSignalID; }
    @JsonProperty("trafficSignalId")
    public void setTrafficSignalID(String value) { this.trafficSignalID = value; }

    /**
     * The high level mode of the traffic signal.
     */
    @JsonProperty("trafficSignalMode")
    public TrafficSignalMode getTrafficSignalMode() { return trafficSignalMode; }
    @JsonProperty("trafficSignalMode")
    public void setTrafficSignalMode(TrafficSignalMode value) { this.trafficSignalMode = value; }

    /**
     * The detailed sub-mode of the traffic signal.
     */
    @JsonProperty("trafficSignalSubMode")
    public TrafficSignalSubMode getTrafficSignalSubMode() { return trafficSignalSubMode; }
    @JsonProperty("trafficSignalSubMode")
    public void setTrafficSignalSubMode(TrafficSignalSubMode value) { this.trafficSignalSubMode = value; }
}
