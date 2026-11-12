/*
 * File:        output/typescript/sdk/detector.ts
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
export interface AllTypes {
    CalibrateDetectorCommand?:    CalibrateDetectorCommand;
    DetectorCommandResponse?:     DetectorCommandResponse;
    DetectorConfiguration?:       DetectorConfiguration;
    DetectorDefinition?:          DetectorDefinition;
    DetectorFault?:               DetectorFault;
    DetectorHistoryEntry?:        DetectorHistoryEntry;
    DetectorType?:                DetectorType;
    FlowDynamic?:                 FlowDynamic;
    HeadwayDynamic?:              HeadwayDynamic;
    OccupancyDynamic?:            OccupancyDynamic;
    QueueDynamic?:                QueueDynamic;
    ResetDetectorCommand?:        ResetDetectorCommand;
    SetThresholdDetectorCommand?: SetThresholdDetectorCommand;
    SpeedDynamic?:                SpeedDynamic;
    SyncCategoryEnum?:            SyncCategoryEnum;
    SyncComplete?:                SyncComplete;
    SyncRequest?:                 SyncRequest;
    SyncStatus?:                  SyncStatusEnum;
    [property: string]: any;
}

/**
 * Trigger a calibration cycle on the detector.
 */
export interface CalibrateDetectorCommand {
    /**
     * Caller-assigned identifier correlating a command request with its response. Not an
     * identifier of the command itself. Named to avoid collision with the AMQP 1.0
     * correlation-id message property.
     */
    commandCorrelationId: number;
    /**
     * Identifier of the detector receiving the command.
     */
    detectorId: string;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Discriminator for the tagged union. The fixed value "calibrate" selects the
     * CalibrateDetectorCommand payload; it is not free-text command content.
     */
    kind: CalibrateDetectorCommandKind;
    /**
     * When the command was issued.
     */
    requestTime: Date;
}

export enum CalibrateDetectorCommandKind {
    Calibrate = "calibrate",
}

/**
 * Detector command response payloads.
 */
