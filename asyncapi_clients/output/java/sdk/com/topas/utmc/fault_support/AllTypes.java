/*
 * File:        output/java/sdk/com/topas/utmc/fault_support/AllTypes.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.fault_support;

import com.fasterxml.jackson.annotation.*;

public class AllTypes {
    private AcknowledgementState acknowledgementState;
    private FaultCategory faultCategory;
    private FaultStatus faultStatus;
    private FaultTypeDefinition faultTypeDefinition;

    @JsonProperty("AcknowledgementState")
    public AcknowledgementState getAcknowledgementState() { return acknowledgementState; }
    @JsonProperty("AcknowledgementState")
    public void setAcknowledgementState(AcknowledgementState value) { this.acknowledgementState = value; }

    @JsonProperty("FaultCategory")
    public FaultCategory getFaultCategory() { return faultCategory; }
    @JsonProperty("FaultCategory")
    public void setFaultCategory(FaultCategory value) { this.faultCategory = value; }

    @JsonProperty("FaultStatus")
    public FaultStatus getFaultStatus() { return faultStatus; }
    @JsonProperty("FaultStatus")
    public void setFaultStatus(FaultStatus value) { this.faultStatus = value; }

    @JsonProperty("FaultTypeDefinition")
    public FaultTypeDefinition getFaultTypeDefinition() { return faultTypeDefinition; }
    @JsonProperty("FaultTypeDefinition")
    public void setFaultTypeDefinition(FaultTypeDefinition value) { this.faultTypeDefinition = value; }
}
