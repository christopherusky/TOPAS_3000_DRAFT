/*
 * File:        output/java/sdk/com/topas/utmc/fault_support/FaultTypeDefinition.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.fault_support;

import com.fasterxml.jackson.annotation.*;
import java.util.Map;

/**
 * Catalogue entry describing one kind of fault that can be reported.
 */
public class FaultTypeDefinition {
    private FaultCategory category;
    private String code;
    private String description;
    private Map<String, Object> extensions;
    private String faultTypeID;
    private String messageTemplate;
    private String[] parameterNames;
    private Long severity;
    private String subsystemTypeID;

    /**
     * Coarse grouping, for filtering rather than semantics.
     */
    @JsonProperty("category")
    public FaultCategory getCategory() { return category; }
    @JsonProperty("category")
    public void setCategory(FaultCategory value) { this.category = value; }

    /**
     * The identifier this fault carries in its source document or vendor documentation — 'A1',
     * 'F3', 'W12'. Distinct from faultTypeId, which is this catalogue's key: a deployment may
     * hold two entries with the same code from different vendors.
     */
    @JsonProperty("code")
    public String getCode() { return code; }
    @JsonProperty("code")
    public void setCode(String value) { this.code = value; }

    /**
     * Human-readable description of the fault type.
     */
    @JsonProperty("description")
    public String getDescription() { return description; }
    @JsonProperty("description")
    public void setDescription(String value) { this.description = value; }

    /**
     * Vendor or local extensions. Open-content object.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * Identifier of this fault type.
     */
    @JsonProperty("faultTypeId")
    public String getFaultTypeID() { return faultTypeID; }
    @JsonProperty("faultTypeId")
    public void setFaultTypeID(String value) { this.faultTypeID = value; }

    /**
     * Human-readable message with positional placeholders, e.g. 'Minimum green not met: stage
     * {0} ran {1}s against a minimum of {2}s'. A fault instance supplies the values in its
     * parameters field. This is what lets a catalogue carry faults whose meaning depends on
     * runtime values — the stuck-stage and interstage timing faults that a closed enum cannot
     * express.
     */
    @JsonProperty("messageTemplate")
    public String getMessageTemplate() { return messageTemplate; }
    @JsonProperty("messageTemplate")
    public void setMessageTemplate(String value) { this.messageTemplate = value; }

    /**
     * Names for the positional placeholders in messageTemplate, in order — ['stageNumber',
     * 'actualSeconds', 'minimumSeconds']. Lets a consumer label the values rather than only
     * interpolate them. Length SHOULD match the number of placeholders.
     */
    @JsonProperty("parameterNames")
    public String[] getParameterNames() { return parameterNames; }
    @JsonProperty("parameterNames")
    public void setParameterNames(String[] value) { this.parameterNames = value; }

    /**
     * Numeric severity (higher = more severe). Optional.
     */
    @JsonProperty("severity")
    public Long getSeverity() { return severity; }
    @JsonProperty("severity")
    public void setSeverity(Long value) { this.severity = value; }

    /**
     * Reference to a SubsystemDefinition entry: the subsystem this fault type belongs to.
     */
    @JsonProperty("subsystemTypeId")
    public String getSubsystemTypeID() { return subsystemTypeID; }
    @JsonProperty("subsystemTypeId")
    public void setSubsystemTypeID(String value) { this.subsystemTypeID = value; }
}
