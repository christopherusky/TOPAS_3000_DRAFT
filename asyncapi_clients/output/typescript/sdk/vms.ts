/*
 * File:        output/typescript/sdk/vms.ts
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
export interface AllTypes {
    ClearSignCommand?:      ClearSignCommand;
    DisplayGraphicCommand?: DisplayGraphicCommand;
    DisplayMessageCommand?: DisplayMessageCommand;
    SetBrightnessCommand?:  SetBrightnessCommand;
    SyncCategoryEnum?:      SyncCategoryEnum;
    SyncComplete?:          SyncComplete;
    SyncRequest?:           SyncRequest;
    SyncStatus?:            SyncStatusEnum;
    VmsColourCapability?:   VmsColourCapability;
    VmsCommandResponse?:    VmsCommandResponse;
    VmsConfiguration?:      VmsConfiguration;
    VmsDefinition?:         VmsDefinition;
    VmsDynamic?:            VmsDynamic;
    VmsFault?:              VmsFault;
    VmsHistoryEntry?:       VmsHistoryEntry;
    VmsStatus?:             VmsStatusEnum;
    VmsType?:               VmsType;
    [property: string]: any;
}

/**
 * Clear the sign (blank the display).
 */
export interface ClearSignCommand {
    /**
     * Caller-assigned identifier.
     */
    commandCorrelationId: number;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Discriminator for the tagged union. The fixed value "clearSign" selects the
     * ClearSignCommand payload; it is not free-text command content.
     */
    kind: ClearSignCommandKind;
    /**
     * When the command was issued.
     */
    requestTime: Date;
    /**
     * Identifier of the VMS receiving the command.
     */
    vmsId: string;
}

export enum ClearSignCommandKind {
    ClearSign = "clearSign",
}

/**
 * Display a graphic on the sign.
 */
export interface DisplayGraphicCommand {
    /**
     * Caller-assigned identifier.
     */
    commandCorrelationId: number;
    /**
     * How long to display the graphic.
     */
    displayDurationSeconds?: number;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * URL of the graphic to display.
     */
    graphicUrl: string;
    /**
     * Discriminator for the tagged union. The fixed value "displayGraphic" selects the
     * DisplayGraphicCommand payload; it is not free-text command content.
     */
    kind: DisplayGraphicCommandKind;
    /**
     * When the command was issued.
     */
    requestTime: Date;
    /**
     * Identifier of the VMS receiving the command.
     */
    vmsId: string;
}

export enum DisplayGraphicCommandKind {
    DisplayGraphic = "displayGraphic",
}

/**
 * Display a text message on the sign.
 */
export interface DisplayMessageCommand {
    /**
     * Caller-assigned identifier.
     */
    commandCorrelationId: number;
    /**
     * How long to display the message.
     */
    displayDurationSeconds?: number;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Discriminator for the tagged union. The fixed value "displayMessage" selects the
     * DisplayMessageCommand payload; it is not free-text command content.
     */
    kind: DisplayMessageCommandKind;
    /**
     * Message text to display.
     */
    message: string;
    /**
     * Message split by lines.
     */
    messageLines?: string[];
    /**
     * When the command was issued.
     */
    requestTime: Date;
    /**
     * Identifier of the VMS receiving the command.
     */
    vmsId: string;
}

export enum DisplayMessageCommandKind {
    DisplayMessage = "displayMessage",
}

/**
 * Set the brightness of the sign.
 */
export interface SetBrightnessCommand {
    /**
     * Brightness percentage to set.
     */
    brightnessPercent: number;
    /**
     * Caller-assigned identifier.
     */
    commandCorrelationId: number;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Discriminator for the tagged union. The fixed value "setBrightness" selects the
     * SetBrightnessCommand payload; it is not free-text command content.
     */
    kind: SetBrightnessCommandKind;
    /**
     * When the command was issued.
     */
    requestTime: Date;
    /**
     * Identifier of the VMS receiving the command.
     */
    vmsId: string;
}

export enum SetBrightnessCommandKind {
    SetBrightness = "setBrightness",
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
 * Terminal message ending a vms sync batch. Sent to the requester's reply node after the
 * last object, or instead of any objects when the filter matched none.
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
 * Request the current state of vms objects. The Data Publisher answers to the request's
 * reply-to address, not the domain's data channels: a sync batch (x-message-mode: sync,
 * x-sync-id/-sequence/-total) followed by a SyncComplete.
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
 * Colour capability of a variable message sign.
 *
 * Colour capability.
 */
export enum VmsColourCapability {
    Amber = "amber",
    FullColour = "fullColour",
    Monochrome = "monochrome",
    MultiColour = "multiColour",
}

/**
 * Variable message sign command response payloads.
 */
