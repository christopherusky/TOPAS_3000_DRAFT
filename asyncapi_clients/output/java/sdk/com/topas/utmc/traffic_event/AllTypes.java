/*
 * File:        output/java/sdk/com/topas/utmc/traffic_event/AllTypes.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.traffic_event;

import com.fasterxml.jackson.annotation.*;

public class AllTypes {
    private AccidentEvent accidentEvent;
    private AccidentType accidentType;
    private EventStatusEnum eventStatus;
    private IncidentEvent incidentEvent;
    private IncidentType incidentType;
    private LanesAffected lanesAffectedType;
    private PlannedEvent plannedEvent;
    private PlannedEventType plannedEventType;
    private RoadworksEvent roadworksEvent;
    private RoadworksType roadworksType;
    private Severity severityType;
    private SyncCategoryEnum syncCategoryEnum;
    private SyncComplete syncComplete;
    private SyncRequest syncRequest;
    private SyncStatusEnum syncStatus;

    @JsonProperty("AccidentEvent")
    public AccidentEvent getAccidentEvent() { return accidentEvent; }
    @JsonProperty("AccidentEvent")
    public void setAccidentEvent(AccidentEvent value) { this.accidentEvent = value; }

    @JsonProperty("AccidentType")
    public AccidentType getAccidentType() { return accidentType; }
    @JsonProperty("AccidentType")
    public void setAccidentType(AccidentType value) { this.accidentType = value; }

    @JsonProperty("EventStatus")
    public EventStatusEnum getEventStatus() { return eventStatus; }
    @JsonProperty("EventStatus")
    public void setEventStatus(EventStatusEnum value) { this.eventStatus = value; }

    @JsonProperty("IncidentEvent")
    public IncidentEvent getIncidentEvent() { return incidentEvent; }
    @JsonProperty("IncidentEvent")
    public void setIncidentEvent(IncidentEvent value) { this.incidentEvent = value; }

    @JsonProperty("IncidentType")
    public IncidentType getIncidentType() { return incidentType; }
    @JsonProperty("IncidentType")
    public void setIncidentType(IncidentType value) { this.incidentType = value; }

    @JsonProperty("LanesAffectedType")
    public LanesAffected getLanesAffectedType() { return lanesAffectedType; }
    @JsonProperty("LanesAffectedType")
    public void setLanesAffectedType(LanesAffected value) { this.lanesAffectedType = value; }

    @JsonProperty("PlannedEvent")
    public PlannedEvent getPlannedEvent() { return plannedEvent; }
    @JsonProperty("PlannedEvent")
    public void setPlannedEvent(PlannedEvent value) { this.plannedEvent = value; }

    @JsonProperty("PlannedEventType")
    public PlannedEventType getPlannedEventType() { return plannedEventType; }
    @JsonProperty("PlannedEventType")
    public void setPlannedEventType(PlannedEventType value) { this.plannedEventType = value; }

    @JsonProperty("RoadworksEvent")
    public RoadworksEvent getRoadworksEvent() { return roadworksEvent; }
    @JsonProperty("RoadworksEvent")
    public void setRoadworksEvent(RoadworksEvent value) { this.roadworksEvent = value; }

    @JsonProperty("RoadworksType")
    public RoadworksType getRoadworksType() { return roadworksType; }
    @JsonProperty("RoadworksType")
    public void setRoadworksType(RoadworksType value) { this.roadworksType = value; }

    @JsonProperty("SeverityType")
    public Severity getSeverityType() { return severityType; }
    @JsonProperty("SeverityType")
    public void setSeverityType(Severity value) { this.severityType = value; }

    @JsonProperty("SyncCategoryEnum")
    public SyncCategoryEnum getSyncCategoryEnum() { return syncCategoryEnum; }
    @JsonProperty("SyncCategoryEnum")
    public void setSyncCategoryEnum(SyncCategoryEnum value) { this.syncCategoryEnum = value; }

    @JsonProperty("SyncComplete")
    public SyncComplete getSyncComplete() { return syncComplete; }
    @JsonProperty("SyncComplete")
    public void setSyncComplete(SyncComplete value) { this.syncComplete = value; }

    @JsonProperty("SyncRequest")
    public SyncRequest getSyncRequest() { return syncRequest; }
    @JsonProperty("SyncRequest")
    public void setSyncRequest(SyncRequest value) { this.syncRequest = value; }

    @JsonProperty("SyncStatus")
    public SyncStatusEnum getSyncStatus() { return syncStatus; }
    @JsonProperty("SyncStatus")
    public void setSyncStatus(SyncStatusEnum value) { this.syncStatus = value; }
}
