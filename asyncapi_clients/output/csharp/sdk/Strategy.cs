/*
 * File:        output/csharp/sdk/Strategy.cs
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
namespace Topas.Utmc.Strategy
{
    using System;
    using System.Collections.Generic;

    using System.Globalization;
    using Newtonsoft.Json;
    using Newtonsoft.Json.Converters;

    public partial class AllTypes
    {
        [JsonProperty("StrategyCommandResponse", NullValueHandling = NullValueHandling.Ignore)]
        public StrategyCommandResponse StrategyCommandResponse { get; set; }

        [JsonProperty("StrategyDefinition", NullValueHandling = NullValueHandling.Ignore)]
        public StrategyDefinition StrategyDefinition { get; set; }

        [JsonProperty("StrategyStatus", NullValueHandling = NullValueHandling.Ignore)]
        public StrategyStatusClass StrategyStatus { get; set; }

        [JsonProperty("StrategyStatusEnum", NullValueHandling = NullValueHandling.Ignore)]
        public StrategyStatusEnumEnum? StrategyStatusEnum { get; set; }

        [JsonProperty("StrategyTrigger", NullValueHandling = NullValueHandling.Ignore)]
        public StrategyTrigger StrategyTrigger { get; set; }

        [JsonProperty("SyncCategoryEnum", NullValueHandling = NullValueHandling.Ignore)]
        public SyncCategoryEnum? SyncCategoryEnum { get; set; }

        [JsonProperty("SyncComplete", NullValueHandling = NullValueHandling.Ignore)]
        public SyncComplete SyncComplete { get; set; }

        [JsonProperty("SyncRequest", NullValueHandling = NullValueHandling.Ignore)]
        public SyncRequest SyncRequest { get; set; }

        [JsonProperty("SyncStatus", NullValueHandling = NullValueHandling.Ignore)]
        public SyncStatusEnum? SyncStatus { get; set; }

        [JsonProperty("TriggerStateEnum", NullValueHandling = NullValueHandling.Ignore)]
        public TriggerState? TriggerStateEnum { get; set; }

        [JsonProperty("TriggerUpdateErrorEnum", NullValueHandling = NullValueHandling.Ignore)]
        public ErrorCode? TriggerUpdateErrorEnum { get; set; }
    }

    /// <summary>
    /// Strategy command response payloads.
    /// </summary>
    public partial class StrategyCommandResponse
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
        /// Strategy-specific rejection category. Only permitted when the outcome is not success or
        /// pending.
        /// </summary>
        [JsonProperty("errorCode", NullValueHandling = NullValueHandling.Ignore)]
        public ErrorCode? ErrorCode { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Human-readable detail.
        /// </summary>
        [JsonProperty("message", NullValueHandling = NullValueHandling.Ignore)]
        public string Message { get; set; }

        /// <summary>
        /// When the response was generated.
        /// </summary>
        [JsonProperty("responseTime")]
        public DateTimeOffset ResponseTime { get; set; }

        /// <summary>
        /// Audit label copied from the command, if it carried one. Never used for authorisation or
        /// idempotency.
        /// </summary>
        [JsonProperty("serviceRequester", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string ServiceRequester { get; set; }

        /// <summary>
        /// Outcome of the command execution.
        /// </summary>
        [JsonProperty("status")]
        public StrategyCommandResponseStatus Status { get; set; }

        /// <summary>
        /// Identifier of the strategy the command targeted.
        /// </summary>
        [JsonProperty("strategyId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string StrategyId { get; set; }
    }

    /// <summary>
    /// Traffic management strategy static metadata.
    /// </summary>
    public partial class StrategyDefinition
    {
        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Geographic centre of the strategy area as WGS84 GeoJSON Point.
        /// </summary>
        [JsonProperty("location", NullValueHandling = NullValueHandling.Ignore)]
        public Location Location { get; set; }

        /// <summary>
        /// Longer description of what the strategy does.
        /// </summary>
        [JsonProperty("strategyDescription", NullValueHandling = NullValueHandling.Ignore)]
        public string StrategyDescription { get; set; }

        /// <summary>
        /// Strategy identifier (e.g. 'STR00000001').
        /// </summary>
        [JsonProperty("strategyId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string StrategyId { get; set; }

        /// <summary>
        /// Human-readable name of the strategy.
        /// </summary>
        [JsonProperty("strategyName")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string StrategyName { get; set; }
    }

    /// <summary>
    /// Geographic centre of the strategy area as WGS84 GeoJSON Point.
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
    /// Traffic management strategy current status.
    /// </summary>
    public partial class StrategyStatusClass
    {
        /// <summary>
        /// Error information (e.g. 'Strategy will never become active').
        /// </summary>
        [JsonProperty("errorMessage", NullValueHandling = NullValueHandling.Ignore)]
        public string ErrorMessage { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Non-error status information (e.g. 'Strategy applied 6 of 10 measures').
        /// </summary>
        [JsonProperty("statusMessage", NullValueHandling = NullValueHandling.Ignore)]
        public string StatusMessage { get; set; }

        /// <summary>
        /// When the strategy status changed.
        /// </summary>
        [JsonProperty("strategyChangeStateTime")]
        public DateTimeOffset StrategyChangeStateTime { get; set; }

        /// <summary>
        /// Identifier of the strategy this status is about.
        /// </summary>
        [JsonProperty("strategyId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string StrategyId { get; set; }

        /// <summary>
        /// Current status (active/inactive).
        /// </summary>
        [JsonProperty("strategyStatus")]
        public StrategyStatusEnumEnum StrategyStatus { get; set; }
    }

    /// <summary>
    /// Strategy trigger command payloads.
    /// </summary>
    public partial class StrategyTrigger
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
        /// When the command was issued.
        /// </summary>
        [JsonProperty("requestTime")]
        public DateTimeOffset RequestTime { get; set; }

        /// <summary>
        /// Audit label naming the requesting system. Sender-supplied, so it is never used for
        /// authorisation or idempotency.
        /// </summary>
        [JsonProperty("serviceRequester", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string ServiceRequester { get; set; }

        /// <summary>
        /// Identifier of the strategy to trigger.
        /// </summary>
        [JsonProperty("strategyId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string StrategyId { get; set; }

        /// <summary>
        /// Desired trigger state (enabled/disabled).
        /// </summary>
        [JsonProperty("triggerState")]
        public TriggerState TriggerState { get; set; }
    }

    /// <summary>
    /// Terminal message ending a strategy sync batch. Sent to the requester's reply node after
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
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
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
        [JsonConverter(typeof(FluffyMinMaxLengthCheckConverter))]
        public string SyncId { get; set; }
    }

    /// <summary>
    /// Request the current state of strategy objects. The Data Publisher answers to the
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
    /// Strategy-specific rejection category. Only permitted when the outcome is not success or
    /// pending.
    ///
    /// Categorisation of strategy trigger rejection reason.
    /// </summary>
    public enum ErrorCode { AccessDenied, NotAuthenticated, Other, StrategyIdDoesNotExist };

    /// <summary>
    /// Outcome of the command execution.
    ///
    /// Outcome of a command execution.
    ///
    /// Every value except ``PENDING`` is terminal. Consumers SHOULD test for
    /// "not pending" rather than enumerating the terminal values, so that a
    /// future profile adding one does not strand them.
    /// </summary>
    public enum StrategyCommandResponseStatus { Failed, Invalid, NotSupported, Pending, Success, TimedOut };

    public enum TypeEnum { Point };

    /// <summary>
    /// Current status (active/inactive).
    ///
    /// Status of a traffic management strategy.
    /// </summary>
    public enum StrategyStatusEnumEnum { Active, Inactive };

    /// <summary>
    /// Desired trigger state (enabled/disabled).
    ///
    /// Desired state when triggering a strategy.
    /// </summary>
    public enum TriggerState { Disabled, Enabled };

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
                ErrorCodeConverter.Singleton,
                StrategyCommandResponseStatusConverter.Singleton,
                TypeEnumConverter.Singleton,
                StrategyStatusEnumEnumConverter.Singleton,
                TriggerStateConverter.Singleton,
                SyncCategoryEnumConverter.Singleton,
                SyncStatusEnumConverter.Singleton,
                new IsoDateTimeConverter { DateTimeStyles = DateTimeStyles.AssumeUniversal }
            },
        };
    }

    internal class ErrorCodeConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(ErrorCode) || t == typeof(ErrorCode?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "accessDenied":
                    return ErrorCode.AccessDenied;
                case "notAuthenticated":
                    return ErrorCode.NotAuthenticated;
                case "other":
                    return ErrorCode.Other;
                case "strategyIdDoesNotExist":
                    return ErrorCode.StrategyIdDoesNotExist;
            }
            throw new Exception("Cannot unmarshal type ErrorCode");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (ErrorCode)untypedValue;
            switch (value)
            {
                case ErrorCode.AccessDenied:
                    serializer.Serialize(writer, "accessDenied");
                    return;
                case ErrorCode.NotAuthenticated:
                    serializer.Serialize(writer, "notAuthenticated");
                    return;
                case ErrorCode.Other:
                    serializer.Serialize(writer, "other");
                    return;
                case ErrorCode.StrategyIdDoesNotExist:
                    serializer.Serialize(writer, "strategyIdDoesNotExist");
                    return;
            }
            throw new Exception("Cannot marshal type ErrorCode");
        }

        public static readonly ErrorCodeConverter Singleton = new ErrorCodeConverter();
    }

    internal class PurpleMinMaxLengthCheckConverter : JsonConverter
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

        public static readonly PurpleMinMaxLengthCheckConverter Singleton = new PurpleMinMaxLengthCheckConverter();
    }

    internal class StrategyCommandResponseStatusConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(StrategyCommandResponseStatus) || t == typeof(StrategyCommandResponseStatus?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "failed":
                    return StrategyCommandResponseStatus.Failed;
                case "invalid":
                    return StrategyCommandResponseStatus.Invalid;
                case "notSupported":
                    return StrategyCommandResponseStatus.NotSupported;
                case "pending":
                    return StrategyCommandResponseStatus.Pending;
                case "success":
                    return StrategyCommandResponseStatus.Success;
                case "timedOut":
                    return StrategyCommandResponseStatus.TimedOut;
            }
            throw new Exception("Cannot unmarshal type StrategyCommandResponseStatus");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (StrategyCommandResponseStatus)untypedValue;
            switch (value)
            {
                case StrategyCommandResponseStatus.Failed:
                    serializer.Serialize(writer, "failed");
                    return;
                case StrategyCommandResponseStatus.Invalid:
                    serializer.Serialize(writer, "invalid");
                    return;
                case StrategyCommandResponseStatus.NotSupported:
                    serializer.Serialize(writer, "notSupported");
                    return;
                case StrategyCommandResponseStatus.Pending:
                    serializer.Serialize(writer, "pending");
                    return;
                case StrategyCommandResponseStatus.Success:
                    serializer.Serialize(writer, "success");
                    return;
                case StrategyCommandResponseStatus.TimedOut:
                    serializer.Serialize(writer, "timedOut");
                    return;
            }
            throw new Exception("Cannot marshal type StrategyCommandResponseStatus");
        }

        public static readonly StrategyCommandResponseStatusConverter Singleton = new StrategyCommandResponseStatusConverter();
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

    internal class StrategyStatusEnumEnumConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(StrategyStatusEnumEnum) || t == typeof(StrategyStatusEnumEnum?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "active":
                    return StrategyStatusEnumEnum.Active;
                case "inactive":
                    return StrategyStatusEnumEnum.Inactive;
            }
            throw new Exception("Cannot unmarshal type StrategyStatusEnumEnum");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (StrategyStatusEnumEnum)untypedValue;
            switch (value)
            {
                case StrategyStatusEnumEnum.Active:
                    serializer.Serialize(writer, "active");
                    return;
                case StrategyStatusEnumEnum.Inactive:
                    serializer.Serialize(writer, "inactive");
                    return;
            }
            throw new Exception("Cannot marshal type StrategyStatusEnumEnum");
        }

        public static readonly StrategyStatusEnumEnumConverter Singleton = new StrategyStatusEnumEnumConverter();
    }

    internal class TriggerStateConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(TriggerState) || t == typeof(TriggerState?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "disabled":
                    return TriggerState.Disabled;
                case "enabled":
                    return TriggerState.Enabled;
            }
            throw new Exception("Cannot unmarshal type TriggerState");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (TriggerState)untypedValue;
            switch (value)
            {
                case TriggerState.Disabled:
                    serializer.Serialize(writer, "disabled");
                    return;
                case TriggerState.Enabled:
                    serializer.Serialize(writer, "enabled");
                    return;
            }
            throw new Exception("Cannot marshal type TriggerState");
        }

        public static readonly TriggerStateConverter Singleton = new TriggerStateConverter();
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

    internal class FluffyMinMaxLengthCheckConverter : JsonConverter
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

        public static readonly FluffyMinMaxLengthCheckConverter Singleton = new FluffyMinMaxLengthCheckConverter();
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
                var converter = TentacledMinMaxLengthCheckConverter.Singleton;
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
                var converter = TentacledMinMaxLengthCheckConverter.Singleton;
                converter.WriteJson(writer, arrayItem, serializer);
            }
            writer.WriteEndArray();
            return;
        }

        public static readonly DecodeArrayConverter Singleton = new DecodeArrayConverter();
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
}

namespace Topas.Utmc.Strategy
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
