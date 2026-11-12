/*
 * File:        output/java/sdk/com/topas/utmc/traffic_event/AccidentEvent.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.traffic_event;

import com.fasterxml.jackson.annotation.*;
import java.time.OffsetDateTime;
import java.util.Map;

/**
 * Accident traffic event payload.
 */
public class AccidentEvent {
    private OffsetDateTime accidentTime;
    private AccidentType accidentType;
    private String confirmedBy;
    private Boolean diversionInForce;
    private String diversionRoute;
    private OffsetDateTime endTime;
    private String eventID;
    private Map<String, Object> extensions;
    private String externalCountryCode;
    private String externalSituationID;
    private Long externalSituationVersion;
    private String externalSystemCode;
    private LanesAffected lanesAffected;
    private Long lanesOperational;
    private Long lanesRestricted;
    private String lightConditions;
    private Location location;
    private String locationDescription;
    private String modifiedBy;
    private String phase;
    private String publicDescription;
    private String reportedBy;
    private String roadConditions;
    private Severity severity;
    private OffsetDateTime statisticDate;
    private EventStatusEnum status;
    private Long vehicleCount;
    private String weatherConditions;

    /**
     * When the accident was reported. Optional: unknown when first reported.
     */
    @JsonProperty("accidentTime")
    public OffsetDateTime getAccidentTime() { return accidentTime; }
    @JsonProperty("accidentTime")
    public void setAccidentTime(OffsetDateTime value) { this.accidentTime = value; }

    /**
     * Classification of the accident.
     */
    @JsonProperty("accidentType")
    public AccidentType getAccidentType() { return accidentType; }
    @JsonProperty("accidentType")
    public void setAccidentType(AccidentType value) { this.accidentType = value; }

    /**
     * Who confirmed.
     */
    @JsonProperty("confirmedBy")
    public String getConfirmedBy() { return confirmedBy; }
    @JsonProperty("confirmedBy")
    public void setConfirmedBy(String value) { this.confirmedBy = value; }

    /**
     * Whether diversion is active.
     */
    @JsonProperty("diversionInForce")
    public Boolean getDiversionInForce() { return diversionInForce; }
    @JsonProperty("diversionInForce")
    public void setDiversionInForce(Boolean value) { this.diversionInForce = value; }

    /**
     * Diversion route description.
     */
    @JsonProperty("diversionRoute")
    public String getDiversionRoute() { return diversionRoute; }
    @JsonProperty("diversionRoute")
    public void setDiversionRoute(String value) { this.diversionRoute = value; }

    /**
     * When cleared.
     */
    @JsonProperty("endTime")
    public OffsetDateTime getEndTime() { return endTime; }
    @JsonProperty("endTime")
    public void setEndTime(OffsetDateTime value) { this.endTime = value; }

    /**
     * Unique identifier of this event.
     */
    @JsonProperty("eventId")
    public String getEventID() { return eventID; }
    @JsonProperty("eventId")
    public void setEventID(String value) { this.eventID = value; }

    /**
     * Vendor or local extensions.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * Cross-ref country code (an XML NMTOKEN: no whitespace).
     */
    @JsonProperty("externalCountryCode")
    public String getExternalCountryCode() { return externalCountryCode; }
    @JsonProperty("externalCountryCode")
    public void setExternalCountryCode(String value) { this.externalCountryCode = value; }

    /**
     * Cross-ref situation identifier.
     */
    @JsonProperty("externalSituationId")
    public String getExternalSituationID() { return externalSituationID; }
    @JsonProperty("externalSituationId")
    public void setExternalSituationID(String value) { this.externalSituationID = value; }

    /**
     * Cross-ref situation version. Requires external_situation_id.
     */
    @JsonProperty("externalSituationVersion")
    public Long getExternalSituationVersion() { return externalSituationVersion; }
    @JsonProperty("externalSituationVersion")
    public void setExternalSituationVersion(Long value) { this.externalSituationVersion = value; }

    /**
     * Cross-ref system code.
     */
    @JsonProperty("externalSystemCode")
    public String getExternalSystemCode() { return externalSystemCode; }
    @JsonProperty("externalSystemCode")
    public void setExternalSystemCode(String value) { this.externalSystemCode = value; }

