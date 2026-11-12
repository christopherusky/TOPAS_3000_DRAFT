/*
 * File:        output/typescript/sdk/traffic_event.ts
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
export interface AllTypes {
    AccidentEvent?:     AccidentEvent;
    AccidentType?:      AccidentType;
    EventStatus?:       EventStatusEnum;
    IncidentEvent?:     IncidentEvent;
    IncidentType?:      IncidentType;
    LanesAffectedType?: LanesAffected;
    PlannedEvent?:      PlannedEvent;
    PlannedEventType?:  PlannedEventType;
    RoadworksEvent?:    RoadworksEvent;
    RoadworksType?:     RoadworksType;
    SeverityType?:      Severity;
    SyncCategoryEnum?:  SyncCategoryEnum;
    SyncComplete?:      SyncComplete;
    SyncRequest?:       SyncRequest;
    SyncStatus?:        SyncStatusEnum;
    [property: string]: any;
}

/**
 * Accident traffic event payload.
 */
export interface AccidentEvent {
    /**
     * When the accident was reported. Optional: unknown when first reported.
     */
    accidentTime?: Date;
    /**
     * Classification of the accident.
     */
    accidentType: AccidentType;
    /**
     * Who confirmed.
     */
    confirmedBy?: string;
    /**
     * Whether diversion is active.
     */
    diversionInForce?: boolean;
    /**
     * Diversion route description.
     */
    diversionRoute?: string;
    /**
     * When cleared.
     */
    endTime?: Date;
    /**
     * Unique identifier of this event.
     */
    eventId: string;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Cross-ref country code (an XML NMTOKEN: no whitespace).
     */
    externalCountryCode?: string;
    /**
     * Cross-ref situation identifier.
     */
    externalSituationId?: string;
    /**
     * Cross-ref situation version. Requires external_situation_id.
     */
    externalSituationVersion?: number;
    /**
     * Cross-ref system code.
     */
    externalSystemCode?: string;
    /**
     * Lane closure pattern (I=open, T=closed).
     */
    lanesAffected?: LanesAffected;
    /**
     * Count of operational lanes.
     */
    lanesOperational?: number;
    /**
     * Count of closed/restricted lanes.
     */
    lanesRestricted?: number;
    /**
     * Light conditions at time.
     */
    lightConditions?: string;
    /**
     * WGS84 point location.
     */
    location?: Location;
    /**
     * Free-text location description.
     */
    locationDescription?: string;
    /**
     * Who last modified.
     */
    modifiedBy?: string;
    /**
     * Event phase (unconstrained).
     */
    phase?: string;
    /**
     * Public-facing summary for journey planners.
     */
    publicDescription?: string;
    /**
     * Who reported the event.
     */
    reportedBy?: string;
    /**
     * Road surface conditions.
     */
    roadConditions?: string;
    /**
     * Event severity.
     */
    severity?: Severity;
    /**
     * When statistics were collected.
     */
    statisticDate?: Date;
    status:         EventStatusEnum;
    /**
     * Number of vehicles involved.
     */
    vehicleCount?: number;
    /**
     * Weather conditions.
     */
    weatherConditions?: string;
}

/**
 * Classification of the accident.
 *
 * Classification of accident (20 values).
 *
 * 18 of the 20 named XSD values, plus ``other``, plus ``overturnedTrailer``,
 * which has no XSD code. XSD codes 5 ("Accident History") and 20 ("Vehicle
 * spun around") are not carried.
 */
export enum AccidentType {
    Accident = "accident",
    BusAccident = "busAccident",
    ChemicalSpillage = "chemicalSpillage",
    Collision = "collision",
    FuelSpillage = "fuelSpillage",
    HazardousMaterials = "hazardousMaterials",
    HeavyLorry = "heavyLorry",
    InvestigationWork = "investigationWork",
    JackKnifedArticulatedLorry = "jackKnifedArticulatedLorry",
    JackKnifedCaravan = "jackKnifedCaravan",
    JackKnifedTrailer = "jackKnifedTrailer",
    MultiVehicle = "multiVehicle",
    OilSpillage = "oilSpillage",
    Other = "other",
    OverturnedHeavyLorry = "overturnedHeavyLorry",
    OverturnedTrailer = "overturnedTrailer",
    OverturnedVehicle = "overturnedVehicle",
    SecondaryAccident = "secondaryAccident",
    SeriousAccident = "seriousAccident",
    ShedLoad = "shedLoad",
}

