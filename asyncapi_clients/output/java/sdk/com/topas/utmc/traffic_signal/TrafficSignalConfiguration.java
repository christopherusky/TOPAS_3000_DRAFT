/*
 * File:        output/java/sdk/com/topas/utmc/traffic_signal/TrafficSignalConfiguration.java
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
 * Traffic signal operational configuration (TS004.0064bis:2017 §3.4).
 */
public class TrafficSignalConfiguration {
    private OffsetDateTime configurationDate;
    private String controllerType;
    private Map<String, Object> extensions;
    private String subArea;
    private String supplier;
    private String trafficSignalID;
    private TrafficSignalTypeEnumEnum trafficSignalType;
    private String userCodeNumber;

    /**
     * When this configuration record became effective.
     */
    @JsonProperty("configurationDate")
    public OffsetDateTime getConfigurationDate() { return configurationDate; }
    @JsonProperty("configurationDate")
    public void setConfigurationDate(OffsetDateTime value) { this.configurationDate = value; }

    /**
     * Textual description of controller type.
     */
    @JsonProperty("controllerType")
    public String getControllerType() { return controllerType; }
    @JsonProperty("controllerType")
    public void setControllerType(String value) { this.controllerType = value; }

    /**
     * Vendor or forward-compatibility extensions. Open-content object.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * Group of signals to which this traffic signal belongs.
     */
    @JsonProperty("subArea")
    public String getSubArea() { return subArea; }
    @JsonProperty("subArea")
    public void setSubArea(String value) { this.subArea = value; }

    /**
     * Name of supplier.
     */
    @JsonProperty("supplier")
    public String getSupplier() { return supplier; }
    @JsonProperty("supplier")
    public void setSupplier(String value) { this.supplier = value; }

    /**
     * Identifier of the traffic signal this configuration applies to.
     */
    @JsonProperty("trafficSignalId")
    public String getTrafficSignalID() { return trafficSignalID; }
    @JsonProperty("trafficSignalId")
    public void setTrafficSignalID(String value) { this.trafficSignalID = value; }

    /**
     * Type of the traffic signal.
     */
    @JsonProperty("trafficSignalType")
    public TrafficSignalTypeEnumEnum getTrafficSignalType() { return trafficSignalType; }
    @JsonProperty("trafficSignalType")
    public void setTrafficSignalType(TrafficSignalTypeEnumEnum value) { this.trafficSignalType = value; }

    /**
     * Operator's own reference for this traffic signal.
     */
    @JsonProperty("userCodeNumber")
    public String getUserCodeNumber() { return userCodeNumber; }
    @JsonProperty("userCodeNumber")
    public void setUserCodeNumber(String value) { this.userCodeNumber = value; }
}
