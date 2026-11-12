/*
 * File:        output/java/sdk/com/topas/utmc/detector/DetectorFault.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.detector;

import com.fasterxml.jackson.annotation.*;
import java.time.OffsetDateTime;
import java.util.Map;

/**
 * Detector fault reports.
 */
public class DetectorFault {
    private OffsetDateTime acknowledgementDate;
    private AcknowledgementState acknowledgementState;
    private OffsetDateTime clearedDate;
    private Boolean communicationsFault;
    private OffsetDateTime creationDate;
    private String description;
    private String detectorID;
    private Boolean equipmentFault;
    private Map<String, Object> extensions;
    private long faultID;
    private String faultTypeID;
    private OffsetDateTime lastUpdated;
    private String[] parameters;
    private Long severity;
    private DetectorFaultStatus status;
    private String subsystemTypeID;
    private String supplierFaultNumber;

    /**
     * When the fault was acknowledged.
     */
    @JsonProperty("acknowledgementDate")
    public OffsetDateTime getAcknowledgementDate() { return acknowledgementDate; }
    @JsonProperty("acknowledgementDate")
    public void setAcknowledgementDate(OffsetDateTime value) { this.acknowledgementDate = value; }

    /**
     * Current acknowledgement state.
     */
    @JsonProperty("acknowledgementState")
    public AcknowledgementState getAcknowledgementState() { return acknowledgementState; }
    @JsonProperty("acknowledgementState")
    public void setAcknowledgementState(AcknowledgementState value) { this.acknowledgementState = value; }

    /**
     * When the fault was cleared. If present, the fault is historical.
     */
    @JsonProperty("clearedDate")
    public OffsetDateTime getClearedDate() { return clearedDate; }
    @JsonProperty("clearedDate")
    public void setClearedDate(OffsetDateTime value) { this.clearedDate = value; }

    /**
     * Whether this is a communications fault. See equipmentFault for how it relates to the
     * catalogue category.
     */
    @JsonProperty("communicationsFault")
    public Boolean getCommunicationsFault() { return communicationsFault; }
    @JsonProperty("communicationsFault")
    public void setCommunicationsFault(Boolean value) { this.communicationsFault = value; }

    /**
     * When the fault was first raised.
     */
    @JsonProperty("creationDate")
    public OffsetDateTime getCreationDate() { return creationDate; }
    @JsonProperty("creationDate")
    public void setCreationDate(OffsetDateTime value) { this.creationDate = value; }

    /**
     * Free-text description of the fault.
     */
    @JsonProperty("description")
    public String getDescription() { return description; }
    @JsonProperty("description")
    public void setDescription(String value) { this.description = value; }

    /**
     * Identifier of the detector this fault is about.
     */
    @JsonProperty("detectorId")
    public String getDetectorID() { return detectorID; }
    @JsonProperty("detectorId")
    public void setDetectorID(String value) { this.detectorID = value; }

    /**
     * Whether this is an equipment fault. The fault type's catalogue category classifies the
     * type; this flag classifies this fault.
     */
    @JsonProperty("equipmentFault")
    public Boolean getEquipmentFault() { return equipmentFault; }
    @JsonProperty("equipmentFault")
    public void setEquipmentFault(Boolean value) { this.equipmentFault = value; }

    /**
     * Vendor or local extensions. Open-content object.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * Unique identifier of this fault report within the owning system.
     */
    @JsonProperty("faultId")
    public long getFaultID() { return faultID; }
    @JsonProperty("faultId")
    public void setFaultID(long value) { this.faultID = value; }

    /**
     * Reference to a DetectorFaultType entry in the fault catalogue.
     */
    @JsonProperty("faultTypeId")
    public String getFaultTypeID() { return faultTypeID; }
    @JsonProperty("faultTypeId")
    public void setFaultTypeID(String value) { this.faultTypeID = value; }

    /**
     * When the fault record was last modified.
     */
    @JsonProperty("lastUpdated")
    public OffsetDateTime getLastUpdated() { return lastUpdated; }
    @JsonProperty("lastUpdated")
    public void setLastUpdated(OffsetDateTime value) { this.lastUpdated = value; }

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
     * Fault severity (higher = more severe).
     */
    @JsonProperty("severity")
    public Long getSeverity() { return severity; }
    @JsonProperty("severity")
    public void setSeverity(Long value) { this.severity = value; }

    /**
     * Whether this fault is still active or has been cleared.
     */
    @JsonProperty("status")
    public DetectorFaultStatus getStatus() { return status; }
    @JsonProperty("status")
    public void setStatus(DetectorFaultStatus value) { this.status = value; }

    /**
     * Reference to a SubsystemDefinition entry: the subsystem the fault arose in.
     */
    @JsonProperty("subsystemTypeId")
    public String getSubsystemTypeID() { return subsystemTypeID; }
    @JsonProperty("subsystemTypeId")
    public void setSubsystemTypeID(String value) { this.subsystemTypeID = value; }

    /**
     * Fault identifier allocated by the supplier or configuration manager for maintenance
     * purposes, if required.
     */
    @JsonProperty("supplierFaultNumber")
    public String getSupplierFaultNumber() { return supplierFaultNumber; }
    @JsonProperty("supplierFaultNumber")
    public void setSupplierFaultNumber(String value) { this.supplierFaultNumber = value; }
}
