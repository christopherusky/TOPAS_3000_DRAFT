/*
 * File:        output/typescript/sdk/traffic_signal.ts
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
export interface AllTypes {
    FaultSeverityEnum?:            FaultSeverityEnum;
    ForceStageCommand?:            ForceStageCommand;
    HoldStageCommand?:             HoldStageCommand;
    PlanLine?:                     PlanLine;
    RequestStatusCommand?:         RequestStatusCommand;
    SetPlanCommand?:               SetPlanCommand;
    SignalGroupTimings?:           Composite;
    SyncCategoryEnum?:             SyncCategoryEnum;
    SyncComplete?:                 SyncComplete;
    SynchronizeClockCommand?:      SynchronizeClockCommand;
    SyncRequest?:                  SyncRequest;
    SyncStatus?:                   SyncStatusEnum;
    TrafficSignalCommandResponse?: TrafficSignalCommandResponse;
    TrafficSignalConfiguration?:   TrafficSignalConfiguration;
    TrafficSignalDefinition?:      TrafficSignalDefinition;
    TrafficSignalDynamicState?:    TrafficSignalDynamicState;
    TrafficSignalFault?:           TrafficSignalFault;
    TrafficSignalFaultEnum?:       TrafficSignalFaultType;
    TrafficSignalHistoryEntry?:    TrafficSignalHistoryEntry;
    TrafficSignalModeEnum?:        TrafficSignalMode;
    TrafficSignalSubModeEnum?:     TrafficSignalSubMode;
    TrafficSignalType?:            TrafficSignalTypeClass;
    TrafficSignalTypeEnum?:        TrafficSignalTypeEnumEnum;
    UrgencyEnum?:                  TrafficSignalFaultUrgency;
    [property: string]: any;
}

/**
 * DATEX II FaultSeverityEnum — inherited via Fault base (PDF §3.3).
 *
 * Severity of the fault.
 */
export enum FaultSeverityEnum {
    High = "high",
    Low = "low",
    Medium = "medium",
    Unknown = "unknown",
}

/**
 * Force a specific stage on a traffic signal controller.
 */
export interface ForceStageCommand {
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
     * Discriminator for the tagged union. The fixed value "forceStage" selects the
     * ForceStageCommand payload; it is not free-text command content.
     */
    kind: ForceStageCommandKind;
    /**
     * When the command was issued.
     */
    requestTime: Date;
    /**
     * Stage number to force.
     */
    stageNumber: number;
    /**
     * Identifier of the traffic signal receiving the command.
     */
    trafficSignalId: string;
}

export enum ForceStageCommandKind {
    ForceStage = "forceStage",
}

/**
 * Hold a specific stage on a traffic signal controller.
 */
export interface HoldStageCommand {
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
     * Discriminator for the tagged union. The fixed value "holdStage" selects the
     * HoldStageCommand payload; it is not free-text command content.
     */
    kind: HoldStageCommandKind;
    /**
     * When the command was issued.
     */
    requestTime: Date;
    /**
     * Stage number to hold.
     */
    stageNumber: number;
    /**
     * Identifier of the traffic signal receiving the command.
     */
    trafficSignalId: string;
}

export enum HoldStageCommandKind {
    HoldStage = "holdStage",
}

/**
 * A signal plan definition carried in the dynamic state (§3.5, §3.6).
 *
 * A plan line for this traffic signal. This is intended to provide information allowing the
 * plan data to be displayed to an operator. It contains the current plan data and would be
 * updated when the plan is changed. For adaptive central control, such as SCOOT, the
 * information should be available periodically and contain the plan at that instant.
 */
export interface PlanLine {
    /**
     * The current cycle time.
     */
    cycleTime?: number;
    /**
     * Vendor or forward-compatibility extensions. Open-content object.
     */
    extensions?: { [key: string]: any };
    /**
     * The offset to the first SignalGroupTiming in the list. Must be present if
     * SignalGroupTimingList is present.
     */
    offset?: number;
    /**
     * A user entered description of the active plan.
     */
    planDescription?: string;
    /**
     * The currently active plan.
     */
    planNumber: number;
    /**
     * The time the plan was started. Optional: a publisher emitting on a fixed interval may not
     * know the start time of the running plan.
     */
    planStartTime?: Date;
    /**
     * The signal group data for the plan.
     */
    signalGroupTimings?: Composite[];
}

