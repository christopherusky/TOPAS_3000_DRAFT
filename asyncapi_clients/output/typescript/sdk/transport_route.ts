/*
 * File:        output/typescript/sdk/transport_route.ts
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
export interface AllTypes {
    RequestTransportRouteStatusCommand?: RequestTransportRouteStatusCommand;
    SyncCategoryEnum?:                   SyncCategoryEnum;
    SyncComplete?:                       SyncComplete;
    SyncRequest?:                        SyncRequest;
    SyncStatus?:                         SyncStatusEnum;
    TransportRouteCommandResponse?:      TransportRouteCommandResponse;
    TransportRouteConfiguration?:        TransportRouteConfiguration;
    TransportRouteDefinition?:           TransportRouteDefinition;
    TransportRouteDynamic?:              TransportRouteDynamic;
    TransportRouteFault?:                TransportRouteFault;
    TransportRouteHistoryEntry?:         TransportRouteHistoryEntry;
    TransportRouteSegmentConfiguration?: TransportRouteSegmentConfiguration;
    TransportRouteStatus?:               RouteStatus;
    TransportRouteType?:                 TransportRouteType;
    [property: string]: any;
}

/**
 * Request the current status of a transport route.
 */
export interface RequestTransportRouteStatusCommand {
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
     * RequestTransportRouteStatusCommand payload; it is not free-text command content.
     */
    kind: Kind;
    /**
     * When the command was issued.
     */
    requestTime: Date;
    /**
     * Identifier of the route receiving the command.
     */
    transportRouteId: string;
}

