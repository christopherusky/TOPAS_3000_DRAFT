/*
 * File:        output/csharp/sdk/TrafficSignal.cs
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
namespace Topas.Utmc.TrafficSignal
{
    using System;
    using System.Collections.Generic;

    using System.Globalization;
    using Newtonsoft.Json;
    using Newtonsoft.Json.Converters;

    public partial class AllTypes
    {
        [JsonProperty("FaultSeverityEnum", NullValueHandling = NullValueHandling.Ignore)]
        public FaultSeverityEnum? FaultSeverityEnum { get; set; }

        [JsonProperty("ForceStageCommand", NullValueHandling = NullValueHandling.Ignore)]
        public ForceStageCommand ForceStageCommand { get; set; }

        [JsonProperty("HoldStageCommand", NullValueHandling = NullValueHandling.Ignore)]
        public HoldStageCommand HoldStageCommand { get; set; }

        [JsonProperty("PlanLine", NullValueHandling = NullValueHandling.Ignore)]
        public PlanLine PlanLine { get; set; }

        [JsonProperty("RequestStatusCommand", NullValueHandling = NullValueHandling.Ignore)]
        public RequestStatusCommand RequestStatusCommand { get; set; }

        [JsonProperty("SetPlanCommand", NullValueHandling = NullValueHandling.Ignore)]
        public SetPlanCommand SetPlanCommand { get; set; }

        [JsonProperty("SignalGroupTimings", NullValueHandling = NullValueHandling.Ignore)]
        public Composite SignalGroupTimings { get; set; }

        [JsonProperty("SyncCategoryEnum", NullValueHandling = NullValueHandling.Ignore)]
        public SyncCategoryEnum? SyncCategoryEnum { get; set; }

        [JsonProperty("SyncComplete", NullValueHandling = NullValueHandling.Ignore)]
        public SyncComplete SyncComplete { get; set; }

        [JsonProperty("SynchronizeClockCommand", NullValueHandling = NullValueHandling.Ignore)]
        public SynchronizeClockCommand SynchronizeClockCommand { get; set; }

        [JsonProperty("SyncRequest", NullValueHandling = NullValueHandling.Ignore)]
        public SyncRequest SyncRequest { get; set; }

        [JsonProperty("SyncStatus", NullValueHandling = NullValueHandling.Ignore)]
        public SyncStatusEnum? SyncStatus { get; set; }

        [JsonProperty("TrafficSignalCommandResponse", NullValueHandling = NullValueHandling.Ignore)]
        public TrafficSignalCommandResponse TrafficSignalCommandResponse { get; set; }

        [JsonProperty("TrafficSignalConfiguration", NullValueHandling = NullValueHandling.Ignore)]
        public TrafficSignalConfiguration TrafficSignalConfiguration { get; set; }

        [JsonProperty("TrafficSignalDefinition", NullValueHandling = NullValueHandling.Ignore)]
        public TrafficSignalDefinition TrafficSignalDefinition { get; set; }

        [JsonProperty("TrafficSignalDynamicState", NullValueHandling = NullValueHandling.Ignore)]
        public TrafficSignalDynamicState TrafficSignalDynamicState { get; set; }

        [JsonProperty("TrafficSignalFault", NullValueHandling = NullValueHandling.Ignore)]
        public TrafficSignalFault TrafficSignalFault { get; set; }

        [JsonProperty("TrafficSignalFaultEnum", NullValueHandling = NullValueHandling.Ignore)]
        public TrafficSignalFaultType? TrafficSignalFaultEnum { get; set; }

        [JsonProperty("TrafficSignalHistoryEntry", NullValueHandling = NullValueHandling.Ignore)]
        public TrafficSignalHistoryEntry TrafficSignalHistoryEntry { get; set; }

        [JsonProperty("TrafficSignalModeEnum", NullValueHandling = NullValueHandling.Ignore)]
        public TrafficSignalMode? TrafficSignalModeEnum { get; set; }

        [JsonProperty("TrafficSignalSubModeEnum", NullValueHandling = NullValueHandling.Ignore)]
        public TrafficSignalSubMode? TrafficSignalSubModeEnum { get; set; }

        [JsonProperty("TrafficSignalType", NullValueHandling = NullValueHandling.Ignore)]
        public TrafficSignalTypeClass TrafficSignalType { get; set; }

        [JsonProperty("TrafficSignalTypeEnum", NullValueHandling = NullValueHandling.Ignore)]
        public TrafficSignalTypeEnumEnum? TrafficSignalTypeEnum { get; set; }

        [JsonProperty("UrgencyEnum", NullValueHandling = NullValueHandling.Ignore)]
        public TrafficSignalFaultUrgency? UrgencyEnum { get; set; }
    }

    /// <summary>
    /// Force a specific stage on a traffic signal controller.
    /// </summary>
    public partial class ForceStageCommand
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
        /// Discriminator for the tagged union. The fixed value "forceStage" selects the
        /// ForceStageCommand payload; it is not free-text command content.
        /// </summary>
        [JsonProperty("kind")]
        public ForceStageCommandKind Kind { get; set; }

        /// <summary>
        /// When the command was issued.
        /// </summary>
        [JsonProperty("requestTime")]
        public DateTimeOffset RequestTime { get; set; }

        /// <summary>
        /// Stage number to force.
        /// </summary>
        [JsonProperty("stageNumber")]
        public long StageNumber { get; set; }

        /// <summary>
        /// Identifier of the traffic signal receiving the command.
        /// </summary>
        [JsonProperty("trafficSignalId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string TrafficSignalId { get; set; }
    }

    /// <summary>
    /// Hold a specific stage on a traffic signal controller.
    /// </summary>
    public partial class HoldStageCommand
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
        /// Discriminator for the tagged union. The fixed value "holdStage" selects the
        /// HoldStageCommand payload; it is not free-text command content.
        /// </summary>
        [JsonProperty("kind")]
        public HoldStageCommandKind Kind { get; set; }

        /// <summary>
        /// When the command was issued.
        /// </summary>
        [JsonProperty("requestTime")]
        public DateTimeOffset RequestTime { get; set; }

        /// <summary>
        /// Stage number to hold.
        /// </summary>
        [JsonProperty("stageNumber")]
        public long StageNumber { get; set; }

        /// <summary>
        /// Identifier of the traffic signal receiving the command.
        /// </summary>
        [JsonProperty("trafficSignalId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string TrafficSignalId { get; set; }
    }

    /// <summary>
    /// A signal plan definition carried in the dynamic state (§3.5, §3.6).
    ///
    /// A plan line for this traffic signal. This is intended to provide information allowing the
    /// plan data to be displayed to an operator. It contains the current plan data and would be
    /// updated when the plan is changed. For adaptive central control, such as SCOOT, the
    /// information should be available periodically and contain the plan at that instant.
    /// </summary>
    public partial class PlanLine
    {
        /// <summary>
        /// The current cycle time.
        /// </summary>
        [JsonProperty("cycleTime", NullValueHandling = NullValueHandling.Ignore)]
        public long? CycleTime { get; set; }

        /// <summary>
        /// Vendor or forward-compatibility extensions. Open-content object.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// The offset to the first SignalGroupTiming in the list. Must be present if
        /// SignalGroupTimingList is present.
        /// </summary>
        [JsonProperty("offset", NullValueHandling = NullValueHandling.Ignore)]
        public long? Offset { get; set; }

        /// <summary>
        /// A user entered description of the active plan.
        /// </summary>
        [JsonProperty("planDescription", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(FluffyMinMaxLengthCheckConverter))]
        public string PlanDescription { get; set; }

        /// <summary>
        /// The currently active plan.
        /// </summary>
        [JsonProperty("planNumber")]
        public long PlanNumber { get; set; }

        /// <summary>
        /// The time the plan was started. Optional: a publisher emitting on a fixed interval may not
        /// know the start time of the running plan.
        /// </summary>
        [JsonProperty("planStartTime", NullValueHandling = NullValueHandling.Ignore)]
        public DateTimeOffset? PlanStartTime { get; set; }

        /// <summary>
        /// The signal group data for the plan.
        /// </summary>
        [JsonProperty("signalGroupTimings", NullValueHandling = NullValueHandling.Ignore)]
        public Composite[] SignalGroupTimings { get; set; }
    }

    /// <summary>
    /// Signal-group timing row within a PlanLine (§3.5, §3.6).
    /// </summary>
    public partial class Composite
    {
        /// <summary>
        /// A text representation of the alternatives to the primary signal group.
        /// </summary>
        [JsonProperty("alternateSignalGroup", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(TentacledMinMaxLengthCheckConverter))]
        public string AlternateSignalGroup { get; set; }

        /// <summary>
        /// Vendor or forward-compatibility extensions. Open-content object.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// A text representation of the primary signal group.
        /// </summary>
        [JsonProperty("primarySignalGroup")]
        [JsonConverter(typeof(TentacledMinMaxLengthCheckConverter))]
        public string PrimarySignalGroup { get; set; }

        /// <summary>
        /// The length of the primary signal group. Note that there is no timing data associated with
        /// the alternate signal group.
        /// </summary>
        [JsonProperty("primarySignalGroupDuration")]
        public long PrimarySignalGroupDuration { get; set; }
    }

    /// <summary>
    /// Request the current status of a traffic signal controller.
    /// </summary>
    public partial class RequestStatusCommand
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
        /// RequestStatusCommand payload; it is not free-text command content.
        /// </summary>
        [JsonProperty("kind")]
        public RequestStatusCommandKind Kind { get; set; }

        /// <summary>
        /// When the command was issued.
        /// </summary>
        [JsonProperty("requestTime")]
        public DateTimeOffset RequestTime { get; set; }

        /// <summary>
        /// Identifier of the traffic signal receiving the command.
        /// </summary>
        [JsonProperty("trafficSignalId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string TrafficSignalId { get; set; }
    }

    /// <summary>
    /// Set the active plan on a traffic signal controller.
    /// </summary>
    public partial class SetPlanCommand
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
        /// Discriminator for the tagged union. The fixed value "setPlan" selects the SetPlanCommand
        /// payload; it is not free-text command content.
        /// </summary>
        [JsonProperty("kind")]
        public SetPlanCommandKind Kind { get; set; }

        /// <summary>
        /// Plan number to activate. Same 1 to 99 range as the plan number a controller reports
        /// (PlanLine.plan_number), so any plan the controller can be told to run can be reported
        /// back.
        /// </summary>
        [JsonProperty("planNumber")]
        public long PlanNumber { get; set; }

        /// <summary>
        /// When the command was issued.
        /// </summary>
        [JsonProperty("requestTime")]
        public DateTimeOffset RequestTime { get; set; }

        /// <summary>
        /// Identifier of the traffic signal receiving the command.
        /// </summary>
        [JsonProperty("trafficSignalId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string TrafficSignalId { get; set; }
    }

    /// <summary>
    /// Terminal message ending a traffic signal sync batch. Sent to the requester's reply node
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
        [JsonConverter(typeof(StickyMinMaxLengthCheckConverter))]
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
        [JsonConverter(typeof(IndigoMinMaxLengthCheckConverter))]
        public string SyncId { get; set; }
    }

    /// <summary>
    /// Request the current state of traffic signal objects. The Data Publisher answers to the
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
        [JsonConverter(typeof(IndecentMinMaxLengthCheckConverter))]
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
        [JsonConverter(typeof(HilariousMinMaxLengthCheckConverter))]
        public string TypeId { get; set; }
    }

    /// <summary>
    /// Synchronize the traffic signal controller's clock.
    /// </summary>
    public partial class SynchronizeClockCommand
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
        /// Discriminator for the tagged union. The fixed value "synchronizeClock" selects the
        /// SynchronizeClockCommand payload; it is not free-text command content.
        /// </summary>
        [JsonProperty("kind")]
        public SynchronizeClockCommandKind Kind { get; set; }

        /// <summary>
        /// When the command was issued.
        /// </summary>
        [JsonProperty("requestTime")]
        public DateTimeOffset RequestTime { get; set; }

        /// <summary>
        /// Target time to synchronize the controller clock to.
        /// </summary>
        [JsonProperty("targetTime")]
        public DateTimeOffset TargetTime { get; set; }

        /// <summary>
        /// Identifier of the traffic signal receiving the command.
        /// </summary>
        [JsonProperty("trafficSignalId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string TrafficSignalId { get; set; }
    }

    /// <summary>
    /// Traffic signal command response (reused from TS004.0061:2010 Annex D).
    /// </summary>
    public partial class TrafficSignalCommandResponse
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
        public TrafficSignalCommandResponseStatus Status { get; set; }

        /// <summary>
        /// Identifier of the traffic signal the command targeted.
        /// </summary>
        [JsonProperty("trafficSignalId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string TrafficSignalId { get; set; }
    }

    /// <summary>
    /// Traffic signal operational configuration (TS004.0064bis:2017 §3.4).
    /// </summary>
    public partial class TrafficSignalConfiguration
    {
        /// <summary>
        /// When this configuration record became effective.
        /// </summary>
        [JsonProperty("configurationDate", NullValueHandling = NullValueHandling.Ignore)]
        public DateTimeOffset? ConfigurationDate { get; set; }

        /// <summary>
        /// Textual description of controller type.
        /// </summary>
        [JsonProperty("controllerType", NullValueHandling = NullValueHandling.Ignore)]
        public string ControllerType { get; set; }

        /// <summary>
        /// Vendor or forward-compatibility extensions. Open-content object.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Group of signals to which this traffic signal belongs.
        /// </summary>
        [JsonProperty("subArea", NullValueHandling = NullValueHandling.Ignore)]
        public string SubArea { get; set; }

        /// <summary>
        /// Name of supplier.
        /// </summary>
        [JsonProperty("supplier", NullValueHandling = NullValueHandling.Ignore)]
        public string Supplier { get; set; }

        /// <summary>
        /// Identifier of the traffic signal this configuration applies to.
        /// </summary>
        [JsonProperty("trafficSignalId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string TrafficSignalId { get; set; }

        /// <summary>
        /// Type of the traffic signal.
        /// </summary>
        [JsonProperty("trafficSignalType", NullValueHandling = NullValueHandling.Ignore)]
        public TrafficSignalTypeEnumEnum? TrafficSignalType { get; set; }

        /// <summary>
        /// Operator's own reference for this traffic signal.
        /// </summary>
        [JsonProperty("userCodeNumber", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(AmbitiousMinMaxLengthCheckConverter))]
        public string UserCodeNumber { get; set; }
    }

    /// <summary>
    /// Traffic signal equipment details.
    /// </summary>
    public partial class TrafficSignalDefinition
    {
        /// <summary>
        /// Optional reference into the TrafficSignalType catalogue identifying the operational
        /// control method (MOVA, SCOOT, fixed-time, ...). Orthogonal to
        /// TrafficSignalConfiguration.trafficSignalType, which classifies the junction kind
        /// (intersection, pelican, puffin, ...).
        /// </summary>
        [JsonProperty("controlMethodId", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(HilariousMinMaxLengthCheckConverter))]
        public string ControlMethodId { get; set; }

        /// <summary>
        /// When this definition record was created.
        /// </summary>
        [JsonProperty("creationDate", NullValueHandling = NullValueHandling.Ignore)]
        public DateTimeOffset? CreationDate { get; set; }

        /// <summary>
        /// Vendor or forward-compatibility extensions. Open-content object.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Location of traffic signal.
        /// </summary>
        [JsonProperty("location", NullValueHandling = NullValueHandling.Ignore)]
        public Location Location { get; set; }

        /// <summary>
        /// Longer description (context, commissioning notes, etc.).
        /// </summary>
        [JsonProperty("longDescription", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(CunningMinMaxLengthCheckConverter))]
        public string LongDescription { get; set; }

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
        /// System that owns this traffic signal definition. Often implicit from context.
        /// </summary>
        [JsonProperty("participantId", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string ParticipantId { get; set; }

        /// <summary>
        /// Short human-readable name (e.g. 'Main St / High St junction').
        /// </summary>
        [JsonProperty("shortDescription", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(IndecentMinMaxLengthCheckConverter))]
        public string ShortDescription { get; set; }

        /// <summary>
        /// Human readable identifier for the Traffic Signal.
        /// </summary>
        [JsonProperty("trafficSignalId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string TrafficSignalId { get; set; }
    }

    /// <summary>
    /// Location of traffic signal.
    ///
    /// A GeoJSON Point geometry (RFC 7946).
    ///
    /// Coordinates are [longitude, latitude] per GeoJSON specification.
    /// </summary>
    public partial class Location
    {
        /// <summary>
        /// [longitude, latitude] in WGS84 decimal degrees.
        /// </summary>
        [JsonProperty("coordinates")]
        public object[] Coordinates { get; set; }

        [JsonProperty("type", NullValueHandling = NullValueHandling.Ignore)]
        public TypeEnum? Type { get; set; }
    }

    /// <summary>
    /// Traffic signal dynamic state with PlanLine/SignalGroupTimings (TS004.0064bis:2017 §3.5).
    /// </summary>
    public partial class TrafficSignalDynamicState
    {
        /// <summary>
        /// Vendor or forward-compatibility extensions. Open-content object.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// A plan line for this traffic signal. This is intended to provide information allowing the
        /// plan data to be displayed to an operator. It contains the current plan data and would be
        /// updated when the plan is changed. For adaptive central control, such as SCOOT, the
        /// information should be available periodically and contain the plan at that instant.
        /// </summary>
        [JsonProperty("planLine", NullValueHandling = NullValueHandling.Ignore)]
        public PlanLine PlanLine { get; set; }

        /// <summary>
        /// Name for the sub-area that identifies the group of signals to which the associated
        /// traffic signal equipment belongs.
        /// </summary>
        [JsonProperty("subArea", NullValueHandling = NullValueHandling.Ignore)]
        public string SubArea { get; set; }

        /// <summary>
        /// Time the state was observed.
        /// </summary>
        [JsonProperty("timestamp")]
        public DateTimeOffset Timestamp { get; set; }

        /// <summary>
        /// Identifier of the traffic signal this reading is about.
        /// </summary>
        [JsonProperty("trafficSignalId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string TrafficSignalId { get; set; }

        /// <summary>
        /// The high level mode of the traffic signal.
        /// </summary>
        [JsonProperty("trafficSignalMode")]
        public TrafficSignalMode TrafficSignalMode { get; set; }

        /// <summary>
        /// The detailed sub-mode of the traffic signal.
        /// </summary>
        [JsonProperty("trafficSignalSubMode", NullValueHandling = NullValueHandling.Ignore)]
        public TrafficSignalSubMode? TrafficSignalSubMode { get; set; }
    }

    /// <summary>
    /// Traffic signal fault reports with flattened DATEX II Fault base (TS004.0064bis:2017 §3.3,
    /// F6).
    /// </summary>
    public partial class TrafficSignalFault
    {
        /// <summary>
        /// Vendor or forward-compatibility extensions. Open-content object.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Time the fault was first recorded.
        /// </summary>
        [JsonProperty("faultCreationTime", NullValueHandling = NullValueHandling.Ignore)]
        public DateTimeOffset? FaultCreationTime { get; set; }

        /// <summary>
        /// Human-readable fault description.
        /// </summary>
        [JsonProperty("faultDescription", NullValueHandling = NullValueHandling.Ignore)]
        public string FaultDescription { get; set; }

        /// <summary>
        /// Unique identifier for this fault.
        /// </summary>
        [JsonProperty("faultIdentifier", NullValueHandling = NullValueHandling.Ignore)]
        public string FaultIdentifier { get; set; }

        /// <summary>
        /// Time fault was last updated.
        /// </summary>
        [JsonProperty("faultLastUpdateTime")]
        public DateTimeOffset FaultLastUpdateTime { get; set; }

        /// <summary>
        /// Severity of the fault.
        /// </summary>
        [JsonProperty("faultSeverity", NullValueHandling = NullValueHandling.Ignore)]
        public FaultSeverityEnum? FaultSeverity { get; set; }

        /// <summary>
        /// Reference to a FaultTypeDefinition entry. The catalogue carries the code, category and
        /// message template; new fault types are added as catalogue entries, with no schema change.
        /// Required unless trafficSignalFaultType is given; when both are present this is the
        /// authoritative classification.
        /// </summary>
        [JsonProperty("faultTypeId", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(HilariousMinMaxLengthCheckConverter))]
        public string FaultTypeId { get; set; }

        /// <summary>
        /// Values for the positional placeholders in the referenced FaultTypeDefinition's
        /// messageTemplate, in order. Rendered as text so a value carries whatever form the fault
        /// reports it in.
        /// </summary>
        [JsonProperty("parameters", NullValueHandling = NullValueHandling.Ignore)]
        public string[] Parameters { get; set; }

        /// <summary>
        /// Time fault was cleared.
        /// </summary>
        [JsonProperty("trafficSignalFaultClearanceTime", NullValueHandling = NullValueHandling.Ignore)]
        public DateTimeOffset? TrafficSignalFaultClearanceTime { get; set; }

        /// <summary>
        /// Coarse DATEX II fault bucket, as in TS004.0064bis:2017. Required unless faultTypeId is
        /// given; when both are present faultTypeId is the authoritative classification.
        /// </summary>
        [JsonProperty("trafficSignalFaultType", NullValueHandling = NullValueHandling.Ignore)]
        public TrafficSignalFaultType? TrafficSignalFaultType { get; set; }

        /// <summary>
        /// The urgency of this fault.
        /// </summary>
        [JsonProperty("trafficSignalFaultUrgency", NullValueHandling = NullValueHandling.Ignore)]
        public TrafficSignalFaultUrgency? TrafficSignalFaultUrgency { get; set; }

        /// <summary>
        /// Identifier of the traffic signal this fault applies to.
        /// </summary>
        [JsonProperty("trafficSignalId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string TrafficSignalId { get; set; }

        /// <summary>
        /// A supplier unique identifier for this specific fault.
        /// </summary>
        [JsonProperty("trafficSignalManufacturerFaultCode", NullValueHandling = NullValueHandling.Ignore)]
        public string TrafficSignalManufacturerFaultCode { get; set; }
    }

    /// <summary>
    /// Traffic signal history entries (reused from TS004.0061:2010 Annex D).
    /// </summary>
    public partial class TrafficSignalHistoryEntry
    {
        /// <summary>
        /// Category or type of the history event (e.g. 'configuration_change', 'fault_cleared').
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
        /// Identifier of the traffic signal this log entry is about.
        /// </summary>
        [JsonProperty("trafficSignalId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string TrafficSignalId { get; set; }
    }

    /// <summary>
    /// Traffic signal type definitions (reused from TS004.0061:2010 Annex D).
    /// </summary>
    public partial class TrafficSignalTypeClass
    {
        /// <summary>
        /// Human-readable description of the control method (e.g. 'MOVA', 'SCOOT', 'fixed-time',
        /// 'vehicle-actuated').
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
        /// URL to an external specification document for this control method.
        /// </summary>
        [JsonProperty("specificationLink", NullValueHandling = NullValueHandling.Ignore)]
        public Uri SpecificationLink { get; set; }

        /// <summary>
        /// Identifier of this control method. Referenced by TrafficSignalDefinition.controlMethodId.
        /// </summary>
        [JsonProperty("typeId")]
        [JsonConverter(typeof(HilariousMinMaxLengthCheckConverter))]
        public string TypeId { get; set; }
    }

    /// <summary>
    /// DATEX II FaultSeverityEnum — inherited via Fault base (PDF §3.3).
    ///
    /// Severity of the fault.
    /// </summary>
    public enum FaultSeverityEnum { High, Low, Medium, Unknown };

    public enum ForceStageCommandKind { ForceStage };

    public enum HoldStageCommandKind { HoldStage };

    public enum RequestStatusCommandKind { RequestStatus };

    public enum SetPlanCommandKind { SetPlan };

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

    public enum SynchronizeClockCommandKind { SynchronizeClock };

    /// <summary>
    /// Outcome of the command execution.
    ///
    /// Outcome of a command execution.
    ///
    /// Every value except ``PENDING`` is terminal. Consumers SHOULD test for
    /// "not pending" rather than enumerating the terminal values, so that a
    /// future profile adding one does not strand them.
    /// </summary>
    public enum TrafficSignalCommandResponseStatus { Failed, Invalid, NotSupported, Pending, Success, TimedOut };

    /// <summary>
    /// Type of the traffic signal.
    ///
    /// PDF §4.2 — traffic-signal types. XSD values (camelCase) are authoritative.
    /// </summary>
    public enum TrafficSignalTypeEnumEnum { Intersection, Other, Pegasus, Pelican, Puffin, Toucan, TramCrossing, WigWag };

    public enum TypeEnum { Point };

    /// <summary>
    /// The high level mode of the traffic signal.
    ///
    /// PDF §4.3 — high-level traffic-signal modes.
    /// </summary>
    public enum TrafficSignalMode { CentralMode, LocalMode, SignalsOffMode, StartUpMode, UnknownMode };

    /// <summary>
    /// The detailed sub-mode of the traffic signal.
    ///
    /// PDF §4.4 — detailed traffic-signal sub-modes.
    ///
    /// Note: §5.1's mode/subMode combinations table references additional values
    /// (hurryCall, cablelessLinkingBaseTime, cablelessLinkingNonBaseTime,
    /// publicServiceVehiclePriority, lightRailTransitMode, pre-emption) that are
    /// NOT defined in §4.4 or the XSD. This enum follows §4.4 + XSD.
    /// </summary>
    public enum TrafficSignalSubMode { CablelessLinking, FixedModeSimpleOrLinked, FixedModeToCurrentMax, ManualMode, ManualStepOn, MovaMode, OtherDistributedMode, PreEmptLevel1, PreEmptLevel2, PreEmptLevel3, UtcFixedTime, UtcOtherCentralControlMethod, UtcScoot, VehicleActuated, WardenControl };

    /// <summary>
    /// Coarse DATEX II fault bucket, as in TS004.0064bis:2017. Required unless faultTypeId is
    /// given; when both are present faultTypeId is the authoritative classification.
    ///
    /// PDF §4.1 — traffic-signal fault types.
    /// </summary>
    public enum TrafficSignalFaultType { DetectorFault, ElectronicsFault, IndicatorFault, LampFault, MechanicalFault, Other, UtilityFault };

    /// <summary>
    /// The urgency of this fault.
    ///
    /// DATEX II UrgencyEnum — inherited by TrafficSignalFault.trafficSignalFaultUrgency (PDF
    /// §3.3).
    ///
    /// ``UNKNOWN`` is a deliberate addition to the DATEX II baseline: a
    /// publisher that cannot determine urgency must be able to say so rather
    /// than defaulting to ``normalUrgency`` and understating a fault. This
    /// mirrors ``FaultSeverityEnum.UNKNOWN``, which already carries the value.
    /// </summary>
    public enum TrafficSignalFaultUrgency { ExtremelyUrgent, NormalUrgency, Unknown, Urgent };

    internal static class Converter
    {
        public static readonly JsonSerializerSettings Settings = new JsonSerializerSettings
        {
            MetadataPropertyHandling = MetadataPropertyHandling.Ignore,
            DateParseHandling = DateParseHandling.None,
            Converters =
            {
                FaultSeverityEnumConverter.Singleton,
                ForceStageCommandKindConverter.Singleton,
                HoldStageCommandKindConverter.Singleton,
                RequestStatusCommandKindConverter.Singleton,
                SetPlanCommandKindConverter.Singleton,
                SyncCategoryEnumConverter.Singleton,
                SyncStatusEnumConverter.Singleton,
                SynchronizeClockCommandKindConverter.Singleton,
                TrafficSignalCommandResponseStatusConverter.Singleton,
                TrafficSignalTypeEnumEnumConverter.Singleton,
                TypeEnumConverter.Singleton,
                TrafficSignalModeConverter.Singleton,
                TrafficSignalSubModeConverter.Singleton,
                TrafficSignalFaultTypeConverter.Singleton,
                TrafficSignalFaultUrgencyConverter.Singleton,
                new IsoDateTimeConverter { DateTimeStyles = DateTimeStyles.AssumeUniversal }
            },
        };
    }

    internal class FaultSeverityEnumConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(FaultSeverityEnum) || t == typeof(FaultSeverityEnum?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "high":
                    return FaultSeverityEnum.High;
                case "low":
                    return FaultSeverityEnum.Low;
                case "medium":
                    return FaultSeverityEnum.Medium;
                case "unknown":
                    return FaultSeverityEnum.Unknown;
            }
            throw new Exception("Cannot unmarshal type FaultSeverityEnum");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (FaultSeverityEnum)untypedValue;
            switch (value)
            {
                case FaultSeverityEnum.High:
                    serializer.Serialize(writer, "high");
                    return;
                case FaultSeverityEnum.Low:
                    serializer.Serialize(writer, "low");
                    return;
                case FaultSeverityEnum.Medium:
                    serializer.Serialize(writer, "medium");
                    return;
                case FaultSeverityEnum.Unknown:
                    serializer.Serialize(writer, "unknown");
                    return;
            }
            throw new Exception("Cannot marshal type FaultSeverityEnum");
        }

        public static readonly FaultSeverityEnumConverter Singleton = new FaultSeverityEnumConverter();
    }

    internal class ForceStageCommandKindConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(ForceStageCommandKind) || t == typeof(ForceStageCommandKind?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            if (value == "forceStage")
            {
                return ForceStageCommandKind.ForceStage;
            }
            throw new Exception("Cannot unmarshal type ForceStageCommandKind");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (ForceStageCommandKind)untypedValue;
            if (value == ForceStageCommandKind.ForceStage)
            {
                serializer.Serialize(writer, "forceStage");
                return;
            }
            throw new Exception("Cannot marshal type ForceStageCommandKind");
        }

        public static readonly ForceStageCommandKindConverter Singleton = new ForceStageCommandKindConverter();
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

    internal class HoldStageCommandKindConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(HoldStageCommandKind) || t == typeof(HoldStageCommandKind?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            if (value == "holdStage")
            {
                return HoldStageCommandKind.HoldStage;
            }
            throw new Exception("Cannot unmarshal type HoldStageCommandKind");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (HoldStageCommandKind)untypedValue;
            if (value == HoldStageCommandKind.HoldStage)
            {
                serializer.Serialize(writer, "holdStage");
                return;
            }
            throw new Exception("Cannot marshal type HoldStageCommandKind");
        }

        public static readonly HoldStageCommandKindConverter Singleton = new HoldStageCommandKindConverter();
    }

    internal class FluffyMinMaxLengthCheckConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(string);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            var value = serializer.Deserialize<string>(reader);
            if (value.Length >= 1 && value.Length <= 50)
            {
                return value;
            }
            throw new Exception("Cannot unmarshal type string");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            var value = (string)untypedValue;
            if (value.Length >= 1 && value.Length <= 50)
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
            if (value.Length >= 1 && value.Length <= 20)
            {
                return value;
            }
            throw new Exception("Cannot unmarshal type string");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            var value = (string)untypedValue;
            if (value.Length >= 1 && value.Length <= 20)
            {
                serializer.Serialize(writer, value);
                return;
            }
            throw new Exception("Cannot marshal type string");
        }

        public static readonly TentacledMinMaxLengthCheckConverter Singleton = new TentacledMinMaxLengthCheckConverter();
    }

    internal class RequestStatusCommandKindConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(RequestStatusCommandKind) || t == typeof(RequestStatusCommandKind?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            if (value == "requestStatus")
            {
                return RequestStatusCommandKind.RequestStatus;
            }
            throw new Exception("Cannot unmarshal type RequestStatusCommandKind");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (RequestStatusCommandKind)untypedValue;
            if (value == RequestStatusCommandKind.RequestStatus)
            {
                serializer.Serialize(writer, "requestStatus");
                return;
            }
            throw new Exception("Cannot marshal type RequestStatusCommandKind");
        }

        public static readonly RequestStatusCommandKindConverter Singleton = new RequestStatusCommandKindConverter();
    }

    internal class SetPlanCommandKindConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(SetPlanCommandKind) || t == typeof(SetPlanCommandKind?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            if (value == "setPlan")
            {
                return SetPlanCommandKind.SetPlan;
            }
            throw new Exception("Cannot unmarshal type SetPlanCommandKind");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (SetPlanCommandKind)untypedValue;
            if (value == SetPlanCommandKind.SetPlan)
            {
                serializer.Serialize(writer, "setPlan");
                return;
            }
            throw new Exception("Cannot marshal type SetPlanCommandKind");
        }

        public static readonly SetPlanCommandKindConverter Singleton = new SetPlanCommandKindConverter();
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

    internal class StickyMinMaxLengthCheckConverter : JsonConverter
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

        public static readonly StickyMinMaxLengthCheckConverter Singleton = new StickyMinMaxLengthCheckConverter();
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

    internal class IndecentMinMaxLengthCheckConverter : JsonConverter
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

        public static readonly IndecentMinMaxLengthCheckConverter Singleton = new IndecentMinMaxLengthCheckConverter();
    }

    internal class HilariousMinMaxLengthCheckConverter : JsonConverter
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

        public static readonly HilariousMinMaxLengthCheckConverter Singleton = new HilariousMinMaxLengthCheckConverter();
    }

    internal class SynchronizeClockCommandKindConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(SynchronizeClockCommandKind) || t == typeof(SynchronizeClockCommandKind?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            if (value == "synchronizeClock")
            {
                return SynchronizeClockCommandKind.SynchronizeClock;
            }
            throw new Exception("Cannot unmarshal type SynchronizeClockCommandKind");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (SynchronizeClockCommandKind)untypedValue;
            if (value == SynchronizeClockCommandKind.SynchronizeClock)
            {
                serializer.Serialize(writer, "synchronizeClock");
                return;
            }
            throw new Exception("Cannot marshal type SynchronizeClockCommandKind");
        }

        public static readonly SynchronizeClockCommandKindConverter Singleton = new SynchronizeClockCommandKindConverter();
    }

    internal class TrafficSignalCommandResponseStatusConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(TrafficSignalCommandResponseStatus) || t == typeof(TrafficSignalCommandResponseStatus?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "failed":
                    return TrafficSignalCommandResponseStatus.Failed;
                case "invalid":
                    return TrafficSignalCommandResponseStatus.Invalid;
                case "notSupported":
                    return TrafficSignalCommandResponseStatus.NotSupported;
                case "pending":
                    return TrafficSignalCommandResponseStatus.Pending;
                case "success":
                    return TrafficSignalCommandResponseStatus.Success;
                case "timedOut":
                    return TrafficSignalCommandResponseStatus.TimedOut;
            }
            throw new Exception("Cannot unmarshal type TrafficSignalCommandResponseStatus");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (TrafficSignalCommandResponseStatus)untypedValue;
            switch (value)
            {
                case TrafficSignalCommandResponseStatus.Failed:
                    serializer.Serialize(writer, "failed");
                    return;
                case TrafficSignalCommandResponseStatus.Invalid:
                    serializer.Serialize(writer, "invalid");
                    return;
                case TrafficSignalCommandResponseStatus.NotSupported:
                    serializer.Serialize(writer, "notSupported");
                    return;
                case TrafficSignalCommandResponseStatus.Pending:
                    serializer.Serialize(writer, "pending");
                    return;
                case TrafficSignalCommandResponseStatus.Success:
                    serializer.Serialize(writer, "success");
                    return;
                case TrafficSignalCommandResponseStatus.TimedOut:
                    serializer.Serialize(writer, "timedOut");
                    return;
            }
            throw new Exception("Cannot marshal type TrafficSignalCommandResponseStatus");
        }

        public static readonly TrafficSignalCommandResponseStatusConverter Singleton = new TrafficSignalCommandResponseStatusConverter();
    }

    internal class TrafficSignalTypeEnumEnumConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(TrafficSignalTypeEnumEnum) || t == typeof(TrafficSignalTypeEnumEnum?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "intersection":
                    return TrafficSignalTypeEnumEnum.Intersection;
                case "other":
                    return TrafficSignalTypeEnumEnum.Other;
                case "pegasus":
                    return TrafficSignalTypeEnumEnum.Pegasus;
                case "pelican":
                    return TrafficSignalTypeEnumEnum.Pelican;
                case "puffin":
                    return TrafficSignalTypeEnumEnum.Puffin;
                case "toucan":
                    return TrafficSignalTypeEnumEnum.Toucan;
                case "tramCrossing":
                    return TrafficSignalTypeEnumEnum.TramCrossing;
                case "wigWag":
                    return TrafficSignalTypeEnumEnum.WigWag;
            }
            throw new Exception("Cannot unmarshal type TrafficSignalTypeEnumEnum");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (TrafficSignalTypeEnumEnum)untypedValue;
            switch (value)
            {
                case TrafficSignalTypeEnumEnum.Intersection:
                    serializer.Serialize(writer, "intersection");
                    return;
                case TrafficSignalTypeEnumEnum.Other:
                    serializer.Serialize(writer, "other");
                    return;
                case TrafficSignalTypeEnumEnum.Pegasus:
                    serializer.Serialize(writer, "pegasus");
                    return;
                case TrafficSignalTypeEnumEnum.Pelican:
                    serializer.Serialize(writer, "pelican");
                    return;
                case TrafficSignalTypeEnumEnum.Puffin:
                    serializer.Serialize(writer, "puffin");
                    return;
                case TrafficSignalTypeEnumEnum.Toucan:
                    serializer.Serialize(writer, "toucan");
                    return;
                case TrafficSignalTypeEnumEnum.TramCrossing:
                    serializer.Serialize(writer, "tramCrossing");
                    return;
                case TrafficSignalTypeEnumEnum.WigWag:
                    serializer.Serialize(writer, "wigWag");
                    return;
            }
            throw new Exception("Cannot marshal type TrafficSignalTypeEnumEnum");
        }

        public static readonly TrafficSignalTypeEnumEnumConverter Singleton = new TrafficSignalTypeEnumEnumConverter();
    }

    internal class AmbitiousMinMaxLengthCheckConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(string);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            var value = serializer.Deserialize<string>(reader);
            if (value.Length <= 32)
            {
                return value;
            }
            throw new Exception("Cannot unmarshal type string");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            var value = (string)untypedValue;
            if (value.Length <= 32)
            {
                serializer.Serialize(writer, value);
                return;
            }
            throw new Exception("Cannot marshal type string");
        }

        public static readonly AmbitiousMinMaxLengthCheckConverter Singleton = new AmbitiousMinMaxLengthCheckConverter();
    }

    internal class TypeEnumConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(TypeEnum) || t == typeof(TypeEnum?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            if (value == "Point")
            {
                return TypeEnum.Point;
            }
            throw new Exception("Cannot unmarshal type TypeEnum");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (TypeEnum)untypedValue;
            if (value == TypeEnum.Point)
            {
                serializer.Serialize(writer, "Point");
                return;
            }
            throw new Exception("Cannot marshal type TypeEnum");
        }

        public static readonly TypeEnumConverter Singleton = new TypeEnumConverter();
    }

    internal class CunningMinMaxLengthCheckConverter : JsonConverter
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

        public static readonly CunningMinMaxLengthCheckConverter Singleton = new CunningMinMaxLengthCheckConverter();
    }

    internal class TrafficSignalModeConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(TrafficSignalMode) || t == typeof(TrafficSignalMode?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "centralMode":
                    return TrafficSignalMode.CentralMode;
                case "localMode":
                    return TrafficSignalMode.LocalMode;
                case "signalsOffMode":
                    return TrafficSignalMode.SignalsOffMode;
                case "startUpMode":
                    return TrafficSignalMode.StartUpMode;
                case "unknownMode":
                    return TrafficSignalMode.UnknownMode;
            }
            throw new Exception("Cannot unmarshal type TrafficSignalMode");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (TrafficSignalMode)untypedValue;
            switch (value)
            {
                case TrafficSignalMode.CentralMode:
                    serializer.Serialize(writer, "centralMode");
                    return;
                case TrafficSignalMode.LocalMode:
                    serializer.Serialize(writer, "localMode");
                    return;
                case TrafficSignalMode.SignalsOffMode:
                    serializer.Serialize(writer, "signalsOffMode");
                    return;
                case TrafficSignalMode.StartUpMode:
                    serializer.Serialize(writer, "startUpMode");
                    return;
                case TrafficSignalMode.UnknownMode:
                    serializer.Serialize(writer, "unknownMode");
                    return;
            }
            throw new Exception("Cannot marshal type TrafficSignalMode");
        }

        public static readonly TrafficSignalModeConverter Singleton = new TrafficSignalModeConverter();
    }

    internal class TrafficSignalSubModeConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(TrafficSignalSubMode) || t == typeof(TrafficSignalSubMode?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "cablelessLinking":
                    return TrafficSignalSubMode.CablelessLinking;
                case "fixedModeSimpleOrLinked":
                    return TrafficSignalSubMode.FixedModeSimpleOrLinked;
                case "fixedModeToCurrentMax":
                    return TrafficSignalSubMode.FixedModeToCurrentMax;
                case "manualMode":
                    return TrafficSignalSubMode.ManualMode;
                case "manualStepOn":
                    return TrafficSignalSubMode.ManualStepOn;
                case "movaMode":
                    return TrafficSignalSubMode.MovaMode;
                case "otherDistributedMode":
                    return TrafficSignalSubMode.OtherDistributedMode;
                case "preEmptLevel1":
                    return TrafficSignalSubMode.PreEmptLevel1;
                case "preEmptLevel2":
                    return TrafficSignalSubMode.PreEmptLevel2;
                case "preEmptLevel3":
                    return TrafficSignalSubMode.PreEmptLevel3;
                case "utcFixedTime":
                    return TrafficSignalSubMode.UtcFixedTime;
                case "utcOtherCentralControlMethod":
                    return TrafficSignalSubMode.UtcOtherCentralControlMethod;
                case "utcScoot":
                    return TrafficSignalSubMode.UtcScoot;
                case "vehicleActuated":
                    return TrafficSignalSubMode.VehicleActuated;
                case "wardenControl":
                    return TrafficSignalSubMode.WardenControl;
            }
            throw new Exception("Cannot unmarshal type TrafficSignalSubMode");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (TrafficSignalSubMode)untypedValue;
            switch (value)
            {
                case TrafficSignalSubMode.CablelessLinking:
                    serializer.Serialize(writer, "cablelessLinking");
                    return;
                case TrafficSignalSubMode.FixedModeSimpleOrLinked:
                    serializer.Serialize(writer, "fixedModeSimpleOrLinked");
                    return;
                case TrafficSignalSubMode.FixedModeToCurrentMax:
                    serializer.Serialize(writer, "fixedModeToCurrentMax");
                    return;
                case TrafficSignalSubMode.ManualMode:
                    serializer.Serialize(writer, "manualMode");
                    return;
                case TrafficSignalSubMode.ManualStepOn:
                    serializer.Serialize(writer, "manualStepOn");
                    return;
                case TrafficSignalSubMode.MovaMode:
                    serializer.Serialize(writer, "movaMode");
                    return;
                case TrafficSignalSubMode.OtherDistributedMode:
                    serializer.Serialize(writer, "otherDistributedMode");
                    return;
                case TrafficSignalSubMode.PreEmptLevel1:
                    serializer.Serialize(writer, "preEmptLevel1");
                    return;
                case TrafficSignalSubMode.PreEmptLevel2:
                    serializer.Serialize(writer, "preEmptLevel2");
                    return;
                case TrafficSignalSubMode.PreEmptLevel3:
                    serializer.Serialize(writer, "preEmptLevel3");
                    return;
                case TrafficSignalSubMode.UtcFixedTime:
                    serializer.Serialize(writer, "utcFixedTime");
                    return;
                case TrafficSignalSubMode.UtcOtherCentralControlMethod:
                    serializer.Serialize(writer, "utcOtherCentralControlMethod");
                    return;
                case TrafficSignalSubMode.UtcScoot:
                    serializer.Serialize(writer, "utcScoot");
                    return;
                case TrafficSignalSubMode.VehicleActuated:
                    serializer.Serialize(writer, "vehicleActuated");
                    return;
                case TrafficSignalSubMode.WardenControl:
                    serializer.Serialize(writer, "wardenControl");
                    return;
            }
            throw new Exception("Cannot marshal type TrafficSignalSubMode");
        }

        public static readonly TrafficSignalSubModeConverter Singleton = new TrafficSignalSubModeConverter();
    }

    internal class TrafficSignalFaultTypeConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(TrafficSignalFaultType) || t == typeof(TrafficSignalFaultType?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "detectorFault":
                    return TrafficSignalFaultType.DetectorFault;
                case "electronicsFault":
                    return TrafficSignalFaultType.ElectronicsFault;
                case "indicatorFault":
                    return TrafficSignalFaultType.IndicatorFault;
                case "lampFault":
                    return TrafficSignalFaultType.LampFault;
                case "mechanicalFault":
                    return TrafficSignalFaultType.MechanicalFault;
                case "other":
                    return TrafficSignalFaultType.Other;
                case "utilityFault":
                    return TrafficSignalFaultType.UtilityFault;
            }
            throw new Exception("Cannot unmarshal type TrafficSignalFaultType");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (TrafficSignalFaultType)untypedValue;
            switch (value)
            {
                case TrafficSignalFaultType.DetectorFault:
                    serializer.Serialize(writer, "detectorFault");
                    return;
                case TrafficSignalFaultType.ElectronicsFault:
                    serializer.Serialize(writer, "electronicsFault");
                    return;
                case TrafficSignalFaultType.IndicatorFault:
                    serializer.Serialize(writer, "indicatorFault");
                    return;
                case TrafficSignalFaultType.LampFault:
                    serializer.Serialize(writer, "lampFault");
                    return;
                case TrafficSignalFaultType.MechanicalFault:
                    serializer.Serialize(writer, "mechanicalFault");
                    return;
                case TrafficSignalFaultType.Other:
                    serializer.Serialize(writer, "other");
                    return;
                case TrafficSignalFaultType.UtilityFault:
                    serializer.Serialize(writer, "utilityFault");
                    return;
            }
            throw new Exception("Cannot marshal type TrafficSignalFaultType");
        }

        public static readonly TrafficSignalFaultTypeConverter Singleton = new TrafficSignalFaultTypeConverter();
    }

    internal class TrafficSignalFaultUrgencyConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(TrafficSignalFaultUrgency) || t == typeof(TrafficSignalFaultUrgency?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "extremelyUrgent":
                    return TrafficSignalFaultUrgency.ExtremelyUrgent;
                case "normalUrgency":
                    return TrafficSignalFaultUrgency.NormalUrgency;
                case "unknown":
                    return TrafficSignalFaultUrgency.Unknown;
                case "urgent":
                    return TrafficSignalFaultUrgency.Urgent;
            }
            throw new Exception("Cannot unmarshal type TrafficSignalFaultUrgency");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (TrafficSignalFaultUrgency)untypedValue;
            switch (value)
            {
                case TrafficSignalFaultUrgency.ExtremelyUrgent:
                    serializer.Serialize(writer, "extremelyUrgent");
                    return;
                case TrafficSignalFaultUrgency.NormalUrgency:
                    serializer.Serialize(writer, "normalUrgency");
                    return;
                case TrafficSignalFaultUrgency.Unknown:
                    serializer.Serialize(writer, "unknown");
                    return;
                case TrafficSignalFaultUrgency.Urgent:
                    serializer.Serialize(writer, "urgent");
                    return;
            }
            throw new Exception("Cannot marshal type TrafficSignalFaultUrgency");
        }

        public static readonly TrafficSignalFaultUrgencyConverter Singleton = new TrafficSignalFaultUrgencyConverter();
    }
}

namespace Topas.Utmc.TrafficSignal
{
    using System;
    using Newtonsoft.Json;
    using Newtonsoft.Json.Linq;

    /// <summary>
    /// Traffic signal command payloads (reused from TS004.0061:2010 Annex D).
    /// </summary>
    /// <remarks>
    /// Tagged union over <c>kind</c>. Deserialise as this interface to
    /// have the converter select the variant; the variant classes are
    /// generated by quicktype and adopt the interface via partial
    /// declarations below.
    /// </remarks>
    [JsonConverter(typeof(TrafficSignalCommandConverter))]
    public interface ITrafficSignalCommand { }

    public partial class ForceStageCommand : ITrafficSignalCommand { }
    public partial class HoldStageCommand : ITrafficSignalCommand { }
    public partial class RequestStatusCommand : ITrafficSignalCommand { }
    public partial class SetPlanCommand : ITrafficSignalCommand { }
    public partial class SynchronizeClockCommand : ITrafficSignalCommand { }

    internal class TrafficSignalCommandConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(ITrafficSignalCommand);

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
                case "forceStage": return Read<ForceStageCommand>(obj, serializer);
                case "holdStage": return Read<HoldStageCommand>(obj, serializer);
                case "requestStatus": return Read<RequestStatusCommand>(obj, serializer);
                case "setPlan": return Read<SetPlanCommand>(obj, serializer);
                case "synchronizeClock": return Read<SynchronizeClockCommand>(obj, serializer);
                default:
                    throw new JsonSerializationException(
                        $"unknown kind '{tag}' for TrafficSignalCommand");
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

namespace Topas.Utmc.TrafficSignal
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
