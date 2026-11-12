/*
 * File:        output/csharp/sdk/FaultSupport.cs
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
namespace Topas.Utmc.FaultSupport
{
    using System;
    using System.Collections.Generic;

    using System.Globalization;
    using Newtonsoft.Json;
    using Newtonsoft.Json.Converters;

    public partial class AllTypes
    {
        [JsonProperty("AcknowledgementState", NullValueHandling = NullValueHandling.Ignore)]
        public AcknowledgementState? AcknowledgementState { get; set; }

        [JsonProperty("FaultCategory", NullValueHandling = NullValueHandling.Ignore)]
        public FaultCategory? FaultCategory { get; set; }

        [JsonProperty("FaultStatus", NullValueHandling = NullValueHandling.Ignore)]
        public FaultStatus? FaultStatus { get; set; }

        [JsonProperty("FaultTypeDefinition", NullValueHandling = NullValueHandling.Ignore)]
        public FaultTypeDefinition FaultTypeDefinition { get; set; }
    }

    /// <summary>
    /// Catalogue entry describing one kind of fault that can be reported.
    /// </summary>
    public partial class FaultTypeDefinition
    {
        /// <summary>
        /// Coarse grouping, for filtering rather than semantics.
        /// </summary>
        [JsonProperty("category", NullValueHandling = NullValueHandling.Ignore)]
        public FaultCategory? Category { get; set; }

        /// <summary>
        /// The identifier this fault carries in its source document or vendor documentation — 'A1',
        /// 'F3', 'W12'. Distinct from faultTypeId, which is this catalogue's key: a deployment may
        /// hold two entries with the same code from different vendors.
        /// </summary>
        [JsonProperty("code", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string Code { get; set; }

        /// <summary>
        /// Human-readable description of the fault type.
        /// </summary>
        [JsonProperty("description")]
        public string Description { get; set; }

        /// <summary>
        /// Vendor or local extensions. Open-content object.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Identifier of this fault type.
        /// </summary>
        [JsonProperty("faultTypeId")]
        [JsonConverter(typeof(FluffyMinMaxLengthCheckConverter))]
        public string FaultTypeId { get; set; }

        /// <summary>
        /// Human-readable message with positional placeholders, e.g. 'Minimum green not met: stage
        /// {0} ran {1}s against a minimum of {2}s'. A fault instance supplies the values in its
        /// parameters field. This is what lets a catalogue carry faults whose meaning depends on
        /// runtime values — the stuck-stage and interstage timing faults that a closed enum cannot
        /// express.
        /// </summary>
        [JsonProperty("messageTemplate", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(TentacledMinMaxLengthCheckConverter))]
        public string MessageTemplate { get; set; }

        /// <summary>
        /// Names for the positional placeholders in messageTemplate, in order — ['stageNumber',
        /// 'actualSeconds', 'minimumSeconds']. Lets a consumer label the values rather than only
        /// interpolate them. Length SHOULD match the number of placeholders.
        /// </summary>
        [JsonProperty("parameterNames", NullValueHandling = NullValueHandling.Ignore)]
        public string[] ParameterNames { get; set; }

        /// <summary>
        /// Numeric severity (higher = more severe). Optional.
        /// </summary>
        [JsonProperty("severity", NullValueHandling = NullValueHandling.Ignore)]
        public long? Severity { get; set; }

        /// <summary>
        /// Reference to a SubsystemDefinition entry: the subsystem this fault type belongs to.
        /// </summary>
        [JsonProperty("subsystemTypeId", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(FluffyMinMaxLengthCheckConverter))]
        public string SubsystemTypeId { get; set; }
    }

    /// <summary>
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
    /// Coarse grouping for catalogue entries.
    ///
    /// Deliberately short. It exists so an operator can filter "show me the
    /// communications faults" without the catalogue having to agree on a
    /// taxonomy first; the precise meaning of a fault is its catalogue entry,
    /// not its category.
    ///
    /// Coarse grouping, for filtering rather than semantics.
    /// </summary>
    public enum FaultCategory { Communications, Environmental, Hardware, Other, SignalTimings };

    /// <summary>
    /// Whether a fault is still active or has been cleared.
    ///
    /// This is the only fault lifecycle the payload carries. Record archival
    /// — distinguishing a live record from a retired one — is not modelled
    /// here and is a matter for the publishing system.
    /// </summary>
    public enum FaultStatus { Active, Cleared };

    internal static class Converter
    {
        public static readonly JsonSerializerSettings Settings = new JsonSerializerSettings
        {
            MetadataPropertyHandling = MetadataPropertyHandling.Ignore,
            DateParseHandling = DateParseHandling.None,
            Converters =
            {
                AcknowledgementStateConverter.Singleton,
                FaultCategoryConverter.Singleton,
                FaultStatusConverter.Singleton,
                new IsoDateTimeConverter { DateTimeStyles = DateTimeStyles.AssumeUniversal }
            },
        };
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

    internal class FaultCategoryConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(FaultCategory) || t == typeof(FaultCategory?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "communications":
                    return FaultCategory.Communications;
                case "environmental":
                    return FaultCategory.Environmental;
                case "hardware":
                    return FaultCategory.Hardware;
                case "other":
                    return FaultCategory.Other;
                case "signalTimings":
                    return FaultCategory.SignalTimings;
            }
            throw new Exception("Cannot unmarshal type FaultCategory");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (FaultCategory)untypedValue;
            switch (value)
            {
                case FaultCategory.Communications:
                    serializer.Serialize(writer, "communications");
                    return;
                case FaultCategory.Environmental:
                    serializer.Serialize(writer, "environmental");
                    return;
                case FaultCategory.Hardware:
                    serializer.Serialize(writer, "hardware");
                    return;
                case FaultCategory.Other:
                    serializer.Serialize(writer, "other");
                    return;
                case FaultCategory.SignalTimings:
                    serializer.Serialize(writer, "signalTimings");
                    return;
            }
            throw new Exception("Cannot marshal type FaultCategory");
        }

        public static readonly FaultCategoryConverter Singleton = new FaultCategoryConverter();
    }

    internal class FaultStatusConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(FaultStatus) || t == typeof(FaultStatus?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "active":
                    return FaultStatus.Active;
                case "cleared":
                    return FaultStatus.Cleared;
            }
            throw new Exception("Cannot unmarshal type FaultStatus");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (FaultStatus)untypedValue;
            switch (value)
            {
                case FaultStatus.Active:
                    serializer.Serialize(writer, "active");
                    return;
                case FaultStatus.Cleared:
                    serializer.Serialize(writer, "cleared");
                    return;
            }
            throw new Exception("Cannot marshal type FaultStatus");
        }

        public static readonly FaultStatusConverter Singleton = new FaultStatusConverter();
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
}

namespace Topas.Utmc.FaultSupport
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
