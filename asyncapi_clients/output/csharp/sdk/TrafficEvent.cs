/*
 * File:        output/csharp/sdk/TrafficEvent.cs
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
namespace Topas.Utmc.TrafficEvent
{
    using System;
    using System.Collections.Generic;

    using System.Globalization;
    using Newtonsoft.Json;
    using Newtonsoft.Json.Converters;

    public partial class AllTypes
    {
        [JsonProperty("AccidentEvent", NullValueHandling = NullValueHandling.Ignore)]
        public AccidentEvent AccidentEvent { get; set; }

        [JsonProperty("AccidentType", NullValueHandling = NullValueHandling.Ignore)]
        public AccidentType? AccidentType { get; set; }

        [JsonProperty("EventStatus", NullValueHandling = NullValueHandling.Ignore)]
        public EventStatusEnum? EventStatus { get; set; }

        [JsonProperty("IncidentEvent", NullValueHandling = NullValueHandling.Ignore)]
        public IncidentEvent IncidentEvent { get; set; }

        [JsonProperty("IncidentType", NullValueHandling = NullValueHandling.Ignore)]
        public IncidentType? IncidentType { get; set; }

        [JsonProperty("LanesAffectedType", NullValueHandling = NullValueHandling.Ignore)]
        public LanesAffected? LanesAffectedType { get; set; }

        [JsonProperty("PlannedEvent", NullValueHandling = NullValueHandling.Ignore)]
        public PlannedEvent PlannedEvent { get; set; }

        [JsonProperty("PlannedEventType", NullValueHandling = NullValueHandling.Ignore)]
        public PlannedEventType? PlannedEventType { get; set; }

        [JsonProperty("RoadworksEvent", NullValueHandling = NullValueHandling.Ignore)]
        public RoadworksEvent RoadworksEvent { get; set; }

        [JsonProperty("RoadworksType", NullValueHandling = NullValueHandling.Ignore)]
        public RoadworksType? RoadworksType { get; set; }

        [JsonProperty("SeverityType", NullValueHandling = NullValueHandling.Ignore)]
        public Severity? SeverityType { get; set; }

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
    /// Accident traffic event payload.
    /// </summary>
    public partial class AccidentEvent
    {
        /// <summary>
        /// When the accident was reported. Optional: unknown when first reported.
        /// </summary>
        [JsonProperty("accidentTime", NullValueHandling = NullValueHandling.Ignore)]
        public DateTimeOffset? AccidentTime { get; set; }

        /// <summary>
        /// Classification of the accident.
        /// </summary>
        [JsonProperty("accidentType")]
        public AccidentType AccidentType { get; set; }

        /// <summary>
        /// Who confirmed.
        /// </summary>
        [JsonProperty("confirmedBy", NullValueHandling = NullValueHandling.Ignore)]
        public string ConfirmedBy { get; set; }

        /// <summary>
        /// Whether diversion is active.
        /// </summary>
        [JsonProperty("diversionInForce", NullValueHandling = NullValueHandling.Ignore)]
        public bool? DiversionInForce { get; set; }

        /// <summary>
        /// Diversion route description.
        /// </summary>
        [JsonProperty("diversionRoute", NullValueHandling = NullValueHandling.Ignore)]
        public string DiversionRoute { get; set; }

        /// <summary>
        /// When cleared.
        /// </summary>
        [JsonProperty("endTime", NullValueHandling = NullValueHandling.Ignore)]
        public DateTimeOffset? EndTime { get; set; }

        /// <summary>
        /// Unique identifier of this event.
        /// </summary>
        [JsonProperty("eventId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string EventId { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Cross-ref country code (an XML NMTOKEN: no whitespace).
        /// </summary>
        [JsonProperty("externalCountryCode", NullValueHandling = NullValueHandling.Ignore)]
        public string ExternalCountryCode { get; set; }

        /// <summary>
        /// Cross-ref situation identifier.
        /// </summary>
        [JsonProperty("externalSituationId", NullValueHandling = NullValueHandling.Ignore)]
        public string ExternalSituationId { get; set; }

        /// <summary>
        /// Cross-ref situation version. Requires external_situation_id.
        /// </summary>
        [JsonProperty("externalSituationVersion", NullValueHandling = NullValueHandling.Ignore)]
        public long? ExternalSituationVersion { get; set; }

        /// <summary>
        /// Cross-ref system code.
        /// </summary>
        [JsonProperty("externalSystemCode", NullValueHandling = NullValueHandling.Ignore)]
        public string ExternalSystemCode { get; set; }

        /// <summary>
        /// Lane closure pattern (I=open, T=closed).
        /// </summary>
        [JsonProperty("lanesAffected", NullValueHandling = NullValueHandling.Ignore)]
        public LanesAffected? LanesAffected { get; set; }

        /// <summary>
        /// Count of operational lanes.
        /// </summary>
        [JsonProperty("lanesOperational", NullValueHandling = NullValueHandling.Ignore)]
        public long? LanesOperational { get; set; }

        /// <summary>
        /// Count of closed/restricted lanes.
        /// </summary>
        [JsonProperty("lanesRestricted", NullValueHandling = NullValueHandling.Ignore)]
        public long? LanesRestricted { get; set; }

        /// <summary>
        /// Light conditions at time.
        /// </summary>
        [JsonProperty("lightConditions", NullValueHandling = NullValueHandling.Ignore)]
        public string LightConditions { get; set; }

        /// <summary>
        /// WGS84 point location.
        /// </summary>
        [JsonProperty("location", NullValueHandling = NullValueHandling.Ignore)]
        public Location Location { get; set; }

        /// <summary>
        /// Free-text location description.
        /// </summary>
        [JsonProperty("locationDescription", NullValueHandling = NullValueHandling.Ignore)]
        public string LocationDescription { get; set; }

        /// <summary>
        /// Who last modified.
        /// </summary>
        [JsonProperty("modifiedBy", NullValueHandling = NullValueHandling.Ignore)]
        public string ModifiedBy { get; set; }

        /// <summary>
        /// Event phase (unconstrained).
        /// </summary>
        [JsonProperty("phase", NullValueHandling = NullValueHandling.Ignore)]
        public string Phase { get; set; }

        /// <summary>
        /// Public-facing summary for journey planners.
        /// </summary>
        [JsonProperty("publicDescription", NullValueHandling = NullValueHandling.Ignore)]
        public string PublicDescription { get; set; }

        /// <summary>
        /// Who reported the event.
        /// </summary>
        [JsonProperty("reportedBy", NullValueHandling = NullValueHandling.Ignore)]
        public string ReportedBy { get; set; }

        /// <summary>
        /// Road surface conditions.
        /// </summary>
        [JsonProperty("roadConditions", NullValueHandling = NullValueHandling.Ignore)]
        public string RoadConditions { get; set; }

        /// <summary>
        /// Event severity.
        /// </summary>
        [JsonProperty("severity", NullValueHandling = NullValueHandling.Ignore)]
        public Severity? Severity { get; set; }

        /// <summary>
        /// When statistics were collected.
        /// </summary>
        [JsonProperty("statisticDate", NullValueHandling = NullValueHandling.Ignore)]
        public DateTimeOffset? StatisticDate { get; set; }

        [JsonProperty("status")]
        public EventStatusEnum Status { get; set; }

        /// <summary>
        /// Number of vehicles involved.
        /// </summary>
        [JsonProperty("vehicleCount", NullValueHandling = NullValueHandling.Ignore)]
        public long? VehicleCount { get; set; }

        /// <summary>
        /// Weather conditions.
        /// </summary>
        [JsonProperty("weatherConditions", NullValueHandling = NullValueHandling.Ignore)]
        public string WeatherConditions { get; set; }
    }

    /// <summary>
    /// WGS84 point location.
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
    /// Incident traffic event payload.
    /// </summary>
    public partial class IncidentEvent
    {
        /// <summary>
        /// Who confirmed.
        /// </summary>
        [JsonProperty("confirmedBy", NullValueHandling = NullValueHandling.Ignore)]
        public string ConfirmedBy { get; set; }

        /// <summary>
        /// Whether diversion is active.
        /// </summary>
        [JsonProperty("diversionInForce", NullValueHandling = NullValueHandling.Ignore)]
        public bool? DiversionInForce { get; set; }

        /// <summary>
        /// Diversion route description.
        /// </summary>
        [JsonProperty("diversionRoute", NullValueHandling = NullValueHandling.Ignore)]
        public string DiversionRoute { get; set; }

        /// <summary>
        /// When cleared.
        /// </summary>
        [JsonProperty("endTime", NullValueHandling = NullValueHandling.Ignore)]
        public DateTimeOffset? EndTime { get; set; }

        /// <summary>
        /// Unique identifier of this event.
        /// </summary>
        [JsonProperty("eventId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string EventId { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Cross-ref country code (an XML NMTOKEN: no whitespace).
        /// </summary>
        [JsonProperty("externalCountryCode", NullValueHandling = NullValueHandling.Ignore)]
        public string ExternalCountryCode { get; set; }

        /// <summary>
        /// Cross-ref situation identifier.
        /// </summary>
        [JsonProperty("externalSituationId", NullValueHandling = NullValueHandling.Ignore)]
        public string ExternalSituationId { get; set; }

        /// <summary>
        /// Cross-ref situation version. Requires external_situation_id.
        /// </summary>
        [JsonProperty("externalSituationVersion", NullValueHandling = NullValueHandling.Ignore)]
        public long? ExternalSituationVersion { get; set; }

        /// <summary>
        /// Cross-ref system code.
        /// </summary>
        [JsonProperty("externalSystemCode", NullValueHandling = NullValueHandling.Ignore)]
        public string ExternalSystemCode { get; set; }

        /// <summary>
        /// When the incident was reported. Optional: unknown when first reported.
        /// </summary>
        [JsonProperty("incidentTime", NullValueHandling = NullValueHandling.Ignore)]
        public DateTimeOffset? IncidentTime { get; set; }

        /// <summary>
        /// Classification of the incident.
        /// </summary>
        [JsonProperty("incidentType")]
        public IncidentType IncidentType { get; set; }

        /// <summary>
        /// Lane closure pattern.
        /// </summary>
        [JsonProperty("lanesAffected", NullValueHandling = NullValueHandling.Ignore)]
        public LanesAffected? LanesAffected { get; set; }

        /// <summary>
        /// Count of operational lanes.
        /// </summary>
        [JsonProperty("lanesOperational", NullValueHandling = NullValueHandling.Ignore)]
        public long? LanesOperational { get; set; }

        /// <summary>
        /// Count of closed/restricted lanes.
        /// </summary>
        [JsonProperty("lanesRestricted", NullValueHandling = NullValueHandling.Ignore)]
        public long? LanesRestricted { get; set; }

        /// <summary>
        /// WGS84 point location.
        /// </summary>
        [JsonProperty("location", NullValueHandling = NullValueHandling.Ignore)]
        public Location Location { get; set; }

        /// <summary>
        /// Free-text location description.
        /// </summary>
        [JsonProperty("locationDescription", NullValueHandling = NullValueHandling.Ignore)]
        public string LocationDescription { get; set; }

        /// <summary>
        /// Who last modified.
        /// </summary>
        [JsonProperty("modifiedBy", NullValueHandling = NullValueHandling.Ignore)]
        public string ModifiedBy { get; set; }

        /// <summary>
        /// Event phase (unconstrained).
        /// </summary>
        [JsonProperty("phase", NullValueHandling = NullValueHandling.Ignore)]
        public string Phase { get; set; }

        /// <summary>
        /// Public-facing summary for journey planners.
        /// </summary>
        [JsonProperty("publicDescription", NullValueHandling = NullValueHandling.Ignore)]
        public string PublicDescription { get; set; }

        /// <summary>
        /// Who reported the event.
        /// </summary>
        [JsonProperty("reportedBy", NullValueHandling = NullValueHandling.Ignore)]
        public string ReportedBy { get; set; }

        /// <summary>
        /// Event severity.
        /// </summary>
        [JsonProperty("severity", NullValueHandling = NullValueHandling.Ignore)]
        public Severity? Severity { get; set; }

        [JsonProperty("status")]
        public EventStatusEnum Status { get; set; }
    }

    /// <summary>
    /// Planned event traffic event payload.
    /// </summary>
    public partial class PlannedEvent
    {
        /// <summary>
        /// Actual end time.
        /// </summary>
        [JsonProperty("actualEnd", NullValueHandling = NullValueHandling.Ignore)]
        public DateTimeOffset? ActualEnd { get; set; }

        /// <summary>
        /// Actual start time.
        /// </summary>
        [JsonProperty("actualStart", NullValueHandling = NullValueHandling.Ignore)]
        public DateTimeOffset? ActualStart { get; set; }

        /// <summary>
        /// Who confirmed.
        /// </summary>
        [JsonProperty("confirmedBy", NullValueHandling = NullValueHandling.Ignore)]
        public string ConfirmedBy { get; set; }

        /// <summary>
        /// Whether diversion is active.
        /// </summary>
        [JsonProperty("diversionInForce", NullValueHandling = NullValueHandling.Ignore)]
        public bool? DiversionInForce { get; set; }

        /// <summary>
        /// Diversion route description.
        /// </summary>
        [JsonProperty("diversionRoute", NullValueHandling = NullValueHandling.Ignore)]
        public string DiversionRoute { get; set; }

        /// <summary>
        /// Unique identifier of this event.
        /// </summary>
        [JsonProperty("eventId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string EventId { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Cross-ref country code (an XML NMTOKEN: no whitespace).
        /// </summary>
        [JsonProperty("externalCountryCode", NullValueHandling = NullValueHandling.Ignore)]
        public string ExternalCountryCode { get; set; }

        /// <summary>
        /// Cross-ref situation identifier.
        /// </summary>
        [JsonProperty("externalSituationId", NullValueHandling = NullValueHandling.Ignore)]
        public string ExternalSituationId { get; set; }

        /// <summary>
        /// Cross-ref situation version. Requires external_situation_id.
        /// </summary>
        [JsonProperty("externalSituationVersion", NullValueHandling = NullValueHandling.Ignore)]
        public long? ExternalSituationVersion { get; set; }

        /// <summary>
        /// Cross-ref system code.
        /// </summary>
        [JsonProperty("externalSystemCode", NullValueHandling = NullValueHandling.Ignore)]
        public string ExternalSystemCode { get; set; }

        /// <summary>
        /// Lane closure pattern.
        /// </summary>
        [JsonProperty("lanesAffected", NullValueHandling = NullValueHandling.Ignore)]
        public LanesAffected? LanesAffected { get; set; }

        /// <summary>
        /// Count of operational lanes.
        /// </summary>
        [JsonProperty("lanesOperational", NullValueHandling = NullValueHandling.Ignore)]
        public long? LanesOperational { get; set; }

        /// <summary>
        /// Count of closed/restricted lanes.
        /// </summary>
        [JsonProperty("lanesRestricted", NullValueHandling = NullValueHandling.Ignore)]
        public long? LanesRestricted { get; set; }

        /// <summary>
        /// WGS84 point location.
        /// </summary>
        [JsonProperty("location", NullValueHandling = NullValueHandling.Ignore)]
        public Location Location { get; set; }

        /// <summary>
        /// Free-text location description.
        /// </summary>
        [JsonProperty("locationDescription", NullValueHandling = NullValueHandling.Ignore)]
        public string LocationDescription { get; set; }

        /// <summary>
        /// Who last modified.
        /// </summary>
        [JsonProperty("modifiedBy", NullValueHandling = NullValueHandling.Ignore)]
        public string ModifiedBy { get; set; }

        /// <summary>
        /// Event organiser.
        /// </summary>
        [JsonProperty("organiser", NullValueHandling = NullValueHandling.Ignore)]
        public string Organiser { get; set; }

        /// <summary>
        /// Event phase (unconstrained).
        /// </summary>
        [JsonProperty("phase", NullValueHandling = NullValueHandling.Ignore)]
        public string Phase { get; set; }

        /// <summary>
        /// Planned end time.
        /// </summary>
        [JsonProperty("plannedEnd", NullValueHandling = NullValueHandling.Ignore)]
        public DateTimeOffset? PlannedEnd { get; set; }

        /// <summary>
        /// Classification of the event.
        /// </summary>
        [JsonProperty("plannedEventType")]
        public PlannedEventType PlannedEventType { get; set; }

        /// <summary>
        /// Planned start time.
        /// </summary>
        [JsonProperty("plannedStart", NullValueHandling = NullValueHandling.Ignore)]
        public DateTimeOffset? PlannedStart { get; set; }

        /// <summary>
        /// Public-facing summary for journey planners.
        /// </summary>
        [JsonProperty("publicDescription", NullValueHandling = NullValueHandling.Ignore)]
        public string PublicDescription { get; set; }

        /// <summary>
        /// Who reported the event.
        /// </summary>
        [JsonProperty("reportedBy", NullValueHandling = NullValueHandling.Ignore)]
        public string ReportedBy { get; set; }

        /// <summary>
        /// Event severity.
        /// </summary>
        [JsonProperty("severity", NullValueHandling = NullValueHandling.Ignore)]
        public Severity? Severity { get; set; }

        [JsonProperty("status")]
        public EventStatusEnum Status { get; set; }

        /// <summary>
        /// Venue name.
        /// </summary>
        [JsonProperty("venueName", NullValueHandling = NullValueHandling.Ignore)]
        public string VenueName { get; set; }
    }

    /// <summary>
    /// Roadworks traffic event payload.
    /// </summary>
    public partial class RoadworksEvent
    {
        /// <summary>
        /// Actual end time.
        /// </summary>
        [JsonProperty("actualEnd", NullValueHandling = NullValueHandling.Ignore)]
        public DateTimeOffset? ActualEnd { get; set; }

        /// <summary>
        /// Actual start time.
        /// </summary>
        [JsonProperty("actualStart", NullValueHandling = NullValueHandling.Ignore)]
        public DateTimeOffset? ActualStart { get; set; }

        /// <summary>
        /// Who confirmed.
        /// </summary>
        [JsonProperty("confirmedBy", NullValueHandling = NullValueHandling.Ignore)]
        public string ConfirmedBy { get; set; }

        /// <summary>
        /// When the roadworks were confirmed.
        /// </summary>
        [JsonProperty("confirmedDate", NullValueHandling = NullValueHandling.Ignore)]
        public DateTimeOffset? ConfirmedDate { get; set; }

        /// <summary>
        /// Contractor name.
        /// </summary>
        [JsonProperty("contractor", NullValueHandling = NullValueHandling.Ignore)]
        public string Contractor { get; set; }

        /// <summary>
        /// Whether contraflow is in operation.
        /// </summary>
        [JsonProperty("contraflow", NullValueHandling = NullValueHandling.Ignore)]
        public bool? Contraflow { get; set; }

        /// <summary>
        /// Whether diversion is active.
        /// </summary>
        [JsonProperty("diversionInForce", NullValueHandling = NullValueHandling.Ignore)]
        public bool? DiversionInForce { get; set; }

        /// <summary>
        /// Diversion route description.
        /// </summary>
        [JsonProperty("diversionRoute", NullValueHandling = NullValueHandling.Ignore)]
        public string DiversionRoute { get; set; }

        /// <summary>
        /// Unique identifier of this event.
        /// </summary>
        [JsonProperty("eventId")]
        [JsonConverter(typeof(PurpleMinMaxLengthCheckConverter))]
        public string EventId { get; set; }

        /// <summary>
        /// Vendor or local extensions.
        /// </summary>
        [JsonProperty("extensions", NullValueHandling = NullValueHandling.Ignore)]
        public Dictionary<string, object> Extensions { get; set; }

        /// <summary>
        /// Cross-ref country code (an XML NMTOKEN: no whitespace).
        /// </summary>
        [JsonProperty("externalCountryCode", NullValueHandling = NullValueHandling.Ignore)]
        public string ExternalCountryCode { get; set; }

        /// <summary>
        /// Cross-ref situation identifier.
        /// </summary>
        [JsonProperty("externalSituationId", NullValueHandling = NullValueHandling.Ignore)]
        public string ExternalSituationId { get; set; }

        /// <summary>
        /// Cross-ref situation version. Requires external_situation_id.
        /// </summary>
        [JsonProperty("externalSituationVersion", NullValueHandling = NullValueHandling.Ignore)]
        public long? ExternalSituationVersion { get; set; }

        /// <summary>
        /// Cross-ref system code.
        /// </summary>
        [JsonProperty("externalSystemCode", NullValueHandling = NullValueHandling.Ignore)]
        public string ExternalSystemCode { get; set; }

        /// <summary>
        /// Lane closure pattern.
        /// </summary>
        [JsonProperty("lanesAffected", NullValueHandling = NullValueHandling.Ignore)]
        public LanesAffected? LanesAffected { get; set; }

        /// <summary>
        /// Count of operational lanes.
        /// </summary>
        [JsonProperty("lanesOperational", NullValueHandling = NullValueHandling.Ignore)]
        public long? LanesOperational { get; set; }

        /// <summary>
        /// Count of closed/restricted lanes.
        /// </summary>
        [JsonProperty("lanesRestricted", NullValueHandling = NullValueHandling.Ignore)]
        public long? LanesRestricted { get; set; }

        /// <summary>
        /// WGS84 point location.
        /// </summary>
        [JsonProperty("location", NullValueHandling = NullValueHandling.Ignore)]
        public Location Location { get; set; }

        /// <summary>
        /// Free-text location description.
        /// </summary>
        [JsonProperty("locationDescription", NullValueHandling = NullValueHandling.Ignore)]
        public string LocationDescription { get; set; }

        /// <summary>
        /// Who last modified.
        /// </summary>
        [JsonProperty("modifiedBy", NullValueHandling = NullValueHandling.Ignore)]
        public string ModifiedBy { get; set; }

        /// <summary>
        /// Event phase (unconstrained).
        /// </summary>
        [JsonProperty("phase", NullValueHandling = NullValueHandling.Ignore)]
        public string Phase { get; set; }

        /// <summary>
        /// Planned end time.
        /// </summary>
        [JsonProperty("plannedEnd", NullValueHandling = NullValueHandling.Ignore)]
        public DateTimeOffset? PlannedEnd { get; set; }

        /// <summary>
        /// Planned start time.
        /// </summary>
        [JsonProperty("plannedStart", NullValueHandling = NullValueHandling.Ignore)]
        public DateTimeOffset? PlannedStart { get; set; }

        /// <summary>
        /// Public-facing summary for journey planners.
        /// </summary>
        [JsonProperty("publicDescription", NullValueHandling = NullValueHandling.Ignore)]
        public string PublicDescription { get; set; }

        /// <summary>
        /// Who reported the event.
        /// </summary>
        [JsonProperty("reportedBy", NullValueHandling = NullValueHandling.Ignore)]
        public string ReportedBy { get; set; }

        /// <summary>
        /// Classification of the roadworks.
        /// </summary>
        [JsonProperty("roadworksType")]
        public RoadworksType RoadworksType { get; set; }

        /// <summary>
        /// Event severity.
        /// </summary>
        [JsonProperty("severity", NullValueHandling = NullValueHandling.Ignore)]
        public Severity? Severity { get; set; }

        [JsonProperty("status")]
        public EventStatusEnum Status { get; set; }

        /// <summary>
        /// Whether temporary traffic signals are in use.
        /// </summary>
        [JsonProperty("trafficSignals", NullValueHandling = NullValueHandling.Ignore)]
        public bool? TrafficSignals { get; set; }
    }

    /// <summary>
    /// Terminal message ending a traffic event sync batch. Sent to the requester's reply node
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
    /// Request the current state of traffic event objects. The Data Publisher answers to the
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
    /// Classification of the accident.
    ///
    /// Classification of accident (20 values).
    ///
    /// 18 of the 20 named XSD values, plus ``other``, plus ``overturnedTrailer``,
    /// which has no XSD code. XSD codes 5 ("Accident History") and 20 ("Vehicle
    /// spun around") are not carried.
    /// </summary>
    public enum AccidentType { Accident, BusAccident, ChemicalSpillage, Collision, FuelSpillage, HazardousMaterials, HeavyLorry, InvestigationWork, JackKnifedArticulatedLorry, JackKnifedCaravan, JackKnifedTrailer, MultiVehicle, OilSpillage, Other, OverturnedHeavyLorry, OverturnedTrailer, OverturnedVehicle, SecondaryAccident, SeriousAccident, ShedLoad };

    /// <summary>
    /// Lane closure pattern (I=open, T=closed).
    ///
    /// Lane closure pattern.
    ///
    /// I = open lane, T = closed lane, read nearside to offside.
    ///
    /// Lane closure pattern.
    /// </summary>
    public enum LanesAffected { AllOpen, I, Ii, Iii, Iiii, Iiit, Iit, Iitt, It, Iti, Itt, Ittt, T, Ti, Tii, Tiii, Tit, Tt, Tti, Ttt, Tttt };

    /// <summary>
    /// Event severity.
    /// </summary>
    public enum Severity { High, Low, Medium, Unknown };

    /// <summary>
    /// Event lifecycle state.
    /// </summary>
    public enum EventStatusEnum { Active, Cleared, Planned, Suspended };

    /// <summary>
    /// Classification of the incident.
    ///
    /// Classification of incident (78 values from the UTMC specification).
    ///
    /// The XSD carries only a catalogue reference, so the value set is not in it.
    /// </summary>
    public enum IncidentType { AbnormalLoad, AirCrash, AirRaid, AnimalsOnRoad, AttackOnVehicle, Avalanche, Blasting, BridgeBlocked, BridgeClosed, BridgeDemolition, BridgeMaintenance, BridgeOpening, BrokenDownBus, BrokenDownLorry, BrokenDownVehicle, BurstPipe, BurstWaterMain, BusDisruption, BusLaneBlocked, BusLaneClosed, CentreLaneBlocked, CentreLaneClosed, ChemicalSpillage, ChildrenOnRoad, CivilEmergency, CivilEmergencyCancelled, Congestion, Contraflow, Convoy, Crowd, Cyclists, Diversion, EmergencyAlert, FacilitiesClosed, Fault, Fire, Flood, Flow, Fog, GawkingTraffic, Gunfire, Hail, HeavyFlow, Ice, LaneClosure, LightFault, Obstruction, Other, PeopleOnRoad, Pollution, PowerFailure, PublicDisturbance, Queue, RadioactiveLeak, Rain, Riot, RoadClosure, RoadsideFire, Sandstorms, SecurityAlert, SecurityIncident, Sightseers, SmogAlert, Snow, SpeedRestriction, Spillage, SprayHazard, StructuralDamage, TelephoneFault, TemporarySignal, TerroristIncident, Tornadoes, TrafficSignalFault, TrainDisruption, UnexplainedDamage, UnexplainedEvent, Washout, Wind };

    /// <summary>
    /// Classification of the event.
    ///
    /// Classification of planned event (32 values from the UTMC specification).
    ///
    /// The XSD carries only a catalogue reference, so the value set is not in it.
    /// </summary>
    public enum PlannedEventType { Athletics, BallGame, Boxing, CycleRace, Demonstration, Exhibition, Fair, Festival, Football, Funfair, Golf, InternationalSports, Marathon, March, Market, Match, Other, Parade, Procession, Race, Rugby, Show, ShowJumping, SportsEvent, SportsMeeting, StateOccasion, Strike, Tennis, Tournament, TradeFair, WaterSports, WinterSports };

    /// <summary>
    /// Classification of the roadworks.
    ///
    /// Classification of roadworks (7 values from XSD).
    /// </summary>
    public enum RoadworksType { CentralReservation, General, Other, Overhead, Surface, Verge, WaterMainWork };

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
                AccidentTypeConverter.Singleton,
                LanesAffectedConverter.Singleton,
                SeverityConverter.Singleton,
                EventStatusEnumConverter.Singleton,
                IncidentTypeConverter.Singleton,
                PlannedEventTypeConverter.Singleton,
                RoadworksTypeConverter.Singleton,
                SyncCategoryEnumConverter.Singleton,
                SyncStatusEnumConverter.Singleton,
                new IsoDateTimeConverter { DateTimeStyles = DateTimeStyles.AssumeUniversal }
            },
        };
    }

    internal class AccidentTypeConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(AccidentType) || t == typeof(AccidentType?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "accident":
                    return AccidentType.Accident;
                case "busAccident":
                    return AccidentType.BusAccident;
                case "chemicalSpillage":
                    return AccidentType.ChemicalSpillage;
                case "collision":
                    return AccidentType.Collision;
                case "fuelSpillage":
                    return AccidentType.FuelSpillage;
                case "hazardousMaterials":
                    return AccidentType.HazardousMaterials;
                case "heavyLorry":
                    return AccidentType.HeavyLorry;
                case "investigationWork":
                    return AccidentType.InvestigationWork;
                case "jackKnifedArticulatedLorry":
                    return AccidentType.JackKnifedArticulatedLorry;
                case "jackKnifedCaravan":
                    return AccidentType.JackKnifedCaravan;
                case "jackKnifedTrailer":
                    return AccidentType.JackKnifedTrailer;
                case "multiVehicle":
                    return AccidentType.MultiVehicle;
                case "oilSpillage":
                    return AccidentType.OilSpillage;
                case "other":
                    return AccidentType.Other;
                case "overturnedHeavyLorry":
                    return AccidentType.OverturnedHeavyLorry;
                case "overturnedTrailer":
                    return AccidentType.OverturnedTrailer;
                case "overturnedVehicle":
                    return AccidentType.OverturnedVehicle;
                case "secondaryAccident":
                    return AccidentType.SecondaryAccident;
                case "seriousAccident":
                    return AccidentType.SeriousAccident;
                case "shedLoad":
                    return AccidentType.ShedLoad;
            }
            throw new Exception("Cannot unmarshal type AccidentType");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (AccidentType)untypedValue;
            switch (value)
            {
                case AccidentType.Accident:
                    serializer.Serialize(writer, "accident");
                    return;
                case AccidentType.BusAccident:
                    serializer.Serialize(writer, "busAccident");
                    return;
                case AccidentType.ChemicalSpillage:
                    serializer.Serialize(writer, "chemicalSpillage");
                    return;
                case AccidentType.Collision:
                    serializer.Serialize(writer, "collision");
                    return;
                case AccidentType.FuelSpillage:
                    serializer.Serialize(writer, "fuelSpillage");
                    return;
                case AccidentType.HazardousMaterials:
                    serializer.Serialize(writer, "hazardousMaterials");
                    return;
                case AccidentType.HeavyLorry:
                    serializer.Serialize(writer, "heavyLorry");
                    return;
                case AccidentType.InvestigationWork:
                    serializer.Serialize(writer, "investigationWork");
                    return;
                case AccidentType.JackKnifedArticulatedLorry:
                    serializer.Serialize(writer, "jackKnifedArticulatedLorry");
                    return;
                case AccidentType.JackKnifedCaravan:
                    serializer.Serialize(writer, "jackKnifedCaravan");
                    return;
                case AccidentType.JackKnifedTrailer:
                    serializer.Serialize(writer, "jackKnifedTrailer");
                    return;
                case AccidentType.MultiVehicle:
                    serializer.Serialize(writer, "multiVehicle");
                    return;
                case AccidentType.OilSpillage:
                    serializer.Serialize(writer, "oilSpillage");
                    return;
                case AccidentType.Other:
                    serializer.Serialize(writer, "other");
                    return;
                case AccidentType.OverturnedHeavyLorry:
                    serializer.Serialize(writer, "overturnedHeavyLorry");
                    return;
                case AccidentType.OverturnedTrailer:
                    serializer.Serialize(writer, "overturnedTrailer");
                    return;
                case AccidentType.OverturnedVehicle:
                    serializer.Serialize(writer, "overturnedVehicle");
                    return;
                case AccidentType.SecondaryAccident:
                    serializer.Serialize(writer, "secondaryAccident");
                    return;
                case AccidentType.SeriousAccident:
                    serializer.Serialize(writer, "seriousAccident");
                    return;
                case AccidentType.ShedLoad:
                    serializer.Serialize(writer, "shedLoad");
                    return;
            }
            throw new Exception("Cannot marshal type AccidentType");
        }

        public static readonly AccidentTypeConverter Singleton = new AccidentTypeConverter();
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

    internal class LanesAffectedConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(LanesAffected) || t == typeof(LanesAffected?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "I":
                    return LanesAffected.I;
                case "II":
                    return LanesAffected.Ii;
                case "III":
                    return LanesAffected.Iii;
                case "IIII":
                    return LanesAffected.Iiii;
                case "IIIT":
                    return LanesAffected.Iiit;
                case "IIT":
                    return LanesAffected.Iit;
                case "IITT":
                    return LanesAffected.Iitt;
                case "IT":
                    return LanesAffected.It;
                case "ITI":
                    return LanesAffected.Iti;
                case "ITT":
                    return LanesAffected.Itt;
                case "ITTT":
                    return LanesAffected.Ittt;
                case "T":
                    return LanesAffected.T;
                case "TI":
                    return LanesAffected.Ti;
                case "TII":
                    return LanesAffected.Tii;
                case "TIII":
                    return LanesAffected.Tiii;
                case "TIT":
                    return LanesAffected.Tit;
                case "TT":
                    return LanesAffected.Tt;
                case "TTI":
                    return LanesAffected.Tti;
                case "TTT":
                    return LanesAffected.Ttt;
                case "TTTT":
                    return LanesAffected.Tttt;
                case "allOpen":
                    return LanesAffected.AllOpen;
            }
            throw new Exception("Cannot unmarshal type LanesAffected");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (LanesAffected)untypedValue;
            switch (value)
            {
                case LanesAffected.I:
                    serializer.Serialize(writer, "I");
                    return;
                case LanesAffected.Ii:
                    serializer.Serialize(writer, "II");
                    return;
                case LanesAffected.Iii:
                    serializer.Serialize(writer, "III");
                    return;
                case LanesAffected.Iiii:
                    serializer.Serialize(writer, "IIII");
                    return;
                case LanesAffected.Iiit:
                    serializer.Serialize(writer, "IIIT");
                    return;
                case LanesAffected.Iit:
                    serializer.Serialize(writer, "IIT");
                    return;
                case LanesAffected.Iitt:
                    serializer.Serialize(writer, "IITT");
                    return;
                case LanesAffected.It:
                    serializer.Serialize(writer, "IT");
                    return;
                case LanesAffected.Iti:
                    serializer.Serialize(writer, "ITI");
                    return;
                case LanesAffected.Itt:
                    serializer.Serialize(writer, "ITT");
                    return;
                case LanesAffected.Ittt:
                    serializer.Serialize(writer, "ITTT");
                    return;
                case LanesAffected.T:
                    serializer.Serialize(writer, "T");
                    return;
                case LanesAffected.Ti:
                    serializer.Serialize(writer, "TI");
                    return;
                case LanesAffected.Tii:
                    serializer.Serialize(writer, "TII");
                    return;
                case LanesAffected.Tiii:
                    serializer.Serialize(writer, "TIII");
                    return;
                case LanesAffected.Tit:
                    serializer.Serialize(writer, "TIT");
                    return;
                case LanesAffected.Tt:
                    serializer.Serialize(writer, "TT");
                    return;
                case LanesAffected.Tti:
                    serializer.Serialize(writer, "TTI");
                    return;
                case LanesAffected.Ttt:
                    serializer.Serialize(writer, "TTT");
                    return;
                case LanesAffected.Tttt:
                    serializer.Serialize(writer, "TTTT");
                    return;
                case LanesAffected.AllOpen:
                    serializer.Serialize(writer, "allOpen");
                    return;
            }
            throw new Exception("Cannot marshal type LanesAffected");
        }

        public static readonly LanesAffectedConverter Singleton = new LanesAffectedConverter();
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

    internal class SeverityConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(Severity) || t == typeof(Severity?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "high":
                    return Severity.High;
                case "low":
                    return Severity.Low;
                case "medium":
                    return Severity.Medium;
                case "unknown":
                    return Severity.Unknown;
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
                case Severity.High:
                    serializer.Serialize(writer, "high");
                    return;
                case Severity.Low:
                    serializer.Serialize(writer, "low");
                    return;
                case Severity.Medium:
                    serializer.Serialize(writer, "medium");
                    return;
                case Severity.Unknown:
                    serializer.Serialize(writer, "unknown");
                    return;
            }
            throw new Exception("Cannot marshal type Severity");
        }

        public static readonly SeverityConverter Singleton = new SeverityConverter();
    }

    internal class EventStatusEnumConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(EventStatusEnum) || t == typeof(EventStatusEnum?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "active":
                    return EventStatusEnum.Active;
                case "cleared":
                    return EventStatusEnum.Cleared;
                case "planned":
                    return EventStatusEnum.Planned;
                case "suspended":
                    return EventStatusEnum.Suspended;
            }
            throw new Exception("Cannot unmarshal type EventStatusEnum");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (EventStatusEnum)untypedValue;
            switch (value)
            {
                case EventStatusEnum.Active:
                    serializer.Serialize(writer, "active");
                    return;
                case EventStatusEnum.Cleared:
                    serializer.Serialize(writer, "cleared");
                    return;
                case EventStatusEnum.Planned:
                    serializer.Serialize(writer, "planned");
                    return;
                case EventStatusEnum.Suspended:
                    serializer.Serialize(writer, "suspended");
                    return;
            }
            throw new Exception("Cannot marshal type EventStatusEnum");
        }

        public static readonly EventStatusEnumConverter Singleton = new EventStatusEnumConverter();
    }

    internal class IncidentTypeConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(IncidentType) || t == typeof(IncidentType?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "abnormalLoad":
                    return IncidentType.AbnormalLoad;
                case "airCrash":
                    return IncidentType.AirCrash;
                case "airRaid":
                    return IncidentType.AirRaid;
                case "animalsOnRoad":
                    return IncidentType.AnimalsOnRoad;
                case "attackOnVehicle":
                    return IncidentType.AttackOnVehicle;
                case "avalanche":
                    return IncidentType.Avalanche;
                case "blasting":
                    return IncidentType.Blasting;
                case "bridgeBlocked":
                    return IncidentType.BridgeBlocked;
                case "bridgeClosed":
                    return IncidentType.BridgeClosed;
                case "bridgeDemolition":
                    return IncidentType.BridgeDemolition;
                case "bridgeMaintenance":
                    return IncidentType.BridgeMaintenance;
                case "bridgeOpening":
                    return IncidentType.BridgeOpening;
                case "brokenDownBus":
                    return IncidentType.BrokenDownBus;
                case "brokenDownLorry":
                    return IncidentType.BrokenDownLorry;
                case "brokenDownVehicle":
                    return IncidentType.BrokenDownVehicle;
                case "burstPipe":
                    return IncidentType.BurstPipe;
                case "burstWaterMain":
                    return IncidentType.BurstWaterMain;
                case "busDisruption":
                    return IncidentType.BusDisruption;
                case "busLaneBlocked":
                    return IncidentType.BusLaneBlocked;
                case "busLaneClosed":
                    return IncidentType.BusLaneClosed;
                case "centreLaneBlocked":
                    return IncidentType.CentreLaneBlocked;
                case "centreLaneClosed":
                    return IncidentType.CentreLaneClosed;
                case "chemicalSpillage":
                    return IncidentType.ChemicalSpillage;
                case "childrenOnRoad":
                    return IncidentType.ChildrenOnRoad;
                case "civilEmergency":
                    return IncidentType.CivilEmergency;
                case "civilEmergencyCancelled":
                    return IncidentType.CivilEmergencyCancelled;
                case "congestion":
                    return IncidentType.Congestion;
                case "contraflow":
                    return IncidentType.Contraflow;
                case "convoy":
                    return IncidentType.Convoy;
                case "crowd":
                    return IncidentType.Crowd;
                case "cyclists":
                    return IncidentType.Cyclists;
                case "diversion":
                    return IncidentType.Diversion;
                case "emergencyAlert":
                    return IncidentType.EmergencyAlert;
                case "facilitiesClosed":
                    return IncidentType.FacilitiesClosed;
                case "fault":
                    return IncidentType.Fault;
                case "fire":
                    return IncidentType.Fire;
                case "flood":
                    return IncidentType.Flood;
                case "flow":
                    return IncidentType.Flow;
                case "fog":
                    return IncidentType.Fog;
                case "gawkingTraffic":
                    return IncidentType.GawkingTraffic;
                case "gunfire":
                    return IncidentType.Gunfire;
                case "hail":
                    return IncidentType.Hail;
                case "heavyFlow":
                    return IncidentType.HeavyFlow;
                case "ice":
                    return IncidentType.Ice;
                case "laneClosure":
                    return IncidentType.LaneClosure;
                case "lightFault":
                    return IncidentType.LightFault;
                case "obstruction":
                    return IncidentType.Obstruction;
                case "other":
                    return IncidentType.Other;
                case "peopleOnRoad":
                    return IncidentType.PeopleOnRoad;
                case "pollution":
                    return IncidentType.Pollution;
                case "powerFailure":
                    return IncidentType.PowerFailure;
                case "publicDisturbance":
                    return IncidentType.PublicDisturbance;
                case "queue":
                    return IncidentType.Queue;
                case "radioactiveLeak":
                    return IncidentType.RadioactiveLeak;
                case "rain":
                    return IncidentType.Rain;
                case "riot":
                    return IncidentType.Riot;
                case "roadClosure":
                    return IncidentType.RoadClosure;
                case "roadsideFire":
                    return IncidentType.RoadsideFire;
                case "sandstorms":
                    return IncidentType.Sandstorms;
                case "securityAlert":
                    return IncidentType.SecurityAlert;
                case "securityIncident":
                    return IncidentType.SecurityIncident;
                case "sightseers":
                    return IncidentType.Sightseers;
                case "smogAlert":
                    return IncidentType.SmogAlert;
                case "snow":
                    return IncidentType.Snow;
                case "speedRestriction":
                    return IncidentType.SpeedRestriction;
                case "spillage":
                    return IncidentType.Spillage;
                case "sprayHazard":
                    return IncidentType.SprayHazard;
                case "structuralDamage":
                    return IncidentType.StructuralDamage;
                case "telephoneFault":
                    return IncidentType.TelephoneFault;
                case "temporarySignal":
                    return IncidentType.TemporarySignal;
                case "terroristIncident":
                    return IncidentType.TerroristIncident;
                case "tornadoes":
                    return IncidentType.Tornadoes;
                case "trafficSignalFault":
                    return IncidentType.TrafficSignalFault;
                case "trainDisruption":
                    return IncidentType.TrainDisruption;
                case "unexplainedDamage":
                    return IncidentType.UnexplainedDamage;
                case "unexplainedEvent":
                    return IncidentType.UnexplainedEvent;
                case "washout":
                    return IncidentType.Washout;
                case "wind":
                    return IncidentType.Wind;
            }
            throw new Exception("Cannot unmarshal type IncidentType");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (IncidentType)untypedValue;
            switch (value)
            {
                case IncidentType.AbnormalLoad:
                    serializer.Serialize(writer, "abnormalLoad");
                    return;
                case IncidentType.AirCrash:
                    serializer.Serialize(writer, "airCrash");
                    return;
                case IncidentType.AirRaid:
                    serializer.Serialize(writer, "airRaid");
                    return;
                case IncidentType.AnimalsOnRoad:
                    serializer.Serialize(writer, "animalsOnRoad");
                    return;
                case IncidentType.AttackOnVehicle:
                    serializer.Serialize(writer, "attackOnVehicle");
                    return;
                case IncidentType.Avalanche:
                    serializer.Serialize(writer, "avalanche");
                    return;
                case IncidentType.Blasting:
                    serializer.Serialize(writer, "blasting");
                    return;
                case IncidentType.BridgeBlocked:
                    serializer.Serialize(writer, "bridgeBlocked");
                    return;
                case IncidentType.BridgeClosed:
                    serializer.Serialize(writer, "bridgeClosed");
                    return;
                case IncidentType.BridgeDemolition:
                    serializer.Serialize(writer, "bridgeDemolition");
                    return;
                case IncidentType.BridgeMaintenance:
                    serializer.Serialize(writer, "bridgeMaintenance");
                    return;
                case IncidentType.BridgeOpening:
                    serializer.Serialize(writer, "bridgeOpening");
                    return;
                case IncidentType.BrokenDownBus:
                    serializer.Serialize(writer, "brokenDownBus");
                    return;
                case IncidentType.BrokenDownLorry:
                    serializer.Serialize(writer, "brokenDownLorry");
                    return;
                case IncidentType.BrokenDownVehicle:
                    serializer.Serialize(writer, "brokenDownVehicle");
                    return;
                case IncidentType.BurstPipe:
                    serializer.Serialize(writer, "burstPipe");
                    return;
                case IncidentType.BurstWaterMain:
                    serializer.Serialize(writer, "burstWaterMain");
                    return;
                case IncidentType.BusDisruption:
                    serializer.Serialize(writer, "busDisruption");
                    return;
                case IncidentType.BusLaneBlocked:
                    serializer.Serialize(writer, "busLaneBlocked");
                    return;
                case IncidentType.BusLaneClosed:
                    serializer.Serialize(writer, "busLaneClosed");
                    return;
                case IncidentType.CentreLaneBlocked:
                    serializer.Serialize(writer, "centreLaneBlocked");
                    return;
                case IncidentType.CentreLaneClosed:
                    serializer.Serialize(writer, "centreLaneClosed");
                    return;
                case IncidentType.ChemicalSpillage:
                    serializer.Serialize(writer, "chemicalSpillage");
                    return;
                case IncidentType.ChildrenOnRoad:
                    serializer.Serialize(writer, "childrenOnRoad");
                    return;
                case IncidentType.CivilEmergency:
                    serializer.Serialize(writer, "civilEmergency");
                    return;
                case IncidentType.CivilEmergencyCancelled:
                    serializer.Serialize(writer, "civilEmergencyCancelled");
                    return;
                case IncidentType.Congestion:
                    serializer.Serialize(writer, "congestion");
                    return;
                case IncidentType.Contraflow:
                    serializer.Serialize(writer, "contraflow");
                    return;
                case IncidentType.Convoy:
                    serializer.Serialize(writer, "convoy");
                    return;
                case IncidentType.Crowd:
                    serializer.Serialize(writer, "crowd");
                    return;
                case IncidentType.Cyclists:
                    serializer.Serialize(writer, "cyclists");
                    return;
                case IncidentType.Diversion:
                    serializer.Serialize(writer, "diversion");
                    return;
                case IncidentType.EmergencyAlert:
                    serializer.Serialize(writer, "emergencyAlert");
                    return;
                case IncidentType.FacilitiesClosed:
                    serializer.Serialize(writer, "facilitiesClosed");
                    return;
                case IncidentType.Fault:
                    serializer.Serialize(writer, "fault");
                    return;
                case IncidentType.Fire:
                    serializer.Serialize(writer, "fire");
                    return;
                case IncidentType.Flood:
                    serializer.Serialize(writer, "flood");
                    return;
                case IncidentType.Flow:
                    serializer.Serialize(writer, "flow");
                    return;
                case IncidentType.Fog:
                    serializer.Serialize(writer, "fog");
                    return;
                case IncidentType.GawkingTraffic:
                    serializer.Serialize(writer, "gawkingTraffic");
                    return;
                case IncidentType.Gunfire:
                    serializer.Serialize(writer, "gunfire");
                    return;
                case IncidentType.Hail:
                    serializer.Serialize(writer, "hail");
                    return;
                case IncidentType.HeavyFlow:
                    serializer.Serialize(writer, "heavyFlow");
                    return;
                case IncidentType.Ice:
                    serializer.Serialize(writer, "ice");
                    return;
                case IncidentType.LaneClosure:
                    serializer.Serialize(writer, "laneClosure");
                    return;
                case IncidentType.LightFault:
                    serializer.Serialize(writer, "lightFault");
                    return;
                case IncidentType.Obstruction:
                    serializer.Serialize(writer, "obstruction");
                    return;
                case IncidentType.Other:
                    serializer.Serialize(writer, "other");
                    return;
                case IncidentType.PeopleOnRoad:
                    serializer.Serialize(writer, "peopleOnRoad");
                    return;
                case IncidentType.Pollution:
                    serializer.Serialize(writer, "pollution");
                    return;
                case IncidentType.PowerFailure:
                    serializer.Serialize(writer, "powerFailure");
                    return;
                case IncidentType.PublicDisturbance:
                    serializer.Serialize(writer, "publicDisturbance");
                    return;
                case IncidentType.Queue:
                    serializer.Serialize(writer, "queue");
                    return;
                case IncidentType.RadioactiveLeak:
                    serializer.Serialize(writer, "radioactiveLeak");
                    return;
                case IncidentType.Rain:
                    serializer.Serialize(writer, "rain");
                    return;
                case IncidentType.Riot:
                    serializer.Serialize(writer, "riot");
                    return;
                case IncidentType.RoadClosure:
                    serializer.Serialize(writer, "roadClosure");
                    return;
                case IncidentType.RoadsideFire:
                    serializer.Serialize(writer, "roadsideFire");
                    return;
                case IncidentType.Sandstorms:
                    serializer.Serialize(writer, "sandstorms");
                    return;
                case IncidentType.SecurityAlert:
                    serializer.Serialize(writer, "securityAlert");
                    return;
                case IncidentType.SecurityIncident:
                    serializer.Serialize(writer, "securityIncident");
                    return;
                case IncidentType.Sightseers:
                    serializer.Serialize(writer, "sightseers");
                    return;
                case IncidentType.SmogAlert:
                    serializer.Serialize(writer, "smogAlert");
                    return;
                case IncidentType.Snow:
                    serializer.Serialize(writer, "snow");
                    return;
                case IncidentType.SpeedRestriction:
                    serializer.Serialize(writer, "speedRestriction");
                    return;
                case IncidentType.Spillage:
                    serializer.Serialize(writer, "spillage");
                    return;
                case IncidentType.SprayHazard:
                    serializer.Serialize(writer, "sprayHazard");
                    return;
                case IncidentType.StructuralDamage:
                    serializer.Serialize(writer, "structuralDamage");
                    return;
                case IncidentType.TelephoneFault:
                    serializer.Serialize(writer, "telephoneFault");
                    return;
                case IncidentType.TemporarySignal:
                    serializer.Serialize(writer, "temporarySignal");
                    return;
                case IncidentType.TerroristIncident:
                    serializer.Serialize(writer, "terroristIncident");
                    return;
                case IncidentType.Tornadoes:
                    serializer.Serialize(writer, "tornadoes");
                    return;
                case IncidentType.TrafficSignalFault:
                    serializer.Serialize(writer, "trafficSignalFault");
                    return;
                case IncidentType.TrainDisruption:
                    serializer.Serialize(writer, "trainDisruption");
                    return;
                case IncidentType.UnexplainedDamage:
                    serializer.Serialize(writer, "unexplainedDamage");
                    return;
                case IncidentType.UnexplainedEvent:
                    serializer.Serialize(writer, "unexplainedEvent");
                    return;
                case IncidentType.Washout:
                    serializer.Serialize(writer, "washout");
                    return;
                case IncidentType.Wind:
                    serializer.Serialize(writer, "wind");
                    return;
            }
            throw new Exception("Cannot marshal type IncidentType");
        }

        public static readonly IncidentTypeConverter Singleton = new IncidentTypeConverter();
    }

    internal class PlannedEventTypeConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(PlannedEventType) || t == typeof(PlannedEventType?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "athletics":
                    return PlannedEventType.Athletics;
                case "ballGame":
                    return PlannedEventType.BallGame;
                case "boxing":
                    return PlannedEventType.Boxing;
                case "cycleRace":
                    return PlannedEventType.CycleRace;
                case "demonstration":
                    return PlannedEventType.Demonstration;
                case "exhibition":
                    return PlannedEventType.Exhibition;
                case "fair":
                    return PlannedEventType.Fair;
                case "festival":
                    return PlannedEventType.Festival;
                case "football":
                    return PlannedEventType.Football;
                case "funfair":
                    return PlannedEventType.Funfair;
                case "golf":
                    return PlannedEventType.Golf;
                case "internationalSports":
                    return PlannedEventType.InternationalSports;
                case "marathon":
                    return PlannedEventType.Marathon;
                case "march":
                    return PlannedEventType.March;
                case "market":
                    return PlannedEventType.Market;
                case "match":
                    return PlannedEventType.Match;
                case "other":
                    return PlannedEventType.Other;
                case "parade":
                    return PlannedEventType.Parade;
                case "procession":
                    return PlannedEventType.Procession;
                case "race":
                    return PlannedEventType.Race;
                case "rugby":
                    return PlannedEventType.Rugby;
                case "show":
                    return PlannedEventType.Show;
                case "showJumping":
                    return PlannedEventType.ShowJumping;
                case "sportsEvent":
                    return PlannedEventType.SportsEvent;
                case "sportsMeeting":
                    return PlannedEventType.SportsMeeting;
                case "stateOccasion":
                    return PlannedEventType.StateOccasion;
                case "strike":
                    return PlannedEventType.Strike;
                case "tennis":
                    return PlannedEventType.Tennis;
                case "tournament":
                    return PlannedEventType.Tournament;
                case "tradeFair":
                    return PlannedEventType.TradeFair;
                case "waterSports":
                    return PlannedEventType.WaterSports;
                case "winterSports":
                    return PlannedEventType.WinterSports;
            }
            throw new Exception("Cannot unmarshal type PlannedEventType");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (PlannedEventType)untypedValue;
            switch (value)
            {
                case PlannedEventType.Athletics:
                    serializer.Serialize(writer, "athletics");
                    return;
                case PlannedEventType.BallGame:
                    serializer.Serialize(writer, "ballGame");
                    return;
                case PlannedEventType.Boxing:
                    serializer.Serialize(writer, "boxing");
                    return;
                case PlannedEventType.CycleRace:
                    serializer.Serialize(writer, "cycleRace");
                    return;
                case PlannedEventType.Demonstration:
                    serializer.Serialize(writer, "demonstration");
                    return;
                case PlannedEventType.Exhibition:
                    serializer.Serialize(writer, "exhibition");
                    return;
                case PlannedEventType.Fair:
                    serializer.Serialize(writer, "fair");
                    return;
                case PlannedEventType.Festival:
                    serializer.Serialize(writer, "festival");
                    return;
                case PlannedEventType.Football:
                    serializer.Serialize(writer, "football");
                    return;
                case PlannedEventType.Funfair:
                    serializer.Serialize(writer, "funfair");
                    return;
                case PlannedEventType.Golf:
                    serializer.Serialize(writer, "golf");
                    return;
                case PlannedEventType.InternationalSports:
                    serializer.Serialize(writer, "internationalSports");
                    return;
                case PlannedEventType.Marathon:
                    serializer.Serialize(writer, "marathon");
                    return;
                case PlannedEventType.March:
                    serializer.Serialize(writer, "march");
                    return;
                case PlannedEventType.Market:
                    serializer.Serialize(writer, "market");
                    return;
                case PlannedEventType.Match:
                    serializer.Serialize(writer, "match");
                    return;
                case PlannedEventType.Other:
                    serializer.Serialize(writer, "other");
                    return;
                case PlannedEventType.Parade:
                    serializer.Serialize(writer, "parade");
                    return;
                case PlannedEventType.Procession:
                    serializer.Serialize(writer, "procession");
                    return;
                case PlannedEventType.Race:
                    serializer.Serialize(writer, "race");
                    return;
                case PlannedEventType.Rugby:
                    serializer.Serialize(writer, "rugby");
                    return;
                case PlannedEventType.Show:
                    serializer.Serialize(writer, "show");
                    return;
                case PlannedEventType.ShowJumping:
                    serializer.Serialize(writer, "showJumping");
                    return;
                case PlannedEventType.SportsEvent:
                    serializer.Serialize(writer, "sportsEvent");
                    return;
                case PlannedEventType.SportsMeeting:
                    serializer.Serialize(writer, "sportsMeeting");
                    return;
                case PlannedEventType.StateOccasion:
                    serializer.Serialize(writer, "stateOccasion");
                    return;
                case PlannedEventType.Strike:
                    serializer.Serialize(writer, "strike");
                    return;
                case PlannedEventType.Tennis:
                    serializer.Serialize(writer, "tennis");
                    return;
                case PlannedEventType.Tournament:
                    serializer.Serialize(writer, "tournament");
                    return;
                case PlannedEventType.TradeFair:
                    serializer.Serialize(writer, "tradeFair");
                    return;
                case PlannedEventType.WaterSports:
                    serializer.Serialize(writer, "waterSports");
                    return;
                case PlannedEventType.WinterSports:
                    serializer.Serialize(writer, "winterSports");
                    return;
            }
            throw new Exception("Cannot marshal type PlannedEventType");
        }

        public static readonly PlannedEventTypeConverter Singleton = new PlannedEventTypeConverter();
    }

    internal class RoadworksTypeConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(RoadworksType) || t == typeof(RoadworksType?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "centralReservation":
                    return RoadworksType.CentralReservation;
                case "general":
                    return RoadworksType.General;
                case "other":
                    return RoadworksType.Other;
                case "overhead":
                    return RoadworksType.Overhead;
                case "surface":
                    return RoadworksType.Surface;
                case "verge":
                    return RoadworksType.Verge;
                case "waterMainWork":
                    return RoadworksType.WaterMainWork;
            }
            throw new Exception("Cannot unmarshal type RoadworksType");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (RoadworksType)untypedValue;
            switch (value)
            {
                case RoadworksType.CentralReservation:
                    serializer.Serialize(writer, "centralReservation");
                    return;
                case RoadworksType.General:
                    serializer.Serialize(writer, "general");
                    return;
                case RoadworksType.Other:
                    serializer.Serialize(writer, "other");
                    return;
                case RoadworksType.Overhead:
                    serializer.Serialize(writer, "overhead");
                    return;
                case RoadworksType.Surface:
                    serializer.Serialize(writer, "surface");
                    return;
                case RoadworksType.Verge:
                    serializer.Serialize(writer, "verge");
                    return;
                case RoadworksType.WaterMainWork:
                    serializer.Serialize(writer, "waterMainWork");
                    return;
            }
            throw new Exception("Cannot marshal type RoadworksType");
        }

        public static readonly RoadworksTypeConverter Singleton = new RoadworksTypeConverter();
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
}

namespace Topas.Utmc.TrafficEvent
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