/**
 * Lane closure pattern (I=open, T=closed).
 *
 * Lane closure pattern.
 *
 * I = open lane, T = closed lane, read nearside to offside.
 *
 * Lane closure pattern.
 */
export enum LanesAffected {
    AllOpen = "allOpen",
    I = "I",
    Ii = "II",
    Iii = "III",
    Iiii = "IIII",
    Iiit = "IIIT",
    Iit = "IIT",
    Iitt = "IITT",
    It = "IT",
    Iti = "ITI",
    Itt = "ITT",
    Ittt = "ITTT",
    T = "T",
    TI = "TI",
    Tii = "TII",
    Tiii = "TIII",
    Tit = "TIT",
    Tt = "TT",
    Tti = "TTI",
    Ttt = "TTT",
    Tttt = "TTTT",
}

/**
 * WGS84 point location.
 *
 * A geographic point: a coordinate plus optional altitude.
 */
export interface Location {
    /**
     * Altitude in metres above mean sea level. Optional. Bounds [-1000, 5000] inherited from
     * utmc_location-v1.0.xsd AltitudeType.
     */
    altitudeMeters?: number;
    /**
     * WGS84 coordinates.
     */
    coordinates: Coordinates;
}

/**
 * WGS84 coordinates.
 *
 * A WGS84 geographic coordinate pair (longitude, latitude).
 */
export interface Coordinates {
    /**
     * Latitude in decimal degrees (WGS84).
     */
    latitude: number;
    /**
     * Longitude in decimal degrees (WGS84).
     */
    longitude: number;
}

/**
 * Event severity.
 */
export enum Severity {
    High = "high",
    Low = "low",
    Medium = "medium",
    Unknown = "unknown",
}

/**
 * Event lifecycle state.
 */
export enum EventStatusEnum {
    Active = "active",
    Cleared = "cleared",
    Planned = "planned",
    Suspended = "suspended",
}

/**
 * Incident traffic event payload.
 */
export interface IncidentEvent {
    /**
     * Who confirmed.
     */
    confirmedBy?: string;
    /**
     * Whether diversion is active.
     */
    diversionInForce?: boolean;
    /**
     * Diversion route description.
     */
    diversionRoute?: string;
    /**
     * When cleared.
     */
    endTime?: Date;
    /**
     * Unique identifier of this event.
     */
    eventId: string;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Cross-ref country code (an XML NMTOKEN: no whitespace).
     */
    externalCountryCode?: string;
    /**
     * Cross-ref situation identifier.
     */
    externalSituationId?: string;
    /**
     * Cross-ref situation version. Requires external_situation_id.
     */
    externalSituationVersion?: number;
    /**
     * Cross-ref system code.
     */
    externalSystemCode?: string;
    /**
     * When the incident was reported. Optional: unknown when first reported.
     */
    incidentTime?: Date;
    /**
     * Classification of the incident.
     */
    incidentType: IncidentType;
    /**
     * Lane closure pattern.
     */
    lanesAffected?: LanesAffected;
    /**
     * Count of operational lanes.
     */
    lanesOperational?: number;
    /**
     * Count of closed/restricted lanes.
     */
    lanesRestricted?: number;
    /**
     * WGS84 point location.
     */
    location?: Location;
    /**
     * Free-text location description.
     */
    locationDescription?: string;
    /**
     * Who last modified.
     */
    modifiedBy?: string;
    /**
     * Event phase (unconstrained).
     */
    phase?: string;
    /**
     * Public-facing summary for journey planners.
     */
    publicDescription?: string;
    /**
     * Who reported the event.
     */
    reportedBy?: string;
    /**
     * Event severity.
     */
    severity?: Severity;
    status:    EventStatusEnum;
}

/**
 * Classification of the incident.
 *
 * Classification of incident (78 values from the UTMC specification).
 *
 * The XSD carries only a catalogue reference, so the value set is not in it.
 */
