/*
 * File:        output/typescript/sdk/strategy.ts
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
export interface AllTypes {
    StrategyCommandResponse?: StrategyCommandResponse;
    StrategyDefinition?:      StrategyDefinition;
    StrategyStatus?:          StrategyStatusClass;
    StrategyStatusEnum?:      StrategyStatusEnumEnum;
    StrategyTrigger?:         StrategyTrigger;
    SyncCategoryEnum?:        SyncCategoryEnum;
    SyncComplete?:            SyncComplete;
    SyncRequest?:             SyncRequest;
    SyncStatus?:              SyncStatusEnum;
    TriggerStateEnum?:        TriggerState;
    TriggerUpdateErrorEnum?:  ErrorCode;
    [property: string]: any;
}

/**
 * Strategy command response payloads.
 */
export interface StrategyCommandResponse {
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
     * Strategy-specific rejection category. Only permitted when the outcome is not success or
     * pending.
     */
    errorCode?: ErrorCode;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Human-readable detail.
     */
    message?: string;
    /**
     * When the response was generated.
     */
    responseTime: Date;
    /**
     * Audit label copied from the command, if it carried one. Never used for authorisation or
     * idempotency.
     */
    serviceRequester?: string;
    /**
     * Outcome of the command execution.
     */
    status: StrategyCommandResponseStatus;
    /**
     * Identifier of the strategy the command targeted.
     */
    strategyId: string;
}

/**
 * Strategy-specific rejection category. Only permitted when the outcome is not success or
 * pending.
 *
 * Categorisation of strategy trigger rejection reason.
 */
export enum ErrorCode {
    AccessDenied = "accessDenied",
    NotAuthenticated = "notAuthenticated",
    Other = "other",
    StrategyIDDoesNotExist = "strategyIdDoesNotExist",
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
export enum StrategyCommandResponseStatus {
    Failed = "failed",
    Invalid = "invalid",
    NotSupported = "notSupported",
    Pending = "pending",
    Success = "success",
    TimedOut = "timedOut",
}

/**
 * Traffic management strategy static metadata.
 */
export interface StrategyDefinition {
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Geographic centre of the strategy area as WGS84 GeoJSON Point.
     */
    location?: Location;
    /**
     * Longer description of what the strategy does.
     */
    strategyDescription?: string;
    /**
     * Strategy identifier (e.g. 'STR00000001').
     */
    strategyId: string;
    /**
     * Human-readable name of the strategy.
     */
    strategyName: string;
}

/**
 * Geographic centre of the strategy area as WGS84 GeoJSON Point.
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
 * Traffic management strategy current status.
 */
export interface StrategyStatusClass {
    /**
     * Error information (e.g. 'Strategy will never become active').
     */
    errorMessage?: string;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Non-error status information (e.g. 'Strategy applied 6 of 10 measures').
     */
    statusMessage?: string;
    /**
     * When the strategy status changed.
     */
    strategyChangeStateTime: Date;
    /**
     * Identifier of the strategy this status is about.
     */
    strategyId: string;
    /**
     * Current status (active/inactive).
     */
    strategyStatus: StrategyStatusEnumEnum;
}

/**
 * Current status (active/inactive).
 *
 * Status of a traffic management strategy.
 */
export enum StrategyStatusEnumEnum {
    Active = "active",
    Inactive = "inactive",
}

/**
 * Strategy trigger command payloads.
 */
export interface StrategyTrigger {
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
     * When the command was issued.
     */
    requestTime: Date;
    /**
     * Audit label naming the requesting system. Sender-supplied, so it is never used for
     * authorisation or idempotency.
     */
    serviceRequester?: string;
    /**
     * Identifier of the strategy to trigger.
     */
    strategyId: string;
    /**
     * Desired trigger state (enabled/disabled).
     */
    triggerState: TriggerState;
}

/**
 * Desired trigger state (enabled/disabled).
 *
 * Desired state when triggering a strategy.
 */
export enum TriggerState {
    Disabled = "disabled",
    Enabled = "enabled",
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
 * Terminal message ending a strategy sync batch. Sent to the requester's reply node after
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
 * Request the current state of strategy objects. The Data Publisher answers to the
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
