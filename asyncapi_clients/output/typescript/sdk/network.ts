/*
 * File:        output/typescript/sdk/network.ts
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
export interface AllTypes {
    NetworkLink?:      NetworkLink;
    NetworkNode?:      NetworkNode;
    NetworkPath?:      NetworkPath;
    NetworkTurn?:      NetworkTurn;
    NetworkZone?:      NetworkZone;
    SyncCategoryEnum?: SyncCategoryEnum;
    SyncComplete?:     SyncComplete;
    SyncRequest?:      SyncRequest;
    SyncStatus?:       Status;
    [property: string]: any;
}

/**
 * Road network link between two nodes.
 */
export interface NetworkLink {
    /**
     * Human-readable name.
     */
    description?: string;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Start node reference.
     */
    fromNodeId: string;
    /**
     * Polyline shape of the link as GeoJSON LineString.
     */
    geometry?: Geometry;
    /**
     * Unique identifier of this link.
     */
    linkId: string;
    /**
     * End node reference.
     */
    toNodeId: string;
}

/**
 * Polyline shape of the link as GeoJSON LineString.
 *
 * A GeoJSON LineString geometry (RFC 7946).
 *
 * An ordered sequence of two or more positions forming a polyline.
 * Coordinates are [longitude, latitude] per GeoJSON specification.
 *
 * Turn path geometry as GeoJSON LineString.
 *
 * Zone boundary geometry as GeoJSON LineString.
 */
export interface Geometry {
    /**
     * Ordered [longitude, latitude] pairs forming a polyline.
     */
    coordinates: Array<any[]>;
    type?:       GeometryType;
}

export enum GeometryType {
    LineString = "LineString",
}

/**
 * Road network node (junction or endpoint).
 */
export interface NetworkNode {
    /**
     * Human-readable name.
     */
    description?: string;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Geographic position as GeoJSON Point.
     */
    location?: Location;
    /**
     * Unique identifier of this node.
     */
    nodeId: string;
}

/**
 * Geographic position as GeoJSON Point.
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
    type?:       LocationType;
}

export enum LocationType {
    Point = "Point",
}

/**
 * Road network path (ordered sequence of links).
 */
export interface NetworkPath {
    /**
     * Human-readable name.
     */
    description?: string;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Distance to trim from the end of the last link.
     */
    finalTrimMeters?: number;
    /**
     * Distance to trim from the start of the first link.
     */
    initialTrimMeters?: number;
    /**
     * Ordered links defining the path. List order is the route sequence. Omit the field rather
     * than send an empty list. A link may appear more than once, since a route can traverse the
     * same link twice.
     */
    linkIds?: string[];
    /**
     * Unique identifier of this path.
     */
    pathId: string;
    /**
     * Whether this is a permanent or temporary path.
     */
    permanent?: boolean;
    /**
     * Path type classification (e.g. bus route, diversion).
     */
    typeId?: string;
}

/**
 * Road network turn (movement through a node between links).
 */
export interface NetworkTurn {
    /**
     * Whether this turn is permitted.
     */
    allowed: boolean;
    /**
     * Human-readable description.
     */
    description?: string;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Approach link.
     */
    fromLinkId: string;
    /**
     * Turn path geometry as GeoJSON LineString.
     */
    geometry?: Geometry;
    /**
     * Departure link.
     */
    toLinkId: string;
    /**
     * Unique identifier of this turn.
     */
    turnId: string;
}

/**
 * Road network zone (area grouping of links/nodes).
 */
export interface NetworkZone {
    /**
     * Human-readable name.
     */
    description?: string;
    /**
     * Vendor or local extensions.
     */
    extensions?: { [key: string]: any };
    /**
     * Zone boundary geometry as GeoJSON LineString.
     */
    geometry?: Geometry;
    /**
     * The links that make up the zone, as a set: the order of the array carries no meaning (v1
     * describes a zone as an unordered sequence of links). Omit the field rather than send an
     * empty list. Each link appears at most once.
     */
    linkIds?: string[];
    /**
     * Whether this is a permanent or temporary zone. Absent means permanent (the v1 default for
     * a zone).
     */
    permanent?: boolean;
    /**
     * Zone type classification (e.g. electoral ward, incident area).
     */
    typeId?: string;
    /**
     * Unique identifier of this zone.
     */
    zoneId: string;
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
 * Terminal message ending a network topology sync batch. Sent to the requester's reply node
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
    status: Status;
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
export enum Status {
    Failed = "failed",
    Rejected = "rejected",
    Success = "success",
}

/**
 * Request the current state of network topology objects. The Data Publisher answers to the
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