export enum IncidentType {
    AbnormalLoad = "abnormalLoad",
    AirCrash = "airCrash",
    AirRAID = "airRaid",
    AnimalsOnRoad = "animalsOnRoad",
    AttackOnVehicle = "attackOnVehicle",
    Avalanche = "avalanche",
    Blasting = "blasting",
    BridgeBlocked = "bridgeBlocked",
    BridgeClosed = "bridgeClosed",
    BridgeDemolition = "bridgeDemolition",
    BridgeMaintenance = "bridgeMaintenance",
    BridgeOpening = "bridgeOpening",
    BrokenDownBus = "brokenDownBus",
    BrokenDownLorry = "brokenDownLorry",
    BrokenDownVehicle = "brokenDownVehicle",
    BurstPipe = "burstPipe",
    BurstWaterMain = "burstWaterMain",
    BusDisruption = "busDisruption",
    BusLaneBlocked = "busLaneBlocked",
    BusLaneClosed = "busLaneClosed",
    CentreLaneBlocked = "centreLaneBlocked",
    CentreLaneClosed = "centreLaneClosed",
    ChemicalSpillage = "chemicalSpillage",
    ChildrenOnRoad = "childrenOnRoad",
    CivilEmergency = "civilEmergency",
    CivilEmergencyCancelled = "civilEmergencyCancelled",
    Congestion = "congestion",
    Contraflow = "contraflow",
    Convoy = "convoy",
    Crowd = "crowd",
    Cyclists = "cyclists",
    Diversion = "diversion",
    EmergencyAlert = "emergencyAlert",
    FacilitiesClosed = "facilitiesClosed",
    Fault = "fault",
    Fire = "fire",
    Flood = "flood",
    Flow = "flow",
    Fog = "fog",
    GawkingTraffic = "gawkingTraffic",
    Gunfire = "gunfire",
    Hail = "hail",
    HeavyFlow = "heavyFlow",
    Ice = "ice",
    LaneClosure = "laneClosure",
    LightFault = "lightFault",
    Obstruction = "obstruction",
    Other = "other",
    PeopleOnRoad = "peopleOnRoad",
    Pollution = "pollution",
    PowerFailure = "powerFailure",
    PublicDisturbance = "publicDisturbance",
    Queue = "queue",
    RadioactiveLeak = "radioactiveLeak",
    Rain = "rain",
    Riot = "riot",
    RoadClosure = "roadClosure",
    RoadsideFire = "roadsideFire",
    Sandstorms = "sandstorms",
    SecurityAlert = "securityAlert",
    SecurityIncident = "securityIncident",
    Sightseers = "sightseers",
    SmogAlert = "smogAlert",
    Snow = "snow",
    SpeedRestriction = "speedRestriction",
    Spillage = "spillage",
    SprayHazard = "sprayHazard",
    StructuralDamage = "structuralDamage",
    TelephoneFault = "telephoneFault",
    TemporarySignal = "temporarySignal",
    TerroristIncident = "terroristIncident",
    Tornadoes = "tornadoes",
    TrafficSignalFault = "trafficSignalFault",
    TrainDisruption = "trainDisruption",
    UnexplainedDamage = "unexplainedDamage",
    UnexplainedEvent = "unexplainedEvent",
    Washout = "washout",
    Wind = "wind",
}

/**
 * Planned event traffic event payload.
 */
