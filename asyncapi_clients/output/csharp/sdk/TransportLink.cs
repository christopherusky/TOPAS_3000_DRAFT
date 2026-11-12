/*
 * File:        output/csharp/sdk/TransportLink.cs
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
namespace Topas.Utmc.TransportLink
{
    using System;
    using System.Collections.Generic;

    using System.Globalization;
    using Newtonsoft.Json;
    using Newtonsoft.Json.Converters;

    public partial class AllTypes
    {
        [JsonProperty("LinkStatusType", NullValueHandling = NullValueHandling.Ignore)]
        public LinkStatusType LinkStatusType { get; set; }

        [JsonProperty("RecalibrateTransportLinkCommand", NullValueHandling = NullValueHandling.Ignore)]
        public RecalibrateTransportLinkCommand RecalibrateTransportLinkCommand { get; set; }

        [JsonProperty("RequestTransportLinkStatusCommand", NullValueHandling = NullValueHandling.Ignore)]
        public RequestTransportLinkStatusCommand RequestTransportLinkStatusCommand { get; set; }

        [JsonProperty("SetStocCommand", NullValueHandling = NullValueHandling.Ignore)]
        public SetStocCommand SetStocCommand { get; set; }

        [JsonProperty("SyncCategoryEnum", NullValueHandling = NullValueHandling.Ignore)]
        public SyncCategoryEnum? SyncCategoryEnum { get; set; }

        [JsonProperty("SyncComplete", NullValueHandling = NullValueHandling.Ignore)]
        public SyncComplete SyncComplete { get; set; }

        [JsonProperty("SyncRequest", NullValueHandling = NullValueHandling.Ignore)]
        public SyncRequest SyncRequest { get; set; }

        [JsonProperty("SyncStatus", NullValueHandling = NullValueHandling.Ignore)]
        public SyncStatusEnum? SyncStatus { get; set; }

        [JsonProperty("TransportLinkANPRConfiguration", NullValueHandling = NullValueHandling.Ignore)]
        public TransportLinkAnprConfiguration TransportLinkAnprConfiguration { get; set; }

        [JsonProperty("TransportLinkANPRDynamic", NullValueHandling = NullValueHandling.Ignore)]
        public TransportLinkAnprDynamic TransportLinkAnprDynamic { get; set; }

        [JsonProperty("TransportLinkCommandResponse", NullValueHandling = NullValueHandling.Ignore)]
        public TransportLinkCommandResponse TransportLinkCommandResponse { get; set; }

        [JsonProperty("TransportLinkDefinition", NullValueHandling = NullValueHandling.Ignore)]
        public TransportLinkDefinition TransportLinkDefinition { get; set; }

        [JsonProperty("TransportLinkFault", NullValueHandling = NullValueHandling.Ignore)]
        public TransportLinkFault TransportLinkFault { get; set; }

        [JsonProperty("TransportLinkGPSConfiguration", NullValueHandling = NullValueHandling.Ignore)]
        public TransportLinkGpsConfiguration TransportLinkGpsConfiguration { get; set; }

        [JsonProperty("TransportLinkGPSDynamic", NullValueHandling = NullValueHandling.Ignore)]
        public TransportLinkGpsDynamic TransportLinkGpsDynamic { get; set; }

        [JsonProperty("TransportLinkHistoryEntry", NullValueHandling = NullValueHandling.Ignore)]
        public TransportLinkHistoryEntry TransportLinkHistoryEntry { get; set; }

        [JsonProperty("TransportLinkSCOOTConfiguration", NullValueHandling = NullValueHandling.Ignore)]
        public TransportLinkScootConfiguration TransportLinkScootConfiguration { get; set; }

        [JsonProperty("TransportLinkSCOOTDynamic", NullValueHandling = NullValueHandling.Ignore)]
        public TransportLinkScootDynamic TransportLinkScootDynamic { get; set; }

        [JsonProperty("TransportLinkType", NullValueHandling = NullValueHandling.Ignore)]
        public TransportLinkType TransportLinkType { get; set; }
    }

    /// <summary>
    /// Catalogue entry for one operational status of a transport link.
    /// </summary>
    public partial class LinkStatusType
    {
        /// <summary>
        /// Human-readable description of the link status (v1: 'normal', 'suspect',
        /// 'noVehiclesInPeriod').
        /// </summary>
        [JsonProperty("description")]
        public string Description { get; set; }

        /// <summary>
        /// Vendor or local extensions. Open-content object.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Discriminator for the tagged union. The fixed value "linkStatus" selects the
        /// LinkStatusType payload; it is not free-text command content.
        /// </summary>
        [JsonProperty("kind")]
        public LinkStatusTypeKind Kind { get; set; }

        /// <summary>
        /// Additional notes about this status value.
        /// </summary>
        [JsonProperty("notes", NullValueHandling = NullValueHandling.Ignore)]
        public string Notes { get; set; }

        /// <summary>
        /// Identifier of this link status type.
        /// </summary>
        [JsonProperty("typeId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string TypeId { get; set; }
    }

    /// <summary>
    /// Trigger a recalibration on a transport link.
    /// </summary>
    public partial class RecalibrateTransportLinkCommand
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
        /// Discriminator for the tagged union. The fixed value "recalibrate" selects the
        /// RecalibrateTransportLinkCommand payload; it is not free-text command content.
        /// </summary>
        [JsonProperty("kind")]
        public RecalibrateTransportLinkCommandKind Kind { get; set; }

        /// <summary>
        /// When the command was issued.
        /// </summary>
        [JsonProperty("requestTime")]
        public DateTimeOffset RequestTime { get; set; }

        /// <summary>
        /// Identifier of the transport link receiving the command.
        /// </summary>
        [JsonProperty("transportLinkId")]
        [JsonConverter(typeof(FluffyMinMaxLengthCheckConverter))]
        public string TransportLinkId { get; set; }
    }

    /// <summary>
    /// Request the current status of a transport link.
    /// </summary>
    public partial class RequestTransportLinkStatusCommand
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
        /// RequestTransportLinkStatusCommand payload; it is not free-text command content.
        /// </summary>
        [JsonProperty("kind")]
        public RequestTransportLinkStatusCommandKind Kind { get; set; }

        /// <summary>
        /// When the command was issued.
        /// </summary>
        [JsonProperty("requestTime")]
        public DateTimeOffset RequestTime { get; set; }

        /// <summary>
        /// Identifier of the transport link receiving the command.
        /// </summary>
        [JsonProperty("transportLinkId")]
        [JsonConverter(typeof(FluffyMinMaxLengthCheckConverter))]
        public string TransportLinkId { get; set; }
    }

    /// <summary>
    /// Set the SCOOT STOC threshold value on a transport link.
    /// </summary>
    public partial class SetStocCommand
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
        /// Discriminator for the tagged union. The fixed value "setStoc" selects the SetStocCommand
        /// payload; it is not free-text command content.
        /// </summary>
        [JsonProperty("kind")]
        public SetStocCommandKind Kind { get; set; }

        /// <summary>
        /// When the command was issued.
        /// </summary>
        [JsonProperty("requestTime")]
        public DateTimeOffset RequestTime { get; set; }

        /// <summary>
        /// STOC value to set.
        /// </summary>
        [JsonProperty("stocValue")]
        public string StocValue { get; set; }

        /// <summary>
        /// Identifier of the transport link receiving the command.
        /// </summary>
        [JsonProperty("transportLinkId")]
        [JsonConverter(typeof(FluffyMinMaxLengthCheckConverter))]
        public string TransportLinkId { get; set; }
    }

    /// <summary>
    /// Terminal message ending a transport link sync batch. Sent to the requester's reply node
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
        [JsonConverter(typeof(TentacledMinMaxLengthCheckConverter))]
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
        [JsonConverter(typeof(StickyMinMaxLengthCheckConverter))]
        public string SyncId { get; set; }
    }

    /// <summary>
    /// Request the current state of transport link objects. The Data Publisher answers to the
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
        [JsonConverter(typeof(IndigoMinMaxLengthCheckConverter))]
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
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string TypeId { get; set; }
    }

    /// <summary>
    /// Operational configuration for an ANPR-monitored transport link.
    /// </summary>
    public partial class TransportLinkAnprConfiguration
    {
        /// <summary>
        /// Maximum flow in vehicles per hour.
        /// </summary>
        [JsonProperty("capacityVehiclesPerHour", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double? CapacityVehiclesPerHour { get; set; }

        /// <summary>
        /// When this configuration record became effective.
        /// </summary>
        [JsonProperty("configurationDate")]
        public DateTimeOffset ConfigurationDate { get; set; }

        /// <summary>
        /// Vendor or local extensions. Open-content object.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Distance from stop line to stop line in metres.
        /// </summary>
        [JsonProperty("fullLengthMetres", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double? FullLengthMetres { get; set; }

        /// <summary>
        /// Discriminator for the tagged union. The fixed value "anpr" selects the
        /// TransportLinkANPRConfiguration payload; it is not free-text command content.
        /// </summary>
        [JsonProperty("kind")]
        public TransportLinkAnprConfigurationKind Kind { get; set; }

        /// <summary>
        /// Distance from detector to stop line in metres.
        /// </summary>
        [JsonProperty("lengthMetres", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double? LengthMetres { get; set; }

        /// <summary>
        /// Maximum speed associated with the link in kilometres per hour.
        /// </summary>
        [JsonProperty("maximumSpeedKph", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double? MaximumSpeedKph { get; set; }

        /// <summary>
        /// Number of lanes on the link.
        /// </summary>
        [JsonProperty("numberOfLanes", NullValueHandling = NullValueHandling.Ignore)]
        public long? NumberOfLanes { get; set; }

        /// <summary>
        /// Identifier of the transport link this configuration applies to.
        /// </summary>
        [JsonProperty("transportLinkId")]
        [JsonConverter(typeof(FluffyMinMaxLengthCheckConverter))]
        public string TransportLinkId { get; set; }
    }

    /// <summary>
    /// ANPR-reported live measurement for a transport link.
    /// </summary>
    public partial class TransportLinkAnprDynamic
    {
        /// <summary>
        /// Estimated average speed in kilometres per hour.
        /// </summary>
        [JsonProperty("averageSpeedKph", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double? AverageSpeedKph { get; set; }

        /// <summary>
        /// Vendor or local extensions. Open-content object.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Discriminator for the tagged union. The fixed value "anpr" selects the
        /// TransportLinkANPRDynamic payload; it is not free-text command content.
        /// </summary>
        [JsonProperty("kind")]
        public TransportLinkAnprConfigurationKind Kind { get; set; }

        /// <summary>
        /// Current link journey time in seconds.
        /// </summary>
        [JsonProperty("linkTravelTimeSeconds", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double? LinkTravelTimeSeconds { get; set; }

        /// <summary>
        /// Number of successful plate reads at the start of the link.
        /// </summary>
        [JsonProperty("numberPlatesIn", NullValueHandling = NullValueHandling.Ignore)]
        public long? NumberPlatesIn { get; set; }

        /// <summary>
        /// Count of number-plate matches in the measurement period.
        /// </summary>
        [JsonProperty("numberPlatesMatches", NullValueHandling = NullValueHandling.Ignore)]
        public long? NumberPlatesMatches { get; set; }

        /// <summary>
        /// Number of successful matches actually used in the travel-time calculation.
        /// </summary>
        [JsonProperty("numberPlatesMatchesUsed", NullValueHandling = NullValueHandling.Ignore)]
        public long? NumberPlatesMatchesUsed { get; set; }

        /// <summary>
        /// Number of successful plate reads at the end of the link.
        /// </summary>
        [JsonProperty("numberPlatesOut", NullValueHandling = NullValueHandling.Ignore)]
        public long? NumberPlatesOut { get; set; }

        /// <summary>
        /// Optional quality statement for this measurement.
        /// </summary>
        [JsonProperty("quality", NullValueHandling = NullValueHandling.Ignore)]
        public Quality Quality { get; set; }

        /// <summary>
        /// Time the measurement was taken.
        /// </summary>
        [JsonProperty("timestamp")]
        public DateTimeOffset Timestamp { get; set; }

        /// <summary>
        /// Identifier of the transport link this reading is about.
        /// </summary>
        [JsonProperty("transportLinkId")]
        [JsonConverter(typeof(FluffyMinMaxLengthCheckConverter))]
        public string TransportLinkId { get; set; }
    }

    /// <summary>
    /// Optional quality statement for this measurement.
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
        [JsonConverter(typeof(FluffyMinMaxLengthCheckConverter))]
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
        [JsonConverter(typeof(TentacledMinMaxLengthCheckConverter))]
        public string SourceName { get; set; }

        /// <summary>
        /// Free-text description of the source kind.
        /// </summary>
        [JsonProperty("sourceType", NullValueHandling = NullValueHandling.Ignore)]
        public string SourceType { get; set; }
    }

    /// <summary>
    /// Transport link command response payloads.
    /// </summary>
    public partial class TransportLinkCommandResponse
    {
        /// <summary>
        /// Machine-readable response code for programmatic handling.
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
        /// Human-readable response message with details.
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
        public TransportLinkCommandResponseStatus Status { get; set; }

        /// <summary>
        /// Identifier of the transport link the command targeted.
        /// </summary>
        [JsonProperty("transportLinkId")]
        [JsonConverter(typeof(FluffyMinMaxLengthCheckConverter))]
        public string TransportLinkId { get; set; }
    }

    /// <summary>
    /// Transport link equipment details.
    /// </summary>
    public partial class TransportLinkDefinition
    {
        /// <summary>
        /// When this definition record was created.
        /// </summary>
        [JsonProperty("creationDate")]
        public DateTimeOffset CreationDate { get; set; }

        /// <summary>
        /// End of the transport link as a WGS84 point. Optional here although the v1 XSD requires
        /// it, matching transport_route.
        /// </summary>
        [JsonProperty("endPoint", NullValueHandling = NullValueHandling.Ignore)]
        public EndPoint EndPoint { get; set; }

        /// <summary>
        /// Vendor or local extensions. Open-content object.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Start of the transport link as a WGS84 point. Optional.
        /// </summary>
        [JsonProperty("location", NullValueHandling = NullValueHandling.Ignore)]
        public EndPoint Location { get; set; }

        /// <summary>
        /// Longer description (context, commissioning notes, etc.).
        /// </summary>
        [JsonProperty("longDescription", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(IndecentMinMaxLengthCheckConverter))]
        public string LongDescription { get; set; }

        /// <summary>
        /// Identifier of this object on the originating (upstream) system. Enables cross-system
        /// traceability.
        /// </summary>
        [JsonProperty("originatingObjectId", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(FluffyMinMaxLengthCheckConverter))]
        public string OriginatingObjectId { get; set; }

        /// <summary>
        /// Identifier of the upstream system that originally created this object (comparable to the
        /// UML DataSourceTypeId).
        /// </summary>
        [JsonProperty("originatingParticipantId", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(FluffyMinMaxLengthCheckConverter))]
        public string OriginatingParticipantId { get; set; }

        /// <summary>
        /// System that owns this transport link definition. Often implicit from context.
        /// </summary>
        [JsonProperty("participantId", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(FluffyMinMaxLengthCheckConverter))]
        public string ParticipantId { get; set; }

        /// <summary>
        /// Short human-readable name (e.g. 'A1 northbound, Jct 5 -> Jct 6').
        /// </summary>
        [JsonProperty("shortDescription", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(IndigoMinMaxLengthCheckConverter))]
        public string ShortDescription { get; set; }

        /// <summary>
        /// Unique identifier of this transport link within its owning system.
        /// </summary>
        [JsonProperty("transportLinkId")]
        [JsonConverter(typeof(FluffyMinMaxLengthCheckConverter))]
        public string TransportLinkId { get; set; }

        /// <summary>
        /// Unique identifier for a transport link type (SCOOT, ANPR, SCOOT Entry, ...).
        /// </summary>
        [JsonProperty("typeId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string TypeId { get; set; }
    }

    /// <summary>
    /// End of the transport link as a WGS84 point. Optional here although the v1 XSD requires
    /// it, matching transport_route.
    ///
    /// A geographic point: a coordinate plus optional altitude.
    ///
    /// Start of the transport link as a WGS84 point. Optional.
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
    /// Transport link fault reports.
    /// </summary>
    public partial class TransportLinkFault
    {
        /// <summary>
        /// When the fault was acknowledged.
        /// </summary>
        [JsonProperty("acknowledgementDate", NullValueHandling = NullValueHandling.Ignore)]
        public DateTimeOffset? AcknowledgementDate { get; set; }

        /// <summary>
        /// Current acknowledgement state.
        /// </summary>
        [JsonProperty("acknowledgementState", NullValueHandling = NullValueHandling.Ignore)]
        public AcknowledgementState? AcknowledgementState { get; set; }

        /// <summary>
        /// When the fault was cleared. If present, the fault is historical.
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
        /// Free-text description of the fault.
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
        /// Vendor or local extensions. Open-content object.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Unique identifier of this fault report within the owning system.
        /// </summary>
        [JsonProperty("faultId")]
        public long FaultId { get; set; }

        /// <summary>
        /// Reference to a FaultTypeDefinition entry in the shared fault catalogue.
        /// </summary>
        [JsonProperty("faultTypeId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string FaultTypeId { get; set; }

        /// <summary>
        /// When the fault record was last modified.
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
        /// Fault severity (higher = more severe).
        /// </summary>
        [JsonProperty("severity", NullValueHandling = NullValueHandling.Ignore)]
        public long? Severity { get; set; }

        /// <summary>
        /// Whether this fault is still active or has been cleared.
        /// </summary>
        [JsonProperty("status", NullValueHandling = NullValueHandling.Ignore)]
        public TransportLinkFaultStatus? Status { get; set; }

        /// <summary>
        /// Reference to a SubsystemDefinition entry: the subsystem the fault arose in.
        /// </summary>
        [JsonProperty("subsystemTypeId", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string SubsystemTypeId { get; set; }

        /// <summary>
        /// Fault identifier allocated by the supplier or configuration manager for maintenance
        /// purposes, if required.
        /// </summary>
        [JsonProperty("supplierFaultNumber", NullValueHandling = NullValueHandling.Ignore)]
        public string SupplierFaultNumber { get; set; }

        /// <summary>
        /// Identifier of the transport link this fault is about.
        /// </summary>
        [JsonProperty("transportLinkId")]
        [JsonConverter(typeof(FluffyMinMaxLengthCheckConverter))]
        public string TransportLinkId { get; set; }
    }

    /// <summary>
    /// Operational configuration for a GPS-monitored transport link.
    /// </summary>
    public partial class TransportLinkGpsConfiguration
    {
        /// <summary>
        /// Maximum flow in vehicles per hour.
        /// </summary>
        [JsonProperty("capacityVehiclesPerHour", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double? CapacityVehiclesPerHour { get; set; }

        /// <summary>
        /// When this configuration record became effective.
        /// </summary>
        [JsonProperty("configurationDate")]
        public DateTimeOffset ConfigurationDate { get; set; }

        /// <summary>
        /// Vendor or local extensions. Open-content object.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Distance from stop line to stop line in metres.
        /// </summary>
        [JsonProperty("fullLengthMetres", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double? FullLengthMetres { get; set; }

        /// <summary>
        /// Discriminator for the tagged union. The fixed value "gps" selects the
        /// TransportLinkGPSConfiguration payload; it is not free-text command content.
        /// </summary>
        [JsonProperty("kind")]
        public TransportLinkGpsConfigurationKind Kind { get; set; }

        /// <summary>
        /// Distance from detector to stop line in metres.
        /// </summary>
        [JsonProperty("lengthMetres", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double? LengthMetres { get; set; }

        /// <summary>
        /// Maximum speed associated with the link in kilometres per hour.
        /// </summary>
        [JsonProperty("maximumSpeedKph", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double? MaximumSpeedKph { get; set; }

        /// <summary>
        /// Number of lanes on the link.
        /// </summary>
        [JsonProperty("numberOfLanes", NullValueHandling = NullValueHandling.Ignore)]
        public long? NumberOfLanes { get; set; }

        /// <summary>
        /// Identifier of the transport link this configuration applies to.
        /// </summary>
        [JsonProperty("transportLinkId")]
        [JsonConverter(typeof(FluffyMinMaxLengthCheckConverter))]
        public string TransportLinkId { get; set; }
    }

    /// <summary>
    /// GPS-probe live measurement for a transport link.
    ///
    /// Field set drawn from the XSD TransportLinkGpsDynamicGroup (Freeflow
    /// Extension): CurrentJourneyTime, the three travel-time statistics
    /// (Mean/Median/VarianceTravelTime), NumberOfVehicles and the reserved-
    /// lane flow count. Shared base fields (transport_link_id, timestamp,
    /// quality, average_speed_kph, link_travel_time_seconds) are inherited
    /// from _TransportLinkDynamicBase.
    /// </summary>
    public partial class TransportLinkGpsDynamic
    {
        /// <summary>
        /// Estimated average speed in kilometres per hour.
        /// </summary>
        [JsonProperty("averageSpeedKph", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double? AverageSpeedKph { get; set; }

        /// <summary>
        /// Current journey time in seconds.
        /// </summary>
        [JsonProperty("currentJourneyTimeSeconds", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double? CurrentJourneyTimeSeconds { get; set; }

        /// <summary>
        /// Vendor or local extensions. Open-content object.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Discriminator for the tagged union. The fixed value "gps" selects the
        /// TransportLinkGPSDynamic payload; it is not free-text command content.
        /// </summary>
        [JsonProperty("kind")]
        public TransportLinkGpsConfigurationKind Kind { get; set; }

        /// <summary>
        /// Current link journey time in seconds.
        /// </summary>
        [JsonProperty("linkTravelTimeSeconds", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double? LinkTravelTimeSeconds { get; set; }

        /// <summary>
        /// Mean travel time along the link in the measured interval, in seconds.
        /// </summary>
        [JsonProperty("meanTravelTimeSeconds", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double? MeanTravelTimeSeconds { get; set; }

        /// <summary>
        /// Median travel time along the link in the measured interval, in seconds.
        /// </summary>
        [JsonProperty("medianTravelTimeSeconds", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double? MedianTravelTimeSeconds { get; set; }

        /// <summary>
        /// Number of vehicles in the measured interval.
        /// </summary>
        [JsonProperty("numberOfVehicles", NullValueHandling = NullValueHandling.Ignore)]
        public long? NumberOfVehicles { get; set; }

        /// <summary>
        /// Optional quality statement for this measurement.
        /// </summary>
        [JsonProperty("quality", NullValueHandling = NullValueHandling.Ignore)]
        public Quality Quality { get; set; }

        /// <summary>
        /// Flow in the reserved lane in the measured interval, in vehicles. A rate, so it may be
        /// fractional.
        /// </summary>
        [JsonProperty("reservedLaneFlowVehicles", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double? ReservedLaneFlowVehicles { get; set; }

        /// <summary>
        /// Time the measurement was taken.
        /// </summary>
        [JsonProperty("timestamp")]
        public DateTimeOffset Timestamp { get; set; }

        /// <summary>
        /// Identifier of the transport link this reading is about.
        /// </summary>
        [JsonProperty("transportLinkId")]
        [JsonConverter(typeof(FluffyMinMaxLengthCheckConverter))]
        public string TransportLinkId { get; set; }

        /// <summary>
        /// Variance of travel times along the link in the measured interval. The unit is seconds
        /// squared (s²), as for any variance of times; the field name keeps 'Seconds' for
        /// compatibility with v1.
        /// </summary>
        [JsonProperty("varianceTravelTimeSeconds", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double? VarianceTravelTimeSeconds { get; set; }
    }

    /// <summary>
    /// Transport link history entries.
    /// </summary>
    public partial class TransportLinkHistoryEntry
    {
        /// <summary>
        /// Category or type of the history event (e.g. 'reconfiguration', 'capacity_update').
        /// </summary>
        [JsonProperty("eventType", NullValueHandling = NullValueHandling.Ignore)]
        public string EventType { get; set; }

        /// <summary>
        /// Vendor or local extensions. Open-content object.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// When the log entry was recorded.
        /// </summary>
        [JsonProperty("logDate")]
        public DateTimeOffset LogDate { get; set; }

        /// <summary>
        /// Equipment log data (free-form text).
        /// </summary>
        [JsonProperty("logEntry")]
        public string LogEntry { get; set; }

        /// <summary>
        /// Unique identifier of this log entry within the owning system.
        /// </summary>
        [JsonProperty("logId")]
        public long LogId { get; set; }

        /// <summary>
        /// User who created the log entry.
        /// </summary>
        [JsonProperty("logUser", NullValueHandling = NullValueHandling.Ignore)]
        public string LogUser { get; set; }

        /// <summary>
        /// Identifier of the transport link this log entry is about.
        /// </summary>
        [JsonProperty("transportLinkId")]
        [JsonConverter(typeof(FluffyMinMaxLengthCheckConverter))]
        public string TransportLinkId { get; set; }
    }

    /// <summary>
    /// Operational configuration for a SCOOT-modelled transport link.
    /// </summary>
    public partial class TransportLinkScootConfiguration
    {
        /// <summary>
        /// Maximum flow in vehicles per hour.
        /// </summary>
        [JsonProperty("capacityVehiclesPerHour", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double? CapacityVehiclesPerHour { get; set; }

        /// <summary>
        /// When this configuration record became effective.
        /// </summary>
        [JsonProperty("configurationDate")]
        public DateTimeOffset ConfigurationDate { get; set; }

        /// <summary>
        /// Vendor or local extensions. Open-content object.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Distance from stop line to stop line in metres.
        /// </summary>
        [JsonProperty("fullLengthMetres", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double? FullLengthMetres { get; set; }

        /// <summary>
        /// Discriminator for the tagged union. The fixed value "scoot" selects the
        /// TransportLinkSCOOTConfiguration payload; it is not free-text command content.
        /// </summary>
        [JsonProperty("kind")]
        public TransportLinkScootConfigurationKind Kind { get; set; }

        /// <summary>
        /// Distance from detector to stop line in metres.
        /// </summary>
        [JsonProperty("lengthMetres", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double? LengthMetres { get; set; }

        /// <summary>
        /// Maximum speed associated with the link in kilometres per hour.
        /// </summary>
        [JsonProperty("maximumSpeedKph", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double? MaximumSpeedKph { get; set; }

        /// <summary>
        /// Number of lanes on the link.
        /// </summary>
        [JsonProperty("numberOfLanes", NullValueHandling = NullValueHandling.Ignore)]
        public long? NumberOfLanes { get; set; }

        /// <summary>
        /// Identifier of the transport link this configuration applies to.
        /// </summary>
        [JsonProperty("transportLinkId")]
        [JsonConverter(typeof(FluffyMinMaxLengthCheckConverter))]
        public string TransportLinkId { get; set; }
    }

    /// <summary>
    /// SCOOT-reported live measurement for a transport link.
    /// </summary>
    public partial class TransportLinkScootDynamic
    {
        /// <summary>
        /// Estimated average speed in kilometres per hour.
        /// </summary>
        [JsonProperty("averageSpeedKph", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double? AverageSpeedKph { get; set; }

        /// <summary>
        /// Current percentage congestion as reported by SCOOT.
        /// </summary>
        [JsonProperty("congestionPercent", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(IndigoMinMaxValueCheckConverter))]
        public double? CongestionPercent { get; set; }

        /// <summary>
        /// Current flow in vehicles over the SCOOT measurement interval. A rate, so it may be
        /// fractional (e.g. a smoothed or averaged flow).
        /// </summary>
        [JsonProperty("currentFlowVehicles", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double? CurrentFlowVehicles { get; set; }

        /// <summary>
        /// Vendor or local extensions. Open-content object.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Discriminator for the tagged union. The fixed value "scoot" selects the
        /// TransportLinkSCOOTDynamic payload; it is not free-text command content.
        /// </summary>
        [JsonProperty("kind")]
        public TransportLinkScootConfigurationKind Kind { get; set; }

        /// <summary>
        /// Current link journey time in seconds.
        /// </summary>
        [JsonProperty("linkTravelTimeSeconds", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double? LinkTravelTimeSeconds { get; set; }

        /// <summary>
        /// Average link occupancy percentage across all detectors on the link.
        /// </summary>
        [JsonProperty("occupancyPercent", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(IndigoMinMaxValueCheckConverter))]
        public double? OccupancyPercent { get; set; }

        /// <summary>
        /// Optional quality statement for this measurement.
        /// </summary>
        [JsonProperty("quality", NullValueHandling = NullValueHandling.Ignore)]
        public Quality Quality { get; set; }

        /// <summary>
        /// Time the measurement was taken.
        /// </summary>
        [JsonProperty("timestamp")]
        public DateTimeOffset Timestamp { get; set; }

        /// <summary>
        /// Identifier of the transport link this reading is about.
        /// </summary>
        [JsonProperty("transportLinkId")]
        [JsonConverter(typeof(FluffyMinMaxLengthCheckConverter))]
        public string TransportLinkId { get; set; }
    }

    /// <summary>
    /// Catalogue entry for one kind of transport link.
    /// </summary>
    public partial class TransportLinkType
    {
        /// <summary>
        /// Human-readable description of the transport link kind.
        /// </summary>
        [JsonProperty("description")]
        public string Description { get; set; }

        /// <summary>
        /// Vendor or local extensions. Open-content object.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Discriminator for the tagged union. The fixed value "transportLink" selects the
        /// TransportLinkType payload; it is not free-text command content.
        /// </summary>
        [JsonProperty("kind")]
        public TransportLinkTypeKind Kind { get; set; }

        /// <summary>
        /// Additional notes, e.g. deployment details.
        /// </summary>
        [JsonProperty("notes", NullValueHandling = NullValueHandling.Ignore)]
        public string Notes { get; set; }

        /// <summary>
        /// URL to an external specification document for this transport link type.
        /// </summary>
        [JsonProperty("specificationLink", NullValueHandling = NullValueHandling.Ignore)]
        public Uri SpecificationLink { get; set; }

        /// <summary>
        /// Identifier of this transport link type.
        /// </summary>
        [JsonProperty("typeId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string TypeId { get; set; }
    }

    public enum LinkStatusTypeKind { LinkStatus };

    public enum RecalibrateTransportLinkCommandKind { Recalibrate };

    public enum RequestTransportLinkStatusCommandKind { RequestStatus };

    public enum SetStocCommandKind { SetStoc };

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

    public enum TransportLinkAnprConfigurationKind { Anpr };

    /// <summary>
    /// Outcome of the command execution.
    ///
    /// Outcome of a command execution.
    ///
    /// Every value except ``PENDING`` is terminal. Consumers SHOULD test for
    /// "not pending" rather than enumerating the terminal values, so that a
    /// future profile adding one does not strand them.
    /// </summary>
    public enum TransportLinkCommandResponseStatus { Failed, Invalid, NotSupported, Pending, Success, TimedOut };

    /// <summary>
    /// Current acknowledgement state.
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
    /// Whether this fault is still active or has been cleared.
    ///
    /// Whether a fault is still active or has been cleared.
    ///
    /// This is the only fault lifecycle the payload carries. Record archival
    /// — distinguishing a live record from a retired one — is not modelled
    /// here and is a matter for the publishing system.
    /// </summary>
    public enum TransportLinkFaultStatus { Active, Cleared };

    public enum TransportLinkGpsConfigurationKind { Gps };

    public enum TransportLinkScootConfigurationKind { Scoot };

    public enum TransportLinkTypeKind { TransportLink };

    internal static class Converter
    {
        public static readonly JsonSerializerSettings Settings = new JsonSerializerSettings
        {
            MetadataPropertyHandling = MetadataPropertyHandling.Ignore,
            DateParseHandling = DateParseHandling.None,
            Converters =
            {
                LinkStatusTypeKindConverter.Singleton,
                RecalibrateTransportLinkCommandKindConverter.Singleton,
                RequestTransportLinkStatusCommandKindConverter.Singleton,
                SetStocCommandKindConverter.Singleton,
                SyncCategoryEnumConverter.Singleton,
                SyncStatusEnumConverter.Singleton,
                TransportLinkAnprConfigurationKindConverter.Singleton,
                TransportLinkCommandResponseStatusConverter.Singleton,
                AcknowledgementStateConverter.Singleton,
                TransportLinkFaultStatusConverter.Singleton,
                TransportLinkGpsConfigurationKindConverter.Singleton,
                TransportLinkScootConfigurationKindConverter.Singleton,
                TransportLinkTypeKindConverter.Singleton,
                new IsoDateTimeConverter { DateTimeStyles = DateTimeStyles.AssumeUniversal }
            },
        };
    }

    internal class LinkStatusTypeKindConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(LinkStatusTypeKind) || t == typeof(LinkStatusTypeKind?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            if (value == "linkStatus")
            {
                return LinkStatusTypeKind.LinkStatus;
            }
            throw new Exception("Cannot unmarshal type LinkStatusTypeKind");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (LinkStatusTypeKind)untypedValue;
            if (value == LinkStatusTypeKind.LinkStatus)
            {
                serializer.Serialize(writer, "linkStatus");
                return;
            }
            throw new Exception("Cannot marshal type LinkStatusTypeKind");
        }

        public static readonly LinkStatusTypeKindConverter Singleton = new LinkStatusTypeKindConverter();
    }

    internal class PurpleMinMaxLengthCheckConverter : JsonConverter
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

        public static readonly PurpleMinMaxLengthCheckConverter Singleton = new PurpleMinMaxLengthCheckConverter();
    }

    internal class RecalibrateTransportLinkCommandKindConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(RecalibrateTransportLinkCommandKind) || t == typeof(RecalibrateTransportLinkCommandKind?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            if (value == "recalibrate")
            {
                return RecalibrateTransportLinkCommandKind.Recalibrate;
            }
            throw new Exception("Cannot unmarshal type RecalibrateTransportLinkCommandKind");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (RecalibrateTransportLinkCommandKind)untypedValue;
            if (value == RecalibrateTransportLinkCommandKind.Recalibrate)
            {
                serializer.Serialize(writer, "recalibrate");
                return;
            }
            throw new Exception("Cannot marshal type RecalibrateTransportLinkCommandKind");
        }

        public static readonly RecalibrateTransportLinkCommandKindConverter Singleton = new RecalibrateTransportLinkCommandKindConverter();
    }

    internal class FluffyMinMaxLengthCheckConverter : JsonConverter
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

        public static readonly FluffyMinMaxLengthCheckConverter Singleton = new FluffyMinMaxLengthCheckConverter();
    }

    internal class RequestTransportLinkStatusCommandKindConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(RequestTransportLinkStatusCommandKind) || t == typeof(RequestTransportLinkStatusCommandKind?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            if (value == "requestStatus")
            {
                return RequestTransportLinkStatusCommandKind.RequestStatus;
            }
            throw new Exception("Cannot unmarshal type RequestTransportLinkStatusCommandKind");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (RequestTransportLinkStatusCommandKind)untypedValue;
            if (value == RequestTransportLinkStatusCommandKind.RequestStatus)
            {
                serializer.Serialize(writer, "requestStatus");
                return;
            }
            throw new Exception("Cannot marshal type RequestTransportLinkStatusCommandKind");
        }

        public static readonly RequestTransportLinkStatusCommandKindConverter Singleton = new RequestTransportLinkStatusCommandKindConverter();
    }

    internal class SetStocCommandKindConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(SetStocCommandKind) || t == typeof(SetStocCommandKind?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            if (value == "setStoc")
            {
                return SetStocCommandKind.SetStoc;
            }
            throw new Exception("Cannot unmarshal type SetStocCommandKind");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (SetStocCommandKind)untypedValue;
            if (value == SetStocCommandKind.SetStoc)
            {
                serializer.Serialize(writer, "setStoc");
                return;
            }
            throw new Exception("Cannot marshal type SetStocCommandKind");
        }

        public static readonly SetStocCommandKindConverter Singleton = new SetStocCommandKindConverter();
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

    internal class TentacledMinMaxLengthCheckConverter : JsonConverter
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

        public static readonly TentacledMinMaxLengthCheckConverter Singleton = new TentacledMinMaxLengthCheckConverter();
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

    internal class StickyMinMaxLengthCheckConverter : JsonConverter
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

        public static readonly StickyMinMaxLengthCheckConverter Singleton = new StickyMinMaxLengthCheckConverter();
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
                var converter = FluffyMinMaxLengthCheckConverter.Singleton;
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
                var converter = FluffyMinMaxLengthCheckConverter.Singleton;
                converter.WriteJson(writer, arrayItem, serializer);
            }
            writer.WriteEndArray();
            return;
        }

        public static readonly DecodeArrayConverter Singleton = new DecodeArrayConverter();
    }

    internal class IndigoMinMaxLengthCheckConverter : JsonConverter
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

        public static readonly IndigoMinMaxLengthCheckConverter Singleton = new IndigoMinMaxLengthCheckConverter();
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

    internal class TransportLinkAnprConfigurationKindConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(TransportLinkAnprConfigurationKind) || t == typeof(TransportLinkAnprConfigurationKind?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            if (value == "anpr")
            {
                return TransportLinkAnprConfigurationKind.Anpr;
            }
            throw new Exception("Cannot unmarshal type TransportLinkAnprConfigurationKind");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (TransportLinkAnprConfigurationKind)untypedValue;
            if (value == TransportLinkAnprConfigurationKind.Anpr)
            {
                serializer.Serialize(writer, "anpr");
                return;
            }
            throw new Exception("Cannot marshal type TransportLinkAnprConfigurationKind");
        }

        public static readonly TransportLinkAnprConfigurationKindConverter Singleton = new TransportLinkAnprConfigurationKindConverter();
    }

    internal class TransportLinkCommandResponseStatusConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(TransportLinkCommandResponseStatus) || t == typeof(TransportLinkCommandResponseStatus?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "failed":
                    return TransportLinkCommandResponseStatus.Failed;
                case "invalid":
                    return TransportLinkCommandResponseStatus.Invalid;
                case "notSupported":
                    return TransportLinkCommandResponseStatus.NotSupported;
                case "pending":
                    return TransportLinkCommandResponseStatus.Pending;
                case "success":
                    return TransportLinkCommandResponseStatus.Success;
                case "timedOut":
                    return TransportLinkCommandResponseStatus.TimedOut;
            }
            throw new Exception("Cannot unmarshal type TransportLinkCommandResponseStatus");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (TransportLinkCommandResponseStatus)untypedValue;
            switch (value)
            {
                case TransportLinkCommandResponseStatus.Failed:
                    serializer.Serialize(writer, "failed");
                    return;
                case TransportLinkCommandResponseStatus.Invalid:
                    serializer.Serialize(writer, "invalid");
                    return;
                case TransportLinkCommandResponseStatus.NotSupported:
                    serializer.Serialize(writer, "notSupported");
                    return;
                case TransportLinkCommandResponseStatus.Pending:
                    serializer.Serialize(writer, "pending");
                    return;
                case TransportLinkCommandResponseStatus.Success:
                    serializer.Serialize(writer, "success");
                    return;
                case TransportLinkCommandResponseStatus.TimedOut:
                    serializer.Serialize(writer, "timedOut");
                    return;
            }
            throw new Exception("Cannot marshal type TransportLinkCommandResponseStatus");
        }

        public static readonly TransportLinkCommandResponseStatusConverter Singleton = new TransportLinkCommandResponseStatusConverter();
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

    internal class TransportLinkFaultStatusConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(TransportLinkFaultStatus) || t == typeof(TransportLinkFaultStatus?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "active":
                    return TransportLinkFaultStatus.Active;
                case "cleared":
                    return TransportLinkFaultStatus.Cleared;
            }
            throw new Exception("Cannot unmarshal type TransportLinkFaultStatus");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (TransportLinkFaultStatus)untypedValue;
            switch (value)
            {
                case TransportLinkFaultStatus.Active:
                    serializer.Serialize(writer, "active");
                    return;
                case TransportLinkFaultStatus.Cleared:
                    serializer.Serialize(writer, "cleared");
                    return;
            }
            throw new Exception("Cannot marshal type TransportLinkFaultStatus");
        }

        public static readonly TransportLinkFaultStatusConverter Singleton = new TransportLinkFaultStatusConverter();
    }

    internal class TransportLinkGpsConfigurationKindConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(TransportLinkGpsConfigurationKind) || t == typeof(TransportLinkGpsConfigurationKind?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            if (value == "gps")
            {
                return TransportLinkGpsConfigurationKind.Gps;
            }
            throw new Exception("Cannot unmarshal type TransportLinkGpsConfigurationKind");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (TransportLinkGpsConfigurationKind)untypedValue;
            if (value == TransportLinkGpsConfigurationKind.Gps)
            {
                serializer.Serialize(writer, "gps");
                return;
            }
            throw new Exception("Cannot marshal type TransportLinkGpsConfigurationKind");
        }

        public static readonly TransportLinkGpsConfigurationKindConverter Singleton = new TransportLinkGpsConfigurationKindConverter();
    }

    internal class TransportLinkScootConfigurationKindConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(TransportLinkScootConfigurationKind) || t == typeof(TransportLinkScootConfigurationKind?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            if (value == "scoot")
            {
                return TransportLinkScootConfigurationKind.Scoot;
            }
            throw new Exception("Cannot unmarshal type TransportLinkScootConfigurationKind");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (TransportLinkScootConfigurationKind)untypedValue;
            if (value == TransportLinkScootConfigurationKind.Scoot)
            {
                serializer.Serialize(writer, "scoot");
                return;
            }
            throw new Exception("Cannot marshal type TransportLinkScootConfigurationKind");
        }

        public static readonly TransportLinkScootConfigurationKindConverter Singleton = new TransportLinkScootConfigurationKindConverter();
    }

    internal class IndigoMinMaxValueCheckConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(double) || t == typeof(double?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<double>(reader);
            if (value >= 0 && value <= 100)
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
            if (value >= 0 && value <= 100)
            {
                serializer.Serialize(writer, value);
                return;
            }
            throw new Exception("Cannot marshal type double");
        }

        public static readonly IndigoMinMaxValueCheckConverter Singleton = new IndigoMinMaxValueCheckConverter();
    }

    internal class TransportLinkTypeKindConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(TransportLinkTypeKind) || t == typeof(TransportLinkTypeKind?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            if (value == "transportLink")
            {
                return TransportLinkTypeKind.TransportLink;
            }
            throw new Exception("Cannot unmarshal type TransportLinkTypeKind");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (TransportLinkTypeKind)untypedValue;
            if (value == TransportLinkTypeKind.TransportLink)
            {
                serializer.Serialize(writer, "transportLink");
                return;
            }
            throw new Exception("Cannot marshal type TransportLinkTypeKind");
        }

        public static readonly TransportLinkTypeKindConverter Singleton = new TransportLinkTypeKindConverter();
    }
}

namespace Topas.Utmc.TransportLink
{
    using System;
    using Newtonsoft.Json;
    using Newtonsoft.Json.Linq;

    /// <summary>
    /// Transport link command payloads.
    /// </summary>
    /// <remarks>
    /// Tagged union over <c>kind</c>. Deserialise as this interface to
    /// have the converter select the variant; the variant classes are
    /// generated by quicktype and adopt the interface via partial
    /// declarations below.
    /// </remarks>
    [JsonConverter(typeof(TransportLinkCommandConverter))]
    public interface ITransportLinkCommand { }

    public partial class RecalibrateTransportLinkCommand : ITransportLinkCommand { }
    public partial class RequestTransportLinkStatusCommand : ITransportLinkCommand { }
    public partial class SetStocCommand : ITransportLinkCommand { }

    internal class TransportLinkCommandConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(ITransportLinkCommand);

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
                case "recalibrate": return Read<RecalibrateTransportLinkCommand>(obj, serializer);
                case "requestStatus": return Read<RequestTransportLinkStatusCommand>(obj, serializer);
                case "setStoc": return Read<SetStocCommand>(obj, serializer);
                default:
                    throw new JsonSerializationException(
                        $"unknown kind '{tag}' for TransportLinkCommand");
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

namespace Topas.Utmc.TransportLink
{
    using System;
    using Newtonsoft.Json;
    using Newtonsoft.Json.Linq;

    /// <summary>
    /// Transport link operational configuration (SCOOT/GPS/ANPR discriminated union).
    /// </summary>
    /// <remarks>
    /// Tagged union over <c>kind</c>. Deserialise as this interface to
    /// have the converter select the variant; the variant classes are
    /// generated by quicktype and adopt the interface via partial
    /// declarations below.
    /// </remarks>
    [JsonConverter(typeof(TransportLinkConfigurationConverter))]
    public interface ITransportLinkConfiguration { }

    public partial class TransportLinkAnprConfiguration : ITransportLinkConfiguration { }
    public partial class TransportLinkGpsConfiguration : ITransportLinkConfiguration { }
    public partial class TransportLinkScootConfiguration : ITransportLinkConfiguration { }

    internal class TransportLinkConfigurationConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(ITransportLinkConfiguration);

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
                case "anpr": return Read<TransportLinkAnprConfiguration>(obj, serializer);
                case "gps": return Read<TransportLinkGpsConfiguration>(obj, serializer);
                case "scoot": return Read<TransportLinkScootConfiguration>(obj, serializer);
                default:
                    throw new JsonSerializationException(
                        $"unknown kind '{tag}' for TransportLinkConfiguration");
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

namespace Topas.Utmc.TransportLink
{
    using System;
    using Newtonsoft.Json;
    using Newtonsoft.Json.Linq;

    /// <summary>
    /// Transport link dynamic measurement data.
    /// </summary>
    /// <remarks>
    /// Tagged union over <c>kind</c>. Deserialise as this interface to
    /// have the converter select the variant; the variant classes are
    /// generated by quicktype and adopt the interface via partial
    /// declarations below.
    /// </remarks>
    [JsonConverter(typeof(TransportLinkMeasurementConverter))]
    public interface ITransportLinkMeasurement { }

    public partial class TransportLinkAnprDynamic : ITransportLinkMeasurement { }
    public partial class TransportLinkGpsDynamic : ITransportLinkMeasurement { }
    public partial class TransportLinkScootDynamic : ITransportLinkMeasurement { }

    internal class TransportLinkMeasurementConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(ITransportLinkMeasurement);

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
                case "anpr": return Read<TransportLinkAnprDynamic>(obj, serializer);
                case "gps": return Read<TransportLinkGpsDynamic>(obj, serializer);
                case "scoot": return Read<TransportLinkScootDynamic>(obj, serializer);
                default:
                    throw new JsonSerializationException(
                        $"unknown kind '{tag}' for TransportLinkMeasurement");
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

namespace Topas.Utmc.TransportLink
{
    using System;
    using Newtonsoft.Json;
    using Newtonsoft.Json.Linq;

    /// <summary>
    /// Transport link type definitions.
    /// </summary>
    /// <remarks>
    /// Tagged union over <c>kind</c>. Deserialise as this interface to
    /// have the converter select the variant; the variant classes are
    /// generated by quicktype and adopt the interface via partial
    /// declarations below.
    /// </remarks>
    [JsonConverter(typeof(TransportLinkTypeUnionConverter))]
    public interface ITransportLinkTypeUnion { }

    public partial class LinkStatusType : ITransportLinkTypeUnion { }
    public partial class TransportLinkType : ITransportLinkTypeUnion { }

    internal class TransportLinkTypeUnionConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(ITransportLinkTypeUnion);

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
                case "linkStatus": return Read<LinkStatusType>(obj, serializer);
                case "transportLink": return Read<TransportLinkType>(obj, serializer);
                default:
                    throw new JsonSerializationException(
                        $"unknown kind '{tag}' for TransportLinkTypeUnion");
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

namespace Topas.Utmc.TransportLink
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