    /**
     * Lane closure pattern (I=open, T=closed).
     */
    @JsonProperty("lanesAffected")
    public LanesAffected getLanesAffected() { return lanesAffected; }
    @JsonProperty("lanesAffected")
    public void setLanesAffected(LanesAffected value) { this.lanesAffected = value; }

    /**
     * Count of operational lanes.
     */
    @JsonProperty("lanesOperational")
    public Long getLanesOperational() { return lanesOperational; }
    @JsonProperty("lanesOperational")
    public void setLanesOperational(Long value) { this.lanesOperational = value; }

    /**
     * Count of closed/restricted lanes.
     */
    @JsonProperty("lanesRestricted")
    public Long getLanesRestricted() { return lanesRestricted; }
    @JsonProperty("lanesRestricted")
    public void setLanesRestricted(Long value) { this.lanesRestricted = value; }

    /**
     * Light conditions at time.
     */
    @JsonProperty("lightConditions")
    public String getLightConditions() { return lightConditions; }
    @JsonProperty("lightConditions")
    public void setLightConditions(String value) { this.lightConditions = value; }

    /**
     * WGS84 point location.
     */
    @JsonProperty("location")
    public Location getLocation() { return location; }
    @JsonProperty("location")
    public void setLocation(Location value) { this.location = value; }

    /**
     * Free-text location description.
     */
    @JsonProperty("locationDescription")
    public String getLocationDescription() { return locationDescription; }
    @JsonProperty("locationDescription")
    public void setLocationDescription(String value) { this.locationDescription = value; }

    /**
     * Who last modified.
     */
    @JsonProperty("modifiedBy")
    public String getModifiedBy() { return modifiedBy; }
    @JsonProperty("modifiedBy")
    public void setModifiedBy(String value) { this.modifiedBy = value; }

    /**
     * Event phase (unconstrained).
     */
    @JsonProperty("phase")
    public String getPhase() { return phase; }
    @JsonProperty("phase")
    public void setPhase(String value) { this.phase = value; }

    /**
     * Public-facing summary for journey planners.
     */
    @JsonProperty("publicDescription")
    public String getPublicDescription() { return publicDescription; }
    @JsonProperty("publicDescription")
    public void setPublicDescription(String value) { this.publicDescription = value; }

    /**
     * Who reported the event.
     */
    @JsonProperty("reportedBy")
    public String getReportedBy() { return reportedBy; }
    @JsonProperty("reportedBy")
    public void setReportedBy(String value) { this.reportedBy = value; }

    /**
     * Road surface conditions.
     */
    @JsonProperty("roadConditions")
    public String getRoadConditions() { return roadConditions; }
    @JsonProperty("roadConditions")
    public void setRoadConditions(String value) { this.roadConditions = value; }

    /**
     * Event severity.
     */
    @JsonProperty("severity")
    public Severity getSeverity() { return severity; }
    @JsonProperty("severity")
    public void setSeverity(Severity value) { this.severity = value; }

    /**
     * When statistics were collected.
     */
    @JsonProperty("statisticDate")
    public OffsetDateTime getStatisticDate() { return statisticDate; }
    @JsonProperty("statisticDate")
    public void setStatisticDate(OffsetDateTime value) { this.statisticDate = value; }

    @JsonProperty("status")
    public EventStatusEnum getStatus() { return status; }
    @JsonProperty("status")
    public void setStatus(EventStatusEnum value) { this.status = value; }

    /**
     * Number of vehicles involved.
     */
    @JsonProperty("vehicleCount")
    public Long getVehicleCount() { return vehicleCount; }
    @JsonProperty("vehicleCount")
    public void setVehicleCount(Long value) { this.vehicleCount = value; }

    /**
     * Weather conditions.
     */
    @JsonProperty("weatherConditions")
    public String getWeatherConditions() { return weatherConditions; }
    @JsonProperty("weatherConditions")
    public void setWeatherConditions(String value) { this.weatherConditions = value; }
}
