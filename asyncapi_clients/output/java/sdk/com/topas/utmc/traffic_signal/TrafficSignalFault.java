/*
 * File:        output/java/sdk/com/topas/utmc/traffic_signal/TrafficSignalFault.java
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
 * Traffic signal fault reports with flattened DATEX II Fault base (TS004.0064bis:2017 §3.3,
 * F6).
 */
public class TrafficSignalFault {
    private Map<String, Object> extensions;
    private OffsetDateTime faultCreationTime;
    private String faultDescription;
    private String faultIdentifier;
    private OffsetDateTime faultLastUpdateTime;
    private FaultSeverityEnum faultSeverity;
    private String faultTypeID;
    private String[] parameters;
    private OffsetDateTime trafficSignalFaultClearanceTime;
    private TrafficSignalFaultType trafficSignalFaultType;
    private TrafficSignalFaultUrgency trafficSignalFaultUrgency;
    private String trafficSignalID;
    private String trafficSignalManufacturerFaultCode;

    /**
     * Vendor or forward-compatibility extensions. Open-content object.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * Time the fault was first recorded.
     */
    @JsonProperty("faultCreationTime")
    public OffsetDateTime getFaultCreationTime() { return faultCreationTime; }
    @JsonProperty("faultCreationTime")
    public void setFaultCreationTime(OffsetDateTime value) { this.faultCreationTime = value; }

    /**
     * Human-readable fault description.
     */
    @JsonProperty("faultDescription")
    public String getFaultDescription() { return faultDescription; }
    @JsonProperty("faultDescription")
    public void setFaultDescription(String value) { this.faultDescription = value; }

    /**
     * Unique identifier for this fault.
     */
    @JsonProperty("faultIdentifier")
    public String getFaultIdentifier() { return faultIdentifier; }
    @JsonProperty("faultIdentifier")
    public void setFaultIdentifier(String value) { this.faultIdentifier = value; }

    /**
     * Time fault was last updated.
     */
    @JsonProperty("faultLastUpdateTime")
    public OffsetDateTime getFaultLastUpdateTime() { return faultLastUpdateTime; }
    @JsonProperty("faultLastUpdateTime")
    public void setFaultLastUpdateTime(OffsetDateTime value) { this.faultLastUpdateTime = value; }

    /**
     * Severity of the fault.
     */
    @JsonProperty("faultSeverity")
    public FaultSeverityEnum getFaultSeverity() { return faultSeverity; }
    @JsonProperty("faultSeverity")
    public void setFaultSeverity(FaultSeverityEnum value) { this.faultSeverity = value; }

    /**
     * Reference to a FaultTypeDefinition entry. The catalogue carries the code, category and
     * message template; new fault types are added as catalogue entries, with no schema change.
     * Required unless trafficSignalFaultType is given; when both are present this is the
     * authoritative classification.
     */
    @JsonProperty("faultTypeId")
    public String getFaultTypeID() { return faultTypeID; }
    @JsonProperty("faultTypeId")
    public void setFaultTypeID(String value) { this.faultTypeID = value; }

    /**
     * Values for the positional placeholders in the referenced FaultTypeDefinition's
     * messageTemplate, in order. Rendered as text so a value carries whatever form the fault
     * reports it in.
     */
    @JsonProperty("parameters")
    public String[] getParameters() { return parameters; }
    @JsonProperty("parameters")
    public void setParameters(String[] value) { this.parameters = value; }

    /**
     * Time fault was cleared.
     */
    @JsonProperty("trafficSignalFaultClearanceTime")
    public OffsetDateTime getTrafficSignalFaultClearanceTime() { return trafficSignalFaultClearanceTime; }
    @JsonProperty("trafficSignalFaultClearanceTime")
    public void setTrafficSignalFaultClearanceTime(OffsetDateTime value) { this.trafficSignalFaultClearanceTime = value; }

    /**
     * Coarse DATEX II fault bucket, as in TS004.0064bis:2017. Required unless faultTypeId is
     * given; when both are present faultTypeId is the authoritative classification.
     */
    @JsonProperty("trafficSignalFaultType")
    public TrafficSignalFaultType getTrafficSignalFaultType() { return trafficSignalFaultType; }
    @JsonProperty("trafficSignalFaultType")
    public void setTrafficSignalFaultType(TrafficSignalFaultType value) { this.trafficSignalFaultType = value; }

    /**
     * The urgency of this fault.
     */
    @JsonProperty("trafficSignalFaultUrgency")
    public TrafficSignalFaultUrgency getTrafficSignalFaultUrgency() { return trafficSignalFaultUrgency; }
    @JsonProperty("trafficSignalFaultUrgency")
    public void setTrafficSignalFaultUrgency(TrafficSignalFaultUrgency value) { this.trafficSignalFaultUrgency = value; }

    /**
     * Identifier of the traffic signal this fault applies to.
     */
    @JsonProperty("trafficSignalId")
    public String getTrafficSignalID() { return trafficSignalID; }
    @JsonProperty("trafficSignalId")
    public void setTrafficSignalID(String value) { this.trafficSignalID = value; }

    /**
     * A supplier unique identifier for this specific fault.
     */
    @JsonProperty("trafficSignalManufacturerFaultCode")
    public String getTrafficSignalManufacturerFaultCode() { return trafficSignalManufacturerFaultCode; }
    @JsonProperty("trafficSignalManufacturerFaultCode")
    public void setTrafficSignalManufacturerFaultCode(String value) { this.trafficSignalManufacturerFaultCode = value; }
}
