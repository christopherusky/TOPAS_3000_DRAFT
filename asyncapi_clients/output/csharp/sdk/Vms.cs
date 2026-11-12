/*
 * File:        output/csharp/sdk/Vms.cs
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
namespace Topas.Utmc.Vms
{
    using System;
    using System.Collections.Generic;

    using System.Globalization;
    using Newtonsoft.Json;
    using Newtonsoft.Json.Converters;

    public partial class AllTypes
    {
        [JsonProperty("ClearSignCommand", NullValueHandling = NullValueHandling.Ignore)]
        public ClearSignCommand ClearSignCommand { get; set; }

        [JsonProperty("DisplayGraphicCommand", NullValueHandling = NullValueHandling.Ignore)]
        public DisplayGraphicCommand DisplayGraphicCommand { get; set; }

        [JsonProperty("DisplayMessageCommand", NullValueHandling = NullValueHandling.Ignore)]
        public DisplayMessageCommand DisplayMessageCommand { get; set; }

        [JsonProperty("SetBrightnessCommand", NullValueHandling = NullValueHandling.Ignore)]
        public SetBrightnessCommand SetBrightnessCommand { get; set; }

        [JsonProperty("SyncCategoryEnum", NullValueHandling = NullValueHandling.Ignore)]
        public SyncCategoryEnum? SyncCategoryEnum { get; set; }

        [JsonProperty("SyncComplete", NullValueHandling = NullValueHandling.Ignore)]
        public SyncComplete SyncComplete { get; set; }

        [JsonProperty("SyncRequest", NullValueHandling = NullValueHandling.Ignore)]
        public SyncRequest SyncRequest { get; set; }

        [JsonProperty("SyncStatus", NullValueHandling = NullValueHandling.Ignore)]
        public SyncStatusEnum? SyncStatus { get; set; }

        [JsonProperty("VmsColourCapability", NullValueHandling = NullValueHandling.Ignore)]
        public VmsColourCapability? VmsColourCapability { get; set; }

        [JsonProperty("VmsCommandResponse", NullValueHandling = NullValueHandling.Ignore)]
        public VmsCommandResponse VmsCommandResponse { get; set; }

        [JsonProperty("VmsConfiguration", NullValueHandling = NullValueHandling.Ignore)]
        public VmsConfiguration VmsConfiguration { get; set; }

        [JsonProperty("VmsDefinition", NullValueHandling = NullValueHandling.Ignore)]
        public VmsDefinition VmsDefinition { get; set; }

        [JsonProperty("VmsDynamic", NullValueHandling = NullValueHandling.Ignore)]
        public VmsDynamic VmsDynamic { get; set; }

        [JsonProperty("VmsFault", NullValueHandling = NullValueHandling.Ignore)]
        public VmsFault VmsFault { get; set; }

        [JsonProperty("VmsHistoryEntry", NullValueHandling = NullValueHandling.Ignore)]
        public VmsHistoryEntry VmsHistoryEntry { get; set; }

        [JsonProperty("VmsStatus", NullValueHandling = NullValueHandling.Ignore)]
        public VmsStatusEnum? VmsStatus { get; set; }

        [JsonProperty("VmsType", NullValueHandling = NullValueHandling.Ignore)]
        public VmsType VmsType { get; set; }
    }

    /// <summary>
    /// Clear the sign (blank the display).
    /// </summary>
    public partial class ClearSignCommand
    {
        /// <summary>
        /// Caller-assigned identifier.
        /// </summary>
        [JsonProperty("commandCorrelationId")]
        public long CommandCorrelationId { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Discriminator for the tagged union. The fixed value "clearSign" selects the
        /// ClearSignCommand payload; it is not free-text command content.
        /// </summary>
        [JsonProperty("kind")]
        public ClearSignCommandKind Kind { get; set; }

        /// <summary>
        /// When the command was issued.
        /// </summary>
        [JsonProperty("requestTime")]
        public DateTimeOffset RequestTime { get; set; }

        /// <summary>
        /// Identifier of the VMS receiving the command.
        /// </summary>
        [JsonProperty("vmsId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string VmsId { get; set; }
    }

    /// <summary>
    /// Display a graphic on the sign.
    /// </summary>
    public partial class DisplayGraphicCommand
    {
        /// <summary>
        /// Caller-assigned identifier.
        /// </summary>
        [JsonProperty("commandCorrelationId")]
        public long CommandCorrelationId { get; set; }

        /// <summary>
        /// How long to display the graphic.
        /// </summary>
        [JsonProperty("displayDurationSeconds", NullValueHandling = NullValueHandling.Ignore)]
        public long? DisplayDurationSeconds { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// URL of the graphic to display.
        /// </summary>
        [JsonProperty("graphicUrl")]
        public Uri GraphicUrl { get; set; }

        /// <summary>
        /// Discriminator for the tagged union. The fixed value "displayGraphic" selects the
        /// DisplayGraphicCommand payload; it is not free-text command content.
        /// </summary>
        [JsonProperty("kind")]
        public DisplayGraphicCommandKind Kind { get; set; }

        /// <summary>
        /// When the command was issued.
        /// </summary>
        [JsonProperty("requestTime")]
        public DateTimeOffset RequestTime { get; set; }

        /// <summary>
        /// Identifier of the VMS receiving the command.
        /// </summary>
        [JsonProperty("vmsId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string VmsId { get; set; }
    }

    /// <summary>
    /// Display a text message on the sign.
    /// </summary>
    public partial class DisplayMessageCommand
    {
        /// <summary>
        /// Caller-assigned identifier.
        /// </summary>
        [JsonProperty("commandCorrelationId")]
        public long CommandCorrelationId { get; set; }

        /// <summary>
        /// How long to display the message.
        /// </summary>
        [JsonProperty("displayDurationSeconds", NullValueHandling = NullValueHandling.Ignore)]
        public long? DisplayDurationSeconds { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Discriminator for the tagged union. The fixed value "displayMessage" selects the
        /// DisplayMessageCommand payload; it is not free-text command content.
        /// </summary>
        [JsonProperty("kind")]
        public DisplayMessageCommandKind Kind { get; set; }

        /// <summary>
        /// Message text to display.
        /// </summary>
        [JsonProperty("message")]
        public string Message { get; set; }

        /// <summary>
        /// Message split by lines.
        /// </summary>
        [JsonProperty("messageLines", NullValueHandling = NullValueHandling.Ignore)]
        public string[] MessageLines { get; set; }

        /// <summary>
        /// When the command was issued.
        /// </summary>
        [JsonProperty("requestTime")]
        public DateTimeOffset RequestTime { get; set; }

        /// <summary>
        /// Identifier of the VMS receiving the command.
        /// </summary>
        [JsonProperty("vmsId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string VmsId { get; set; }
    }

    /// <summary>
    /// Set the brightness of the sign.
    /// </summary>
    public partial class SetBrightnessCommand
    {
        /// <summary>
        /// Brightness percentage to set.
        /// </summary>
        [JsonProperty("brightnessPercent")]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double BrightnessPercent { get; set; }

        /// <summary>
        /// Caller-assigned identifier.
        /// </summary>
        [JsonProperty("commandCorrelationId")]
        public long CommandCorrelationId { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Discriminator for the tagged union. The fixed value "setBrightness" selects the
        /// SetBrightnessCommand payload; it is not free-text command content.
        /// </summary>
        [JsonProperty("kind")]
        public SetBrightnessCommandKind Kind { get; set; }

        /// <summary>
        /// When the command was issued.
        /// </summary>
        [JsonProperty("requestTime")]
        public DateTimeOffset RequestTime { get; set; }

        /// <summary>
        /// Identifier of the VMS receiving the command.
        /// </summary>
        [JsonProperty("vmsId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string VmsId { get; set; }
    }

    /// <summary>
    /// Terminal message ending a vms sync batch. Sent to the requester's reply node after the
    /// last object, or instead of any objects when the filter matched none.
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
    /// Request the current state of vms objects. The Data Publisher answers to the request's
    /// reply-to address, not the domain's data channels: a sync batch (x-message-mode: sync,
    /// x-sync-id/-sequence/-total) followed by a SyncComplete.
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
    /// Variable message sign command response payloads.
    /// </summary>
    public partial class VmsCommandResponse
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
        public VmsCommandResponseStatus Status { get; set; }

        /// <summary>
        /// Identifier of the VMS the command targeted.
        /// </summary>
        [JsonProperty("vmsId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string VmsId { get; set; }
    }

    /// <summary>
    /// Variable message sign operational configuration.
    /// </summary>
    public partial class VmsConfiguration
    {
        /// <summary>
        /// Whether auto-brightness is enabled.
        /// </summary>
        [JsonProperty("autoBrightness", NullValueHandling = NullValueHandling.Ignore)]
        public bool? AutoBrightness { get; set; }

        /// <summary>
        /// When effective.
        /// </summary>
        [JsonProperty("configurationDate")]
        public DateTimeOffset ConfigurationDate { get; set; }

        /// <summary>
        /// Default brightness percentage.
        /// </summary>
        [JsonProperty("defaultBrightnessPercent", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double? DefaultBrightnessPercent { get; set; }

        /// <summary>
        /// Default message display time in seconds.
        /// </summary>
        [JsonProperty("defaultDisplayTimeSeconds", NullValueHandling = NullValueHandling.Ignore)]
        public long? DefaultDisplayTimeSeconds { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Text scroll speed.
        /// </summary>
        [JsonProperty("scrollSpeed", NullValueHandling = NullValueHandling.Ignore)]
        public long? ScrollSpeed { get; set; }

        /// <summary>
        /// VMS this configuration applies to.
        /// </summary>
        [JsonProperty("vmsId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string VmsId { get; set; }
    }

    /// <summary>
    /// Variable message sign equipment details.
    /// </summary>
    public partial class VmsDefinition
    {
        /// <summary>
        /// Characters per line.
        /// </summary>
        [JsonProperty("charactersPerLine", NullValueHandling = NullValueHandling.Ignore)]
        public long? CharactersPerLine { get; set; }

        /// <summary>
        /// Colour capability.
        /// </summary>
        [JsonProperty("colourCapability", NullValueHandling = NullValueHandling.Ignore)]
        public VmsColourCapability? ColourCapability { get; set; }

        /// <summary>
        /// When this definition was created.
        /// </summary>
        [JsonProperty("creationDate")]
        public DateTimeOffset CreationDate { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Distance along that transport link, in metres. Optional.
        /// </summary>
        [JsonProperty("linkDistanceMetres", NullValueHandling = NullValueHandling.Ignore)]
        public double? LinkDistanceMetres { get; set; }

        /// <summary>
        /// Physical location.
        /// </summary>
        [JsonProperty("location", NullValueHandling = NullValueHandling.Ignore)]
        public Location Location { get; set; }

        /// <summary>
        /// Longer description.
        /// </summary>
        [JsonProperty("longDescription", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(IndecentMinMaxLengthCheckConverter))]
        public string LongDescription { get; set; }

        /// <summary>
        /// Network link the sign stands on. At most one of this and networkPathId. Optional.
        /// </summary>
        [JsonProperty("networkLinkId", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string NetworkLinkId { get; set; }

        /// <summary>
        /// Network path the sign stands on. At most one of this and networkLinkId. Optional.
        /// </summary>
        [JsonProperty("networkPathId", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string NetworkPathId { get; set; }

        /// <summary>
        /// Number of text lines.
        /// </summary>
        [JsonProperty("numberOfLines", NullValueHandling = NullValueHandling.Ignore)]
        public long? NumberOfLines { get; set; }

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
        /// Sign height in pixels/units.
        /// </summary>
        [JsonProperty("signHeight", NullValueHandling = NullValueHandling.Ignore)]
        public long? SignHeight { get; set; }

        /// <summary>
        /// Sign width in pixels/units.
        /// </summary>
        [JsonProperty("signWidth", NullValueHandling = NullValueHandling.Ignore)]
        public long? SignWidth { get; set; }

        /// <summary>
        /// Whether the sign supports graphics.
        /// </summary>
        [JsonProperty("supportsGraphics", NullValueHandling = NullValueHandling.Ignore)]
        public bool? SupportsGraphics { get; set; }

        /// <summary>
        /// Transport link this sign monitors or sits on. Optional.
        /// </summary>
        [JsonProperty("transportLinkId", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string TransportLinkId { get; set; }

        /// <summary>
        /// Unique identifier for a VMS type (sign type catalogue entry).
        /// </summary>
        [JsonProperty("typeId")]
        [JsonConverter(typeof(IndigoMinMaxLengthCheckConverter))]
        public string TypeId { get; set; }

        /// <summary>
        /// Unique identifier of this VMS.
        /// </summary>
        [JsonProperty("vmsId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string VmsId { get; set; }
    }

    /// <summary>
    /// Physical location.
    ///
    /// A geographic point: a coordinate plus optional altitude.
    /// </summary>
    public partial class Location
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
    /// Variable message sign dynamic measurement data.
    /// </summary>
    public partial class VmsDynamic
    {
        /// <summary>
        /// Current brightness percentage.
        /// </summary>
        [JsonProperty("brightnessPercent", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
        public double? BrightnessPercent { get; set; }

        /// <summary>
        /// Currently displayed message text.
        /// </summary>
        [JsonProperty("currentMessage", NullValueHandling = NullValueHandling.Ignore)]
        public string CurrentMessage { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// URL of the currently displayed graphic.
        /// </summary>
        [JsonProperty("graphicUrl", NullValueHandling = NullValueHandling.Ignore)]
        public Uri GraphicUrl { get; set; }

        /// <summary>
        /// When the message expires.
        /// </summary>
        [JsonProperty("messageExpiryTime", NullValueHandling = NullValueHandling.Ignore)]
        public DateTimeOffset? MessageExpiryTime { get; set; }

        /// <summary>
        /// Message split by lines.
        /// </summary>
        [JsonProperty("messageLines", NullValueHandling = NullValueHandling.Ignore)]
        public string[] MessageLines { get; set; }

        /// <summary>
        /// When the current message was set.
        /// </summary>
        [JsonProperty("messageSetTime", NullValueHandling = NullValueHandling.Ignore)]
        public DateTimeOffset? MessageSetTime { get; set; }

        /// <summary>
        /// Optional quality statement.
        /// </summary>
        [JsonProperty("quality", NullValueHandling = NullValueHandling.Ignore)]
        public Quality Quality { get; set; }

        /// <summary>
        /// Operational status.
        /// </summary>
        [JsonProperty("status", NullValueHandling = NullValueHandling.Ignore)]
        public VmsStatusEnum? Status { get; set; }

        /// <summary>
        /// Time of the observation.
        /// </summary>
        [JsonProperty("timestamp")]
        public DateTimeOffset Timestamp { get; set; }

        /// <summary>
        /// Identifier of the VMS.
        /// </summary>
        [JsonProperty("vmsId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string VmsId { get; set; }
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
    /// Variable message sign fault reports.
    /// </summary>
    public partial class VmsFault
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
        /// When the fault was cleared.
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
        /// Reference to a FaultTypeDefinition entry in the shared fault catalogue.
        /// </summary>
        [JsonProperty("faultTypeId")]
        [JsonConverter(typeof(IndigoMinMaxLengthCheckConverter))]
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
        public VmsFaultStatus? Status { get; set; }

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
        /// Identifier of the VMS this fault is about.
        /// </summary>
        [JsonProperty("vmsId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string VmsId { get; set; }
    }

    /// <summary>
    /// Variable message sign history entries.
    /// </summary>
    public partial class VmsHistoryEntry
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
        /// Equipment log data (free-form text).
        /// </summary>
        [JsonProperty("logEntry")]
        public string LogEntry { get; set; }

        /// <summary>
        /// Unique identifier of this log entry.
        /// </summary>
        [JsonProperty("logId")]
        public long LogId { get; set; }

        /// <summary>
        /// User who created the log entry.
        /// </summary>
        [JsonProperty("logUser", NullValueHandling = NullValueHandling.Ignore)]
        public string LogUser { get; set; }

        /// <summary>
        /// Identifier of the VMS this log entry is about.
        /// </summary>
        [JsonProperty("vmsId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string VmsId { get; set; }
    }

    /// <summary>
    /// Variable message sign type definitions.
    /// </summary>
    public partial class VmsType
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
        /// URL to an external specification document.
        /// </summary>
        [JsonProperty("specificationLink", NullValueHandling = NullValueHandling.Ignore)]
        public Uri SpecificationLink { get; set; }

        /// <summary>
        /// Identifier of this VMS type.
        /// </summary>
        [JsonProperty("typeId")]
        [JsonConverter(typeof(IndigoMinMaxLengthCheckConverter))]
        public string TypeId { get; set; }
    }

    public enum ClearSignCommandKind { ClearSign };

    public enum DisplayGraphicCommandKind { DisplayGraphic };

    public enum DisplayMessageCommandKind { DisplayMessage };

    public enum SetBrightnessCommandKind { SetBrightness };

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
    /// Colour capability of a variable message sign.
    ///
    /// Colour capability.
    /// </summary>
    public enum VmsColourCapability { Amber, FullColour, Monochrome, MultiColour };

    /// <summary>
    /// Outcome of the command execution.
    ///
    /// Outcome of a command execution.
    ///
    /// Every value except ``PENDING`` is terminal. Consumers SHOULD test for
    /// "not pending" rather than enumerating the terminal values, so that a
    /// future profile adding one does not strand them.
    /// </summary>
    public enum VmsCommandResponseStatus { Failed, Invalid, NotSupported, Pending, Success, TimedOut };

    /// <summary>
    /// Operational status.
    ///
    /// Operational status of a variable message sign.
    /// </summary>
    public enum VmsStatusEnum { Blank, Displaying, Fault, Off, On, Unknown };

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
    public enum VmsFaultStatus { Active, Cleared };

    internal static class Converter
    {
        public static readonly JsonSerializerSettings Settings = new JsonSerializerSettings
        {
            MetadataPropertyHandling = MetadataPropertyHandling.Ignore,
            DateParseHandling = DateParseHandling.None,
            Converters =
            {
                ClearSignCommandKindConverter.Singleton,
                DisplayGraphicCommandKindConverter.Singleton,
                DisplayMessageCommandKindConverter.Singleton,
                SetBrightnessCommandKindConverter.Singleton,
                SyncCategoryEnumConverter.Singleton,
                SyncStatusEnumConverter.Singleton,
                VmsColourCapabilityConverter.Singleton,
                VmsCommandResponseStatusConverter.Singleton,
                VmsStatusEnumConverter.Singleton,
                AcknowledgementStateConverter.Singleton,
                VmsFaultStatusConverter.Singleton,
                new IsoDateTimeConverter { DateTimeStyles = DateTimeStyles.AssumeUniversal }
            },
        };
    }

    internal class ClearSignCommandKindConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(ClearSignCommandKind) || t == typeof(ClearSignCommandKind?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            if (value == "clearSign")
            {
                return ClearSignCommandKind.ClearSign;
            }
            throw new Exception("Cannot unmarshal type ClearSignCommandKind");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (ClearSignCommandKind)untypedValue;
            if (value == ClearSignCommandKind.ClearSign)
            {
                serializer.Serialize(writer, "clearSign");
                return;
            }
            throw new Exception("Cannot marshal type ClearSignCommandKind");
        }

        public static readonly ClearSignCommandKindConverter Singleton = new ClearSignCommandKindConverter();
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

    internal class DisplayGraphicCommandKindConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(DisplayGraphicCommandKind) || t == typeof(DisplayGraphicCommandKind?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            if (value == "displayGraphic")
            {
                return DisplayGraphicCommandKind.DisplayGraphic;
            }
            throw new Exception("Cannot unmarshal type DisplayGraphicCommandKind");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (DisplayGraphicCommandKind)untypedValue;
            if (value == DisplayGraphicCommandKind.DisplayGraphic)
            {
                serializer.Serialize(writer, "displayGraphic");
                return;
            }
            throw new Exception("Cannot marshal type DisplayGraphicCommandKind");
        }

        public static readonly DisplayGraphicCommandKindConverter Singleton = new DisplayGraphicCommandKindConverter();
    }

    internal class DisplayMessageCommandKindConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(DisplayMessageCommandKind) || t == typeof(DisplayMessageCommandKind?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            if (value == "displayMessage")
            {
                return DisplayMessageCommandKind.DisplayMessage;
            }
            throw new Exception("Cannot unmarshal type DisplayMessageCommandKind");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (DisplayMessageCommandKind)untypedValue;
            if (value == DisplayMessageCommandKind.DisplayMessage)
            {
                serializer.Serialize(writer, "displayMessage");
                return;
            }
            throw new Exception("Cannot marshal type DisplayMessageCommandKind");
        }

        public static readonly DisplayMessageCommandKindConverter Singleton = new DisplayMessageCommandKindConverter();
    }

    internal class PurpleMinMaxValueCheckConverter : JsonConverter
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

        public static readonly PurpleMinMaxValueCheckConverter Singleton = new PurpleMinMaxValueCheckConverter();
    }

    internal class SetBrightnessCommandKindConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(SetBrightnessCommandKind) || t == typeof(SetBrightnessCommandKind?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            if (value == "setBrightness")
            {
                return SetBrightnessCommandKind.SetBrightness;
            }
            throw new Exception("Cannot unmarshal type SetBrightnessCommandKind");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (SetBrightnessCommandKind)untypedValue;
            if (value == SetBrightnessCommandKind.SetBrightness)
            {
                serializer.Serialize(writer, "setBrightness");
                return;
            }
            throw new Exception("Cannot marshal type SetBrightnessCommandKind");
        }

        public static readonly SetBrightnessCommandKindConverter Singleton = new SetBrightnessCommandKindConverter();
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

    internal class VmsColourCapabilityConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(VmsColourCapability) || t == typeof(VmsColourCapability?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "amber":
                    return VmsColourCapability.Amber;
                case "fullColour":
                    return VmsColourCapability.FullColour;
                case "monochrome":
                    return VmsColourCapability.Monochrome;
                case "multiColour":
                    return VmsColourCapability.MultiColour;
            }
            throw new Exception("Cannot unmarshal type VmsColourCapability");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (VmsColourCapability)untypedValue;
            switch (value)
            {
                case VmsColourCapability.Amber:
                    serializer.Serialize(writer, "amber");
                    return;
                case VmsColourCapability.FullColour:
                    serializer.Serialize(writer, "fullColour");
                    return;
                case VmsColourCapability.Monochrome:
                    serializer.Serialize(writer, "monochrome");
                    return;
                case VmsColourCapability.MultiColour:
                    serializer.Serialize(writer, "multiColour");
                    return;
            }
            throw new Exception("Cannot marshal type VmsColourCapability");
        }

        public static readonly VmsColourCapabilityConverter Singleton = new VmsColourCapabilityConverter();
    }

    internal class VmsCommandResponseStatusConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(VmsCommandResponseStatus) || t == typeof(VmsCommandResponseStatus?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "failed":
                    return VmsCommandResponseStatus.Failed;
                case "invalid":
                    return VmsCommandResponseStatus.Invalid;
                case "notSupported":
                    return VmsCommandResponseStatus.NotSupported;
                case "pending":
                    return VmsCommandResponseStatus.Pending;
                case "success":
                    return VmsCommandResponseStatus.Success;
                case "timedOut":
                    return VmsCommandResponseStatus.TimedOut;
            }
            throw new Exception("Cannot unmarshal type VmsCommandResponseStatus");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (VmsCommandResponseStatus)untypedValue;
            switch (value)
            {
                case VmsCommandResponseStatus.Failed:
                    serializer.Serialize(writer, "failed");
                    return;
                case VmsCommandResponseStatus.Invalid:
                    serializer.Serialize(writer, "invalid");
                    return;
                case VmsCommandResponseStatus.NotSupported:
                    serializer.Serialize(writer, "notSupported");
                    return;
                case VmsCommandResponseStatus.Pending:
                    serializer.Serialize(writer, "pending");
                    return;
                case VmsCommandResponseStatus.Success:
                    serializer.Serialize(writer, "success");
                    return;
                case VmsCommandResponseStatus.TimedOut:
                    serializer.Serialize(writer, "timedOut");
                    return;
            }
            throw new Exception("Cannot marshal type VmsCommandResponseStatus");
        }

        public static readonly VmsCommandResponseStatusConverter Singleton = new VmsCommandResponseStatusConverter();
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

    internal class VmsStatusEnumConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(VmsStatusEnum) || t == typeof(VmsStatusEnum?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "blank":
                    return VmsStatusEnum.Blank;
                case "displaying":
                    return VmsStatusEnum.Displaying;
                case "fault":
                    return VmsStatusEnum.Fault;
                case "off":
                    return VmsStatusEnum.Off;
                case "on":
                    return VmsStatusEnum.On;
                case "unknown":
                    return VmsStatusEnum.Unknown;
            }
            throw new Exception("Cannot unmarshal type VmsStatusEnum");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (VmsStatusEnum)untypedValue;
            switch (value)
            {
                case VmsStatusEnum.Blank:
                    serializer.Serialize(writer, "blank");
                    return;
                case VmsStatusEnum.Displaying:
                    serializer.Serialize(writer, "displaying");
                    return;
                case VmsStatusEnum.Fault:
                    serializer.Serialize(writer, "fault");
                    return;
                case VmsStatusEnum.Off:
                    serializer.Serialize(writer, "off");
                    return;
                case VmsStatusEnum.On:
                    serializer.Serialize(writer, "on");
                    return;
                case VmsStatusEnum.Unknown:
                    serializer.Serialize(writer, "unknown");
                    return;
            }
            throw new Exception("Cannot marshal type VmsStatusEnum");
        }

        public static readonly VmsStatusEnumConverter Singleton = new VmsStatusEnumConverter();
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

    internal class VmsFaultStatusConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(VmsFaultStatus) || t == typeof(VmsFaultStatus?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "active":
                    return VmsFaultStatus.Active;
                case "cleared":
                    return VmsFaultStatus.Cleared;
            }
            throw new Exception("Cannot unmarshal type VmsFaultStatus");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (VmsFaultStatus)untypedValue;
            switch (value)
            {
                case VmsFaultStatus.Active:
                    serializer.Serialize(writer, "active");
                    return;
                case VmsFaultStatus.Cleared:
                    serializer.Serialize(writer, "cleared");
                    return;
            }
            throw new Exception("Cannot marshal type VmsFaultStatus");
        }

        public static readonly VmsFaultStatusConverter Singleton = new VmsFaultStatusConverter();
    }
}

namespace Topas.Utmc.Vms
{
    using System;
    using Newtonsoft.Json;
    using Newtonsoft.Json.Linq;

    /// <summary>
    /// Variable message sign command payloads.
    /// </summary>
    /// <remarks>
    /// Tagged union over <c>kind</c>. Deserialise as this interface to
    /// have the converter select the variant; the variant classes are
    /// generated by quicktype and adopt the interface via partial
    /// declarations below.
    /// </remarks>
    [JsonConverter(typeof(VmsCommandConverter))]
    public interface IVmsCommand { }

    public partial class ClearSignCommand : IVmsCommand { }
    public partial class DisplayGraphicCommand : IVmsCommand { }
    public partial class DisplayMessageCommand : IVmsCommand { }
    public partial class SetBrightnessCommand : IVmsCommand { }

    internal class VmsCommandConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(IVmsCommand);

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
                case "clearSign": return Read<ClearSignCommand>(obj, serializer);
                case "displayGraphic": return Read<DisplayGraphicCommand>(obj, serializer);
                case "displayMessage": return Read<DisplayMessageCommand>(obj, serializer);
                case "setBrightness": return Read<SetBrightnessCommand>(obj, serializer);
                default:
                    throw new JsonSerializationException(
                        $"unknown kind '{tag}' for VmsCommand");
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

namespace Topas.Utmc.Vms
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