/**
 * Signal-group timing row within a PlanLine (§3.5, §3.6).
 */
export interface Composite {
    /**
     * A text representation of the alternatives to the primary signal group.
     */
    alternateSignalGroup?: string;
    /**
     * Vendor or forward-compatibility extensions. Open-content object.
     */
    extensions?: { [key: string]: any };
    /**
     * A text representation of the primary signal group.
     */
    primarySignalGroup: string;
    /**
     * The length of the primary signal group. Note that there is no timing data associated with
     * the alternate signal group.
     */
    primarySignalGroupDuration: number;
}

/**
 * Request the current status of a traffic signal controller.
 */
export interface RequestStatusCommand {
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
     * RequestStatusCommand payload; it is not free-text command content.
     */
    kind: RequestStatusCommandKind;
    /**
     * When the command was issued.
     */
    requestTime: Date;
    /**
     * Identifier of the traffic signal receiving the command.
     */
    trafficSignalId: string;
}

export enum RequestStatusCommandKind {
    RequestStatus = "requestStatus",
}

/**
 * Set the active plan on a traffic signal controller.
 */
export interface SetPlanCommand {
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
     * Discriminator for the tagged union. The fixed value "setPlan" selects the SetPlanCommand
     * payload; it is not free-text command content.
     */
    kind: SetPlanCommandKind;
    /**
     * Plan number to activate. Same 1 to 99 range as the plan number a controller reports
     * (PlanLine.plan_number), so any plan the controller can be told to run can be reported
     * back.
     */
    planNumber: number;
    /**
     * When the command was issued.
     */
    requestTime: Date;
    /**
     * Identifier of the traffic signal receiving the command.
     */
    trafficSignalId: string;
}