export interface DetectorCommandResponse {
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
     * Identifier of the detector the command targeted.
     */
    detectorId: string;
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
    status: DetectorCommandResponseStatus;
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
export enum DetectorCommandResponseStatus {
    Failed = "failed",
    Invalid = "invalid",
    NotSupported = "notSupported",
    Pending = "pending",
    Success = "success",
    TimedOut = "timedOut",
}

/**
 * Detector operational configuration.
 */
export interface DetectorConfiguration {
    /**
     * When this configuration record became effective.
     */
    configurationDate: Date;
    /**
     * Identifier of the detector this configuration applies to.
     */
    detectorId: string;
    /**
     * Vendor or local extensions. Open-content object.
     */
    extensions?: { [key: string]: any };
    /**
     * Downward threshold for alerting.
     */
    thresholdDown?: number;
    /**
     * Upward threshold for alerting (e.g. occupancy threshold).
     */
    thresholdUp?: number;
}

/**
 * Detector equipment details.
 */
export interface DetectorDefinition {
    /**
     * When this definition record was created.
     */
    creationDate: Date;
    /**
     * Unique identifier of this detector within its owning system.
     */
    detectorId: string;
    /**
     * Vendor or local extensions. Open-content object.
     */
    extensions?: { [key: string]: any };
    /**
     * Distance along that transport link, in metres. Optional.
     */
    linkDistanceMetres?: number;
    /**
     * Physical location of the detector. Optional.
     */
    location?: Location;
    /**
     * Longer description (context, installation notes, etc.).
     */
    longDescription?: string;
    /**
     * Network link the detector stands on. At most one of this and networkPathId. Optional.
     */
    networkLinkId?: string;
    /**
     * Network path the detector stands on. At most one of this and networkLinkId. Optional.
     */
    networkPathId?: string;
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
     * System that owns this detector definition. Often implicit from context.
     */
    participantId?: string;
    /**
     * Short human-readable name (e.g. 'A1 southbound').
     */
    shortDescription?: string;
    /**
     * Transport link this detector monitors or sits on. Optional.
     */
    transportLinkId?: string;
    /**
     * Unique identifier for a detector type (loop, radar, ANPR, ...).
     */
    typeId: string;
}

/**
 * Physical location of the detector. Optional.
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
 * Detector fault reports.
 */
export interface DetectorFault {
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
     * Identifier of the detector this fault is about.
     */
    detectorId: string;
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
     * Reference to a DetectorFaultType entry in the fault catalogue.
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
    status?: DetectorFaultStatus;
    /**
     * Reference to a SubsystemDefinition entry: the subsystem the fault arose in.
     */
    subsystemTypeId?: string;
    /**
     * Fault identifier allocated by the supplier or configuration manager for maintenance
     * purposes, if required.
     */
    supplierFaultNumber?: string;
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
export enum DetectorFaultStatus {
    Active = "active",
    Cleared = "cleared",
}

/**
 * Detector history entries.
 */
export interface DetectorHistoryEntry {
    /**
     * Identifier of the detector this log entry is about.
     */
    detectorId: string;
    /**
     * Category or type of the history event (e.g. 'recalibration', 'threshold_change').
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
}

/**
 * Detector type definitions.
 */
export interface DetectorType {
    /**
     * Human-readable description of the detector kind.
     */
    description: string;
    /**
     * Vendor or local extensions. Open-content object.
     */
    extensions?: { [key: string]: any };
    /**
     * Additional notes, e.g. deployment details.
     */
    notes?: string;
    /**
     * URL to an external specification document for this detector type.
     */
    specificationLink?: string;
    /**
     * Identifier of this detector type.
     */
    typeId: string;
}

/**
 * Vehicle flow count over a measurement period.
 */
export interface FlowDynamic {
    /**
     * Per-class vehicle counts in positional order — index 0 is Class1, index 7 is Class8. May
     * be shorter if trailing classes are absent. A null entry means that class was not
     * reported, as distinct from a count of zero.
     */
    classCounts?: Array<number | null>;
    /**
     * Identifier of the detector this reading is about.
     */
    detectorId: string;
    /**
     * Vendor or local extensions. Open-content object.
     */
    extensions?: { [key: string]: any };
    /**
     * Discriminator for the tagged union. The fixed value "flow" selects the FlowDynamic
     * payload; it is not free-text command content.
     */
    kind: FlowDynamicKind;
    /**
     * Duration of the measurement period in seconds.
     */
    periodSeconds?: number;
    /**
     * Optional quality statement for this measurement.
     */
    quality?: Quality;
    /**
     * Time the measurement was taken.
     */
    timestamp: Date;
    /**
     * Total vehicle count over the period. Omitted when only class counts are reported.
     */
    totalFlow?: number;
}

export enum FlowDynamicKind {
    Flow = "flow",
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
 * Mean inter-vehicle headway over a measurement period.
 */
export interface HeadwayDynamic {
    /**
     * Identifier of the detector this reading is about.
     */
    detectorId: string;
    /**
     * Vendor or local extensions. Open-content object.
     */
    extensions?: { [key: string]: any };
    /**
     * Discriminator for the tagged union. The fixed value "headway" selects the HeadwayDynamic
     * payload; it is not free-text command content.
     */
    kind: HeadwayDynamicKind;
    /**
     * Mean inter-vehicle headway in seconds.
     */
    meanHeadwaySeconds: number;
    /**
     * Duration of the measurement period in seconds.
     */
    periodSeconds?: number;
    /**
     * Optional quality statement for this measurement.
     */
    quality?: Quality;
    /**
     * Time the measurement was taken.
     */
    timestamp: Date;
}

export enum HeadwayDynamicKind {
    Headway = "headway",
}

/**
 * Detector occupancy (percentage of period occupied).
 */
export interface OccupancyDynamic {
    /**
     * Identifier of the detector this reading is about.
     */
    detectorId: string;
    /**
     * Vendor or local extensions. Open-content object.
     */
    extensions?: { [key: string]: any };
    /**
     * Discriminator for the tagged union. The fixed value "occupancy" selects the
     * OccupancyDynamic payload; it is not free-text command content.
     */
    kind: OccupancyDynamicKind;
    /**
     * Percentage occupancy over the period.
     */
    occupancyPercent: number;
    /**
     * Duration of the measurement period in seconds.
     */
    periodSeconds?: number;
    /**
     * Optional quality statement for this measurement.
     */
    quality?: Quality;
    /**
     * Time the measurement was taken.
     */
    timestamp: Date;
}

export enum OccupancyDynamicKind {
    Occupancy = "occupancy",
}

/**
 * Queue presence and severity reported by the detector.
 */
export interface QueueDynamic {
    /**
     * Identifier of the detector this reading is about.
     */
    detectorId: string;
    /**
     * Vendor or local extensions. Open-content object.
     */
    extensions?: { [key: string]: any };
    /**
     * Discriminator for the tagged union. The fixed value "queue" selects the QueueDynamic
     * payload; it is not free-text command content.
     */
    kind: QueueDynamicKind;
    /**
     * Optional quality statement for this measurement.
     */
    quality?: Quality;
    /**
     * Whether a queue is currently detected.
     */
    queuePresent: boolean;
    /**
     * Queue severity when a queue is present.
     */
    severity?: Severity;
    /**
     * Time the measurement was taken.
     */
    timestamp: Date;
}

export enum QueueDynamicKind {
    Queue = "queue",
}

/**
 * Queue severity when a queue is present.
 */
export enum Severity {
    Congested = "congested",
    Delayed = "delayed",
    Dense = "dense",
    Normal = "normal",
    Severe = "severe",
    StopAndGo = "stopAndGo",
}

/**
 * Reset the detector to its default state.
 */
export interface ResetDetectorCommand {
    /**
     * Caller-assigned identifier correlating a command request with its response. Not an
     * identifier of the command itself. Named to avoid collision with the AMQP 1.0
     * correlation-id message property.
     */
    commandCorrelationId: number;
    /**
     * Identifier of the detector receiving the command.
     */
    detectorId: string;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Discriminator for the tagged union. The fixed value "reset" selects the
     * ResetDetectorCommand payload; it is not free-text command content.
     */
    kind: ResetDetectorCommandKind;
    /**
     * When the command was issued.
     */
    requestTime: Date;
}

export enum ResetDetectorCommandKind {
    Reset = "reset",
}

/**
 * Set occupancy/flow threshold values on the detector.
 */
export interface SetThresholdDetectorCommand {
    /**
     * Caller-assigned identifier correlating a command request with its response. Not an
     * identifier of the command itself. Named to avoid collision with the AMQP 1.0
     * correlation-id message property.
     */
    commandCorrelationId: number;
    /**
     * Identifier of the detector receiving the command.
     */
    detectorId: string;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Discriminator for the tagged union. The fixed value "setThreshold" selects the
     * SetThresholdDetectorCommand payload; it is not free-text command content.
     */
    kind: SetThresholdDetectorCommandKind;
    /**
     * When the command was issued.
     */
    requestTime: Date;
    /**
     * Downward threshold value to set.
     */
    thresholdDown: number;
    /**
     * Upward threshold value to set.
     */
    thresholdUp: number;
}

export enum SetThresholdDetectorCommandKind {
    SetThreshold = "setThreshold",
}

/**
 * Mean vehicle speed over a measurement period.
 */
export interface SpeedDynamic {
    /**
     * Identifier of the detector this reading is about.
     */
    detectorId: string;
    /**
     * Vendor or local extensions. Open-content object.
     */
    extensions?: { [key: string]: any };
    /**
     * Discriminator for the tagged union. The fixed value "speed" selects the SpeedDynamic
     * payload; it is not free-text command content.
     */
    kind: SpeedDynamicKind;
    /**
     * Mean vehicle speed in kilometres per hour.
     */
    meanSpeedKph: number;
    /**
     * Duration of the measurement period in seconds.
     */
    periodSeconds?: number;
    /**
     * Optional quality statement for this measurement.
     */
    quality?: Quality;
    /**
     * Time the measurement was taken.
     */
    timestamp: Date;
}

export enum SpeedDynamicKind {
    Speed = "speed",
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
 * Terminal message ending a detector sync batch. Sent to the requester's reply node after
 * the last object, or instead of any objects when the filter matched none.
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
 * Request the current state of detector objects. The Data Publisher answers to the
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
 * Detector command payloads.
 *
 * Discriminated on `kind`. Narrow with a switch to have
 * the compiler check exhaustiveness.
 */
export type DetectorCommand =
    | CalibrateDetectorCommand
    | ResetDetectorCommand
    | SetThresholdDetectorCommand;

/**
 * Detector dynamic measurement data.
 *
 * Discriminated on `kind`. Narrow with a switch to have
 * the compiler check exhaustiveness.
 */
export type DetectorMeasurement =
    | FlowDynamic
    | HeadwayDynamic
    | OccupancyDynamic
    | QueueDynamic
    | SpeedDynamic;
