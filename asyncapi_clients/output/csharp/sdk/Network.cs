/*
 * File:        output/csharp/sdk/Network.cs
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
namespace Topas.Utmc.Network
{
    using System;
    using System.Collections.Generic;

    using System.Globalization;
    using Newtonsoft.Json;
    using Newtonsoft.Json.Converters;

    public partial class AllTypes
    {
        [JsonProperty("NetworkLink", NullValueHandling = NullValueHandling.Ignore)]
        public NetworkLink NetworkLink { get; set; }

        [JsonProperty("NetworkNode", NullValueHandling = NullValueHandling.Ignore)]
        public NetworkNode NetworkNode { get; set; }

        [JsonProperty("NetworkPath", NullValueHandling = NullValueHandling.Ignore)]
        public NetworkPath NetworkPath { get; set; }

        [JsonProperty("NetworkTurn", NullValueHandling = NullValueHandling.Ignore)]
        public NetworkTurn NetworkTurn { get; set; }

        [JsonProperty("NetworkZone", NullValueHandling = NullValueHandling.Ignore)]
        public NetworkZone NetworkZone { get; set; }

        [JsonProperty("SyncCategoryEnum", NullValueHandling = NullValueHandling.Ignore)]
        public SyncCategoryEnum? SyncCategoryEnum { get; set; }

        [JsonProperty("SyncComplete", NullValueHandling = NullValueHandling.Ignore)]
        public SyncComplete SyncComplete { get; set; }

        [JsonProperty("SyncRequest", NullValueHandling = NullValueHandling.Ignore)]
        public SyncRequest SyncRequest { get; set; }

        [JsonProperty("SyncStatus", NullValueHandling = NullValueHandling.Ignore)]
        public Status? SyncStatus { get; set; }
    }

    /// <summary>
    /// Road network link between two nodes.
    /// </summary>
    public partial class NetworkLink
    {
        /// <summary>
        /// Human-readable name.
        /// </summary>
        [JsonProperty("description", NullValueHandling = NullValueHandling.Ignore)]
        public string Description { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Start node reference.
        /// </summary>
        [JsonProperty("fromNodeId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string FromNodeId { get; set; }

        /// <summary>
        /// Polyline shape of the link as GeoJSON LineString.
        /// </summary>
        [JsonProperty("geometry", NullValueHandling = NullValueHandling.Ignore)]
        public Geometry Geometry { get; set; }

        /// <summary>
        /// Unique identifier of this link.
        /// </summary>
        [JsonProperty("linkId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string LinkId { get; set; }

        /// <summary>
        /// End node reference.
        /// </summary>
        [JsonProperty("toNodeId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string ToNodeId { get; set; }
    }

    /// <summary>
    /// Polyline shape of the link as GeoJSON LineString.
    ///
    /// A GeoJSON LineString geometry (RFC 7946).
    ///
    /// An ordered sequence of two or more positions forming a polyline.
    /// Coordinates are [longitude, latitude] per GeoJSON specification.
    ///
    /// Turn path geometry as GeoJSON LineString.
    ///
    /// Zone boundary geometry as GeoJSON LineString.
    /// </summary>
    public partial class Geometry
    {
        /// <summary>
        /// Ordered [longitude, latitude] pairs forming a polyline.
        /// </summary>
        [JsonProperty("coordinates")]
        public object[][] Coordinates { get; set; }

        [JsonProperty("type", NullValueHandling = NullValueHandling.Ignore)]
        public GeometryType? Type { get; set; }
    }

    /// <summary>
    /// Road network node (junction or endpoint).
    /// </summary>
    public partial class NetworkNode
    {
        /// <summary>
        /// Human-readable name.
        /// </summary>
        [JsonProperty("description", NullValueHandling = NullValueHandling.Ignore)]
        public string Description { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Geographic position as GeoJSON Point.
        /// </summary>
        [JsonProperty("location", NullValueHandling = NullValueHandling.Ignore)]
        public Location Location { get; set; }

        /// <summary>
        /// Unique identifier of this node.
        /// </summary>
        [JsonProperty("nodeId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string NodeId { get; set; }
    }

    /// <summary>
    /// Geographic position as GeoJSON Point.
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
        public LocationType? Type { get; set; }
    }

    /// <summary>
    /// Road network path (ordered sequence of links).
    /// </summary>
    public partial class NetworkPath
    {
        /// <summary>
        /// Human-readable name.
        /// </summary>
        [JsonProperty("description", NullValueHandling = NullValueHandling.Ignore)]
        public string Description { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Distance to trim from the end of the last link.
        /// </summary>
        [JsonProperty("finalTrimMeters", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(MinMaxValueCheckConverter))]
        public double? FinalTrimMeters { get; set; }

        /// <summary>
        /// Distance to trim from the start of the first link.
        /// </summary>
        [JsonProperty("initialTrimMeters", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(MinMaxValueCheckConverter))]
        public double? InitialTrimMeters { get; set; }

        /// <summary>
        /// Ordered links defining the path. List order is the route sequence. Omit the field rather
        /// than send an empty list. A link may appear more than once, since a route can traverse the
        /// same link twice.
        /// </summary>
        [JsonProperty("linkIds", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(DecodeArrayConverter))]
        public string[] LinkIds { get; set; }

        /// <summary>
        /// Unique identifier of this path.
        /// </summary>
        [JsonProperty("pathId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string PathId { get; set; }

        /// <summary>
        /// Whether this is a permanent or temporary path.
        /// </summary>
        [JsonProperty("permanent", NullValueHandling = NullValueHandling.Ignore)]
        public bool? Permanent { get; set; }

        /// <summary>
        /// Path type classification (e.g. bus route, diversion).
        /// </summary>
        [JsonProperty("typeId", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(FluffyMinMaxLengthCheckConverter))]
        public string TypeId { get; set; }
    }

    /// <summary>
    /// Road network turn (movement through a node between links).
    /// </summary>
    public partial class NetworkTurn
    {
        /// <summary>
        /// Whether this turn is permitted.
        /// </summary>
        [JsonProperty("allowed")]
        public bool Allowed { get; set; }

        /// <summary>
        /// Human-readable description.
        /// </summary>
        [JsonProperty("description", NullValueHandling = NullValueHandling.Ignore)]
        public string Description { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Approach link.
        /// </summary>
        [JsonProperty("fromLinkId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string FromLinkId { get; set; }

        /// <summary>
        /// Turn path geometry as GeoJSON LineString.
        /// </summary>
        [JsonProperty("geometry", NullValueHandling = NullValueHandling.Ignore)]
        public Geometry Geometry { get; set; }

        /// <summary>
        /// Departure link.
        /// </summary>
        [JsonProperty("toLinkId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string ToLinkId { get; set; }

        /// <summary>
        /// Unique identifier of this turn.
        /// </summary>
        [JsonProperty("turnId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string TurnId { get; set; }
    }

    /// <summary>
    /// Road network zone (area grouping of links/nodes).
    /// </summary>
    public partial class NetworkZone
    {
        /// <summary>
        /// Human-readable name.
        /// </summary>
        [JsonProperty("description", NullValueHandling = NullValueHandling.Ignore)]
        public string Description { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Zone boundary geometry as GeoJSON LineString.
        /// </summary>
        [JsonProperty("geometry", NullValueHandling = NullValueHandling.Ignore)]
        public Geometry Geometry { get; set; }

        /// <summary>
        /// The links that make up the zone, as a set: the order of the array carries no meaning (v1
        /// describes a zone as an unordered sequence of links). Omit the field rather than send an
        /// empty list. Each link appears at most once.
        /// </summary>
        [JsonProperty("linkIds", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(DecodeArrayConverter))]
        public string[] LinkIds { get; set; }

        /// <summary>
        /// Whether this is a permanent or temporary zone. Absent means permanent (the v1 default for
        /// a zone).
        /// </summary>
        [JsonProperty("permanent", NullValueHandling = NullValueHandling.Ignore)]
        public bool? Permanent { get; set; }

        /// <summary>
        /// Zone type classification (e.g. electoral ward, incident area).
        /// </summary>
        [JsonProperty("typeId", NullValueHandling = NullValueHandling.Ignore)]
        [JsonConverter(typeof(FluffyMinMaxLengthCheckConverter))]
        public string TypeId { get; set; }

        /// <summary>
        /// Unique identifier of this zone.
        /// </summary>
        [JsonProperty("zoneId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string ZoneId { get; set; }
    }

    /// <summary>
    /// Terminal message ending a network topology sync batch. Sent to the requester's reply node
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
        public Status Status { get; set; }

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
    /// Request the current state of network topology objects. The Data Publisher answers to the
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
        [JsonConverter(typeof(FluffyMinMaxLengthCheckConverter))]
        public string TypeId { get; set; }
    }

    public enum GeometryType { LineString };

    public enum LocationType { Point };

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
    public enum Status { Failed, Rejected, Success };

    internal static class Converter
    {
        public static readonly JsonSerializerSettings Settings = new JsonSerializerSettings
        {
            MetadataPropertyHandling = MetadataPropertyHandling.Ignore,
            DateParseHandling = DateParseHandling.None,
            Converters =
            {
                GeometryTypeConverter.Singleton,
                LocationTypeConverter.Singleton,
                SyncCategoryEnumConverter.Singleton,
                StatusConverter.Singleton,
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

    internal class GeometryTypeConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(GeometryType) || t == typeof(GeometryType?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            if (value == "LineString")
            {
                return GeometryType.LineString;
            }
            throw new Exception("Cannot unmarshal type GeometryType");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (GeometryType)untypedValue;
            if (value == GeometryType.LineString)
            {
                serializer.Serialize(writer, "LineString");
                return;
            }
            throw new Exception("Cannot marshal type GeometryType");
        }

        public static readonly GeometryTypeConverter Singleton = new GeometryTypeConverter();
    }

    internal class LocationTypeConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(LocationType) || t == typeof(LocationType?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            if (value == "Point")
            {
                return LocationType.Point;
            }
            throw new Exception("Cannot unmarshal type LocationType");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (LocationType)untypedValue;
            if (value == LocationType.Point)
            {
                serializer.Serialize(writer, "Point");
                return;
            }
            throw new Exception("Cannot marshal type LocationType");
        }

        public static readonly LocationTypeConverter Singleton = new LocationTypeConverter();
    }

    internal class MinMaxValueCheckConverter : JsonConverter
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

        public static readonly MinMaxValueCheckConverter Singleton = new MinMaxValueCheckConverter();
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

    internal class StatusConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(Status) || t == typeof(Status?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "failed":
                    return Status.Failed;
                case "rejected":
                    return Status.Rejected;
                case "success":
                    return Status.Success;
            }
            throw new Exception("Cannot unmarshal type Status");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (Status)untypedValue;
            switch (value)
            {
                case Status.Failed:
                    serializer.Serialize(writer, "failed");
                    return;
                case Status.Rejected:
                    serializer.Serialize(writer, "rejected");
                    return;
                case Status.Success:
                    serializer.Serialize(writer, "success");
                    return;
            }
            throw new Exception("Cannot marshal type Status");
        }

        public static readonly StatusConverter Singleton = new StatusConverter();
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
}

namespace Topas.Utmc.Network
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