export interface PlannedEvent {
    /**
     * Actual end time.
     */
    actualEnd?: Date;
    /**
     * Actual start time.
     */
    actualStart?: Date;
    /**
     * Who confirmed.
     */
    confirmedBy?: string;
    /**
     * Whether diversion is active.
     */
    diversionInForce?: boolean;
    /**
     * Diversion route description.
     */
    diversionRoute?: string;
    /**
     * Unique identifier of this event.
     */
    eventId: string;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Cross-ref country code (an XML NMTOKEN: no whitespace).
     */
    externalCountryCode?: string;
    /**
     * Cross-ref situation identifier.
     */
    externalSituationId?: string;
    /**
     * Cross-ref situation version. Requires external_situation_id.
     */
    externalSituationVersion?: number;
    /**
     * Cross-ref system code.
     */
    externalSystemCode?: string;
    /**
     * Lane closure pattern.
     */
    lanesAffected?: LanesAffected;
    /**
     * Count of operational lanes.
     */
    lanesOperational?: number;
    /**
     * Count of closed/restricted lanes.
     */
    lanesRestricted?: number;
    /**
     * WGS84 point location.
     */
    location?: Location;
    /**
     * Free-text location description.
     */
    locationDescription?: string;
    /**
     * Who last modified.
     */
    modifiedBy?: string;
    /**
     * Event organiser.
     */
    organiser?: string;
    /**
     * Event phase (unconstrained).
     */
    phase?: string;
    /**
     * Planned end time.
     */
    plannedEnd?: Date;
    /**
     * Classification of the event.
     */
    plannedEventType: PlannedEventType;
    /**
     * Planned start time.
     */
    plannedStart?: Date;
    /**
     * Public-facing summary for journey planners.
     */
    publicDescription?: string;
    /**
     * Who reported the event.
     */
    reportedBy?: string;
    /**
     * Event severity.
     */
    severity?: Severity;
    status:    EventStatusEnum;
    /**
     * Venue name.
     */
    venueName?: string;
}

/**
 * Classification of the event.
 *
 * Classification of planned event (32 values from the UTMC specification).
 *
 * The XSD carries only a catalogue reference, so the value set is not in it.
 */
export enum PlannedEventType {
    Athletics = "athletics",
    BallGame = "ballGame",
    Boxing = "boxing",
    CycleRace = "cycleRace",
    Demonstration = "demonstration",
    Exhibition = "exhibition",
    Fair = "fair",
    Festival = "festival",
    Football = "football",
    Funfair = "funfair",
    Golf = "golf",
    InternationalSports = "internationalSports",
    Marathon = "marathon",
    March = "march",
    Market = "market",
    Match = "match",
    Other = "other",
    Parade = "parade",
    Procession = "procession",
    Race = "race",
    Rugby = "rugby",
    Show = "show",
    ShowJumping = "showJumping",
    SportsEvent = "sportsEvent",
    SportsMeeting = "sportsMeeting",
    StateOccasion = "stateOccasion",
    Strike = "strike",
    Tennis = "tennis",
    Tournament = "tournament",
    TradeFair = "tradeFair",
    WaterSports = "waterSports",
    WinterSports = "winterSports",
}

/**
 * Roadworks traffic event payload.
 */
export interface RoadworksEvent {
    /**
     * Actual end time.
     */
    actualEnd?: Date;
    /**
     * Actual start time.
     */
    actualStart?: Date;
    /**
     * Who confirmed.
     */
    confirmedBy?: string;
    /**
     * When the roadworks were confirmed.
     */
    confirmedDate?: Date;
    /**
     * Contractor name.
     */
    contractor?: string;
    /**
     * Whether contraflow is in operation.
     */
    contraflow?: boolean;
    /**
     * Whether diversion is active.
     */
    diversionInForce?: boolean;
    /**
     * Diversion route description.
     */
    diversionRoute?: string;
    /**
     * Unique identifier of this event.
     */
    eventId: string;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Cross-ref country code (an XML NMTOKEN: no whitespace).
     */
    externalCountryCode?: string;
    /**
     * Cross-ref situation identifier.
     */
    externalSituationId?: string;
    /**
     * Cross-ref situation version. Requires external_situation_id.
     */
    externalSituationVersion?: number;
    /**
     * Cross-ref system code.
     */
    externalSystemCode?: string;
    /**
     * Lane closure pattern.
     */
    lanesAffected?: LanesAffected;
    /**
     * Count of operational lanes.
     */
    lanesOperational?: number;
    /**
     * Count of closed/restricted lanes.
     */
    lanesRestricted?: number;
    /**
     * WGS84 point location.
     */
    location?: Location;
    /**
     * Free-text location description.
     */
    locationDescription?: string;
    /**
     * Who last modified.
     */
    modifiedBy?: string;
    /**
     * Event phase (unconstrained).
     */
    phase?: string;
    /**
     * Planned end time.
     */
    plannedEnd?: Date;
    /**
     * Planned start time.
     */
    plannedStart?: Date;
    /**
     * Public-facing summary for journey planners.
     */
    publicDescription?: string;
    /**
     * Who reported the event.
     */
    reportedBy?: string;
    /**
     * Classification of the roadworks.
     */
    roadworksType: RoadworksType;
    /**
     * Event severity.
     */
    severity?: Severity;
    status:    EventStatusEnum;
    /**
     * Whether temporary traffic signals are in use.
     */
    trafficSignals?: boolean;
}