export enum Kind {
    RequestStatus = "requestStatus",
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
 * Terminal message ending a transport route sync batch. Sent to the requester's reply node
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
 * Request the current state of transport route objects. The Data Publisher answers to the
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
 * Transport route command response payloads.
 */
export interface TransportRouteCommandResponse {
    /**
     * Machine-readable response code.
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
     * Human-readable response message.
     */
    message?: string;
    /**
     * When the response was generated.
     */
    responseTime: Date;
    /**
     * Outcome of the command execution.
     */
    status: TransportRouteCommandResponseStatus;
    /**
     * Identifier of the route the command targeted.
     */
    transportRouteId: string;
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
export enum TransportRouteCommandResponseStatus {
    Failed = "failed",
    Invalid = "invalid",
    NotSupported = "notSupported",
    Pending = "pending",
    Success = "success",
    TimedOut = "timedOut",
}

/**
 * Transport route configuration and segment mapping.
 */
export interface TransportRouteConfiguration {
    /**
     * When this configuration became effective.
     */
    configurationDate: Date;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Route end description.
     */
    routeEnd?: string;
    /**
     * Route length in meters.
     */
    routeLengthMeters?: number;
    /**
     * Human-readable route name.
     */
    routeName?: string;
    /**
     * Route start description.
     */
    routeStart?: string;
    /**
     * Via point description.
     */
    routeVia?: string;
    /**
     * Route this configuration applies to.
     */
    transportRouteId: string;
}

/**
 * Transport route equipment details.
 */
export interface TransportRouteDefinition {
    /**
     * When this definition was created.
     */
    creationDate: Date;
    /**
     * Route endpoint location.
     */
    endPoint?: EndPoint;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Start point location.
     */
    location?: EndPoint;
    /**
     * Longer description.
     */
    longDescription?: string;
    /**
     * ID on originating system.
     */
    originatingObjectId?: string;
    /**
     * Upstream system.
     */
    originatingParticipantId?: string;
    /**
     * Owner or operator.
     */
    ownerOperator?: string;
    /**
     * Owning system.
     */
    participantId?: string;
    /**
     * Short name.
     */
    shortDescription?: string;
    /**
     * Unique identifier of this route.
     */
    transportRouteId: string;
    /**
     * Unique identifier for a transport route type (road, bus, rail, ...).
     */
    typeId: string;
}

/**
 * Route endpoint location.
 *
 * A geographic point: a coordinate plus optional altitude.
 *
 * Start point location.
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
 * Transport route dynamic measurement data.
 */
export interface TransportRouteDynamic {
    /**
     * Current journey time in seconds.
     */
    currentJourneyTimeSeconds?: number;
    /**
     * Estimated speed along the route in km/h.
     */
    estimatedSpeedKph?: number;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Optional quality statement.
     */
    quality?: Quality;
    /**
     * Route status (inRange/outOfRange).
     */
    routeStatus?: RouteStatus;
    /**
     * Time of the observation.
     */
    timestamp: Date;
    /**
     * Identifier of the route.
     */
    transportRouteId: string;
}

/**
 * Optional quality statement.
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
 * Route status (inRange/outOfRange).
 *
 * Route status (a closed string enum, not an extensible TypeId catalogue).
 */
export enum RouteStatus {
    InRange = "inRange",
    OutOfRange = "outOfRange",
}

/**
 * Transport route fault reports.
 */
export interface TransportRouteFault {
    /**
     * When acknowledged.
     */
    acknowledgementDate?: Date;
    /**
     * Acknowledgement state.
     */
    acknowledgementState?: AcknowledgementState;
    /**
     * When cleared.
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
     * Free-text description.
     */
    description?: string;
    /**
     * Whether this is an equipment fault. The fault type's catalogue category classifies the
     * type; this flag classifies this fault.
     */
    equipmentFault?: boolean;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Unique identifier of this fault report.
     */
    faultId: number;
    /**
     * Reference to a FaultTypeDefinition entry.
     */
    faultTypeId: string;
    /**
     * When last modified.
     */
    lastUpdated?: Date;
    /**
     * Values for the positional placeholders in the referenced FaultTypeDefinition's
     * messageTemplate, in order. Rendered as text so a value carries whatever form the fault
     * reports it in.
     */
    parameters?: string[];
    /**
     * Fault severity.
     */
    severity?: number;
    /**
     * Active or cleared.
     */
    status?: TransportRouteFaultStatus;
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
     * Identifier of the route this fault is about.
     */
    transportRouteId: string;
}

/**
 * Acknowledgement state.
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
 * Active or cleared.
 *
 * Whether a fault is still active or has been cleared.
 *
 * This is the only fault lifecycle the payload carries. Record archival
 * — distinguishing a live record from a retired one — is not modelled
 * here and is a matter for the publishing system.
 */
export enum TransportRouteFaultStatus {
    Active = "active",
    Cleared = "cleared",
}

/**
 * Transport route history entries.
 */
export interface TransportRouteHistoryEntry {
    /**
     * Category of the history event.
     */
    eventType?: string;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * When the log entry was recorded.
     */
    logDate: Date;
    /**
     * Equipment log data.
     */
    logEntry: string;
    /**
     * Unique identifier of this log entry.
     */
    logId: number;
    /**
     * User who created the entry.
     */
    logUser?: string;
    /**
     * Identifier of the route.
     */
    transportRouteId: string;
}

/**
 * Segment configuration — links a route segment to a transport link.
 *
 * Sibling type in the configuration file.
 */
export interface TransportRouteSegmentConfiguration {
    /**
     * When effective.
     */
    configurationDate: Date;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Ordered segment identifier within the route.
     */
    routeSegmentId: number;
    /**
     * Cross-domain reference to a transport link.
     */
    transportLinkId: string;
    /**
     * Route this segment belongs to.
     */
    transportRouteId: string;
}

/**
 * Transport route type definitions.
 */
export interface TransportRouteType {
    /**
     * Human-readable description.
     */
    description: string;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Additional notes.
     */
    notes?: string;
    /**
     * URL to specification.
     */
    specificationLink?: string;
    /**
     * Identifier of this transport route type.
     */
    typeId: string;
}

/**
 * Transport route command payloads.
 *
 * Discriminated on `kind`. Narrow with a switch to have
 * the compiler check exhaustiveness.
 */
export type TransportRouteCommand =
    | RequestTransportRouteStatusCommand;
