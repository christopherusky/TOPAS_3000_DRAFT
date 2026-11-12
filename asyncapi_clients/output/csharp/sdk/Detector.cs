/*
 * File:        output/csharp/sdk/Detector.cs
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
namespace Topas.Utmc.Detector
{
    using System;
    using System.Collections.Generic;

    using System.Globalization;
    using Newtonsoft.Json;
    using Newtonsoft.Json.Converters;

    public partial class AllTypes
    {
        [JsonProperty("CalibrateDetectorCommand", NullValueHandling = NullValueHandling.Ignore)]
        public CalibrateDetectorCommand CalibrateDetectorCommand { get; set; }

        [JsonProperty("DetectorCommandResponse", NullValueHandling = NullValueHandling.Ignore)]
        public DetectorCommandResponse DetectorCommandResponse { get; set; }

        [JsonProperty("DetectorConfiguration", NullValueHandling = NullValueHandling.Ignore)]
        public DetectorConfiguration DetectorConfiguration { get; set; }

        [JsonProperty("DetectorDefinition", NullValueHandling = NullValueHandling.Ignore)]
        public DetectorDefinition DetectorDefinition { get; set; }

        [JsonProperty("DetectorFault", NullValueHandling = NullValueHandling.Ignore)]
        public DetectorFault DetectorFault { get; set; }

        [JsonProperty("DetectorHistoryEntry", NullValueHandling = NullValueHandling.Ignore)]
        public DetectorHistoryEntry DetectorHistoryEntry { get; set; }

        [JsonProperty("DetectorType", NullValueHandling = NullValueHandling.Ignore)]
        public DetectorType DetectorType { get; set; }

        [JsonProperty("FlowDynamic", NullValueHandling = NullValueHandling.Ignore)]
        public FlowDynamic FlowDynamic { get; set; }

        [JsonProperty("HeadwayDynamic", NullValueHandling = NullValueHandling.Ignore)]
        public HeadwayDynamic HeadwayDynamic { get; set; }

        [JsonProperty("OccupancyDynamic", NullValueHandling = NullValueHandling.Ignore)]
        public OccupancyDynamic OccupancyDynamic { get; set; }

        [JsonProperty("QueueDynamic", NullValueHandling = NullValueHandling.Ignore)]
        public QueueDynamic QueueDynamic { get; set; }

        [JsonProperty("ResetDetectorCommand", NullValueHandling = NullValueHandling.Ignore)]
        public ResetDetectorCommand ResetDetectorCommand { get; set; }

        [JsonProperty("SetThresholdDetectorCommand", NullValueHandling = NullValueHandling.Ignore)]
        public SetThresholdDetectorCommand SetThresholdDetectorCommand { get; set; }

        [JsonProperty("SpeedDynamic", NullValueHandling = NullValueHandling.Ignore)]
        public SpeedDynamic SpeedDynamic { get; set; }

        [JsonProperty("SyncCategoryEnum", NullValueHandling = NullValueHandling.Ignore)]
        public SyncCategoryEnum? SyncCategoryEnum { get; set; }

        [JsonProperty("SyncComplete", NullValueHandling = NullValueHandling.Ignore)]
        public SyncComplete SyncComplete { get; set; }

        [JsonProperty("SyncRequest", NullValueHandling = NullValueHandling.Ignore)]
        public SyncRequest SyncRequest { get; set; }

        [JsonProperty("SyncStatus", NullValueHandling = NullValueHandling.Ignore)]
        public SyncStatusEnum? SyncStatus { get; set; }
    }

    /// <summary>
    /// Trigger a calibration cycle on the detector.
    /// </summary>
    public partial class CalibrateDetectorCommand
    {
        /// <summary>
        /// Caller-assigned identifier correlating a command request with its response. Not an
        /// identifier of the command itself. Named to avoid collision with the AMQP 1.0
        /// correlation-id message property.
        /// </summary>
        [JsonProperty("commandCorrelationId")]
        public long CommandCorrelationId { get; set; }

        /// <summary>
        /// Identifier of the detector receiving the command.
        /// </summary>
        [JsonProperty("detectorId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string DetectorId { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Discriminator for the tagged union. The fixed value "calibrate" selects the
        /// CalibrateDetectorCommand payload; it is not free-text command content.
        /// </summary>
        [JsonProperty("kind")]
        public CalibrateDetectorCommandKind Kind { get; set; }

        /// <summary>
        /// When the command was issued.
        /// </summary>
        [JsonProperty("requestTime")]
        public DateTimeOffset RequestTime { get; set; }
    }

    /// <summary>
    /// Detector command response payloads.
    /// </summary>
    public partial class DetectorCommandResponse
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
        /// Identifier of the detector the command targeted.
        /// </summary>
        [JsonProperty("detectorId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string DetectorId { get; set; }

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
        public DetectorCommandResponseStatus Status { get; set; }
    }

    /// <summary>
    /// Detector operational configuration.
    /// </summary>
    public partial class DetectorConfiguration
    {
        /// <summary>
        /// When this configuration record became effective.
        /// </summary>
        [JsonProperty("configurationDate")]
        public DateTimeOffset ConfigurationDate { get; set; }

        /// <summary>
        /// Identifier of the detector this configuration applies to.
        /// </summary>
        [JsonProperty("detectorId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string DetectorId { get; set; }

        /// <summary>
        /// Vendor or local extensions. Open-content object.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Downward threshold for alerting.
        /// </summary>
        [JsonProperty("thresholdDown", NullValueHandling = NullValueHandling.Ignore)]
        public long? ThresholdDown { get; set; }

        /// <summary>
        /// Upward threshold for alerting (e.g. occupancy threshold).
        /// </summary>
        [JsonProperty("thresholdUp", NullValueHandling = NullValueHandling.Ignore)]
        public long? ThresholdUp { get; set; }
    }

    /// <summary>
    /// Detector equipment details.
    /// </summary>
    public partial class DetectorDefinition
    {
        /// <summary>
        /// When this definition record was created.
        /// </summary>
        [JsonProperty("creationDate")]
        public DateTimeOffset CreationDate { get; set; }

        /// <summary>
        /// Unique identifier of this detector within its owning system.
        /// </summary>
        [JsonProperty("detectorId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string DetectorId { get; set; }

        /// <summary>
        /// Vendor or local extensions. Open-content object.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Distance along that transport link, in metres. Optional.
        /// </summary>
        [JsonProperty("linkDistanceMetres", NullValueHandling = NullValueHandling.Ignore)]
        public double? LinkDistanceMetres { get; set; }

        /// <summary>
        /// Physical location of the detector. Optional.
        /// </summary>
        [JsonProperty("location", NullValueHandling = NullValueHandling.Ignore)]
        public Location Location { get; set; }

        /// <summary>
        /// Longer description (context, installation notes, etc.).
        /// </summary>
        [JsonProperty("longDescription", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(FluffyMinMaxLengthCheckConverter))]
        public string LongDescription { get; set; }

        /// <summary>
        /// Network link the detector stands on. At most one of this and networkPathId. Optional.
        /// </summary>
        [JsonProperty("networkLinkId", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string NetworkLinkId { get; set; }

        /// <summary>
        /// Network path the detector stands on. At most one of this and networkLinkId. Optional.
        /// </summary>
        [JsonProperty("networkPathId", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string NetworkPathId { get; set; }

        /// <summary>
        /// Identifier of this object on the originating (upstream) system. Enables cross-system
        /// traceability.
        /// </summary>
        [JsonProperty("originatingObjectId", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string OriginatingObjectId { get; set; }

        /// <summary>
        /// Identifier of the upstream system that originally created this object (comparable to the
        /// UML DataSourceTypeId).
        /// </summary>
        [JsonProperty("originatingParticipantId", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string OriginatingParticipantId { get; set; }

        /// <summary>
        /// System that owns this detector definition. Often implicit from context.
        /// </summary>
        [JsonProperty("participantId", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string ParticipantId { get; set; }

        /// <summary>
        /// Short human-readable name (e.g. 'A1 southbound').
        /// </summary>
        [JsonProperty("shortDescription", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(TentacledMinMaxLengthCheckConverter))]
        public string ShortDescription { get; set; }

        /// <summary>
        /// Transport link this detector monitors or sits on. Optional.
        /// </summary>
        [JsonProperty("transportLinkId", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string TransportLinkId { get; set; }

        /// <summary>
        /// Unique identifier for a detector type (loop, radar, ANPR, ...).
        /// </summary>
        [JsonProperty("typeId")]
        [JsonConverter(typeof(StickyMinMaxLengthCheckConverter))]
        public string TypeId { get; set; }
    }

    /// <summary>
    /// Physical location of the detector. Optional.
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
        [JsonConverter(typeof(PurpleMinMaxValueCheckConverter))]
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
        [JsonConverter(typeof(FluffyMinMaxValueCheckConverter))]
        public double Latitude { get; set; }

        /// <summary>
        /// Longitude in decimal degrees (WGS84).
        /// </summary>
        [JsonProperty("longitude")]
        [JsonConverter(typeof(TentacledMinMaxValueCheckConverter))]
        public double Longitude { get; set; }
    }

    /// <summary>
    /// Detector fault reports.
    /// </summary>
    public partial class DetectorFault
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
        /// Identifier of the detector this fault is about.
        /// </summary>
        [JsonProperty("detectorId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string DetectorId { get; set; }

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
        /// Reference to a DetectorFaultType entry in the fault catalogue.
        /// </summary>
        [JsonProperty("faultTypeId")]
        [JsonConverter(typeof(StickyMinMaxLengthCheckConverter))]
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
        public DetectorFaultStatus? Status { get; set; }

        /// <summary>
        /// Reference to a SubsystemDefinition entry: the subsystem the fault arose in.
        /// </summary>
        [JsonProperty("subsystemTypeId", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(StickyMinMaxLengthCheckConverter))]
        public string SubsystemTypeId { get; set; }

        /// <summary>
        /// Fault identifier allocated by the supplier or configuration manager for maintenance
        /// purposes, if required.
        /// </summary>
        [JsonProperty("supplierFaultNumber", NullValueHandling = NullValueHandling.Ignore)]
        public string SupplierFaultNumber { get; set; }
    }

    /// <summary>
    /// Detector history entries.
    /// </summary>
    public partial class DetectorHistoryEntry
    {
        /// <summary>
        /// Identifier of the detector this log entry is about.
        /// </summary>
        [JsonProperty("detectorId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string DetectorId { get; set; }

        /// <summary>
        /// Category or type of the history event (e.g. 'recalibration', 'threshold_change').
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
    }

    /// <summary>
    /// Detector type definitions.
    /// </summary>
    public partial class DetectorType
    {
        /// <summary>
        /// Human-readable description of the detector kind.
        /// </summary>
        [JsonProperty("description")]
        public string Description { get; set; }

        /// <summary>
        /// Vendor or local extensions. Open-content object.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Additional notes, e.g. deployment details.
        /// </summary>
        [JsonProperty("notes", NullValueHandling = NullValueHandling.Ignore)]
        public string Notes { get; set; }

        /// <summary>
        /// URL to an external specification document for this detector type.
        /// </summary>
        [JsonProperty("specificationLink", NullValueHandling = NullValueHandling.Ignore)]
        public Uri SpecificationLink { get; set; }

        /// <summary>
        /// Identifier of this detector type.
        /// </summary>
        [JsonProperty("typeId")]
        [JsonConverter(typeof(StickyMinMaxLengthCheckConverter))]
        public string TypeId { get; set; }
    }

    /// <summary>
    /// Vehicle flow count over a measurement period.
    /// </summary>
    public partial class FlowDynamic
    {
        /// <summary>
        /// Per-class vehicle counts in positional order — index 0 is Class1, index 7 is Class8. May
        /// be shorter if trailing classes are absent. A null entry means that class was not
        /// reported, as distinct from a count of zero.
        /// </summary>
        [JsonProperty("classCounts", NullValueHandling = NullValueHandling.Ignore)]
        public long?[] ClassCounts { get; set; }

        /// <summary>
        /// Identifier of the detector this reading is about.
        /// </summary>
        [JsonProperty("detectorId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string DetectorId { get; set; }

        /// <summary>
        /// Vendor or local extensions. Open-content object.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Discriminator for the tagged union. The fixed value "flow" selects the FlowDynamic
        /// payload; it is not free-text command content.
        /// </summary>
        [JsonProperty("kind")]
        public FlowDynamicKind Kind { get; set; }

        /// <summary>
        /// Duration of the measurement period in seconds.
        /// </summary>
        [JsonProperty("periodSeconds", NullValueHandling = NullValueHandling.Ignore)]
        public long? PeriodSeconds { get; set; }

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
        /// Total vehicle count over the period. Omitted when only class counts are reported.
        /// </summary>
        [JsonProperty("totalFlow", NullValueHandling = NullValueHandling.Ignore)]
        public long? TotalFlow { get; set; }
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
        [JsonConverter(typeof(IndigoMinMaxLengthCheckConverter))]
        public string SourceName { get; set; }

        /// <summary>
        /// Free-text description of the source kind.
        /// </summary>
        [JsonProperty("sourceType", NullValueHandling = NullValueHandling.Ignore)]
        public string SourceType { get; set; }
    }

    /// <summary>
    /// Mean inter-vehicle headway over a measurement period.
    /// </summary>
    public partial class HeadwayDynamic
    {
        /// <summary>
        /// Identifier of the detector this reading is about.
        /// </summary>
        [JsonProperty("detectorId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string DetectorId { get; set; }

        /// <summary>
        /// Vendor or local extensions. Open-content object.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Discriminator for the tagged union. The fixed value "headway" selects the HeadwayDynamic
        /// payload; it is not free-text command content.
        /// </summary>
        [JsonProperty("kind")]
        public HeadwayDynamicKind Kind { get; set; }

        /// <summary>
        /// Mean inter-vehicle headway in seconds.
        /// </summary>
        [JsonProperty("meanHeadwaySeconds")]
        [JsonConverter(typeof(StickyMinMaxValueCheckConverter))]
        public double MeanHeadwaySeconds { get; set; }

        /// <summary>
        /// Duration of the measurement period in seconds.
        /// </summary>
        [JsonProperty("periodSeconds", NullValueHandling = NullValueHandling.Ignore)]
        public long? PeriodSeconds { get; set; }

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
    }

    /// <summary>
    /// Detector occupancy (percentage of period occupied).
    /// </summary>
    public partial class OccupancyDynamic
    {
        /// <summary>
        /// Identifier of the detector this reading is about.
        /// </summary>
        [JsonProperty("detectorId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string DetectorId { get; set; }

        /// <summary>
        /// Vendor or local extensions. Open-content object.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Discriminator for the tagged union. The fixed value "occupancy" selects the
        /// OccupancyDynamic payload; it is not free-text command content.
        /// </summary>
        [JsonProperty("kind")]
        public OccupancyDynamicKind Kind { get; set; }

        /// <summary>
        /// Percentage occupancy over the period.
        /// </summary>
        [JsonProperty("occupancyPercent")]
        [JsonConverter(typeof(IndigoMinMaxValueCheckConverter))]
        public double OccupancyPercent { get; set; }

        /// <summary>
        /// Duration of the measurement period in seconds.
        /// </summary>
        [JsonProperty("periodSeconds", NullValueHandling = NullValueHandling.Ignore)]
        public long? PeriodSeconds { get; set; }

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
    }

    /// <summary>
    /// Queue presence and severity reported by the detector.
    /// </summary>
    public partial class QueueDynamic
    {
        /// <summary>
        /// Identifier of the detector this reading is about.
        /// </summary>
        [JsonProperty("detectorId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string DetectorId { get; set; }

        /// <summary>
        /// Vendor or local extensions. Open-content object.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Discriminator for the tagged union. The fixed value "queue" selects the QueueDynamic
        /// payload; it is not free-text command content.
        /// </summary>
        [JsonProperty("kind")]
        public QueueDynamicKind Kind { get; set; }

        /// <summary>
        /// Optional quality statement for this measurement.
        /// </summary>
        [JsonProperty("quality", NullValueHandling = NullValueHandling.Ignore)]
        public Quality Quality { get; set; }

        /// <summary>
        /// Whether a queue is currently detected.
        /// </summary>
        [JsonProperty("queuePresent")]
        public bool QueuePresent { get; set; }

        /// <summary>
        /// Queue severity when a queue is present.
        /// </summary>
        [JsonProperty("severity", NullValueHandling = NullValueHandling.Ignore)]
        public Severity? Severity { get; set; }

        /// <summary>
        /// Time the measurement was taken.
        /// </summary>
        [JsonProperty("timestamp")]
        public DateTimeOffset Timestamp { get; set; }
    }

    /// <summary>
    /// Reset the detector to its default state.
    /// </summary>
    public partial class ResetDetectorCommand
    {
        /// <summary>
        /// Caller-assigned identifier correlating a command request with its response. Not an
        /// identifier of the command itself. Named to avoid collision with the AMQP 1.0
        /// correlation-id message property.
        /// </summary>
        [JsonProperty("commandCorrelationId")]
        public long CommandCorrelationId { get; set; }

        /// <summary>
        /// Identifier of the detector receiving the command.
        /// </summary>
        [JsonProperty("detectorId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string DetectorId { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Discriminator for the tagged union. The fixed value "reset" selects the
        /// ResetDetectorCommand payload; it is not free-text command content.
        /// </summary>
        [JsonProperty("kind")]
        public ResetDetectorCommandKind Kind { get; set; }

        /// <summary>
        /// When the command was issued.
        /// </summary>
        [JsonProperty("requestTime")]
        public DateTimeOffset RequestTime { get; set; }
    }

    /// <summary>
    /// Set occupancy/flow threshold values on the detector.
    /// </summary>
    public partial class SetThresholdDetectorCommand
    {
        /// <summary>
        /// Caller-assigned identifier correlating a command request with its response. Not an
        /// identifier of the command itself. Named to avoid collision with the AMQP 1.0
        /// correlation-id message property.
        /// </summary>
        [JsonProperty("commandCorrelationId")]
        public long CommandCorrelationId { get; set; }

        /// <summary>
        /// Identifier of the detector receiving the command.
        /// </summary>
        [JsonProperty("detectorId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string DetectorId { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Discriminator for the tagged union. The fixed value "setThreshold" selects the
        /// SetThresholdDetectorCommand payload; it is not free-text command content.
        /// </summary>
        [JsonProperty("kind")]
        public SetThresholdDetectorCommandKind Kind { get; set; }

        /// <summary>
        /// When the command was issued.
        /// </summary>
        [JsonProperty("requestTime")]
        public DateTimeOffset RequestTime { get; set; }

        /// <summary>
        /// Downward threshold value to set.
        /// </summary>
        [JsonProperty("thresholdDown")]
        public long ThresholdDown { get; set; }

        /// <summary>
        /// Upward threshold value to set.
        /// </summary>
        [JsonProperty("thresholdUp")]
        public long ThresholdUp { get; set; }
    }

    /// <summary>
    /// Mean vehicle speed over a measurement period.
    /// </summary>
    public partial class SpeedDynamic
    {
        /// <summary>
        /// Identifier of the detector this reading is about.
        /// </summary>
        [JsonProperty("detectorId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string DetectorId { get; set; }

        /// <summary>
        /// Vendor or local extensions. Open-content object.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Discriminator for the tagged union. The fixed value "speed" selects the SpeedDynamic
        /// payload; it is not free-text command content.
        /// </summary>
        [JsonProperty("kind")]
        public SpeedDynamicKind Kind { get; set; }

        /// <summary>
        /// Mean vehicle speed in kilometres per hour.
        /// </summary>
        [JsonProperty("meanSpeedKph")]
        [JsonConverter(typeof(StickyMinMaxValueCheckConverter))]
        public double MeanSpeedKph { get; set; }

        /// <summary>
        /// Duration of the measurement period in seconds.
        /// </summary>
        [JsonProperty("periodSeconds", NullValueHandling = NullValueHandling.Ignore)]
        public long? PeriodSeconds { get; set; }

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
    }

    /// <summary>
    /// Terminal message ending a detector sync batch. Sent to the requester's reply node after
    /// the last object, or instead of any objects when the filter matched none.
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
        [JsonConverter(typeof(IndigoMinMaxLengthCheckConverter))]
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
        [JsonConverter(typeof(IndecentMinMaxLengthCheckConverter))]
        public string SyncId { get; set; }
    }

    /// <summary>
    /// Request the current state of detector objects. The Data Publisher answers to the
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
        [JsonConverter(typeof(TentacledMinMaxLengthCheckConverter))]
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
        [JsonConverter(typeof(StickyMinMaxLengthCheckConverter))]
        public string TypeId { get; set; }
    }

    public enum CalibrateDetectorCommandKind { Calibrate };

    /// <summary>
    /// Outcome of the command execution.
    ///
    /// Outcome of a command execution.
    ///
    /// Every value except ``PENDING`` is terminal. Consumers SHOULD test for
    /// "not pending" rather than enumerating the terminal values, so that a
    /// future profile adding one does not strand them.
    /// </summary>
    public enum DetectorCommandResponseStatus { Failed, Invalid, NotSupported, Pending, Success, TimedOut };

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
    public enum DetectorFaultStatus { Active, Cleared };

    public enum FlowDynamicKind { Flow };

    public enum HeadwayDynamicKind { Headway };

    public enum OccupancyDynamicKind { Occupancy };

    public enum QueueDynamicKind { Queue };

    /// <summary>
    /// Queue severity when a queue is present.
    /// </summary>
    public enum Severity { Congested, Delayed, Dense, Normal, Severe, StopAndGo };

    public enum ResetDetectorCommandKind { Reset };

    public enum SetThresholdDetectorCommandKind { SetThreshold };

    public enum SpeedDynamicKind { Speed };

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

    internal static class Converter
    {
        public static readonly JsonSerializerSettings Settings = new JsonSerializerSettings
        {
            MetadataPropertyHandling = MetadataPropertyHandling.Ignore,
            DateParseHandling = DateParseHandling.None,
            Converters =
            {
                CalibrateDetectorCommandKindConverter.Singleton,
                DetectorCommandResponseStatusConverter.Singleton,
                AcknowledgementStateConverter.Singleton,
                DetectorFaultStatusConverter.Singleton,
                FlowDynamicKindConverter.Singleton,
                HeadwayDynamicKindConverter.Singleton,
                OccupancyDynamicKindConverter.Singleton,
                QueueDynamicKindConverter.Singleton,
                SeverityConverter.Singleton,
                ResetDetectorCommandKindConverter.Singleton,
                SetThresholdDetectorCommandKindConverter.Singleton,
                SpeedDynamicKindConverter.Singleton,
                SyncCategoryEnumConverter.Singleton,
                SyncStatusEnumConverter.Singleton,
                new IsoDateTimeConverter { DateTimeStyles = DateTimeStyles.AssumeUniversal }
            },
        };
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

    internal class CalibrateDetectorCommandKindConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(CalibrateDetectorCommandKind) || t == typeof(CalibrateDetectorCommandKind?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            if (value == "calibrate")
            {
                return CalibrateDetectorCommandKind.Calibrate;
            }
            throw new Exception("Cannot unmarshal type CalibrateDetectorCommandKind");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (CalibrateDetectorCommandKind)untypedValue;
            if (value == CalibrateDetectorCommandKind.Calibrate)
            {
                serializer.Serialize(writer, "calibrate");
                return;
            }
            throw new Exception("Cannot marshal type CalibrateDetectorCommandKind");
        }

        public static readonly CalibrateDetectorCommandKindConverter Singleton = new CalibrateDetectorCommandKindConverter();
    }

    internal class DetectorCommandResponseStatusConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(DetectorCommandResponseStatus) || t == typeof(DetectorCommandResponseStatus?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "failed":
                    return DetectorCommandResponseStatus.Failed;
                case "invalid":
                    return DetectorCommandResponseStatus.Invalid;
                case "notSupported":
                    return DetectorCommandResponseStatus.NotSupported;
                case "pending":
                    return DetectorCommandResponseStatus.Pending;
                case "success":
                    return DetectorCommandResponseStatus.Success;
                case "timedOut":
                    return DetectorCommandResponseStatus.TimedOut;
            }
            throw new Exception("Cannot unmarshal type DetectorCommandResponseStatus");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (DetectorCommandResponseStatus)untypedValue;
            switch (value)
            {
                case DetectorCommandResponseStatus.Failed:
                    serializer.Serialize(writer, "failed");
                    return;
                case DetectorCommandResponseStatus.Invalid:
                    serializer.Serialize(writer, "invalid");
                    return;
                case DetectorCommandResponseStatus.NotSupported:
                    serializer.Serialize(writer, "notSupported");
                    return;
                case DetectorCommandResponseStatus.Pending:
                    serializer.Serialize(writer, "pending");
                    return;
                case DetectorCommandResponseStatus.Success:
                    serializer.Serialize(writer, "success");
                    return;
                case DetectorCommandResponseStatus.TimedOut:
                    serializer.Serialize(writer, "timedOut");
                    return;
            }
            throw new Exception("Cannot marshal type DetectorCommandResponseStatus");
        }

        public static readonly DetectorCommandResponseStatusConverter Singleton = new DetectorCommandResponseStatusConverter();
    }

    internal class PurpleMinMaxValueCheckConverter : JsonConverter
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

        public static readonly PurpleMinMaxValueCheckConverter Singleton = new PurpleMinMaxValueCheckConverter();
    }

    internal class FluffyMinMaxValueCheckConverter : JsonConverter
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

        public static readonly FluffyMinMaxValueCheckConverter Singleton = new FluffyMinMaxValueCheckConverter();
    }

    internal class TentacledMinMaxValueCheckConverter : JsonConverter
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

        public static readonly TentacledMinMaxValueCheckConverter Singleton = new TentacledMinMaxValueCheckConverter();
    }

    internal class FluffyMinMaxLengthCheckConverter : JsonConverter
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

        public static readonly FluffyMinMaxLengthCheckConverter Singleton = new FluffyMinMaxLengthCheckConverter();
    }

    internal class TentacledMinMaxLengthCheckConverter : JsonConverter
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

        public static readonly TentacledMinMaxLengthCheckConverter Singleton = new TentacledMinMaxLengthCheckConverter();
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

    internal class DetectorFaultStatusConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(DetectorFaultStatus) || t == typeof(DetectorFaultStatus?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "active":
                    return DetectorFaultStatus.Active;
                case "cleared":
                    return DetectorFaultStatus.Cleared;
            }
            throw new Exception("Cannot unmarshal type DetectorFaultStatus");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (DetectorFaultStatus)untypedValue;
            switch (value)
            {
                case DetectorFaultStatus.Active:
                    serializer.Serialize(writer, "active");
                    return;
                case DetectorFaultStatus.Cleared:
                    serializer.Serialize(writer, "cleared");
                    return;
            }
            throw new Exception("Cannot marshal type DetectorFaultStatus");
        }

        public static readonly DetectorFaultStatusConverter Singleton = new DetectorFaultStatusConverter();
    }

    internal class FlowDynamicKindConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(FlowDynamicKind) || t == typeof(FlowDynamicKind?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            if (value == "flow")
            {
                return FlowDynamicKind.Flow;
            }
            throw new Exception("Cannot unmarshal type FlowDynamicKind");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (FlowDynamicKind)untypedValue;
            if (value == FlowDynamicKind.Flow)
            {
                serializer.Serialize(writer, "flow");
                return;
            }
            throw new Exception("Cannot marshal type FlowDynamicKind");
        }

        public static readonly FlowDynamicKindConverter Singleton = new FlowDynamicKindConverter();
    }

    internal class IndigoMinMaxLengthCheckConverter : JsonConverter
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

        public static readonly IndigoMinMaxLengthCheckConverter Singleton = new IndigoMinMaxLengthCheckConverter();
    }

    internal class HeadwayDynamicKindConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(HeadwayDynamicKind) || t == typeof(HeadwayDynamicKind?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            if (value == "headway")
            {
                return HeadwayDynamicKind.Headway;
            }
            throw new Exception("Cannot unmarshal type HeadwayDynamicKind");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (HeadwayDynamicKind)untypedValue;
            if (value == HeadwayDynamicKind.Headway)
            {
                serializer.Serialize(writer, "headway");
                return;
            }
            throw new Exception("Cannot marshal type HeadwayDynamicKind");
        }

        public static readonly HeadwayDynamicKindConverter Singleton = new HeadwayDynamicKindConverter();
    }

    internal class StickyMinMaxValueCheckConverter : JsonConverter
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

        public static readonly StickyMinMaxValueCheckConverter Singleton = new StickyMinMaxValueCheckConverter();
    }

    internal class OccupancyDynamicKindConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(OccupancyDynamicKind) || t == typeof(OccupancyDynamicKind?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            if (value == "occupancy")
            {
                return OccupancyDynamicKind.Occupancy;
            }
            throw new Exception("Cannot unmarshal type OccupancyDynamicKind");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (OccupancyDynamicKind)untypedValue;
            if (value == OccupancyDynamicKind.Occupancy)
            {
                serializer.Serialize(writer, "occupancy");
                return;
            }
            throw new Exception("Cannot marshal type OccupancyDynamicKind");
        }

        public static readonly OccupancyDynamicKindConverter Singleton = new OccupancyDynamicKindConverter();
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

    internal class QueueDynamicKindConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(QueueDynamicKind) || t == typeof(QueueDynamicKind?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            if (value == "queue")
            {
                return QueueDynamicKind.Queue;
            }
            throw new Exception("Cannot unmarshal type QueueDynamicKind");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (QueueDynamicKind)untypedValue;
            if (value == QueueDynamicKind.Queue)
            {
                serializer.Serialize(writer, "queue");
                return;
            }
            throw new Exception("Cannot marshal type QueueDynamicKind");
        }

        public static readonly QueueDynamicKindConverter Singleton = new QueueDynamicKindConverter();
    }

    internal class SeverityConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(Severity) || t == typeof(Severity?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "congested":
                    return Severity.Congested;
                case "delayed":
                    return Severity.Delayed;
                case "dense":
                    return Severity.Dense;
                case "normal":
                    return Severity.Normal;
                case "severe":
                    return Severity.Severe;
                case "stopAndGo":
                    return Severity.StopAndGo;
            }
            throw new Exception("Cannot unmarshal type Severity");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (Severity)untypedValue;
            switch (value)
            {
                case Severity.Congested:
                    serializer.Serialize(writer, "congested");
                    return;
                case Severity.Delayed:
                    serializer.Serialize(writer, "delayed");
                    return;
                case Severity.Dense:
                    serializer.Serialize(writer, "dense");
                    return;
                case Severity.Normal:
                    serializer.Serialize(writer, "normal");
                    return;
                case Severity.Severe:
                    serializer.Serialize(writer, "severe");
                    return;
                case Severity.StopAndGo:
                    serializer.Serialize(writer, "stopAndGo");
                    return;
            }
            throw new Exception("Cannot marshal type Severity");
        }

        public static readonly SeverityConverter Singleton = new SeverityConverter();
    }

    internal class ResetDetectorCommandKindConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(ResetDetectorCommandKind) || t == typeof(ResetDetectorCommandKind?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            if (value == "reset")
            {
                return ResetDetectorCommandKind.Reset;
            }
            throw new Exception("Cannot unmarshal type ResetDetectorCommandKind");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (ResetDetectorCommandKind)untypedValue;
            if (value == ResetDetectorCommandKind.Reset)
            {
                serializer.Serialize(writer, "reset");
                return;
            }
            throw new Exception("Cannot marshal type ResetDetectorCommandKind");
        }

        public static readonly ResetDetectorCommandKindConverter Singleton = new ResetDetectorCommandKindConverter();
    }

    internal class SetThresholdDetectorCommandKindConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(SetThresholdDetectorCommandKind) || t == typeof(SetThresholdDetectorCommandKind?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            if (value == "setThreshold")
            {
                return SetThresholdDetectorCommandKind.SetThreshold;
            }
            throw new Exception("Cannot unmarshal type SetThresholdDetectorCommandKind");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (SetThresholdDetectorCommandKind)untypedValue;
            if (value == SetThresholdDetectorCommandKind.SetThreshold)
            {
                serializer.Serialize(writer, "setThreshold");
                return;
            }
            throw new Exception("Cannot marshal type SetThresholdDetectorCommandKind");
        }

        public static readonly SetThresholdDetectorCommandKindConverter Singleton = new SetThresholdDetectorCommandKindConverter();
    }

    internal class SpeedDynamicKindConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(SpeedDynamicKind) || t == typeof(SpeedDynamicKind?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            if (value == "speed")
            {
                return SpeedDynamicKind.Speed;
            }
            throw new Exception("Cannot unmarshal type SpeedDynamicKind");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (SpeedDynamicKind)untypedValue;
            if (value == SpeedDynamicKind.Speed)
            {
                serializer.Serialize(writer, "speed");
                return;
            }
            throw new Exception("Cannot marshal type SpeedDynamicKind");
        }

        public static readonly SpeedDynamicKindConverter Singleton = new SpeedDynamicKindConverter();
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

    internal class IndecentMinMaxLengthCheckConverter : JsonConverter
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

        public static readonly IndecentMinMaxLengthCheckConverter Singleton = new IndecentMinMaxLengthCheckConverter();
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
}

namespace Topas.Utmc.Detector
{
    using System;
    using Newtonsoft.Json;
    using Newtonsoft.Json.Linq;

    /// <summary>
    /// Detector command payloads.
    /// </summary>
    /// <remarks>
    /// Tagged union over <c>kind</c>. Deserialise as this interface to
    /// have the converter select the variant; the variant classes are
    /// generated by quicktype and adopt the interface via partial
    /// declarations below.
    /// </remarks>
    [JsonConverter(typeof(DetectorCommandConverter))]
    public interface IDetectorCommand { }

    public partial class CalibrateDetectorCommand : IDetectorCommand { }
    public partial class ResetDetectorCommand : IDetectorCommand { }
    public partial class SetThresholdDetectorCommand : IDetectorCommand { }

    internal class DetectorCommandConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(IDetectorCommand);

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
                case "calibrate": return Read<CalibrateDetectorCommand>(obj, serializer);
                case "reset": return Read<ResetDetectorCommand>(obj, serializer);
                case "setThreshold": return Read<SetThresholdDetectorCommand>(obj, serializer);
                default:
                    throw new JsonSerializationException(
                        $"unknown kind '{tag}' for DetectorCommand");
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

namespace Topas.Utmc.Detector
{
    using System;
    using Newtonsoft.Json;
    using Newtonsoft.Json.Linq;

    /// <summary>
    /// Detector dynamic measurement data.
    /// </summary>
    /// <remarks>
    /// Tagged union over <c>kind</c>. Deserialise as this interface to
    /// have the converter select the variant; the variant classes are
    /// generated by quicktype and adopt the interface via partial
    /// declarations below.
    /// </remarks>
    [JsonConverter(typeof(DetectorMeasurementConverter))]
    public interface IDetectorMeasurement { }

    public partial class FlowDynamic : IDetectorMeasurement { }
    public partial class HeadwayDynamic : IDetectorMeasurement { }
    public partial class OccupancyDynamic : IDetectorMeasurement { }
    public partial class QueueDynamic : IDetectorMeasurement { }
    public partial class SpeedDynamic : IDetectorMeasurement { }

    internal class DetectorMeasurementConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(IDetectorMeasurement);

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
                case "flow": return Read<FlowDynamic>(obj, serializer);
                case "headway": return Read<HeadwayDynamic>(obj, serializer);
                case "occupancy": return Read<OccupancyDynamic>(obj, serializer);
                case "queue": return Read<QueueDynamic>(obj, serializer);
                case "speed": return Read<SpeedDynamic>(obj, serializer);
                default:
                    throw new JsonSerializationException(
                        $"unknown kind '{tag}' for DetectorMeasurement");
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

namespace Topas.Utmc.Detector
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