/**
 * Classification of the roadworks.
 *
 * Classification of roadworks (7 values from XSD).
 */
export enum RoadworksType {
    CentralReservation = "centralReservation",
    General = "general",
    Other = "other",
    Overhead = "overhead",
    Surface = "surface",
    Verge = "verge",
    WaterMainWork = "waterMainWork",
}

/**
 * Data categories a sync may be restricted to.
 *
 * Mirrors the schema-per-category pattern: a subscriber that only holds
 * a fault dashboard asks for ``fault`` alone rather than provoking a
 * full republication of every definition and configuration it already
 * has.
 */
export enum SyncCategoryEnum {
    Configuration = "configuration",
    Definition = "definition",
    Fault = "fault",
    Measurement = "measurement",
    Status = "status",
    Type = "type",
}

/**
 * Terminal message ending a traffic event sync batch. Sent to the requester's reply node
 * after the last object, or instead of any objects when the filter matched none.
 */
export interface SyncComplete {
    /**
     * When the publisher finished the sync.
     */
    completedTime: Date;
    /**
     * Vendor or forward-compatibility extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Human-readable explanation, for rejected or failed.
     */
    message?: string;
    /**
     * Number of objects delivered in this batch. Zero is a valid answer and is the reason this
     * message exists.
     */
    objectCount: number;
    /**
     * Outcome of the sync.
     */
    status: SyncStatusEnum;
    /**
     * Copied unchanged from the SyncRequest this completes, so a requester with several syncs
     * in flight can tell them apart.
     */
    syncCorrelationId: number;
    /**
     * The x-sync-id shared by the messages of this batch. Normally absent when no objects
     * matched, since there was no batch to identify. Only a rejected sync is required to omit
     * it; a success or a failed sync may carry one (a failed sync began a batch).
     */
    syncId?: string;
}

/**
 * Outcome of the sync.
 *
 * Outcome of a sync request (A12).
 */
export enum SyncStatusEnum {
    Failed = "failed",
    Rejected = "rejected",
    Success = "success",
}

/**
 * Request the current state of traffic event objects. The Data Publisher answers to the
 * request's reply-to address, not the domain's data channels: a sync batch (x-message-mode:
 * sync, x-sync-id/-sequence/-total) followed by a SyncComplete.
 */
export interface SyncRequest {
    /**
     * Restrict the sync to objects in an active state — for faults, those not yet cleared.
     * Omitted is equivalent to false. This is the filter behind "send me every active fault":
     * without it a subscriber would receive the full fault history it does not want.
     */
    activeOnly?: boolean;
    /**
     * Restrict the sync to these data categories. Omitted means every category the domain
     * publishes.
     */
    categories?: SyncCategoryEnum[];
    /**
     * Vendor or forward-compatibility extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Restrict the sync to these objects. Omitted means every object of the requested
     * categories.
     */
    objectIds?: string[];
    /**
     * When the request was issued.
     */
    requestTime: Date;
    /**
     * Restrict the sync to objects changed at or after this time. Intended for a subscriber
     * recovering from a known-length outage; omitted means no lower bound.
     */
    since?: Date;
    /**
     * Restrict the sync to objects in this sub-area. Omitted means every sub-area.
     */
    subArea?: string;
    /**
     * Caller-assigned identifier correlating this request with the sync batch that answers it.
     * The Data Publisher echoes it in the batch's x-sync-request-id header. Not an identifier
     * of the sync itself — that is x-sync-id. Named to avoid collision with the AMQP 1.0
     * correlation-id message property, as for commandCorrelationId.
     */
    syncCorrelationId: number;
    /**
     * Restrict the sync to objects of this type. Omitted means every type.
     */
    typeId?: string;
}
