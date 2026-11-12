/*
 * File:        output/typescript/sdk/transport_link.ts
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
export interface AllTypes {
    LinkStatusType?:                    LinkStatusType;
    RecalibrateTransportLinkCommand?:   RecalibrateTransportLinkCommand;
    RequestTransportLinkStatusCommand?: RequestTransportLinkStatusCommand;
    SetStocCommand?:                    SetStocCommand;
    SyncCategoryEnum?:                  SyncCategoryEnum;
    SyncComplete?:                      SyncComplete;
    SyncRequest?:                       SyncRequest;
    SyncStatus?:                        SyncStatusEnum;
    TransportLinkANPRConfiguration?:    TransportLinkAnprConfiguration;
    TransportLinkANPRDynamic?:          TransportLinkAnprDynamic;
    TransportLinkCommandResponse?:      TransportLinkCommandResponse;
    TransportLinkDefinition?:           TransportLinkDefinition;
    TransportLinkFault?:                TransportLinkFault;
    TransportLinkGPSConfiguration?:     TransportLinkGpsConfiguration;
    TransportLinkGPSDynamic?:           TransportLinkGpsDynamic;
    TransportLinkHistoryEntry?:         TransportLinkHistoryEntry;
    TransportLinkSCOOTConfiguration?:   TransportLinkScootConfiguration;
    TransportLinkSCOOTDynamic?:         TransportLinkScootDynamic;
    TransportLinkType?:                 TransportLinkType;
    [property: string]: any;
}

/**
 * Catalogue entry for one operational status of a transport link.
 */
export interface LinkStatusType {
    /**
     * Human-readable description of the link status (v1: 'normal', 'suspect',
     * 'noVehiclesInPeriod').
     */
    description: string;
    /**
     * Vendor or local extensions. Open-content object.
     */
    extensions?: { [key: string]: any };
    /**
     * Discriminator for the tagged union. The fixed value "linkStatus" selects the
     * LinkStatusType payload; it is not free-text command content.
     */
    kind: LinkStatusTypeKind;
    /**
     * Additional notes about this status value.
     */
    notes?: string;
    /**
     * Identifier of this link status type.
     */
    typeId: string;
}

export enum LinkStatusTypeKind {
    LinkStatus = "linkStatus",
}

/**
 * Trigger a recalibration on a transport link.
 */
export interface RecalibrateTransportLinkCommand {
    /**
     * Caller-assigned identifier correlating a command request with its response. Not an
     * identifier of the command itself. Named to avoid collision with the AMQP 1.0
     * correlation-id message property.
     */
    commandCorrelationId: number;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Discriminator for the tagged union. The fixed value "recalibrate" selects the
     * RecalibrateTransportLinkCommand payload; it is not free-text command content.
     */
    kind: RecalibrateTransportLinkCommandKind;
    /**
     * When the command was issued.
     */
    requestTime: Date;
    /**
     * Identifier of the transport link receiving the command.
     */
    transportLinkId: string;
}

export enum RecalibrateTransportLinkCommandKind {
    Recalibrate = "recalibrate",
}

/**
 * Request the current status of a transport link.
 */
export interface RequestTransportLinkStatusCommand {
    /**
     * Caller-assigned identifier correlating a command request with its response. Not an
     * identifier of the command itself. Named to avoid collision with the AMQP 1.0
     * correlation-id message property.
     */
    commandCorrelationId: number;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Discriminator for the tagged union. The fixed value "requestStatus" selects the
     * RequestTransportLinkStatusCommand payload; it is not free-text command content.
     */
    kind: RequestTransportLinkStatusCommandKind;
    /**
     * When the command was issued.
     */
    requestTime: Date;
    /**
     * Identifier of the transport link receiving the command.
     */
    transportLinkId: string;
}

export enum RequestTransportLinkStatusCommandKind {
    RequestStatus = "requestStatus",
}

/**
 * Set the SCOOT STOC threshold value on a transport link.
 */
export interface SetStocCommand {
    /**
     * Caller-assigned identifier correlating a command request with its response. Not an
     * identifier of the command itself. Named to avoid collision with the AMQP 1.0
     * correlation-id message property.
     */
    commandCorrelationId: number;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Discriminator for the tagged union. The fixed value "setStoc" selects the SetStocCommand
     * payload; it is not free-text command content.
     */
    kind: SetStocCommandKind;
    /**
     * When the command was issued.
     */
    requestTime: Date;
    /**
     * STOC value to set.
     */
    stocValue: string;
    /**
     * Identifier of the transport link receiving the command.
     */
    transportLinkId: string;
}