export interface VmsCommandResponse {
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
    status: VmsCommandResponseStatus;
    /**
     * Identifier of the VMS the command targeted.
     */
    vmsId: string;
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
export enum VmsCommandResponseStatus {
    Failed = "failed",
    Invalid = "invalid",
    NotSupported = "notSupported",
    Pending = "pending",
    Success = "success",
    TimedOut = "timedOut",
}

/**
 * Variable message sign operational configuration.
 */
export interface VmsConfiguration {
    /**
     * Whether auto-brightness is enabled.
     */
    autoBrightness?: boolean;
    /**
     * When effective.
     */
    configurationDate: Date;
    /**
     * Default brightness percentage.
     */
    defaultBrightnessPercent?: number;
    /**
     * Default message display time in seconds.
     */
    defaultDisplayTimeSeconds?: number;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Text scroll speed.
     */
    scrollSpeed?: number;
    /**
     * VMS this configuration applies to.
     */
    vmsId: string;
}

/**
 * Variable message sign equipment details.
 */
export interface VmsDefinition {
    /**
     * Characters per line.
     */
    charactersPerLine?: number;
    /**
     * Colour capability.
     */
    colourCapability?: VmsColourCapability;
    /**
     * When this definition was created.
     */
    creationDate: Date;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Distance along that transport link, in metres. Optional.
     */
    linkDistanceMetres?: number;
    /**
     * Physical location.
     */
    location?: Location;
    /**
     * Longer description.
     */
    longDescription?: string;
    /**
     * Network link the sign stands on. At most one of this and networkPathId. Optional.
     */
    networkLinkId?: string;
    /**
     * Network path the sign stands on. At most one of this and networkLinkId. Optional.
     */
    networkPathId?: string;
    /**
     * Number of text lines.
     */
    numberOfLines?: number;
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
     * Sign height in pixels/units.
     */
    signHeight?: number;
    /**
     * Sign width in pixels/units.
     */
    signWidth?: number;
    /**
     * Whether the sign supports graphics.
     */
    supportsGraphics?: boolean;
    /**
     * Transport link this sign monitors or sits on. Optional.
     */
    transportLinkId?: string;
    /**
     * Unique identifier for a VMS type (sign type catalogue entry).
     */
    typeId: string;
    /**
     * Unique identifier of this VMS.
     */
    vmsId: string;
}

/**
 * Physical location.
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
 * Variable message sign dynamic measurement data.
 */
export interface VmsDynamic {
    /**
     * Current brightness percentage.
     */
    brightnessPercent?: number;
    /**
     * Currently displayed message text.
     */
    currentMessage?: string;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * URL of the currently displayed graphic.
     */
    graphicUrl?: string;
    /**
     * When the message expires.
     */
    messageExpiryTime?: Date;
    /**
     * Message split by lines.
     */
    messageLines?: string[];
    /**
     * When the current message was set.
     */
    messageSetTime?: Date;
    /**
     * Optional quality statement.
     */
    quality?: Quality;
    /**
     * Operational status.
     */
    status?: VmsStatusEnum;
    /**
     * Time of the observation.
     */
    timestamp: Date;
    /**
     * Identifier of the VMS.
     */
    vmsId: string;
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
 * Operational status.
 *
 * Operational status of a variable message sign.
 */
export enum VmsStatusEnum {
    Blank = "blank",
    Displaying = "displaying",
    Fault = "fault",
    Off = "off",
    On = "on",
    Unknown = "unknown",
}

/**
 * Variable message sign fault reports.
 */
export interface VmsFault {
    /**
     * When the fault was acknowledged.
     */
    acknowledgementDate?: Date;
    /**
     * Current acknowledgement state.
     */
    acknowledgementState?: AcknowledgementState;
    /**
     * When the fault was cleared.
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
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Unique identifier of this fault report.
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
    status?: VmsFaultStatus;
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
     * Identifier of the VMS this fault is about.
     */
    vmsId: string;
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
export enum VmsFaultStatus {
    Active = "active",
    Cleared = "cleared",
}

/**
 * Variable message sign history entries.
 */
export interface VmsHistoryEntry {
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
     * Equipment log data (free-form text).
     */
    logEntry: string;
    /**
     * Unique identifier of this log entry.
     */
    logId: number;
    /**
     * User who created the log entry.
     */
    logUser?: string;
    /**
     * Identifier of the VMS this log entry is about.
     */
    vmsId: string;
}

/**
 * Variable message sign type definitions.
 */
export interface VmsType {
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
     * URL to an external specification document.
     */
    specificationLink?: string;
    /**
     * Identifier of this VMS type.
     */
    typeId: string;
}

/**
 * Variable message sign command payloads.
 *
 * Discriminated on `kind`. Narrow with a switch to have
 * the compiler check exhaustiveness.
 */
export type VmsCommand =
    | ClearSignCommand
    | DisplayGraphicCommand
    | DisplayMessageCommand
    | SetBrightnessCommand;
