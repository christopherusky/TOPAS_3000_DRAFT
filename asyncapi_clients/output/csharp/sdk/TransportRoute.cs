/*
 * File:        output/csharp/sdk/TransportRoute.cs
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
namespace Topas.Utmc.TransportRoute
{
    using System;
    using System.Collections.Generic;

    using System.Globalization;
    using Newtonsoft.Json;
    using Newtonsoft.Json.Converters;

    public partial class AllTypes
    {
        [JsonProperty("RequestTransportRouteStatusCommand", NullValueHandling = NullValueHandling.Ignore)]
        public RequestTransportRouteStatusCommand RequestTransportRouteStatusCommand { get; set; }

        [JsonProperty("SyncCategoryEnum", NullValueHandling = NullValueHandling.Ignore)]
        public SyncCategoryEnum? SyncCategoryEnum { get; set; }

        [JsonProperty("SyncComplete", NullValueHandling = NullValueHandling.Ignore)]
        public SyncComplete SyncComplete { get; set; }

        [JsonProperty("SyncRequest", NullValueHandling = NullValueHandling.Ignore)]
        public SyncRequest SyncRequest { get; set; }

        [JsonProperty("SyncStatus", NullValueHandling = NullValueHandling.Ignore)]
        public SyncStatusEnum? SyncStatus { get; set; }

        [JsonProperty("TransportRouteCommandResponse", NullValueHandling = NullValueHandling.Ignore)]
        public TransportRouteCommandResponse TransportRouteCommandResponse { get; set; }

        [JsonProperty("TransportRouteConfiguration", NullValueHandling = NullValueHandling.Ignore)]
        public TransportRouteConfiguration TransportRouteConfiguration { get; set; }

        [JsonProperty("TransportRouteDefinition", NullValueHandling = NullValueHandling.Ignore)]
        public TransportRouteDefinition TransportRouteDefinition { get; set; }

        [JsonProperty("TransportRouteDynamic", NullValueHandling = NullValueHandling.Ignore)]
        public TransportRouteDynamic TransportRouteDynamic { get; set; }

        [JsonProperty("TransportRouteFault", NullValueHandling = NullValueHandling.Ignore)]
        public TransportRouteFault TransportRouteFault { get; set; }

        [JsonProperty("TransportRouteHistoryEntry", NullValueHandling = NullValueHandling.Ignore)]
        public TransportRouteHistoryEntry TransportRouteHistoryEntry { get; set; }

        [JsonProperty("TransportRouteSegmentConfiguration", NullValueHandling = NullValueHandling.Ignore)]
        public TransportRouteSegmentConfiguration TransportRouteSegmentConfiguration { get; set; }

        [JsonProperty("TransportRouteStatus", NullValueHandling = NullValueHandling.Ignore)]
        public RouteStatus? TransportRouteStatus { get; set; }

        [JsonProperty("TransportRouteType", NullValueHandling = NullValueHandling.Ignore)]
        public TransportRouteType TransportRouteType { get; set; }
    }

    /// <summary>
    /// Request the current status of a transport route.
    /// </summary>
    public partial class RequestTransportRouteStatusCommand
    {
        /// <summary>
        /// Caller-assigned identifier correlating a command request with its response. Not an
        /// identifier of the command itself. Named to avoid collision with the AMQP 1.0
        /// correlation-id message property.
        /// </summary>
        [JsonProperty("commandCorrelationId")]
        public long CommandCorrelationId { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Discriminator for the tagged union. The fixed value "requestStatus" selects the
        /// RequestTransportRouteStatusCommand payload; it is not free-text command content.
        /// </summary>
        [JsonProperty("kind")]
        public Kind Kind { get; set; }

        /// <summary>
        /// When the command was issued.
        /// </summary>
        [JsonProperty("requestTime")]
        public DateTimeOffset RequestTime { get; set; }

        /// <summary>
        /// Identifier of the route receiving the command.
        /// </summary>
        [JsonProperty("transportRouteId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string TransportRouteId { get; set; }
    }

    /// <summary>
    /// Terminal message ending a transport route sync batch. Sent to the requester's reply node
    /// after the last object, or instead of any objects when the filter matched none.
    /// </summary>
    public partial class SyncComplete
    {
        /// <summary>
        /// When the publisher finished the sync.
        /// </summary>
        [JsonProperty("completedTime")]
        public DateTimeOffset CompletedTime { get; set; }

        /// <summary>
        /// Vendor or forward-compatibility extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Human-readable explanation, for rejected or failed.
        /// </summary>
        [JsonProperty("message", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(FluffyMinMaxLengthCheckConverter))]
        public string Message { get; set; }

        /// <summary>
        /// Number of objects delivered in this batch. Zero is a valid answer and is the reason this
        /// message exists.
        /// </summary>
        [JsonProperty("objectCount")]
        public long ObjectCount { get; set; }

        /// <summary>
        /// Outcome of the sync.
        /// </summary>
        [JsonProperty("status")]
        public SyncStatusEnum Status { get; set; }

        /// <summary>
        /// Copied unchanged from the SyncRequest this completes, so a requester with several syncs
        /// in flight can tell them apart.
        /// </summary>
        [JsonProperty("syncCorrelationId")]
        public long SyncCorrelationId { get; set; }

        /// <summary>
        /// The x-sync-id shared by the messages of this batch. Normally absent when no objects
        /// matched, since there was no batch to identify. Only a rejected sync is required to omit
        /// it; a success or a failed sync may carry one (a failed sync began a batch).
        /// </summary>
        [JsonProperty("syncId", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(TentacledMinMaxLengthCheckConverter))]
        public string SyncId { get; set; }
    }

    /// <summary>
    /// Request the current state of transport route objects. The Data Publisher answers to the
    /// request's reply-to address, not the domain's data channels: a sync batch (x-message-mode:
    /// sync, x-sync-id/-sequence/-total) followed by a SyncComplete.
    /// </summary>
    public partial class SyncRequest
    {
        /// <summary>
        /// Restrict the sync to objects in an active state — for faults, those not yet cleared.
        /// Omitted is equivalent to false. This is the filter behind "send me every active fault":
        /// without it a subscriber would receive the full fault history it does not want.
        /// </summary>
        [JsonProperty("activeOnly", NullValueHandling = NullValueHandling.Ignore)]
        public bool? ActiveOnly { get; set; }

        /// <summary>
        /// Restrict the sync to these data categories. Omitted means every category the domain
        /// publishes.
        /// </summary>
        [JsonProperty("categories", NullValueHandling = NullValueHandling.Ignore)]
        public SyncCategoryEnum[] Categories { get; set; }

        /// <summary>
        /// Vendor or forward-compatibility extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Restrict the sync to these objects. Omitted means every object of the requested
        /// categories.
        /// </summary>
        [JsonProperty("objectIds", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(DecodeArrayConverter))]
        public string[] ObjectIds { get; set; }

        /// <summary>
        /// When the request was issued.
        /// </summary>
        [JsonProperty("requestTime")]
        public DateTimeOffset RequestTime { get; set; }

        /// <summary>
        /// Restrict the sync to objects changed at or after this time. Intended for a subscriber
        /// recovering from a known-length outage; omitted means no lower bound.
        /// </summary>
        [JsonProperty("since", NullValueHandling = NullValueHandling.Ignore)]
        public DateTimeOffset? Since { get; set; }

        /// <summary>
        /// Restrict the sync to objects in this sub-area. Omitted means every sub-area.
        /// </summary>
        [JsonProperty("subArea", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(StickyMinMaxLengthCheckConverter))]
        public string SubArea { get; set; }

        /// <summary>
        /// Caller-assigned identifier correlating this request with the sync batch that answers it.
        /// The Data Publisher echoes it in the batch's x-sync-request-id header. Not an identifier
        /// of the sync itself — that is x-sync-id. Named to avoid collision with the AMQP 1.0
        /// correlation-id message property, as for commandCorrelationId.
        /// </summary>
        [JsonProperty("syncCorrelationId")]
        public long SyncCorrelationId { get; set; }

        /// <summary>
        /// Restrict the sync to objects of this type. Omitted means every type.
        /// </summary>
        [JsonProperty("typeId", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(IndigoMinMaxLengthCheckConverter))]
        public string TypeId { get; set; }
    }

    /// <summary>
    /// Transport route command response payloads.
    /// </summary>
    public partial class TransportRouteCommandResponse
    {
        /// <summary>
        /// Machine-readable response code.
        /// </summary>
        [JsonProperty("code", NullValueHandling = NullValueHandling.Ignore)]
        public string Code { get; set; }

        /// <summary>
        /// Correlation identifier copied verbatim from the originating command request. Not an
        /// identifier of the command itself.
        /// </summary>
        [JsonProperty("commandCorrelationId")]
        public long CommandCorrelationId { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Human-readable response message.
        /// </summary>
        [JsonProperty("message", NullValueHandling = NullValueHandling.Ignore)]
        public string Message { get; set; }

        /// <summary>
        /// When the response was generated.
        /// </summary>
        [JsonProperty("responseTime")]
        public DateTimeOffset ResponseTime { get; set; }

        /// <summary>
        /// Outcome of the command execution.
        /// </summary>
        [JsonProperty("status")]
        public TransportRouteCommandResponseStatus Status { get; set; }

        /// <summary>
        /// Identifier of the route the command targeted.
        /// </summary>
        [JsonProperty("transportRouteId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string TransportRouteId { get; set; }
    }

    /// <summary>
    /// Transport route configuration and segment mapping.
    /// </summary>
    public partial class TransportRouteConfiguration
    {
        /// <summary>
        /// When this configuration became effective.
        /// </summary>
        [JsonProperty("configurationDate")]
        public DateTimeOffset ConfigurationDate { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Route end description.
        /// </summary>
        [JsonProperty("routeEnd", NullValueHandling = NullValueHandling.Ignore)]
        public string RouteEnd { get; set; }

        /// <summary>
        /// Route length in meters.
        /// </summary>
        [JsonProperty("routeLengthMeters", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double? RouteLengthMeters { get; set; }

        /// <summary>
        /// Human-readable route name.
        /// </summary>
        [JsonProperty("routeName", NullValueHandling = NullValueHandling.Ignore)]
        public string RouteName { get; set; }

        /// <summary>
        /// Route start description.
        /// </summary>
        [JsonProperty("routeStart", NullValueHandling = NullValueHandling.Ignore)]
        public string RouteStart { get; set; }

        /// <summary>
        /// Via point description.
        /// </summary>
        [JsonProperty("routeVia", NullValueHandling = NullValueHandling.Ignore)]
        public string RouteVia { get; set; }

        /// <summary>
        /// Route this configuration applies to.
        /// </summary>
        [JsonProperty("transportRouteId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string TransportRouteId { get; set; }
    }

    /// <summary>
    /// Transport route equipment details.
    /// </summary>
    public partial class TransportRouteDefinition
    {
        /// <summary>
        /// When this definition was created.
        /// </summary>
        [JsonProperty("creationDate")]
        public DateTimeOffset CreationDate { get; set; }

        /// <summary>
        /// Route endpoint location.
        /// </summary>
        [JsonProperty("endPoint", NullValueHandling = NullValueHandling.Ignore)]
        public EndPoint EndPoint { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Start point location.
        /// </summary>
        [JsonProperty("location", NullValueHandling = NullValueHandling.Ignore)]
        public EndPoint Location { get; set; }

        /// <summary>
        /// Longer description.
        /// </summary>
        [JsonProperty("longDescription", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(IndecentMinMaxLengthCheckConverter))]
        public string LongDescription { get; set; }

        /// <summary>
        /// ID on originating system.
        /// </summary>
        [JsonProperty("originatingObjectId", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string OriginatingObjectId { get; set; }

        /// <summary>
        /// Upstream system.
        /// </summary>
        [JsonProperty("originatingParticipantId", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string OriginatingParticipantId { get; set; }

        /// <summary>
        /// Owner or operator.
        /// </summary>
        [JsonProperty("ownerOperator", NullValueHandling = NullValueHandling.Ignore)]
        public string OwnerOperator { get; set; }

        /// <summary>
        /// Owning system.
        /// </summary>
        [JsonProperty("participantId", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string ParticipantId { get; set; }

        /// <summary>
        /// Short name.
        /// </summary>
        [JsonProperty("shortDescription", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(StickyMinMaxLengthCheckConverter))]
        public string ShortDescription { get; set; }

        /// <summary>
        /// Unique identifier of this route.
        /// </summary>
        [JsonProperty("transportRouteId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string TransportRouteId { get; set; }

        /// <summary>
        /// Unique identifier for a transport route type (road, bus, rail, ...).
        /// </summary>
        [JsonProperty("typeId")]
        [JsonConverter(typeof(IndigoMinMaxLengthCheckConverter))]
        public string TypeId { get; set; }
    }

    /// <summary>
    /// Route endpoint location.
    ///
    /// A geographic point: a coordinate plus optional altitude.
    ///
    /// Start point location.
    /// </summary>
    public partial class EndPoint
    {
        /// <summary>
        /// Altitude in metres above mean sea level. Optional. Bounds [-1000, 5000] inherited from
        /// utmc_location-v1.0.xsd AltitudeType.
        /// </summary>
        [JsonProperty("altitudeMeters", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(FluffyMinMaxValueCheckConverter))]
        public double? AltitudeMeters { get; set; }

        /// <summary>
        /// WGS84 coordinates.
        /// </summary>
        [JsonProperty("coordinates")]
        public Coordinates Coordinates { get; set; }
    }

    /// <summary>
    /// WGS84 coordinates.
    ///
    /// A WGS84 geographic coordinate pair (longitude, latitude).
    /// </summary>
    public partial class Coordinates
    {
        /// <summary>
        /// Latitude in decimal degrees (WGS84).
        /// </summary>
        [JsonProperty("latitude")]
        [JsonConverter(typeof(TentacledMinMaxValueCheckConverter))]
        public double Latitude { get; set; }

        /// <summary>
        /// Longitude in decimal degrees (WGS84).
        /// </summary>
        [JsonProperty("longitude")]
        [JsonConverter(typeof(StickyMinMaxValueCheckConverter))]
        public double Longitude { get; set; }
    }

    /// <summary>
    /// Transport route dynamic measurement data.
    /// </summary>
    public partial class TransportRouteDynamic
    {
        /// <summary>
        /// Current journey time in seconds.
        /// </summary>
        [JsonProperty("currentJourneyTimeSeconds", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double? CurrentJourneyTimeSeconds { get; set; }

        /// <summary>
        /// Estimated speed along the route in km/h.
        /// </summary>
        [JsonProperty("estimatedSpeedKph", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double? EstimatedSpeedKph { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Optional quality statement.
        /// </summary>
        [JsonProperty("quality", NullValueHandling = NullValueHandling.Ignore)]
        public Quality Quality { get; set; }

        /// <summary>
        /// Route status (inRange/outOfRange).
        /// </summary>
        [JsonProperty("routeStatus", NullValueHandling = NullValueHandling.Ignore)]
        public RouteStatus? RouteStatus { get; set; }

        /// <summary>
        /// Time of the observation.
        /// </summary>
        [JsonProperty("timestamp")]
        public DateTimeOffset Timestamp { get; set; }

        /// <summary>
        /// Identifier of the route.
        /// </summary>
        [JsonProperty("transportRouteId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string TransportRouteId { get; set; }
    }

    /// <summary>
    /// Optional quality statement.
    ///
    /// Quality statement attached to a UTMC data object or measurement.
    /// </summary>
    public partial class Quality
    {
        /// <summary>
        /// Vendor or local extensions. Open-content object.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// When this quality statement was last updated.
        /// </summary>
        [JsonProperty("lastUpdated", NullValueHandling = NullValueHandling.Ignore)]
        public DateTimeOffset? LastUpdated { get; set; }

        /// <summary>
        /// Identifier of the UTMC participant system that owns this quality statement.
        /// </summary>
        [JsonProperty("participantId", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string ParticipantId { get; set; }

        /// <summary>
        /// Identifier of this Quality statement.
        /// </summary>
        [JsonProperty("qualityId")]
        public long QualityId { get; set; }

        /// <summary>
        /// Name of the data source (e.g. 'SCOOT', 'MIDAS'). Required: a quality statement naming no
        /// source cannot be interpreted.
        /// </summary>
        [JsonProperty("sourceName")]
        [JsonConverter(typeof(FluffyMinMaxLengthCheckConverter))]
        public string SourceName { get; set; }

        /// <summary>
        /// Free-text description of the source kind.
        /// </summary>
        [JsonProperty("sourceType", NullValueHandling = NullValueHandling.Ignore)]
        public string SourceType { get; set; }
    }

    /// <summary>
    /// Transport route fault reports.
    /// </summary>
    public partial class TransportRouteFault
    {
        /// <summary>
        /// When acknowledged.
        /// </summary>
        [JsonProperty("acknowledgementDate", NullValueHandling = NullValueHandling.Ignore)]
        public DateTimeOffset? AcknowledgementDate { get; set; }

        /// <summary>
        /// Acknowledgement state.
        /// </summary>
        [JsonProperty("acknowledgementState", NullValueHandling = NullValueHandling.Ignore)]
        public AcknowledgementState? AcknowledgementState { get; set; }

        /// <summary>
        /// When cleared.
        /// </summary>
        [JsonProperty("clearedDate", NullValueHandling = NullValueHandling.Ignore)]
        public DateTimeOffset? ClearedDate { get; set; }

        /// <summary>
        /// Whether this is a communications fault. See equipmentFault for how it relates to the
        /// catalogue category.
        /// </summary>
        [JsonProperty("communicationsFault", NullValueHandling = NullValueHandling.Ignore)]
        public bool? CommunicationsFault { get; set; }

        /// <summary>
        /// When the fault was first raised.
        /// </summary>
        [JsonProperty("creationDate")]
        public DateTimeOffset CreationDate { get; set; }

        /// <summary>
        /// Free-text description.
        /// </summary>
        [JsonProperty("description", NullValueHandling = NullValueHandling.Ignore)]
        public string Description { get; set; }

        /// <summary>
        /// Whether this is an equipment fault. The fault type's catalogue category classifies the
        /// type; this flag classifies this fault.
        /// </summary>
        [JsonProperty("equipmentFault", NullValueHandling = NullValueHandling.Ignore)]
        public bool? EquipmentFault { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Unique identifier of this fault report.
        /// </summary>
        [JsonProperty("faultId")]
        public long FaultId { get; set; }

        /// <summary>
        /// Reference to a FaultTypeDefinition entry.
        /// </summary>
        [JsonProperty("faultTypeId")]
        [JsonConverter(typeof(IndigoMinMaxLengthCheckConverter))]
        public string FaultTypeId { get; set; }

        /// <summary>
        /// When last modified.
        /// </summary>
        [JsonProperty("lastUpdated", NullValueHandling = NullValueHandling.Ignore)]
        public DateTimeOffset? LastUpdated { get; set; }

        /// <summary>
        /// Values for the positional placeholders in the referenced FaultTypeDefinition's
        /// messageTemplate, in order. Rendered as text so a value carries whatever form the fault
        /// reports it in.
        /// </summary>
        [JsonProperty("parameters", NullValueHandling = NullValueHandling.Ignore)]
        public string[] Parameters { get; set; }

        /// <summary>
        /// Fault severity.
        /// </summary>
        [JsonProperty("severity", NullValueHandling = NullValueHandling.Ignore)]
        public long? Severity { get; set; }

        /// <summary>
        /// Active or cleared.
        /// </summary>
        [JsonProperty("status", NullValueHandling = NullValueHandling.Ignore)]
        public TransportRouteFaultStatus? Status { get; set; }

        /// <summary>
        /// Reference to a SubsystemDefinition entry: the subsystem the fault arose in.
        /// </summary>
        [JsonProperty("subsystemTypeId", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(IndigoMinMaxLengthCheckConverter))]
        public string SubsystemTypeId { get; set; }

        /// <summary>
        /// Fault identifier allocated by the supplier or configuration manager for maintenance
        /// purposes, if required.
        /// </summary>
        [JsonProperty("supplierFaultNumber", NullValueHandling = NullValueHandling.Ignore)]
        public string SupplierFaultNumber { get; set; }

        /// <summary>
        /// Identifier of the route this fault is about.
        /// </summary>
        [JsonProperty("transportRouteId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string TransportRouteId { get; set; }
    }

    /// <summary>
    /// Transport route history entries.
    /// </summary>
    public partial class TransportRouteHistoryEntry
    {
        /// <summary>
        /// Category of the history event.
        /// </summary>
        [JsonProperty("eventType", NullValueHandling = NullValueHandling.Ignore)]
        public string EventType { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// When the log entry was recorded.
        /// </summary>
        [JsonProperty("logDate")]
        public DateTimeOffset LogDate { get; set; }

        /// <summary>
        /// Equipment log data.
        /// </summary>
        [JsonProperty("logEntry")]
        public string LogEntry { get; set; }

        /// <summary>
        /// Unique identifier of this log entry.
        /// </summary>
        [JsonProperty("logId")]
        public long LogId { get; set; }

        /// <summary>
        /// User who created the entry.
        /// </summary>
        [JsonProperty("logUser", NullValueHandling = NullValueHandling.Ignore)]
        public string LogUser { get; set; }

        /// <summary>
        /// Identifier of the route.
        /// </summary>
        [JsonProperty("transportRouteId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string TransportRouteId { get; set; }
    }

    /// <summary>
    /// Segment configuration — links a route segment to a transport link.
    ///
    /// Sibling type in the configuration file.
    /// </summary>
    public partial class TransportRouteSegmentConfiguration
    {
        /// <summary>
        /// When effective.
        /// </summary>
        [JsonProperty("configurationDate")]
        public DateTimeOffset ConfigurationDate { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Ordered segment identifier within the route.
        /// </summary>
        [JsonProperty("routeSegmentId")]
        public long RouteSegmentId { get; set; }

        /// <summary>
        /// Cross-domain reference to a transport link.
        /// </summary>
        [JsonProperty("transportLinkId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string TransportLinkId { get; set; }

        /// <summary>
        /// Route this segment belongs to.
        /// </summary>
        [JsonProperty("transportRouteId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string TransportRouteId { get; set; }
    }

    /// <summary>
    /// Transport route type definitions.
    /// </summary>
    public partial class TransportRouteType
    {
        /// <summary>
        /// Human-readable description.
        /// </summary>
        [JsonProperty("description")]
        public string Description { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Additional notes.
        /// </summary>
        [JsonProperty("notes", NullValueHandling = NullValueHandling.Ignore)]
        public string Notes { get; set; }

        /// <summary>
        /// URL to specification.
        /// </summary>
        [JsonProperty("specificationLink", NullValueHandling = NullValueHandling.Ignore)]
        public Uri SpecificationLink { get; set; }

        /// <summary>
        /// Identifier of this transport route type.
        /// </summary>
        [JsonProperty("typeId")]
        [JsonConverter(typeof(IndigoMinMaxLengthCheckConverter))]
        public string TypeId { get; set; }
    }

    public enum Kind { RequestStatus };

    /// <summary>
    /// Data categories a sync may be restricted to.
    ///
    /// Mirrors the schema-per-category pattern: a subscriber that only holds
    /// a fault dashboard asks for ``fault`` alone rather than provoking a
    /// full republication of every definition and configuration it already
    /// has.
    /// </summary>
    public enum SyncCategoryEnum { Configuration, Definition, Fault, Measurement, Status, Type };

    /// <summary>
    /// Outcome of the sync.
    ///
    /// Outcome of a sync request (A12).
    /// </summary>
    public enum SyncStatusEnum { Failed, Rejected, Success };

    /// <summary>
    /// Outcome of the command execution.
    ///
    /// Outcome of a command execution.
    ///
    /// Every value except ``PENDING`` is terminal. Consumers SHOULD test for
    /// "not pending" rather than enumerating the terminal values, so that a
    /// future profile adding one does not strand them.
    /// </summary>
    public enum TransportRouteCommandResponseStatus { Failed, Invalid, NotSupported, Pending, Success, TimedOut };

    /// <summary>
    /// Route status (inRange/outOfRange).
    ///
    /// Route status (a closed string enum, not an extensible TypeId catalogue).
    /// </summary>
    public enum RouteStatus { InRange, OutOfRange };

    /// <summary>
    /// Acknowledgement state.
    ///
    /// Acknowledgement state of a device fault.
    ///
    /// Maps from legacy XSD integer values:
    /// - 0 -> notAcknowledged
    /// - 1 -> acknowledged
    /// - 2 -> deferred
    /// - 3 -> recordedForAction
    /// </summary>
    public enum AcknowledgementState { Acknowledged, Deferred, NotAcknowledged, RecordedForAction };

    /// <summary>
    /// Active or cleared.
    ///
    /// Whether a fault is still active or has been cleared.
    ///
    /// This is the only fault lifecycle the payload carries. Record archival
    /// — distinguishing a live record from a retired one — is not modelled
    /// here and is a matter for the publishing system.
    /// </summary>
    public enum TransportRouteFaultStatus { Active, Cleared };

    internal static class Converter
    {
        public static readonly JsonSerializerSettings Settings = new JsonSerializerSettings
        {
            MetadataPropertyHandling = MetadataPropertyHandling.Ignore,
            DateParseHandling = DateParseHandling.None,
            Converters =
            {
                KindConverter.Singleton,
                SyncCategoryEnumConverter.Singleton,
                SyncStatusEnumConverter.Singleton,
                TransportRouteCommandResponseStatusConverter.Singleton,
                RouteStatusConverter.Singleton,
                AcknowledgementStateConverter.Singleton,
                TransportRouteFaultStatusConverter.Singleton,
                new IsoDateTimeConverter { DateTimeStyles = DateTimeStyles.AssumeUniversal }
            },
        };
    }

    internal class KindConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(Kind) || t == typeof(Kind?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            if (value == "requestStatus")
            {
                return Kind.RequestStatus;
            }
            throw new Exception("Cannot unmarshal type Kind");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (Kind)untypedValue;
            if (value == Kind.RequestStatus)
            {
                serializer.Serialize(writer, "requestStatus");
                return;
            }
            throw new Exception("Cannot marshal type Kind");
        }

        public static readonly KindConverter Singleton = new KindConverter();
    }

    internal class PurpleMinMaxLengthCheckConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(string);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            var value = serializer.Deserialize<string>(reader);
            if (value.Length >= 1 && value.Length <= 32)
            {
                return value;
            }
            throw new Exception("Cannot unmarshal type string");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            var value = (string)untypedValue;
            if (value.Length >= 1 && value.Length <= 32)
            {
                serializer.Serialize(writer, value);
                return;
            }
            throw new Exception("Cannot marshal type string");
        }

        public static readonly PurpleMinMaxLengthCheckConverter Singleton = new PurpleMinMaxLengthCheckConverter();
    }

    internal class SyncCategoryEnumConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(SyncCategoryEnum) || t == typeof(SyncCategoryEnum?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "configuration":
                    return SyncCategoryEnum.Configuration;
                case "definition":
                    return SyncCategoryEnum.Definition;
                case "fault":
                    return SyncCategoryEnum.Fault;
                case "measurement":
                    return SyncCategoryEnum.Measurement;
                case "status":
                    return SyncCategoryEnum.Status;
                case "type":
                    return SyncCategoryEnum.Type;
            }
            throw new Exception("Cannot unmarshal type SyncCategoryEnum");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (SyncCategoryEnum)untypedValue;
            switch (value)
            {
                case SyncCategoryEnum.Configuration:
                    serializer.Serialize(writer, "configuration");
                    return;
                case SyncCategoryEnum.Definition:
                    serializer.Serialize(writer, "definition");
                    return;
                case SyncCategoryEnum.Fault:
                    serializer.Serialize(writer, "fault");
                    return;
                case SyncCategoryEnum.Measurement:
                    serializer.Serialize(writer, "measurement");
                    return;
                case SyncCategoryEnum.Status:
                    serializer.Serialize(writer, "status");
                    return;
                case SyncCategoryEnum.Type:
                    serializer.Serialize(writer, "type");
                    return;
            }
            throw new Exception("Cannot marshal type SyncCategoryEnum");
        }

        public static readonly SyncCategoryEnumConverter Singleton = new SyncCategoryEnumConverter();
    }

    internal class FluffyMinMaxLengthCheckConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(string);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            var value = serializer.Deserialize<string>(reader);
            if (value.Length >= 1)
            {
                return value;
            }
            throw new Exception("Cannot unmarshal type string");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            var value = (string)untypedValue;
            if (value.Length >= 1)
            {
                serializer.Serialize(writer, value);
                return;
            }
            throw new Exception("Cannot marshal type string");
        }

        public static readonly FluffyMinMaxLengthCheckConverter Singleton = new FluffyMinMaxLengthCheckConverter();
    }

    internal class SyncStatusEnumConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(SyncStatusEnum) || t == typeof(SyncStatusEnum?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "failed":
                    return SyncStatusEnum.Failed;
                case "rejected":
                    return SyncStatusEnum.Rejected;
                case "success":
                    return SyncStatusEnum.Success;
            }
            throw new Exception("Cannot unmarshal type SyncStatusEnum");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (SyncStatusEnum)untypedValue;
            switch (value)
            {
                case SyncStatusEnum.Failed:
                    serializer.Serialize(writer, "failed");
                    return;
                case SyncStatusEnum.Rejected:
                    serializer.Serialize(writer, "rejected");
                    return;
                case SyncStatusEnum.Success:
                    serializer.Serialize(writer, "success");
                    return;
            }
            throw new Exception("Cannot marshal type SyncStatusEnum");
        }

        public static readonly SyncStatusEnumConverter Singleton = new SyncStatusEnumConverter();
    }

    internal class TentacledMinMaxLengthCheckConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(string);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            var value = serializer.Deserialize<string>(reader);
            if (value.Length >= 1 && value.Length <= 64)
            {
                return value;
            }
            throw new Exception("Cannot unmarshal type string");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            var value = (string)untypedValue;
            if (value.Length >= 1 && value.Length <= 64)
            {
                serializer.Serialize(writer, value);
                return;
            }
            throw new Exception("Cannot marshal type string");
        }

        public static readonly TentacledMinMaxLengthCheckConverter Singleton = new TentacledMinMaxLengthCheckConverter();
    }

    internal class DecodeArrayConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(string[]);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            reader.Read();
            var value = new List<string>();
            while (reader.TokenType != JsonToken.EndArray)
            {
                var converter = PurpleMinMaxLengthCheckConverter.Singleton;
                var arrayItem = (string)converter.ReadJson(reader, typeof(string), null, serializer);
                value.Add(arrayItem);
                reader.Read();
            }
            return value.ToArray();
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            var value = (string[])untypedValue;
            writer.WriteStartArray();
            foreach (var arrayItem in value)
            {
                var converter = PurpleMinMaxLengthCheckConverter.Singleton;
                converter.WriteJson(writer, arrayItem, serializer);
            }
            writer.WriteEndArray();
            return;
        }

        public static readonly DecodeArrayConverter Singleton = new DecodeArrayConverter();
    }

    internal class StickyMinMaxLengthCheckConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(string);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            var value = serializer.Deserialize<string>(reader);
            if (value.Length >= 1 && value.Length <= 32)
            {
                return value;
            }
            throw new Exception("Cannot unmarshal type string");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            var value = (string)untypedValue;
            if (value.Length >= 1 && value.Length <= 32)
            {
                serializer.Serialize(writer, value);
                return;
            }
            throw new Exception("Cannot marshal type string");
        }

        public static readonly StickyMinMaxLengthCheckConverter Singleton = new StickyMinMaxLengthCheckConverter();
    }

    internal class IndigoMinMaxLengthCheckConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(string);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            var value = serializer.Deserialize<string>(reader);
            if (value.Length >= 1 && value.Length <= 64)
            {
                return value;
            }
            throw new Exception("Cannot unmarshal type string");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            var value = (string)untypedValue;
            if (value.Length >= 1 && value.Length <= 64)
            {
                serializer.Serialize(writer, value);
                return;
            }
            throw new Exception("Cannot marshal type string");
        }

        public static readonly IndigoMinMaxLengthCheckConverter Singleton = new IndigoMinMaxLengthCheckConverter();
    }

    internal class TransportRouteCommandResponseStatusConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(TransportRouteCommandResponseStatus) || t == typeof(TransportRouteCommandResponseStatus?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "failed":
                    return TransportRouteCommandResponseStatus.Failed;
                case "invalid":
                    return TransportRouteCommandResponseStatus.Invalid;
                case "notSupported":
                    return TransportRouteCommandResponseStatus.NotSupported;
                case "pending":
                    return TransportRouteCommandResponseStatus.Pending;
                case "success":
                    return TransportRouteCommandResponseStatus.Success;
                case "timedOut":
                    return TransportRouteCommandResponseStatus.TimedOut;
            }
            throw new Exception("Cannot unmarshal type TransportRouteCommandResponseStatus");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (TransportRouteCommandResponseStatus)untypedValue;
            switch (value)
            {
                case TransportRouteCommandResponseStatus.Failed:
                    serializer.Serialize(writer, "failed");
                    return;
                case TransportRouteCommandResponseStatus.Invalid:
                    serializer.Serialize(writer, "invalid");
                    return;
                case TransportRouteCommandResponseStatus.NotSupported:
                    serializer.Serialize(writer, "notSupported");
                    return;
                case TransportRouteCommandResponseStatus.Pending:
                    serializer.Serialize(writer, "pending");
                    return;
                case TransportRouteCommandResponseStatus.Success:
                    serializer.Serialize(writer, "success");
                    return;
                case TransportRouteCommandResponseStatus.TimedOut:
                    serializer.Serialize(writer, "timedOut");
                    return;
            }
            throw new Exception("Cannot marshal type TransportRouteCommandResponseStatus");
        }

        public static readonly TransportRouteCommandResponseStatusConverter Singleton = new TransportRouteCommandResponseStatusConverter();
    }

    internal class PurpleMinMaxValueCheckConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(double) || t == typeof(double?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<double>(reader);
            if (value >= 0)
            {
                return value;
            }
            throw new Exception("Cannot unmarshal type double");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (double)untypedValue;
            if (value >= 0)
            {
                serializer.Serialize(writer, value);
                return;
            }
            throw new Exception("Cannot marshal type double");
        }

        public static readonly PurpleMinMaxValueCheckConverter Singleton = new PurpleMinMaxValueCheckConverter();
    }

    internal class FluffyMinMaxValueCheckConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(double) || t == typeof(double?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<double>(reader);
            if (value >= -1000 && value <= 5000)
            {
                return value;
            }
            throw new Exception("Cannot unmarshal type double");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (double)untypedValue;
            if (value >= -1000 && value <= 5000)
            {
                serializer.Serialize(writer, value);
                return;
            }
            throw new Exception("Cannot marshal type double");
        }

        public static readonly FluffyMinMaxValueCheckConverter Singleton = new FluffyMinMaxValueCheckConverter();
    }

    internal class TentacledMinMaxValueCheckConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(double) || t == typeof(double?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<double>(reader);
            if (value >= -90 && value <= 90)
            {
                return value;
            }
            throw new Exception("Cannot unmarshal type double");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (double)untypedValue;
            if (value >= -90 && value <= 90)
            {
                serializer.Serialize(writer, value);
                return;
            }
            throw new Exception("Cannot marshal type double");
        }

        public static readonly TentacledMinMaxValueCheckConverter Singleton = new TentacledMinMaxValueCheckConverter();
    }

    internal class StickyMinMaxValueCheckConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(double) || t == typeof(double?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<double>(reader);
            if (value >= -180 && value <= 180)
            {
                return value;
            }
            throw new Exception("Cannot unmarshal type double");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (double)untypedValue;
            if (value >= -180 && value <= 180)
            {
                serializer.Serialize(writer, value);
                return;
            }
            throw new Exception("Cannot marshal type double");
        }

        public static readonly StickyMinMaxValueCheckConverter Singleton = new StickyMinMaxValueCheckConverter();
    }

    internal class IndecentMinMaxLengthCheckConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(string);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            var value = serializer.Deserialize<string>(reader);
            if (value.Length >= 1 && value.Length <= 2000)
            {
                return value;
            }
            throw new Exception("Cannot unmarshal type string");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            var value = (string)untypedValue;
            if (value.Length >= 1 && value.Length <= 2000)
            {
                serializer.Serialize(writer, value);
                return;
            }
            throw new Exception("Cannot marshal type string");
        }

        public static readonly IndecentMinMaxLengthCheckConverter Singleton = new IndecentMinMaxLengthCheckConverter();
    }

    internal class RouteStatusConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(RouteStatus) || t == typeof(RouteStatus?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "inRange":
                    return RouteStatus.InRange;
                case "outOfRange":
                    return RouteStatus.OutOfRange;
            }
            throw new Exception("Cannot unmarshal type RouteStatus");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (RouteStatus)untypedValue;
            switch (value)
            {
                case RouteStatus.InRange:
                    serializer.Serialize(writer, "inRange");
                    return;
                case RouteStatus.OutOfRange:
                    serializer.Serialize(writer, "outOfRange");
                    return;
            }
            throw new Exception("Cannot marshal type RouteStatus");
        }

        public static readonly RouteStatusConverter Singleton = new RouteStatusConverter();
    }

    internal class AcknowledgementStateConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(AcknowledgementState) || t == typeof(AcknowledgementState?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "acknowledged":
                    return AcknowledgementState.Acknowledged;
                case "deferred":
                    return AcknowledgementState.Deferred;
                case "notAcknowledged":
                    return AcknowledgementState.NotAcknowledged;
                case "recordedForAction":
                    return AcknowledgementState.RecordedForAction;
            }
            throw new Exception("Cannot unmarshal type AcknowledgementState");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (AcknowledgementState)untypedValue;
            switch (value)
            {
                case AcknowledgementState.Acknowledged:
                    serializer.Serialize(writer, "acknowledged");
                    return;
                case AcknowledgementState.Deferred:
                    serializer.Serialize(writer, "deferred");
                    return;
                case AcknowledgementState.NotAcknowledged:
                    serializer.Serialize(writer, "notAcknowledged");
                    return;
                case AcknowledgementState.RecordedForAction:
                    serializer.Serialize(writer, "recordedForAction");
                    return;
            }
            throw new Exception("Cannot marshal type AcknowledgementState");
        }

        public static readonly AcknowledgementStateConverter Singleton = new AcknowledgementStateConverter();
    }

    internal class TransportRouteFaultStatusConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(TransportRouteFaultStatus) || t == typeof(TransportRouteFaultStatus?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "active":
                    return TransportRouteFaultStatus.Active;
                case "cleared":
                    return TransportRouteFaultStatus.Cleared;
            }
            throw new Exception("Cannot unmarshal type TransportRouteFaultStatus");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (TransportRouteFaultStatus)untypedValue;
            switch (value)
            {
                case TransportRouteFaultStatus.Active:
                    serializer.Serialize(writer, "active");
                    return;
                case TransportRouteFaultStatus.Cleared:
                    serializer.Serialize(writer, "cleared");
                    return;
            }
            throw new Exception("Cannot marshal type TransportRouteFaultStatus");
        }

        public static readonly TransportRouteFaultStatusConverter Singleton = new TransportRouteFaultStatusConverter();
    }
}

namespace Topas.Utmc.TransportRoute
{
    using System;
    using Newtonsoft.Json;
    using Newtonsoft.Json.Linq;

    /// <summary>
    /// Transport route command payloads.
    /// </summary>
    /// <remarks>
    /// Tagged union over <c>kind</c>. Deserialise as this interface to
    /// have the converter select the variant; the variant classes are
    /// generated by quicktype and adopt the interface via partial
    /// declarations below.
    /// </remarks>
    [JsonConverter(typeof(TransportRouteCommandConverter))]
    public interface ITransportRouteCommand { }

    public partial class RequestTransportRouteStatusCommand : ITransportRouteCommand { }

    internal class TransportRouteCommandConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(ITransportRouteCommand);

        // Read-only. Newtonsoft inherits a [JsonConverter] declared on an
        // interface onto its implementing classes, so writing through this
        // converter re-enters it: `Self referencing loop detected`. The
        // variant already carries its own tag, so default serialisation is
        // exactly right and CanWrite=false selects it.
        public override bool CanWrite => false;

        public override void WriteJson(
            JsonWriter writer, object value, JsonSerializer serializer) =>
            throw new NotSupportedException();

        public override object ReadJson(
            JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            JObject obj = JObject.Load(reader);
            string tag = (string)obj["kind"];
            switch (tag)
            {
                case "requestStatus": return Read<RequestTransportRouteStatusCommand>(obj, serializer);
                default:
                    throw new JsonSerializationException(
                        $"unknown kind '{tag}' for TransportRouteCommand");
            }
        }

        // Populate, not ToObject: the same inherited-attribute problem
        // applies on read, and constructing the variant through the
        // serializer would recurse into this converter until the stack
        // overflowed. Filling an existing instance does not.
        private static T Read<T>(JObject obj, JsonSerializer serializer)
            where T : new()
        {
            T instance = new T();
            using (var reader = obj.CreateReader())
            {
                serializer.Populate(reader, instance);
            }
            return instance;
        }
    }
}

namespace Topas.Utmc.TransportRoute
{
    using Newtonsoft.Json;

    /// <summary>
    /// Serializer settings for this namespace's generated types.
    /// </summary>
    /// <remarks>
    /// Always serialise and deserialise through these settings.
    /// They register the converters that map enum members to their
    /// wire values; without them a discriminator round-trips as
    /// <c>"Calibrate"</c> rather than <c>"calibrate"</c> and the
    /// payload no longer validates against the schema.
    /// </remarks>
    public static class UtmcJson
    {
        public static JsonSerializerSettings Settings => Converter.Settings;

        public static T Deserialize<T>(string json) =>
            JsonConvert.DeserializeObject<T>(json, Converter.Settings);

        public static string Serialize(object value) =>
            JsonConvert.SerializeObject(value, Converter.Settings);
    }
}
