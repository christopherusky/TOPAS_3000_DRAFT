/*
 * File:        output/java/sdk/com/topas/utmc/transport_route/TransportRouteFault.java
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
 * Transport route fault reports.
 */
public class TransportRouteFault {
    private OffsetDateTime acknowledgementDate;
    private AcknowledgementState acknowledgementState;
    private OffsetDateTime clearedDate;
    private Boolean communicationsFault;
    private OffsetDateTime creationDate;
    private String description;
    private Boolean equipmentFault;
    private Map<String, Object> extensions;
    private long faultID;
    private String faultTypeID;
    private OffsetDateTime lastUpdated;
    private String[] parameters;
    private Long severity;
    private TransportRouteFaultStatus status;
    private String subsystemTypeID;
    private String supplierFaultNumber;
    private String transportRouteID;

    /**
     * When acknowledged.
     */
    @JsonProperty("acknowledgementDate")
    public OffsetDateTime getAcknowledgementDate() { return acknowledgementDate; }
    @JsonProperty("acknowledgementDate")
    public void setAcknowledgementDate(OffsetDateTime value) { this.acknowledgementDate = value; }

    /**
     * Acknowledgement state.
     */
    @JsonProperty("acknowledgementState")
    public AcknowledgementState getAcknowledgementState() { return acknowledgementState; }
    @JsonProperty("acknowledgementState")
    public void setAcknowledgementState(AcknowledgementState value) { this.acknowledgementState = value; }

    /**
     * When cleared.
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
     * Free-text description.
     */
    @JsonProperty("description")
    public String getDescription() { return description; }
    @JsonProperty("description")
    public void setDescription(String value) { this.description = value; }

    /**
     * Whether this is an equipment fault. The fault type's catalogue category classifies the
     * type; this flag classifies this fault.
     */
    @JsonProperty("equipmentFault")
    public Boolean getEquipmentFault() { return equipmentFault; }
    @JsonProperty("equipmentFault")
    public void setEquipmentFault(Boolean value) { this.equipmentFault = value; }

    /**
     * Vendor or local extensions.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * Unique identifier of this fault report.
     */
    @JsonProperty("faultId")
    public long getFaultID() { return faultID; }
    @JsonProperty("faultId")
    public void setFaultID(long value) { this.faultID = value; }

    /**
     * Reference to a FaultTypeDefinition entry.
     */
    @JsonProperty("faultTypeId")
    public String getFaultTypeID() { return faultTypeID; }
    @JsonProperty("faultTypeId")
    public void setFaultTypeID(String value) { this.faultTypeID = value; }

    /**
     * When last modified.
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
     * Fault severity.
     */
    @JsonProperty("severity")
    public Long getSeverity() { return severity; }
    @JsonProperty("severity")
    public void setSeverity(Long value) { this.severity = value; }

    /**
     * Active or cleared.
     */
    @JsonProperty("status")
    public TransportRouteFaultStatus getStatus() { return status; }
    @JsonProperty("status")
    public void setStatus(TransportRouteFaultStatus value) { this.status = value; }

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

    /**
     * Identifier of the route this fault is about.
     */
    @JsonProperty("transportRouteId")
    public String getTransportRouteID() { return transportRouteID; }
    @JsonProperty("transportRouteId")
    public void setTransportRouteID(String value) { this.transportRouteID = value; }
}