export enum SetStocCommandKind {
    SetStoc = "setStoc",
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
 * Terminal message ending a transport link sync batch. Sent to the requester's reply node
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
 * Request the current state of transport link objects. The Data Publisher answers to the
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

/**
 * Operational configuration for an ANPR-monitored transport link.
 */
export interface TransportLinkAnprConfiguration {
    /**
     * Maximum flow in vehicles per hour.
     */
    capacityVehiclesPerHour?: number;
    /**
     * When this configuration record became effective.
     */
    configurationDate: Date;
    /**
     * Vendor or local extensions. Open-content object.
     */
    extensions?: { [key: string]: any };
    /**
     * Distance from stop line to stop line in metres.
     */
    fullLengthMetres?: number;
    /**
     * Discriminator for the tagged union. The fixed value "anpr" selects the
     * TransportLinkANPRConfiguration payload; it is not free-text command content.
     */
    kind: TransportLinkANPRConfigurationKind;
    /**
     * Distance from detector to stop line in metres.
     */
    lengthMetres?: number;
    /**
     * Maximum speed associated with the link in kilometres per hour.
     */
    maximumSpeedKph?: number;
    /**
     * Number of lanes on the link.
     */
    numberOfLanes?: number;
    /**
     * Identifier of the transport link this configuration applies to.
     */
    transportLinkId: string;
}

export enum TransportLinkANPRConfigurationKind {
    Anpr = "anpr",
}

/**
 * ANPR-reported live measurement for a transport link.
 */
export interface TransportLinkAnprDynamic {
    /**
     * Estimated average speed in kilometres per hour.
     */
    averageSpeedKph?: number;
    /**
     * Vendor or local extensions. Open-content object.
     */
    extensions?: { [key: string]: any };
    /**
     * Discriminator for the tagged union. The fixed value "anpr" selects the
     * TransportLinkANPRDynamic payload; it is not free-text command content.
     */
    kind: TransportLinkANPRConfigurationKind;
    /**
     * Current link journey time in seconds.
     */
    linkTravelTimeSeconds?: number;
    /**
     * Number of successful plate reads at the start of the link.
     */
    numberPlatesIn?: number;
    /**
     * Count of number-plate matches in the measurement period.
     */
    numberPlatesMatches?: number;
    /**
     * Number of successful matches actually used in the travel-time calculation.
     */
    numberPlatesMatchesUsed?: number;
    /**
     * Number of successful plate reads at the end of the link.
     */
    numberPlatesOut?: number;
    /**
     * Optional quality statement for this measurement.
     */
    quality?: Quality;
    /**
     * Time the measurement was taken.
     */
    timestamp: Date;
    /**
     * Identifier of the transport link this reading is about.
     */
    transportLinkId: string;
}

/**
 * Optional quality statement for this measurement.
 *
 * Quality statement attached to a UTMC data object or measurement.
 */
export interface Quality {
    /**
     * Vendor or local extensions. Open-content object.
     */
    extensions?: { [key: string]: any };
    /**
     * When this quality statement was last updated.
     */
    lastUpdated?: Date;
    /**
     * Identifier of the UTMC participant system that owns this quality statement.
     */
    participantId?: string;
    /**
     * Identifier of this Quality statement.
     */
    qualityId: number;
    /**
     * Name of the data source (e.g. 'SCOOT', 'MIDAS'). Required: a quality statement naming no
     * source cannot be interpreted.
     */
    sourceName: string;
    /**
     * Free-text description of the source kind.
     */
    sourceType?: string;
}

/**
 * Transport link command response payloads.
 */
export interface TransportLinkCommandResponse {
    /**
     * Machine-readable response code for programmatic handling.
     */
    code?: string;
    /**
     * Correlation identifier copied verbatim from the originating command request. Not an
     * identifier of the command itself.
     */
    commandCorrelationId: number;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Human-readable response message with details.
     */
    message?: string;
    /**
     * When the response was generated.
     */
    responseTime: Date;
    /**
     * Outcome of the command execution.
     */
    status: TransportLinkCommandResponseStatus;
    /**
     * Identifier of the transport link the command targeted.
     */
    transportLinkId: string;
}

/**
 * Outcome of the command execution.
 *
 * Outcome of a command execution.
 *
 * Every value except ``PENDING`` is terminal. Consumers SHOULD test for
 * "not pending" rather than enumerating the terminal values, so that a
 * future profile adding one does not strand them.
 */
export enum TransportLinkCommandResponseStatus {
    Failed = "failed",
    Invalid = "invalid",
    NotSupported = "notSupported",
    Pending = "pending",
    Success = "success",
    TimedOut = "timedOut",
}

/**
 * Transport link equipment details.
 */
export interface TransportLinkDefinition {
    /**
     * When this definition record was created.
     */
    creationDate: Date;
    /**
     * End of the transport link as a WGS84 point. Optional here although the v1 XSD requires
     * it, matching transport_route.
     */
    endPoint?: EndPoint;
    /**
     * Vendor or local extensions. Open-content object.
     */
    extensions?: { [key: string]: any };
    /**
     * Start of the transport link as a WGS84 point. Optional.
     */
    location?: EndPoint;
    /**
     * Longer description (context, commissioning notes, etc.).
     */
    longDescription?: string;
    /**
     * Identifier of this object on the originating (upstream) system. Enables cross-system
     * traceability.
     */
    originatingObjectId?: string;
    /**
     * Identifier of the upstream system that originally created this object (comparable to the
     * UML DataSourceTypeId).
     */
    originatingParticipantId?: string;
    /**
     * System that owns this transport link definition. Often implicit from context.
     */
    participantId?: string;
    /**
     * Short human-readable name (e.g. 'A1 northbound, Jct 5 -> Jct 6').
     */
    shortDescription?: string;
    /**
     * Unique identifier of this transport link within its owning system.
     */
    transportLinkId: string;
    /**
     * Unique identifier for a transport link type (SCOOT, ANPR, SCOOT Entry, ...).
     */
    typeId: string;
}

/**
 * End of the transport link as a WGS84 point. Optional here although the v1 XSD requires
 * it, matching transport_route.
 *
 * A geographic point: a coordinate plus optional altitude.
 *
 * Start of the transport link as a WGS84 point. Optional.
 */
export interface EndPoint {
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
 * Transport link fault reports.
 */
export interface TransportLinkFault {
    /**
     * When the fault was acknowledged.
     */
    acknowledgementDate?: Date;
    /**
     * Current acknowledgement state.
     */
    acknowledgementState?: AcknowledgementState;
    /**
     * When the fault was cleared. If present, the fault is historical.
     */
    clearedDate?: Date;
    /**
     * Whether this is a communications fault. See equipmentFault for how it relates to the
     * catalogue category.
     */
    communicationsFault?: boolean;
    /**
     * When the fault was first raised.
     */
    creationDate: Date;
    /**
     * Free-text description of the fault.
     */
    description?: string;
    /**
     * Whether this is an equipment fault. The fault type's catalogue category classifies the
     * type; this flag classifies this fault.
     */
    equipmentFault?: boolean;
    /**
     * Vendor or local extensions. Open-content object.
     */
    extensions?: { [key: string]: any };
    /**
     * Unique identifier of this fault report within the owning system.
     */
    faultId: number;
    /**
     * Reference to a FaultTypeDefinition entry in the shared fault catalogue.
     */
    faultTypeId: string;
    /**
     * When the fault record was last modified.
     */
    lastUpdated?: Date;
    /**
     * Values for the positional placeholders in the referenced FaultTypeDefinition's
     * messageTemplate, in order. Rendered as text so a value carries whatever form the fault
     * reports it in.
     */
    parameters?: string[];
    /**
     * Fault severity (higher = more severe).
     */
    severity?: number;
    /**
     * Whether this fault is still active or has been cleared.
     */
    status?: TransportLinkFaultStatus;
    /**
     * Reference to a SubsystemDefinition entry: the subsystem the fault arose in.
     */
    subsystemTypeId?: string;
    /**
     * Fault identifier allocated by the supplier or configuration manager for maintenance
     * purposes, if required.
     */
    supplierFaultNumber?: string;
    /**
     * Identifier of the transport link this fault is about.
     */
    transportLinkId: string;
}

/**
 * Current acknowledgement state.
 *
 * Acknowledgement state of a device fault.
 *
 * Maps from legacy XSD integer values:
 * - 0 -> notAcknowledged
 * - 1 -> acknowledged
 * - 2 -> deferred
 * - 3 -> recordedForAction
 */
export enum AcknowledgementState {
    Acknowledged = "acknowledged",
    Deferred = "deferred",
    NotAcknowledged = "notAcknowledged",
    RecordedForAction = "recordedForAction",
}

/**
 * Whether this fault is still active or has been cleared.
 *
 * Whether a fault is still active or has been cleared.
 *
 * This is the only fault lifecycle the payload carries. Record archival
 * — distinguishing a live record from a retired one — is not modelled
 * here and is a matter for the publishing system.
 */
export enum TransportLinkFaultStatus {
    Active = "active",
    Cleared = "cleared",
}

/**
 * Operational configuration for a GPS-monitored transport link.
 */
export interface TransportLinkGpsConfiguration {
    /**
     * Maximum flow in vehicles per hour.
     */
    capacityVehiclesPerHour?: number;
    /**
     * When this configuration record became effective.
     */
    configurationDate: Date;
    /**
     * Vendor or local extensions. Open-content object.
     */
    extensions?: { [key: string]: any };
    /**
     * Distance from stop line to stop line in metres.
     */
    fullLengthMetres?: number;
    /**
     * Discriminator for the tagged union. The fixed value "gps" selects the
     * TransportLinkGPSConfiguration payload; it is not free-text command content.
     */
    kind: TransportLinkGPSConfigurationKind;
    /**
     * Distance from detector to stop line in metres.
     */
    lengthMetres?: number;
    /**
     * Maximum speed associated with the link in kilometres per hour.
     */
    maximumSpeedKph?: number;
    /**
     * Number of lanes on the link.
     */
    numberOfLanes?: number;
    /**
     * Identifier of the transport link this configuration applies to.
     */
    transportLinkId: string;
}

export enum TransportLinkGPSConfigurationKind {
    Gps = "gps",
}

/**
 * GPS-probe live measurement for a transport link.
 *
 * Field set drawn from the XSD TransportLinkGpsDynamicGroup (Freeflow
 * Extension): CurrentJourneyTime, the three travel-time statistics
 * (Mean/Median/VarianceTravelTime), NumberOfVehicles and the reserved-
 * lane flow count. Shared base fields (transport_link_id, timestamp,
 * quality, average_speed_kph, link_travel_time_seconds) are inherited
 * from _TransportLinkDynamicBase.
 */
export interface TransportLinkGpsDynamic {
    /**
     * Estimated average speed in kilometres per hour.
     */
    averageSpeedKph?: number;
    /**
     * Current journey time in seconds.
     */
    currentJourneyTimeSeconds?: number;
    /**
     * Vendor or local extensions. Open-content object.
     */
    extensions?: { [key: string]: any };
    /**
     * Discriminator for the tagged union. The fixed value "gps" selects the
     * TransportLinkGPSDynamic payload; it is not free-text command content.
     */
    kind: TransportLinkGPSConfigurationKind;
    /**
     * Current link journey time in seconds.
     */
    linkTravelTimeSeconds?: number;
    /**
     * Mean travel time along the link in the measured interval, in seconds.
     */
    meanTravelTimeSeconds?: number;
    /**
     * Median travel time along the link in the measured interval, in seconds.
     */
    medianTravelTimeSeconds?: number;
    /**
     * Number of vehicles in the measured interval.
     */
    numberOfVehicles?: number;
    /**
     * Optional quality statement for this measurement.
     */
    quality?: Quality;
    /**
     * Flow in the reserved lane in the measured interval, in vehicles. A rate, so it may be
     * fractional.
     */
    reservedLaneFlowVehicles?: number;
    /**
     * Time the measurement was taken.
     */
    timestamp: Date;
    /**
     * Identifier of the transport link this reading is about.
     */
    transportLinkId: string;
    /**
     * Variance of travel times along the link in the measured interval. The unit is seconds
     * squared (s²), as for any variance of times; the field name keeps 'Seconds' for
     * compatibility with v1.
     */
    varianceTravelTimeSeconds?: number;
}

/**
 * Transport link history entries.
 */
export interface TransportLinkHistoryEntry {
    /**
     * Category or type of the history event (e.g. 'reconfiguration', 'capacity_update').
     */
    eventType?: string;
    /**
     * Vendor or local extensions. Open-content object.
     */
    extensions?: { [key: string]: any };
    /**
     * When the log entry was recorded.
     */
    logDate: Date;
    /**
     * Equipment log data (free-form text).
     */
    logEntry: string;
    /**
     * Unique identifier of this log entry within the owning system.
     */
    logId: number;
    /**
     * User who created the log entry.
     */
    logUser?: string;
    /**
     * Identifier of the transport link this log entry is about.
     */
    transportLinkId: string;
}

/**
 * Operational configuration for a SCOOT-modelled transport link.
 */
export interface TransportLinkScootConfiguration {
    /**
     * Maximum flow in vehicles per hour.
     */
    capacityVehiclesPerHour?: number;
    /**
     * When this configuration record became effective.
     */
    configurationDate: Date;
    /**
     * Vendor or local extensions. Open-content object.
     */
    extensions?: { [key: string]: any };
    /**
     * Distance from stop line to stop line in metres.
     */
    fullLengthMetres?: number;
    /**
     * Discriminator for the tagged union. The fixed value "scoot" selects the
     * TransportLinkSCOOTConfiguration payload; it is not free-text command content.
     */
    kind: TransportLinkSCOOTConfigurationKind;
    /**
     * Distance from detector to stop line in metres.
     */
    lengthMetres?: number;
    /**
     * Maximum speed associated with the link in kilometres per hour.
     */
    maximumSpeedKph?: number;
    /**
     * Number of lanes on the link.
     */
    numberOfLanes?: number;
    /**
     * Identifier of the transport link this configuration applies to.
     */
    transportLinkId: string;
}

export enum TransportLinkSCOOTConfigurationKind {
    Scoot = "scoot",
}

/**
 * SCOOT-reported live measurement for a transport link.
 */
export interface TransportLinkScootDynamic {
    /**
     * Estimated average speed in kilometres per hour.
     */
    averageSpeedKph?: number;
    /**
     * Current percentage congestion as reported by SCOOT.
     */
    congestionPercent?: number;
    /**
     * Current flow in vehicles over the SCOOT measurement interval. A rate, so it may be
     * fractional (e.g. a smoothed or averaged flow).
     */
    currentFlowVehicles?: number;
    /**
     * Vendor or local extensions. Open-content object.
     */
    extensions?: { [key: string]: any };
    /**
     * Discriminator for the tagged union. The fixed value "scoot" selects the
     * TransportLinkSCOOTDynamic payload; it is not free-text command content.
     */
    kind: TransportLinkSCOOTConfigurationKind;
    /**
     * Current link journey time in seconds.
     */
    linkTravelTimeSeconds?: number;
    /**
     * Average link occupancy percentage across all detectors on the link.
     */
    occupancyPercent?: number;
    /**
     * Optional quality statement for this measurement.
     */
    quality?: Quality;
    /**
     * Time the measurement was taken.
     */
    timestamp: Date;
    /**
     * Identifier of the transport link this reading is about.
     */
    transportLinkId: string;
}

/**
 * Catalogue entry for one kind of transport link.
 */
export interface TransportLinkType {
    /**
     * Human-readable description of the transport link kind.
     */
    description: string;
    /**
     * Vendor or local extensions. Open-content object.
     */
    extensions?: { [key: string]: any };
    /**
     * Discriminator for the tagged union. The fixed value "transportLink" selects the
     * TransportLinkType payload; it is not free-text command content.
     */
    kind: TransportLinkTypeKind;
    /**
     * Additional notes, e.g. deployment details.
     */
    notes?: string;
    /**
     * URL to an external specification document for this transport link type.
     */
    specificationLink?: string;
    /**
     * Identifier of this transport link type.
     */
    typeId: string;
}

export enum TransportLinkTypeKind {
    TransportLink = "transportLink",
}

/**
 * Transport link command payloads.
 *
 * Discriminated on `kind`. Narrow with a switch to have
 * the compiler check exhaustiveness.
 */
export type TransportLinkCommand =
    | RecalibrateTransportLinkCommand
    | RequestTransportLinkStatusCommand
    | SetStocCommand;

/**
 * Transport link operational configuration (SCOOT/GPS/ANPR discriminated union).
 *
 * Discriminated on `kind`. Narrow with a switch to have
 * the compiler check exhaustiveness.
 */
export type TransportLinkConfiguration =
    | TransportLinkAnprConfiguration
    | TransportLinkGpsConfiguration
    | TransportLinkScootConfiguration;

/**
 * Transport link dynamic measurement data.
 *
 * Discriminated on `kind`. Narrow with a switch to have
 * the compiler check exhaustiveness.
 */
export type TransportLinkMeasurement =
    | TransportLinkAnprDynamic
    | TransportLinkGpsDynamic
    | TransportLinkScootDynamic;

/**
 * Transport link type definitions.
 *
 * Discriminated on `kind`. Narrow with a switch to have
 * the compiler check exhaustiveness.
 */
export type TransportLinkTypeUnion =
    | LinkStatusType
    | TransportLinkType;
