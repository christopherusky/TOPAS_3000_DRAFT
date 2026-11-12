/*
 * File:        output/java/sdk/com/topas/utmc/traffic_signal/PlanLine.java
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
 * A signal plan definition carried in the dynamic state (§3.5, §3.6).
 *
 * A plan line for this traffic signal. This is intended to provide information allowing the
 * plan data to be displayed to an operator. It contains the current plan data and would be
 * updated when the plan is changed. For adaptive central control, such as SCOOT, the
 * information should be available periodically and contain the plan at that instant.
 */
public class PlanLine {
    private Long cycleTime;
    private Map<String, Object> extensions;
    private Long offset;
    private String planDescription;
    private long planNumber;
    private OffsetDateTime planStartTime;
    private Composite[] signalGroupTimings;

    /**
     * The current cycle time.
     */
    @JsonProperty("cycleTime")
    public Long getCycleTime() { return cycleTime; }
    @JsonProperty("cycleTime")
    public void setCycleTime(Long value) { this.cycleTime = value; }

    /**
     * Vendor or forward-compatibility extensions. Open-content object.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * The offset to the first SignalGroupTiming in the list. Must be present if
     * SignalGroupTimingList is present.
     */
    @JsonProperty("offset")
    public Long getOffset() { return offset; }
    @JsonProperty("offset")
    public void setOffset(Long value) { this.offset = value; }

    /**
     * A user entered description of the active plan.
     */
    @JsonProperty("planDescription")
    public String getPlanDescription() { return planDescription; }
    @JsonProperty("planDescription")
    public void setPlanDescription(String value) { this.planDescription = value; }

    /**
     * The currently active plan.
     */
    @JsonProperty("planNumber")
    public long getPlanNumber() { return planNumber; }
    @JsonProperty("planNumber")
    public void setPlanNumber(long value) { this.planNumber = value; }

    /**
     * The time the plan was started. Optional: a publisher emitting on a fixed interval may not
     * know the start time of the running plan.
     */
    @JsonProperty("planStartTime")
    public OffsetDateTime getPlanStartTime() { return planStartTime; }
    @JsonProperty("planStartTime")
    public void setPlanStartTime(OffsetDateTime value) { this.planStartTime = value; }

    /**
     * The signal group data for the plan.
     */
    @JsonProperty("signalGroupTimings")
    public Composite[] getSignalGroupTimings() { return signalGroupTimings; }
    @JsonProperty("signalGroupTimings")
    public void setSignalGroupTimings(Composite[] value) { this.signalGroupTimings = value; }
}