export enum SetPlanCommandKind {
    SetPlan = "setPlan",
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
 * Terminal message ending a traffic signal sync batch. Sent to the requester's reply node
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
 * Request the current state of traffic signal objects. The Data Publisher answers to the
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
 * Synchronize the traffic signal controller's clock.
 */
export interface SynchronizeClockCommand {
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
     * Discriminator for the tagged union. The fixed value "synchronizeClock" selects the
     * SynchronizeClockCommand payload; it is not free-text command content.
     */
    kind: SynchronizeClockCommandKind;
    /**
     * When the command was issued.
     */
    requestTime: Date;
    /**
     * Target time to synchronize the controller clock to.
     */
    targetTime: Date;
    /**
     * Identifier of the traffic signal receiving the command.
     */
    trafficSignalId: string;
}

export enum SynchronizeClockCommandKind {
    SynchronizeClock = "synchronizeClock",
}

/**
 * Traffic signal command response (reused from TS004.0061:2010 Annex D).
 */
export interface TrafficSignalCommandResponse {
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
    status: TrafficSignalCommandResponseStatus;
    /**
     * Identifier of the traffic signal the command targeted.
     */
    trafficSignalId: string;
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
export enum TrafficSignalCommandResponseStatus {
    Failed = "failed",
    Invalid = "invalid",
    NotSupported = "notSupported",
    Pending = "pending",
    Success = "success",
    TimedOut = "timedOut",
}

/**
 * Traffic signal operational configuration (TS004.0064bis:2017 §3.4).
 */
export interface TrafficSignalConfiguration {
    /**
     * When this configuration record became effective.
     */
    configurationDate?: Date;
    /**
     * Textual description of controller type.
     */
    controllerType?: string;
    /**
     * Vendor or forward-compatibility extensions. Open-content object.
     */
    extensions?: { [key: string]: any };
    /**
     * Group of signals to which this traffic signal belongs.
     */
    subArea?: string;
    /**
     * Name of supplier.
     */
    supplier?: string;
    /**
     * Identifier of the traffic signal this configuration applies to.
     */
    trafficSignalId: string;
    /**
     * Type of the traffic signal.
     */
    trafficSignalType?: TrafficSignalTypeEnumEnum;
    /**
     * Operator's own reference for this traffic signal.
     */
    userCodeNumber?: string;
}

/**
 * Type of the traffic signal.
 *
 * PDF §4.2 — traffic-signal types. XSD values (camelCase) are authoritative.
 */
export enum TrafficSignalTypeEnumEnum {
    Intersection = "intersection",
    Other = "other",
    Pegasus = "pegasus",
    Pelican = "pelican",
    Puffin = "puffin",
    Toucan = "toucan",
    TramCrossing = "tramCrossing",
    WigWag = "wigWag",
}

/**
 * Traffic signal equipment details.
 */
export interface TrafficSignalDefinition {
    /**
     * Optional reference into the TrafficSignalType catalogue identifying the operational
     * control method (MOVA, SCOOT, fixed-time, ...). Orthogonal to
     * TrafficSignalConfiguration.trafficSignalType, which classifies the junction kind
     * (intersection, pelican, puffin, ...).
     */
    controlMethodId?: string;
    /**
     * When this definition record was created.
     */
    creationDate?: Date;
    /**
     * Vendor or forward-compatibility extensions. Open-content object.
     */
    extensions?: { [key: string]: any };
    /**
     * Location of traffic signal.
     */
    location?: Location;
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
     * System that owns this traffic signal definition. Often implicit from context.
     */
    participantId?: string;
    /**
     * Short human-readable name (e.g. 'Main St / High St junction').
     */
    shortDescription?: string;
    /**
     * Human readable identifier for the Traffic Signal.
     */
    trafficSignalId: string;
}

/**
 * Location of traffic signal.
 *
 * A GeoJSON Point geometry (RFC 7946).
 *
 * Coordinates are [longitude, latitude] per GeoJSON specification.
 */
export interface Location {
    /**
     * [longitude, latitude] in WGS84 decimal degrees.
     */
    coordinates: any[];
    type?:       Type;
}

export enum Type {
    Point = "Point",
}

/**
 * Traffic signal dynamic state with PlanLine/SignalGroupTimings (TS004.0064bis:2017 §3.5).
 */
export interface TrafficSignalDynamicState {
    /**
     * Vendor or forward-compatibility extensions. Open-content object.
     */
    extensions?: { [key: string]: any };
    /**
     * A plan line for this traffic signal. This is intended to provide information allowing the
     * plan data to be displayed to an operator. It contains the current plan data and would be
     * updated when the plan is changed. For adaptive central control, such as SCOOT, the
     * information should be available periodically and contain the plan at that instant.
     */
    planLine?: PlanLine;
    /**
     * Name for the sub-area that identifies the group of signals to which the associated
     * traffic signal equipment belongs.
     */
    subArea?: string;
    /**
     * Time the state was observed.
     */
    timestamp: Date;
    /**
     * Identifier of the traffic signal this reading is about.
     */
    trafficSignalId: string;
    /**
     * The high level mode of the traffic signal.
     */
    trafficSignalMode: TrafficSignalMode;
    /**
     * The detailed sub-mode of the traffic signal.
     */
    trafficSignalSubMode?: TrafficSignalSubMode;
}

/**
 * The high level mode of the traffic signal.
 *
 * PDF §4.3 — high-level traffic-signal modes.
 */
export enum TrafficSignalMode {
    CentralMode = "centralMode",
    LocalMode = "localMode",
    SignalsOffMode = "signalsOffMode",
    StartUpMode = "startUpMode",
    UnknownMode = "unknownMode",
}

/**
 * The detailed sub-mode of the traffic signal.
 *
 * PDF §4.4 — detailed traffic-signal sub-modes.
 *
 * Note: §5.1's mode/subMode combinations table references additional values
 * (hurryCall, cablelessLinkingBaseTime, cablelessLinkingNonBaseTime,
 * publicServiceVehiclePriority, lightRailTransitMode, pre-emption) that are
 * NOT defined in §4.4 or the XSD. This enum follows §4.4 + XSD.
 */
export enum TrafficSignalSubMode {
    CablelessLinking = "cablelessLinking",
    FixedModeSimpleOrLinked = "fixedModeSimpleOrLinked",
    FixedModeToCurrentMax = "fixedModeToCurrentMax",
    ManualMode = "manualMode",
    ManualStepOn = "manualStepOn",
    MovaMode = "movaMode",
    OtherDistributedMode = "otherDistributedMode",
    PreEmptLevel1 = "preEmptLevel1",
    PreEmptLevel2 = "preEmptLevel2",
    PreEmptLevel3 = "preEmptLevel3",
    UTCFixedTime = "utcFixedTime",
    UTCOtherCentralControlMethod = "utcOtherCentralControlMethod",
    UTCScoot = "utcScoot",
    VehicleActuated = "vehicleActuated",
    WardenControl = "wardenControl",
}

/**
 * Traffic signal fault reports with flattened DATEX II Fault base (TS004.0064bis:2017 §3.3,
 * F6).
 */
export interface TrafficSignalFault {
    /**
     * Vendor or forward-compatibility extensions. Open-content object.
     */
    extensions?: { [key: string]: any };
    /**
     * Time the fault was first recorded.
     */
    faultCreationTime?: Date;
    /**
     * Human-readable fault description.
     */
    faultDescription?: string;
    /**
     * Unique identifier for this fault.
     */
    faultIdentifier?: string;
    /**
     * Time fault was last updated.
     */
    faultLastUpdateTime: Date;
    /**
     * Severity of the fault.
     */
    faultSeverity?: FaultSeverityEnum;
    /**
     * Reference to a FaultTypeDefinition entry. The catalogue carries the code, category and
     * message template; new fault types are added as catalogue entries, with no schema change.
     * Required unless trafficSignalFaultType is given; when both are present this is the
     * authoritative classification.
     */
    faultTypeId?: string;
    /**
     * Values for the positional placeholders in the referenced FaultTypeDefinition's
     * messageTemplate, in order. Rendered as text so a value carries whatever form the fault
     * reports it in.
     */
    parameters?: string[];
    /**
     * Time fault was cleared.
     */
    trafficSignalFaultClearanceTime?: Date;
    /**
     * Coarse DATEX II fault bucket, as in TS004.0064bis:2017. Required unless faultTypeId is
     * given; when both are present faultTypeId is the authoritative classification.
     */
    trafficSignalFaultType?: TrafficSignalFaultType;
    /**
     * The urgency of this fault.
     */
    trafficSignalFaultUrgency?: TrafficSignalFaultUrgency;
    /**
     * Identifier of the traffic signal this fault applies to.
     */
    trafficSignalId: string;
    /**
     * A supplier unique identifier for this specific fault.
     */
    trafficSignalManufacturerFaultCode?: string;
}

/**
 * Coarse DATEX II fault bucket, as in TS004.0064bis:2017. Required unless faultTypeId is
 * given; when both are present faultTypeId is the authoritative classification.
 *
 * PDF §4.1 — traffic-signal fault types.
 */
export enum TrafficSignalFaultType {
    DetectorFault = "detectorFault",
    ElectronicsFault = "electronicsFault",
    IndicatorFault = "indicatorFault",
    LampFault = "lampFault",
    MechanicalFault = "mechanicalFault",
    Other = "other",
    UtilityFault = "utilityFault",
}

/**
 * The urgency of this fault.
 *
 * DATEX II UrgencyEnum — inherited by TrafficSignalFault.trafficSignalFaultUrgency (PDF
 * §3.3).
 *
 * ``UNKNOWN`` is a deliberate addition to the DATEX II baseline: a
 * publisher that cannot determine urgency must be able to say so rather
 * than defaulting to ``normalUrgency`` and understating a fault. This
 * mirrors ``FaultSeverityEnum.UNKNOWN``, which already carries the value.
 */
export enum TrafficSignalFaultUrgency {
    ExtremelyUrgent = "extremelyUrgent",
    NormalUrgency = "normalUrgency",
    Unknown = "unknown",
    Urgent = "urgent",
}

/**
 * Traffic signal history entries (reused from TS004.0061:2010 Annex D).
 */
export interface TrafficSignalHistoryEntry {
    /**
     * Category or type of the history event (e.g. 'configuration_change', 'fault_cleared').
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
     * Identifier of the traffic signal this log entry is about.
     */
    trafficSignalId: string;
}

/**
 * Traffic signal type definitions (reused from TS004.0061:2010 Annex D).
 */
export interface TrafficSignalTypeClass {
    /**
     * Human-readable description of the control method (e.g. 'MOVA', 'SCOOT', 'fixed-time',
     * 'vehicle-actuated').
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
     * URL to an external specification document for this control method.
     */
    specificationLink?: string;
    /**
     * Identifier of this control method. Referenced by TrafficSignalDefinition.controlMethodId.
     */
    typeId: string;
}

/**
 * Traffic signal command payloads (reused from TS004.0061:2010 Annex D).
 *
 * Discriminated on `kind`. Narrow with a switch to have
 * the compiler check exhaustiveness.
 */
export type TrafficSignalCommand =
    | ForceStageCommand
    | HoldStageCommand
    | RequestStatusCommand
    | SetPlanCommand
    | SynchronizeClockCommand;
