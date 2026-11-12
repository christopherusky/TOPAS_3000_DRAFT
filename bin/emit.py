# ──────────────────────────────────────────────────────────────────────
# Origin:     Cross-cutting emit pipeline for TOPAS 3000A
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-05
# Version:    1.4
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""JSON Schema emit pipeline.

Walks the Pydantic intermediate model and writes one JSON Schema file per
(domain, category) into Schemas/. Each file has a single canonical
top-level type (or is a library file with just $defs), refs to shared
types are rewritten as cross-file $refs, and $defs are sorted
alphabetically for stable diffs.

Usage: python bin/emit.py
"""

from __future__ import annotations

import json
import sys
from copy import deepcopy
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

from pydantic import TypeAdapter

# Make importable when running as a script
ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))

from bin.scope import ScopeError, load_scope  # noqa: E402

from topas_3000_model.car_park.command import (  # noqa: E402
    CarParkCommand,
    RequestCarParkStatusCommand,
    SetCapacityCommand,
    SetOccupancyCommand,
    SetThresholdsCommand,
)
from topas_3000_model.car_park.definition import (  # noqa: E402
    CarParkAccessLocation,
    CarParkConfiguration,
    CarParkDefinition,
    CarParkDetectorsConfiguration,
    CarParkOpeningTimesConfiguration,
    CarParkTariffConfiguration,
    CarParkZoneListEntry,
)
from topas_3000_model.car_park.fault import CarParkFault  # noqa: E402
from topas_3000_model.car_park.history import CarParkHistoryEntry  # noqa: E402
from topas_3000_model.car_park.measurement import CarParkDynamic  # noqa: E402
from topas_3000_model.car_park.type_id import CarParkType  # noqa: E402
from topas_3000_model.common.command import CommandResponseStatus  # noqa: E402
from topas_3000_model.common.day_type import DateTypeDefinition, DayTypeDefinition  # noqa: E402
from topas_3000_model.common.location import Coordinates, Point  # noqa: E402
from topas_3000_model.common.quality import Quality  # noqa: E402
from topas_3000_model.common.subsystem import SubsystemDefinition  # noqa: E402
from topas_3000_model.car_park.response import CarParkCommandResponse  # noqa: E402
from topas_3000_model.access_control.command import (  # noqa: E402
    AccessControlCommand,
    DisableAccessCommand,
    EnableAccessCommand,
)
from topas_3000_model.air_quality.command import (  # noqa: E402
    AirQualityCommand,
    RequestAirQualityStatusCommand,
)
from topas_3000_model.air_quality.definition import (  # noqa: E402
    AirQualityConfiguration,
    AirQualityDefinition,
)
from topas_3000_model.air_quality.fault import AirQualityFault  # noqa: E402
from topas_3000_model.air_quality.history import AirQualityHistoryEntry  # noqa: E402
from topas_3000_model.air_quality.measurement import AirQualityDynamic  # noqa: E402
from topas_3000_model.air_quality.response import AirQualityCommandResponse  # noqa: E402
from topas_3000_model.air_quality.type_id import AirQualityType  # noqa: E402
from topas_3000_model.air_quality_enhanced.configuration import AirQualityEnhancedConfiguration  # noqa: E402
from topas_3000_model.air_quality_enhanced.definition import AirQualityEnhancedDefinition  # noqa: E402
from topas_3000_model.air_quality_enhanced.measurement import AirQualityEnhancedDynamic  # noqa: E402
from topas_3000_model.meteorological.command import (  # noqa: E402
    MeteorologicalCommand,
    RequestMeteorologicalStatusCommand,
)
from topas_3000_model.meteorological.definition import MeteorologicalDefinition  # noqa: E402
from topas_3000_model.meteorological.fault import MeteorologicalFault  # noqa: E402
from topas_3000_model.meteorological.history import MeteorologicalHistoryEntry  # noqa: E402
from topas_3000_model.meteorological.measurement import MeteorologicalDynamic  # noqa: E402
from topas_3000_model.meteorological.response import MeteorologicalCommandResponse  # noqa: E402
from topas_3000_model.meteorological.type_id import MeteorologicalType  # noqa: E402
from topas_3000_model.access_control.definition import (  # noqa: E402
    AccessControlConfiguration,
    AccessControlDefinition,
)
from topas_3000_model.access_control.fault import AccessControlFault  # noqa: E402
from topas_3000_model.access_control.history import AccessControlHistoryEntry  # noqa: E402
from topas_3000_model.access_control.measurement import AccessControlDynamic  # noqa: E402
from topas_3000_model.access_control.response import AccessControlCommandResponse  # noqa: E402
from topas_3000_model.access_control.type_id import AccessControlType  # noqa: E402
from topas_3000_model.detector.command import (  # noqa: E402
    CalibrateDetectorCommand,
    DetectorCommand,
    ResetDetectorCommand,
    SetThresholdDetectorCommand,
)
from topas_3000_model.detector.response import DetectorCommandResponse  # noqa: E402
from topas_3000_model.detector.definition import (  # noqa: E402
    DetectorConfiguration,
    DetectorDefinition,
)
from topas_3000_model.detector.fault import DetectorFault  # noqa: E402
from topas_3000_model.detector.history import DetectorHistoryEntry  # noqa: E402
from topas_3000_model.detector.measurement import (  # noqa: E402
    DetectorMeasurement,
    FlowDynamic,
    HeadwayDynamic,
    OccupancyDynamic,
    QueueDynamic,
    SpeedDynamic,
)
from topas_3000_model.detector.type_id import DetectorType  # noqa: E402
from topas_3000_model.fault_support.enums import AcknowledgementState, FaultStatus  # noqa: E402
from topas_3000_model.common.sync import SyncCategoryEnum, SyncComplete, SyncRequest, SyncStatus  # noqa: E402
from topas_3000_model.fault_support.fault import FaultTypeDefinition  # noqa: E402
from topas_3000_model.traffic_signal.command import TrafficSignalCommand, SetPlanCommand, ForceStageCommand, HoldStageCommand, SynchronizeClockCommand, RequestStatusCommand  # noqa: E402
from topas_3000_model.traffic_signal.response import TrafficSignalCommandResponse  # noqa: E402
from topas_3000_model.traffic_signal.definition import TrafficSignalDefinition  # noqa: E402
from topas_3000_model.traffic_signal.configuration import TrafficSignalConfiguration  # noqa: E402
from topas_3000_model.traffic_signal.measurement import TrafficSignalDynamicState  # noqa: E402
from topas_3000_model.traffic_signal.fault import TrafficSignalFault  # noqa: E402
from topas_3000_model.traffic_signal.history import TrafficSignalHistoryEntry  # noqa: E402
from topas_3000_model.traffic_signal.type_id import TrafficSignalType  # noqa: E402
from topas_3000_model.transport_link.command import (  # noqa: E402
    RecalibrateTransportLinkCommand,
    RequestTransportLinkStatusCommand,
    SetStocCommand,
    TransportLinkCommand,
)
from topas_3000_model.transport_link.response import TransportLinkCommandResponse  # noqa: E402
from topas_3000_model.transport_link.definition import (  # noqa: E402
    TransportLinkANPRConfiguration,
    TransportLinkConfiguration,
    TransportLinkDefinition,
    TransportLinkGPSConfiguration,
    TransportLinkSCOOTConfiguration,
)
from topas_3000_model.transport_link.fault import TransportLinkFault  # noqa: E402
from topas_3000_model.transport_link.history import TransportLinkHistoryEntry  # noqa: E402
from topas_3000_model.transport_link.measurement import (  # noqa: E402
    TransportLinkANPRDynamic,
    TransportLinkGPSDynamic,
    TransportLinkMeasurement,
    TransportLinkSCOOTDynamic,
)
from topas_3000_model.transport_link.type_id import (  # noqa: E402
    LinkStatusType,
    TransportLinkType,
    TransportLinkTypeUnion,
)
from topas_3000_model.transport_route.command import (  # noqa: E402
    RequestTransportRouteStatusCommand,
    TransportRouteCommand,
)
from topas_3000_model.transport_route.definition import (  # noqa: E402
    TransportRouteConfiguration,
    TransportRouteDefinition,
    TransportRouteSegmentConfiguration,
)
from topas_3000_model.transport_route.fault import TransportRouteFault  # noqa: E402
from topas_3000_model.transport_route.history import TransportRouteHistoryEntry  # noqa: E402
from topas_3000_model.transport_route.measurement import TransportRouteDynamic  # noqa: E402
from topas_3000_model.transport_route.response import TransportRouteCommandResponse  # noqa: E402
from topas_3000_model.transport_route.type_id import TransportRouteType  # noqa: E402
from topas_3000_model.vms.command import (  # noqa: E402
    ClearSignCommand,
    DisplayGraphicCommand,
    DisplayMessageCommand,
    SetBrightnessCommand,
    VmsCommand,
)
from topas_3000_model.vms.definition import VmsConfiguration, VmsDefinition  # noqa: E402
from topas_3000_model.vms.fault import VmsFault  # noqa: E402
from topas_3000_model.vms.history import VmsHistoryEntry  # noqa: E402
from topas_3000_model.vms.measurement import VmsDynamic  # noqa: E402
from topas_3000_model.vms.response import VmsCommandResponse  # noqa: E402
from topas_3000_model.vms.type_id import VmsType  # noqa: E402
from topas_3000_model.cctv.command import (  # noqa: E402
    CctvCommand,
    RequestCctvStatusCommand,
)
from topas_3000_model.cctv.definition import (  # noqa: E402
    CctvConfiguration,
    CctvDefinition,
)
from topas_3000_model.cctv.fault import CctvFault  # noqa: E402
from topas_3000_model.cctv.history import CctvHistoryEntry  # noqa: E402
from topas_3000_model.cctv.measurement import (  # noqa: E402
    CctvAnprDynamic,
    CctvAnprVrnDynamic,
    CctvBroadcastDynamic,
    CctvFlowDynamic,
    CctvImageDynamic,
    CctvMeasurement,
    CctvQueueDynamic,
)
from topas_3000_model.cctv.response import CctvCommandResponse  # noqa: E402
from topas_3000_model.cctv.type_id import CctvType  # noqa: E402
from topas_3000_model.traffic_event.accident import AccidentEvent  # noqa: E402
from topas_3000_model.traffic_event.common import (  # noqa: E402
    EventStatus,
    LanesAffectedType,
    SeverityType,
)
from topas_3000_model.traffic_event.event import PlannedEvent  # noqa: E402
from topas_3000_model.traffic_event.incident import IncidentEvent  # noqa: E402
from topas_3000_model.traffic_event.roadworks import RoadworksEvent  # noqa: E402
from topas_3000_model.common.geometry import GeoJsonLineString, GeoJsonPoint  # noqa: E402
from topas_3000_model.network.link import NetworkLink  # noqa: E402
from topas_3000_model.network.node import NetworkNode  # noqa: E402
from topas_3000_model.network.path import NetworkPath  # noqa: E402
from topas_3000_model.network.turn import NetworkTurn  # noqa: E402
from topas_3000_model.network.zone import NetworkZone  # noqa: E402
from topas_3000_model.strategy.command import StrategyTrigger  # noqa: E402
from topas_3000_model.strategy.definition import StrategyDefinition  # noqa: E402
from topas_3000_model.strategy.response import StrategyCommandResponse  # noqa: E402
from topas_3000_model.strategy.status import StrategyStatus  # noqa: E402

DEFAULT_OUTPUT_DIR = ROOT / "Schemas"
OUTPUT_DIR = DEFAULT_OUTPUT_DIR
# PLACEHOLDER. The schemas are to be hosted on GitHub, at a location not yet
# known. Replace this one constant when it is, then regenerate: every schema's
# `$id` is built from it as `<base>/<domain>/<file>.json`, so the relative
# `../<domain>/<file>.json` references between schemas resolve against it.
# `.invalid` is a reserved TLD that never resolves, so a placeholder left in
# place fails loudly rather than pointing at somewhere real.
SCHEMA_ID_BASE = "https://schemas.example.invalid/topas3000a/v1.0"

# Class name -> (domain_subdir, source_filename). Used to rewrite cross-file $refs.
# Any class in a domain module that refs one of these gets a cross-file $ref
# with path ../{domain}/{filename}#/$defs/{ClassName}.
EXTERNAL_SOURCE: dict[str, tuple[str, str]] = {
    # common library
    "Point": ("common", "common-v1.0.json"),
    "Coordinates": ("common", "common-v1.0.json"),
    "Quality": ("common", "common-v1.0.json"),
    "CommandResponseStatus": ("common", "common-v1.0.json"),
    "GeoJsonPoint": ("common", "common-v1.0.json"),
    "GeoJsonLineString": ("common", "common-v1.0.json"),
    "DayTypeDefinition": ("common", "common-v1.0.json"),
    "DateTypeDefinition": ("common", "common-v1.0.json"),
    "SubsystemDefinition": ("common", "common-v1.0.json"),
    # fault_support library
    "AcknowledgementState": ("fault_support", "fault_support-v1.0.json"),
    "FaultStatus": ("fault_support", "fault_support-v1.0.json"),
    "FaultTypeDefinition": ("fault_support", "fault_support-v1.0.json"),
    # traffic_event_common library
    "EventStatus": ("traffic_event", "traffic_event_common-v1.0.json"),
    "SeverityType": ("traffic_event", "traffic_event_common-v1.0.json"),
    "LanesAffectedType": ("traffic_event", "traffic_event_common-v1.0.json"),
}


@dataclass
class FileSpec:
    """Describes one output JSON Schema file."""

    filename: str
    title: str
    description: str
    canonical: type | Any | None  # Pydantic class, TypeAdapter-compatible type, or None for library files
    library_types: list[type] = field(default_factory=list)  # extra classes to include for library files
    canonical_name: str | None = None  # explicit $def key for the canonical type (required when canonical is an Annotated alias)
    domain: str | None = None  # auto-derived from canonical.__module__ if None
    # ── Request/reply pairing (A11/A12) ───────────────────────────────
    # Replies are addressed to the requester's reply-to node, not published
    # to a channel of their own. A reply_only spec still emits a schema and
    # a components/messages entry, but no channel — a channel would describe
    # an address that does not exist at runtime, which provisioning tooling
    # can reasonably act on.
    reply_only: bool = False
    # On a request spec: the filename of the reply message it pairs with.
    reply_filename: str | None = None

    @property
    def schema_id(self) -> str:
        """The schema's `$id`: the URL the file would be served from.

        Equal to ``SCHEMA_ID_BASE`` plus the file's own path under ``Schemas/``,
        so the ``../<domain>/<file>.json`` references between schemas resolve
        against it exactly as they do on disk.
        """
        return f"{SCHEMA_ID_BASE}/{_resolve_domain(self)}/{self.filename}"

    def resolved_canonical_name(self) -> str | None:
        """Return the $def key for this file's canonical type, if any."""
        if self.canonical is None:
            return None
        if self.canonical_name is not None:
            return self.canonical_name
        return _class_name(self.canonical)


FILE_SPECS: list[FileSpec] = [
    FileSpec(
        filename="common-v1.0.json",
        title="UTMC Common Primitives",
        description="Shared primitive types referenced by every domain schema.",
        canonical=None,
        library_types=[Coordinates, Point, Quality, CommandResponseStatus, GeoJsonPoint, GeoJsonLineString, DayTypeDefinition, DateTypeDefinition, SubsystemDefinition],
    ),
    FileSpec(
        filename="fault_support-v1.0.json",
        title="UTMC Fault Support",
        description="Shared fault support types (acknowledgement state, fault type definition).",
        canonical=None,
        library_types=[AcknowledgementState, FaultStatus, FaultTypeDefinition],
    ),
    FileSpec(
        filename="detector_definition-v1.0.json",
        title="UTMC Detector Definition",
        description="Detector equipment details.",
        canonical=DetectorDefinition,
    ),
    FileSpec(
        filename="detector_configuration-v1.0.json",
        title="UTMC Detector Configuration",
        description="Detector operational configuration.",
        canonical=DetectorConfiguration,
    ),
    FileSpec(
        filename="detector_measurement-v1.0.json",
        title="UTMC Detector Measurement",
        description="Detector dynamic measurement data.",
        canonical=DetectorMeasurement,
        canonical_name="DetectorMeasurement",
        library_types=[
            FlowDynamic,
            OccupancyDynamic,
            QueueDynamic,
            SpeedDynamic,
            HeadwayDynamic,
        ],
    ),
    FileSpec(
        filename="detector_command-v1.0.json",
        title="UTMC Detector Command",
        description="Detector command payloads.",
        canonical=DetectorCommand,
        canonical_name="DetectorCommand",
        library_types=[ResetDetectorCommand, CalibrateDetectorCommand, SetThresholdDetectorCommand],
        reply_filename="detector_command_response-v1.0.json",
    ),
    FileSpec(
        filename="detector_command_response-v1.0.json",
        title="UTMC Detector Command Response",
        description="Detector command response payloads.",
        canonical=DetectorCommandResponse,
        reply_only=True,
    ),
    FileSpec(
        filename="detector_fault-v1.0.json",
        title="UTMC Detector Fault",
        description="Detector fault reports.",
        canonical=DetectorFault,
    ),
    FileSpec(
        filename="detector_type-v1.0.json",
        title="UTMC Detector Type",
        description="Detector type definitions.",
        canonical=DetectorType,
    ),
    FileSpec(
        filename="traffic_signal_definition-v1.0.json",
        title="UTMC Traffic Signal 0064 Definition",
        description="Traffic signal equipment details.",
        canonical=TrafficSignalDefinition,
    ),
    FileSpec(
        filename="traffic_signal_configuration-v1.0.json",
        title="UTMC Traffic Signal 0064 Configuration",
        description="Traffic signal operational configuration (TS004.0064bis:2017 §3.4).",
        canonical=TrafficSignalConfiguration,
    ),
    FileSpec(
        filename="traffic_signal_measurement-v1.0.json",
        title="UTMC Traffic Signal 0064 Dynamic State",
        description="Traffic signal dynamic state with PlanLine/SignalGroupTimings (TS004.0064bis:2017 §3.5).",
        canonical=TrafficSignalDynamicState,
    ),
    FileSpec(
        filename="traffic_signal_command-v1.0.json",
        title="UTMC Traffic Signal 0064 Command",
        description="Traffic signal command payloads (reused from TS004.0061:2010 Annex D).",
        canonical=TrafficSignalCommand,
        canonical_name="TrafficSignalCommand",
        library_types=[SetPlanCommand, ForceStageCommand, HoldStageCommand, SynchronizeClockCommand, RequestStatusCommand],
        reply_filename="traffic_signal_command_response-v1.0.json",
    ),
    FileSpec(
        filename="traffic_signal_command_response-v1.0.json",
        title="UTMC Traffic Signal 0064 Command Response",
        description="Traffic signal command response (reused from TS004.0061:2010 Annex D).",
        canonical=TrafficSignalCommandResponse,
        reply_only=True,
    ),
    FileSpec(
        filename="traffic_signal_fault-v1.0.json",
        title="UTMC Traffic Signal 0064 Fault",
        description="Traffic signal fault reports with flattened DATEX II Fault base (TS004.0064bis:2017 §3.3, F6).",
        canonical=TrafficSignalFault,
    ),
    FileSpec(
        filename="traffic_signal_type-v1.0.json",
        title="UTMC Traffic Signal 0064 Type",
        description="Traffic signal type definitions (reused from TS004.0061:2010 Annex D).",
        canonical=TrafficSignalType,
    ),
    FileSpec(
        filename="traffic_signal_history-v1.0.json",
        title="UTMC Traffic Signal 0064 History",
        description="Traffic signal history entries (reused from TS004.0061:2010 Annex D).",
        canonical=TrafficSignalHistoryEntry,
    ),
    FileSpec(
        filename="transport_link_definition-v1.0.json",
        title="UTMC Transport Link Definition",
        description="Transport link equipment details.",
        canonical=TransportLinkDefinition,
    ),
    FileSpec(
        filename="transport_link_configuration-v1.0.json",
        title="UTMC Transport Link Configuration",
        description="Transport link operational configuration (SCOOT/GPS/ANPR discriminated union).",
        canonical=TransportLinkConfiguration,
        canonical_name="TransportLinkConfiguration",
        library_types=[
            TransportLinkSCOOTConfiguration,
            TransportLinkGPSConfiguration,
            TransportLinkANPRConfiguration,
        ],
    ),
    FileSpec(
        filename="transport_link_measurement-v1.0.json",
        title="UTMC Transport Link Measurement",
        description="Transport link dynamic measurement data.",
        canonical=TransportLinkMeasurement,
        canonical_name="TransportLinkMeasurement",
        library_types=[
            TransportLinkSCOOTDynamic,
            TransportLinkANPRDynamic,
            TransportLinkGPSDynamic,
        ],
    ),
    FileSpec(
        filename="transport_link_command-v1.0.json",
        title="UTMC Transport Link Command",
        description="Transport link command payloads.",
        canonical=TransportLinkCommand,
        canonical_name="TransportLinkCommand",
        library_types=[SetStocCommand, RecalibrateTransportLinkCommand, RequestTransportLinkStatusCommand],
        reply_filename="transport_link_command_response-v1.0.json",
    ),
    FileSpec(
        filename="transport_link_command_response-v1.0.json",
        title="UTMC Transport Link Command Response",
        description="Transport link command response payloads.",
        canonical=TransportLinkCommandResponse,
        reply_only=True,
    ),
    FileSpec(
        filename="transport_link_fault-v1.0.json",
        title="UTMC Transport Link Fault",
        description="Transport link fault reports.",
        canonical=TransportLinkFault,
    ),
    FileSpec(
        filename="transport_link_type-v1.0.json",
        title="UTMC Transport Link Type",
        description="Transport link type definitions.",
        canonical=TransportLinkTypeUnion,
        canonical_name="TransportLinkTypeUnion",
        library_types=[TransportLinkType, LinkStatusType],
    ),
    FileSpec(
        filename="car_park_definition-v1.0.json",
        title="UTMC Car Park Definition",
        description="Car park equipment details.",
        canonical=CarParkDefinition,
    ),
    FileSpec(
        filename="car_park_configuration-v1.0.json",
        title="UTMC Car Park Configuration",
        description="Car park capacity and threshold configuration.",
        canonical=CarParkConfiguration,
        library_types=[CarParkDetectorsConfiguration],
    ),
    FileSpec(
        filename="car_park_measurement-v1.0.json",
        title="UTMC Car Park Measurement",
        description="Car park dynamic measurement data.",
        canonical=CarParkDynamic,
    ),
    FileSpec(
        filename="car_park_command-v1.0.json",
        title="UTMC Car Park Command",
        description="Car park command payloads.",
        canonical=CarParkCommand,
        canonical_name="CarParkCommand",
        library_types=[SetOccupancyCommand, SetCapacityCommand, SetThresholdsCommand, RequestCarParkStatusCommand],
        reply_filename="car_park_command_response-v1.0.json",
    ),
    FileSpec(
        filename="car_park_command_response-v1.0.json",
        title="UTMC Car Park Command Response",
        description="Car park command response payloads.",
        canonical=CarParkCommandResponse,
        reply_only=True,
    ),
    FileSpec(
        filename="car_park_fault-v1.0.json",
        title="UTMC Car Park Fault",
        description="Car park fault reports.",
        canonical=CarParkFault,
    ),
    FileSpec(
        filename="car_park_type-v1.0.json",
        title="UTMC Car Park Type",
        description="Car park type definitions.",
        canonical=CarParkType,
    ),
    FileSpec(
        filename="car_park_tariff_configuration-v1.0.json",
        title="UTMC Car Park Tariff Configuration",
        description="Per-day-type, per-time-band car park pricing entry.",
        canonical=CarParkTariffConfiguration,
    ),
    FileSpec(
        filename="car_park_opening_times_configuration-v1.0.json",
        title="UTMC Car Park Opening Times Configuration",
        description="Per-day-type car park open/close time pair.",
        canonical=CarParkOpeningTimesConfiguration,
    ),
    FileSpec(
        filename="car_park_access_location-v1.0.json",
        title="UTMC Car Park Access Location",
        description="Car park entry/exit point with coordinates and optional network path reference.",
        canonical=CarParkAccessLocation,
    ),
    FileSpec(
        filename="car_park_zone_list_entry-v1.0.json",
        title="UTMC Car Park Zone List Entry",
        description="Membership of a car park in a network zone (e.g. for VMS advertisement).",
        canonical=CarParkZoneListEntry,
    ),
    FileSpec(
        filename="detector_history-v1.0.json",
        title="UTMC Detector History",
        description="Detector history entries.",
        canonical=DetectorHistoryEntry,
    ),
    FileSpec(
        filename="transport_link_history-v1.0.json",
        title="UTMC Transport Link History",
        description="Transport link history entries.",
        canonical=TransportLinkHistoryEntry,
    ),
    FileSpec(
        filename="car_park_history-v1.0.json",
        title="UTMC Car Park History",
        description="Car park history entries.",
        canonical=CarParkHistoryEntry,
    ),
    FileSpec(
        filename="access_control_definition-v1.0.json",
        title="UTMC Access Control Definition",
        description="Access control device details.",
        canonical=AccessControlDefinition,
    ),
    FileSpec(
        filename="access_control_configuration-v1.0.json",
        title="UTMC Access Control Configuration",
        description="Access control device configuration.",
        canonical=AccessControlConfiguration,
    ),
    FileSpec(
        filename="access_control_measurement-v1.0.json",
        title="UTMC Access Control Measurement",
        description="Access control dynamic measurement data.",
        canonical=AccessControlDynamic,
    ),
    FileSpec(
        filename="access_control_command-v1.0.json",
        title="UTMC Access Control Command",
        description="Access control command payloads.",
        canonical=AccessControlCommand,
        canonical_name="AccessControlCommand",
        library_types=[EnableAccessCommand, DisableAccessCommand],
        reply_filename="access_control_command_response-v1.0.json",
    ),
    FileSpec(
        filename="access_control_command_response-v1.0.json",
        title="UTMC Access Control Command Response",
        description="Access control command response payloads.",
        canonical=AccessControlCommandResponse,
        reply_only=True,
    ),
    FileSpec(
        filename="access_control_fault-v1.0.json",
        title="UTMC Access Control Fault",
        description="Access control fault reports.",
        canonical=AccessControlFault,
    ),
    FileSpec(
        filename="access_control_type-v1.0.json",
        title="UTMC Access Control Type",
        description="Access control type definitions.",
        canonical=AccessControlType,
    ),
    FileSpec(
        filename="access_control_history-v1.0.json",
        title="UTMC Access Control History",
        description="Access control history entries.",
        canonical=AccessControlHistoryEntry,
    ),
    FileSpec(
        filename="air_quality_definition-v1.0.json",
        title="UTMC Air Quality Definition",
        description="Air quality monitor details.",
        canonical=AirQualityDefinition,
    ),
    FileSpec(
        filename="air_quality_configuration-v1.0.json",
        title="UTMC Air Quality Configuration",
        description="Air quality pollutant threshold configuration.",
        canonical=AirQualityConfiguration,
    ),
    FileSpec(
        filename="air_quality_measurement-v1.0.json",
        title="UTMC Air Quality Measurement",
        description="Air quality dynamic measurement data.",
        canonical=AirQualityDynamic,
    ),
    FileSpec(
        filename="air_quality_command-v1.0.json",
        title="UTMC Air Quality Command",
        description="Air quality command payloads.",
        canonical=AirQualityCommand,
        canonical_name="AirQualityCommand",
        library_types=[RequestAirQualityStatusCommand],
        reply_filename="air_quality_command_response-v1.0.json",
    ),
    FileSpec(
        filename="air_quality_command_response-v1.0.json",
        title="UTMC Air Quality Command Response",
        description="Air quality command response payloads.",
        canonical=AirQualityCommandResponse,
        reply_only=True,
    ),
    FileSpec(
        filename="air_quality_fault-v1.0.json",
        title="UTMC Air Quality Fault",
        description="Air quality fault reports.",
        canonical=AirQualityFault,
    ),
    FileSpec(
        filename="air_quality_type-v1.0.json",
        title="UTMC Air Quality Type",
        description="Air quality type definitions.",
        canonical=AirQualityType,
    ),
    FileSpec(
        filename="air_quality_history-v1.0.json",
        title="UTMC Air Quality History",
        description="Air quality history entries.",
        canonical=AirQualityHistoryEntry,
    ),
    FileSpec(
        filename="air_quality_enhanced_definition-v1.0.json",
        title="UTMC Air Quality Enhanced Definition",
        description="Air quality enhanced monitor details with per-pollutant units.",
        canonical=AirQualityEnhancedDefinition,
    ),
    FileSpec(
        filename="air_quality_enhanced_configuration-v1.0.json",
        title="UTMC Air Quality Enhanced Configuration",
        description="Air quality enhanced multi-level threshold configuration.",
        canonical=AirQualityEnhancedConfiguration,
    ),
    FileSpec(
        filename="air_quality_enhanced_measurement-v1.0.json",
        title="UTMC Air Quality Enhanced Measurement",
        description="Air quality enhanced dynamic measurement data with extended pollutants.",
        canonical=AirQualityEnhancedDynamic,
    ),
    FileSpec(
        filename="meteorological_definition-v1.0.json",
        title="UTMC Meteorological Definition",
        description="Meteorological station details.",
        canonical=MeteorologicalDefinition,
    ),
    FileSpec(
        filename="meteorological_measurement-v1.0.json",
        title="UTMC Meteorological Measurement",
        description="Meteorological dynamic measurement data.",
        canonical=MeteorologicalDynamic,
    ),
    FileSpec(
        filename="meteorological_command-v1.0.json",
        title="UTMC Meteorological Command",
        description="Meteorological command payloads.",
        canonical=MeteorologicalCommand,
        canonical_name="MeteorologicalCommand",
        library_types=[RequestMeteorologicalStatusCommand],
        reply_filename="meteorological_command_response-v1.0.json",
    ),
    FileSpec(
        filename="meteorological_command_response-v1.0.json",
        title="UTMC Meteorological Command Response",
        description="Meteorological command response payloads.",
        canonical=MeteorologicalCommandResponse,
        reply_only=True,
    ),
    FileSpec(
        filename="meteorological_fault-v1.0.json",
        title="UTMC Meteorological Fault",
        description="Meteorological fault reports.",
        canonical=MeteorologicalFault,
    ),
    FileSpec(
        filename="meteorological_type-v1.0.json",
        title="UTMC Meteorological Type",
        description="Meteorological type definitions.",
        canonical=MeteorologicalType,
    ),
    FileSpec(
        filename="meteorological_history-v1.0.json",
        title="UTMC Meteorological History",
        description="Meteorological history entries.",
        canonical=MeteorologicalHistoryEntry,
    ),
    FileSpec(
        filename="transport_route_definition-v1.0.json",
        title="UTMC Transport Route Definition",
        description="Transport route equipment details.",
        canonical=TransportRouteDefinition,
    ),
    FileSpec(
        filename="transport_route_configuration-v1.0.json",
        title="UTMC Transport Route Configuration",
        description="Transport route configuration and segment mapping.",
        canonical=TransportRouteConfiguration,
        library_types=[TransportRouteSegmentConfiguration],
    ),
    FileSpec(
        filename="transport_route_measurement-v1.0.json",
        title="UTMC Transport Route Measurement",
        description="Transport route dynamic measurement data.",
        canonical=TransportRouteDynamic,
    ),
    FileSpec(
        filename="transport_route_command-v1.0.json",
        title="UTMC Transport Route Command",
        description="Transport route command payloads.",
        canonical=TransportRouteCommand,
        canonical_name="TransportRouteCommand",
        library_types=[RequestTransportRouteStatusCommand],
        reply_filename="transport_route_command_response-v1.0.json",
    ),
    FileSpec(
        filename="transport_route_command_response-v1.0.json",
        title="UTMC Transport Route Command Response",
        description="Transport route command response payloads.",
        canonical=TransportRouteCommandResponse,
        reply_only=True,
    ),
    FileSpec(
        filename="transport_route_fault-v1.0.json",
        title="UTMC Transport Route Fault",
        description="Transport route fault reports.",
        canonical=TransportRouteFault,
    ),
    FileSpec(
        filename="transport_route_type-v1.0.json",
        title="UTMC Transport Route Type",
        description="Transport route type definitions.",
        canonical=TransportRouteType,
    ),
    FileSpec(
        filename="transport_route_history-v1.0.json",
        title="UTMC Transport Route History",
        description="Transport route history entries.",
        canonical=TransportRouteHistoryEntry,
    ),
    FileSpec(
        filename="vms_definition-v1.0.json",
        title="UTMC VMS Definition",
        description="Variable message sign equipment details.",
        canonical=VmsDefinition,
    ),
    FileSpec(
        filename="vms_configuration-v1.0.json",
        title="UTMC VMS Configuration",
        description="Variable message sign operational configuration.",
        canonical=VmsConfiguration,
    ),
    FileSpec(
        filename="vms_measurement-v1.0.json",
        title="UTMC VMS Measurement",
        description="Variable message sign dynamic measurement data.",
        canonical=VmsDynamic,
    ),
    FileSpec(
        filename="vms_command-v1.0.json",
        title="UTMC VMS Command",
        description="Variable message sign command payloads.",
        canonical=VmsCommand,
        canonical_name="VmsCommand",
        library_types=[DisplayMessageCommand, DisplayGraphicCommand, SetBrightnessCommand, ClearSignCommand],
        reply_filename="vms_command_response-v1.0.json",
    ),
    FileSpec(
        filename="vms_command_response-v1.0.json",
        title="UTMC VMS Command Response",
        description="Variable message sign command response payloads.",
        canonical=VmsCommandResponse,
        reply_only=True,
    ),
    FileSpec(
        filename="vms_fault-v1.0.json",
        title="UTMC VMS Fault",
        description="Variable message sign fault reports.",
        canonical=VmsFault,
    ),
    FileSpec(
        filename="vms_type-v1.0.json",
        title="UTMC VMS Type",
        description="Variable message sign type definitions.",
        canonical=VmsType,
    ),
    FileSpec(
        filename="vms_history-v1.0.json",
        title="UTMC VMS History",
        description="Variable message sign history entries.",
        canonical=VmsHistoryEntry,
    ),
    FileSpec(
        filename="cctv_definition-v1.0.json",
        title="UTMC CCTV Definition",
        description="CCTV camera details.",
        canonical=CctvDefinition,
    ),
    FileSpec(
        filename="cctv_configuration-v1.0.json",
        title="UTMC CCTV Configuration",
        description="CCTV camera operational configuration.",
        canonical=CctvConfiguration,
    ),
    FileSpec(
        filename="cctv_measurement-v1.0.json",
        title="UTMC CCTV Measurement",
        description="CCTV dynamic measurement data.",
        canonical=CctvMeasurement,
        canonical_name="CctvMeasurement",
        library_types=[
            CctvImageDynamic, CctvAnprDynamic, CctvAnprVrnDynamic,
            CctvBroadcastDynamic, CctvFlowDynamic, CctvQueueDynamic,
        ],
    ),
    FileSpec(
        filename="cctv_command-v1.0.json",
        title="UTMC CCTV Command",
        description="CCTV command payloads.",
        canonical=CctvCommand,
        canonical_name="CctvCommand",
        library_types=[RequestCctvStatusCommand],
        reply_filename="cctv_command_response-v1.0.json",
    ),
    FileSpec(
        filename="cctv_command_response-v1.0.json",
        title="UTMC CCTV Command Response",
        description="CCTV command response payloads.",
        canonical=CctvCommandResponse,
        reply_only=True,
    ),
    FileSpec(
        filename="cctv_fault-v1.0.json",
        title="UTMC CCTV Fault",
        description="CCTV fault reports.",
        canonical=CctvFault,
    ),
    FileSpec(
        filename="cctv_type-v1.0.json",
        title="UTMC CCTV Type",
        description="CCTV type definitions.",
        canonical=CctvType,
    ),
    FileSpec(
        filename="cctv_history-v1.0.json",
        title="UTMC CCTV History",
        description="CCTV history entries.",
        canonical=CctvHistoryEntry,
    ),
    FileSpec(
        filename="traffic_event_common-v1.0.json",
        title="UTMC Traffic Event Common",
        description="Shared traffic event types (event status, severity, lanes affected).",
        canonical=None,
        library_types=[EventStatus, SeverityType, LanesAffectedType],
    ),
    FileSpec(
        filename="traffic_event_accident-v1.0.json",
        title="UTMC Traffic Event Accident",
        description="Accident traffic event payload.",
        canonical=AccidentEvent,
    ),
    FileSpec(
        filename="traffic_event_event-v1.0.json",
        title="UTMC Traffic Event Planned Event",
        description="Planned event traffic event payload.",
        canonical=PlannedEvent,
    ),
    FileSpec(
        filename="traffic_event_incident-v1.0.json",
        title="UTMC Traffic Event Incident",
        description="Incident traffic event payload.",
        canonical=IncidentEvent,
    ),
    FileSpec(
        filename="traffic_event_roadworks-v1.0.json",
        title="UTMC Traffic Event Roadworks",
        description="Roadworks traffic event payload.",
        canonical=RoadworksEvent,
    ),
    FileSpec(
        filename="network_node-v1.0.json",
        title="UTMC Network Node",
        description="Road network node (junction or endpoint).",
        canonical=NetworkNode,
    ),
    FileSpec(
        filename="network_link-v1.0.json",
        title="UTMC Network Link",
        description="Road network link between two nodes.",
        canonical=NetworkLink,
    ),
    FileSpec(
        filename="network_zone-v1.0.json",
        title="UTMC Network Zone",
        description="Road network zone (area grouping of links/nodes).",
        canonical=NetworkZone,
    ),
    FileSpec(
        filename="network_path-v1.0.json",
        title="UTMC Network Path",
        description="Road network path (ordered sequence of links).",
        canonical=NetworkPath,
    ),
    FileSpec(
        filename="network_turn-v1.0.json",
        title="UTMC Network Turn",
        description="Road network turn (movement through a node between links).",
        canonical=NetworkTurn,
    ),
    FileSpec(
        filename="strategy_definition-v1.0.json",
        title="UTMC Strategy Definition",
        description="Traffic management strategy static metadata.",
        canonical=StrategyDefinition,
    ),
    FileSpec(
        filename="strategy_status-v1.0.json",
        title="UTMC Strategy Status",
        description="Traffic management strategy current status.",
        canonical=StrategyStatus,
    ),
    FileSpec(
        filename="strategy_command-v1.0.json",
        title="UTMC Strategy Command",
        description="Strategy trigger command payloads.",
        canonical=StrategyTrigger,
        reply_filename="strategy_command_response-v1.0.json",
    ),
    FileSpec(
        filename="strategy_command_response-v1.0.json",
        title="UTMC Strategy Command Response",
        description="Strategy command response payloads.",
        canonical=StrategyCommandResponse,
        reply_only=True,
    ),
]


# ── Sync request channels ───────────────────────────────────────
#
# One `<domain>SyncRequest` channel per data-bearing domain, all carrying
# the same `SyncRequest` payload from `topas_3000_model/common/sync.py`. The
# request is domain-agnostic — the domain is the channel, and the filters
# inside it are the same everywhere — so the model lives in common/ per
# the "promote shared types" rule rather than being duplicated per domain.
#
# `common` and `fault_support` are excluded: they are library schemas with
# no objects of their own to synchronise.
SYNC_REQUEST_DOMAINS: list[tuple[str, str]] = [
    ("access_control", "Access control"),
    ("air_quality", "Air quality"),
    ("air_quality_enhanced", "Enhanced air quality"),
    ("car_park", "Car park"),
    ("cctv", "CCTV"),
    ("detector", "Detector"),
    ("meteorological", "Meteorological"),
    ("network", "Network topology"),
    ("strategy", "Strategy"),
    ("traffic_event", "Traffic event"),
    ("traffic_signal", "Traffic signal"),
    ("transport_link", "Transport link"),
    ("transport_route", "Transport route"),
    ("vms", "VMS"),
]

FILE_SPECS.extend(
    FileSpec(
        filename=f"{domain}_sync_request-v1.0.json",
        title=f"UTMC {label} Sync Request",
        description=(
            f"Request the current state of {label.lower()} objects. The "
            "Data Publisher answers to the request's reply-to address, not "
            "the domain's data channels: a sync batch (x-message-mode: sync, "
            "x-sync-id/-sequence/-total) followed by a SyncComplete."
        ),
        canonical=SyncRequest,
        library_types=[SyncCategoryEnum],
        domain=domain,
        reply_filename=f"{domain}_sync_complete-v1.0.json",
    )
    for domain, label in SYNC_REQUEST_DOMAINS
)

# The terminal message of a sync batch (A12). Addressed to the requester's
# reply node like any other reply, so it emits a schema and a components
# message but no channel of its own.
FILE_SPECS.extend(
    FileSpec(
        filename=f"{domain}_sync_complete-v1.0.json",
        title=f"UTMC {label} Sync Complete",
        description=(
            f"Terminal message ending a {label.lower()} sync batch. Sent to "
            "the requester's reply node after the last object, or instead of "
            "any objects when the filter matched none."
        ),
        canonical=SyncComplete,
        library_types=[SyncStatus],
        domain=domain,
        reply_only=True,
    )
    for domain, label in SYNC_REQUEST_DOMAINS
)


def _class_name(cls: type | Any) -> str:
    """Get the JSON Schema $defs key Pydantic uses for a class."""
    return getattr(cls, "__name__", None) or str(cls)


def _resolve_domain(spec: "FileSpec") -> str:
    """Return the domain subdirectory for this spec.

    Priority:
      1. ``spec.domain`` if set.
      2. Segment of ``spec.canonical.__module__`` immediately after ``topas_3000_model``.
      3. Same extraction from the first entry in ``spec.library_types`` whose
         ``__module__`` yields a usable segment, if canonical is None or has no
         usable ``__module__`` (e.g. some Annotated aliases).

    Raises ValueError if none of the above yields a domain — fail-fast beats
    silently writing a file into the wrong directory.
    """
    if spec.domain is not None:
        return spec.domain

    def _from_module(obj: Any) -> str | None:
        mod = getattr(obj, "__module__", None)
        if not isinstance(mod, str):
            return None
        parts = mod.split(".")
        if len(parts) >= 2 and parts[0] == "topas_3000_model":
            return parts[1]
        return None

    if spec.canonical is not None:
        domain = _from_module(spec.canonical)
        if domain is not None:
            return domain

    for lt in spec.library_types:
        domain = _from_module(lt)
        if domain is not None:
            return domain

    raise ValueError(
        f"Could not resolve domain for FileSpec(filename={spec.filename!r}). "
        "Set FileSpec.domain explicitly."
    )


# ── Scope ───────────────────────────────────────────────────────
#
# `scope.toml` says which domains are live. The registry above lists every
# domain's specs; `FILE_SPECS` is narrowed to the live phase here, in place, so
# every importer (the AsyncAPI generator, the tests) sees only what is live.
# The SDK and diagram generators read the emitted `Schemas/` and follow.
SCOPE = load_scope()
ALL_FILE_SPECS: list[FileSpec] = list(FILE_SPECS)
"""Every spec for every domain, whatever the phase. Use ``specs_for_phase``."""


def specs_for_phase(phase: int | None = None) -> list[FileSpec]:
    """The specs emitted for ``phase`` (default: the live phase)."""
    included = set(SCOPE.included(phase))
    return [s for s in ALL_FILE_SPECS if _resolve_domain(s) in included]


def _check_scope_covers_specs() -> None:
    listed = set(SCOPE.domains) | set(SCOPE.libraries)
    unknown = sorted({_resolve_domain(s) for s in ALL_FILE_SPECS} - listed)
    if unknown:
        raise ScopeError(
            f"file specs exist for {unknown}, which scope.toml does not list; "
            "give each a phase there"
        )


_check_scope_covers_specs()
FILE_SPECS[:] = specs_for_phase()


def _raw_schema_for(type_: type | Any) -> dict:
    """Get the raw JSON Schema (with $defs) for a Pydantic type or Annotated union."""
    adapter = TypeAdapter(type_)
    return adapter.json_schema(by_alias=True, ref_template="#/$defs/{model}")


_PRESERVED_TITLES = frozenset({"ObjectId", "ParticipantId", "QualityId", "TypeId"})
"""Titles that survive `_clean_def`.

These are the names of the shared `Annotated[..., Field(title=...)]` aliases
in `topas_3000_model/common/ids.py`. Surfacing them at property level in the
emitted JSON Schema lets `bin/generate_diagrams.py` show semantic type
names ("TypeId" rather than "integer") on ER diagrams. quicktype ignores
property-level titles so the SDKs are unaffected.

Keep this set in sync with the alias declarations in `common/ids.py` and
the matching list in `bin/generate_diagrams.py::_TYPE_ALIAS_TITLES`.
"""


def _clean_def(node: dict) -> dict:
    """Remove Pydantic's auto-added fluff from a single $def."""
    out = deepcopy(node)
    # Pydantic adds title=<ClassName> on every $def and title="Field Name"
    # on every property. Strip them, except when the title names a shared
    # type alias (TypeId / ObjectId / ParticipantId / QualityId) — those
    # are deliberately surfaced for the diagram generator.
    if out.get("title") not in _PRESERVED_TITLES:
        out.pop("title", None)
    # Recursively clean nested objects
    if isinstance(out.get("properties"), dict):
        for key, val in out["properties"].items():
            if isinstance(val, dict):
                # Strip redundant "default": null — anyOf with null already
                # conveys nullability, and PyCharm's meta-schema validator flags it.
                if "default" in val and val["default"] is None:
                    del val["default"]
                out["properties"][key] = _clean_def(val)
    if isinstance(out.get("items"), dict):
        out["items"] = _clean_def(out["items"])
    return out


def _simplify_nullable_anyof(node: dict) -> dict:
    """Strip nullable wrappers from optional fields.

    Pydantic v2 emits nullable fields as:
        {"anyOf": [{"type": "string", ...}, {"type": "null"}]}

    We simplify to just:
        {"type": "string", ...}

    dropping the null alternative entirely.  The field stays optional (not in
    ``required``), but ``null`` is no longer a valid value — the convention is
    "present with a value, or absent".

    For ``$ref`` + null (e.g. acknowledgementState, quality, status):
        {"anyOf": [{"$ref": "..."}, {"type": "null"}]}  ->  {"$ref": "..."}
    """
    out = deepcopy(node)
    if isinstance(out.get("properties"), dict):
        for key, val in list(out["properties"].items()):
            if isinstance(val, dict):
                val = _simplify_nullable_anyof(val)
                out["properties"][key] = val
                any_of = val.get("anyOf")
                if isinstance(any_of, list) and len(any_of) == 2:
                    non_null = null_branch = ref_branch = None
                    for branch in any_of:
                        if isinstance(branch, dict) and branch.get("type") == "null" and len(branch) == 1:
                            null_branch = branch
                        elif isinstance(branch, dict) and "$ref" in branch:
                            ref_branch = branch
                        elif isinstance(branch, dict) and "type" in branch:
                            non_null = branch

                    if null_branch is not None and non_null is not None:
                        # Primitive + null -> just the primitive type (no null)
                        simplified = {}
                        simplified["type"] = non_null["type"]
                        for k, v in non_null.items():
                            if k != "type":
                                simplified[k] = v
                        # Preserve sibling keys from the property (description, etc.)
                        for k, v in val.items():
                            if k != "anyOf":
                                simplified[k] = v
                        out["properties"][key] = simplified
                    elif null_branch is not None and ref_branch is not None:
                        # $ref + null -> just $ref
                        simplified = dict(ref_branch)
                        # Preserve sibling keys from the property (description, etc.)
                        for k, v in val.items():
                            if k != "anyOf":
                                simplified[k] = v
                        out["properties"][key] = simplified
    # Also recurse into items, allOf branches, etc.
    if isinstance(out.get("items"), dict):
        out["items"] = _simplify_nullable_anyof(out["items"])
    for kw in ("allOf", "oneOf"):
        if isinstance(out.get(kw), list):
            out[kw] = [_simplify_nullable_anyof(item) if isinstance(item, dict) else item for item in out[kw]]
    return out


def _enforce_discriminator_required(node: dict) -> dict:
    """If a $def's properties contain a `kind` with a `const`, ensure `kind` is required
    and has no default. This is a fix for Pydantic's Literal-with-default emission, which
    technically makes the discriminator optional in draft 2020-12.
    """
    out = deepcopy(node)
    properties = out.get("properties")
    if isinstance(properties, dict):
        kind_prop = properties.get("kind")
        if isinstance(kind_prop, dict) and "const" in kind_prop:
            # Drop the default if present
            kind_prop.pop("default", None)
            # Ensure 'kind' is in required
            required = out.setdefault("required", [])
            if "kind" not in required:
                required.append("kind")
                # Sort required for stable output
                required.sort()
    return out


def _rewrite_external_refs(
    defs: dict[str, dict], filename: str, current_domain: str
) -> dict[str, dict]:
    """Rewrite $refs to external classes as cross-file refs, and drop those defs.

    External refs are expressed relative to the current file's directory:
      - same domain:     '{filename}#/$defs/{Name}'
      - different domain: '../{target_domain}/{filename}#/$defs/{Name}'
    """
    # A class is external if it's listed in EXTERNAL_SOURCE and this file is not its home.
    external_here = {
        name: (domain, source)
        for name, (domain, source) in EXTERNAL_SOURCE.items()
        if source != filename
    }

    def _ref_path(target_domain: str, target_filename: str) -> str:
        # Same-domain: bare filename (e.g. traffic_event specs referencing
        # EventStatus in traffic_event_common-v1.0.json).
        # Different domain: ../{target_domain}/{filename}.
        if target_domain == current_domain:
            return target_filename
        return f"../{target_domain}/{target_filename}"

    def _walk(node):
        if isinstance(node, dict):
            if "$ref" in node and isinstance(node["$ref"], str):
                ref = node["$ref"]
                if ref.startswith("#/$defs/"):
                    target = ref.removeprefix("#/$defs/")
                    if target in external_here:
                        target_domain, target_filename = external_here[target]
                        node["$ref"] = (
                            f"{_ref_path(target_domain, target_filename)}#/$defs/{target}"
                        )
            for val in node.values():
                _walk(val)
        elif isinstance(node, list):
            for item in node:
                _walk(item)

    cleaned_defs = {}
    for name, defn in defs.items():
        if name in external_here:
            continue  # drop the $def since it lives in another file
        cloned = deepcopy(defn)
        _walk(cloned)
        cleaned_defs[name] = cloned
    return cleaned_defs


def build_file_schema(spec: FileSpec) -> dict:
    """Build the full JSON Schema document for one output file."""
    collected_defs: dict[str, dict] = {}
    canonical_name = spec.resolved_canonical_name()

    # Collect $defs from the canonical type (if any) and all library types.
    types_to_collect = []
    if spec.canonical is not None:
        types_to_collect.append(spec.canonical)
    types_to_collect.extend(spec.library_types)

    for t in types_to_collect:
        raw = _raw_schema_for(t)
        raw_defs = raw.get("$defs", {})

        type_name = _class_name(t)
        is_canonical = spec.canonical is t

        # For unions (Annotated[Union[...], discriminator=...]), the top-level IS the oneOf,
        # and the canonical name (e.g. DetectorMeasurement) needs to be added explicitly.
        if is_canonical and "oneOf" in raw:
            assert canonical_name is not None
            collected_defs[canonical_name] = {
                k: v for k, v in raw.items() if k != "$defs"
            }
        elif "properties" in raw:
            # Plain Pydantic model — add its top-level schema as a $def keyed by its class name.
            if type_name not in raw_defs:
                defn = {k: v for k, v in raw.items() if k not in ("$defs",)}
                collected_defs[type_name] = defn
        else:
            # Enum or primitive type (e.g. AcknowledgementState, an Annotated[str, Field(...)] alias).
            # Raw schema has neither properties nor oneOf — it's a leaf type. Emit it directly
            # under its class name (or canonical_name override) as a $def.
            if type_name not in raw_defs:
                defn = {k: v for k, v in raw.items() if k != "$defs"}
                collected_defs[type_name] = defn

        # Add all $defs from the raw schema
        for name, defn in raw_defs.items():
            collected_defs[name] = defn

    # Clean and rewrite external refs
    cleaned = {name: _clean_def(d) for name, d in collected_defs.items()}
    cleaned = _rewrite_external_refs(cleaned, spec.filename, _resolve_domain(spec))

    # Enforce that discriminator-style 'kind' fields are required and have no default.
    # Pydantic emits Literal["..."] = "..." defaults that make 'kind' technically optional
    # under draft 2020-12, but for discriminated-union variants that's a footgun — every
    # consumer expects to see the discriminator in every payload. Strip the default and
    # ensure kind is in the required list.
    cleaned = {name: _enforce_discriminator_required(d) for name, d in cleaned.items()}

    # Simplify Pydantic's anyOf nullable pattern for primitive types:
    # {"anyOf": [{"type": X}, {"type": "null"}]} → {"type": [X, "null"]}
    cleaned = {name: _simplify_nullable_anyof(d) for name, d in cleaned.items()}

    # Sort alphabetically for stable diffs
    sorted_defs = dict(sorted(cleaned.items()))

    schema: dict = {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": spec.schema_id,
    }
    if canonical_name is not None:
        schema["$ref"] = f"#/$defs/{canonical_name}"
        # Add title/description to the canonical $def rather than the root,
        # so the root $ref has no application-keyword siblings (avoids IDE
        # meta-schema warnings from validators that don't fully support
        # draft 2020-12's relaxed $ref-sibling rules).
        if canonical_name in sorted_defs:
            sorted_defs[canonical_name].setdefault("title", spec.title)
            # Always set description from the FileSpec so that definition
            # files carrying multiple sibling types can document that fact.
            sorted_defs[canonical_name]["description"] = spec.description
    else:
        # Library files (no $ref) keep title/description at the root.
        schema["title"] = spec.title
        schema["description"] = spec.description
    schema["$defs"] = sorted_defs
    return schema


def _display_path(p: Path) -> str:
    """Return a path relative to ROOT when possible; otherwise absolute."""
    try:
        return str(p.relative_to(ROOT))
    except ValueError:
        return str(p)


def emit_all(phase: int | None = None, output_dir: Path | None = None) -> None:
    """Build and write every output file under ``{output_dir}/{domain}/{filename}``.

    ``phase`` defaults to the live phase in ``scope.toml`` and ``output_dir`` to
    ``Schemas/``. Emitting any other phase into the committed ``Schemas/`` tree
    is refused: the drift gates compare that tree with the live scope, so a
    preview of a later phase belongs in its own directory (``--output-dir``).

    Only schemas this tool produced are ever deleted from ``output_dir`` (see
    ``_prune_stale``), so pointing it at a directory with other files is safe.
    """
    phase = SCOPE.live_phase if phase is None else phase
    out_dir = (OUTPUT_DIR if output_dir is None else output_dir).resolve()
    if phase != SCOPE.live_phase and out_dir == DEFAULT_OUTPUT_DIR.resolve():
        raise ScopeError(
            f"refusing to emit phase {phase} into {_display_path(DEFAULT_OUTPUT_DIR)}/: "
            f"the live phase is {SCOPE.live_phase}. Use --output-dir for a preview."
        )
    specs = specs_for_phase(phase)
    out_dir.mkdir(parents=True, exist_ok=True)

    # Resolve every domain up-front so a misconfigured FileSpec fails fast,
    # before any filesystem mutation occurs.
    spec_domains: list[tuple["FileSpec", str]] = [
        (spec, _resolve_domain(spec)) for spec in specs
    ]

    # Create per-domain directories in one pre-pass.
    for domain in {d for _, d in spec_domains}:
        (out_dir / domain).mkdir(parents=True, exist_ok=True)

    # Cleanup: remove any stale flat-layout files at the root of Schemas/
    # that correspond to a current FileSpec. Scoped — only removes files we own.
    known_filenames = {spec.filename for spec in specs}
    for stale in out_dir.glob("*.json"):
        if stale.name in known_filenames:
            stale.unlink()

    written: set[Path] = set()
    for spec, domain in spec_domains:
        schema = build_file_schema(spec)
        out_path = out_dir / domain / spec.filename
        out_path.write_text(json.dumps(schema, indent=2, sort_keys=False) + "\n")
        written.add(out_path.resolve())
        print(f"  wrote {_display_path(out_path)} ({len(schema['$defs'])} defs)")

    _prune_stale(out_dir, written)


def _is_ours(path: Path) -> bool:
    """True if ``path`` is a schema this tool emitted (its ``$id`` is ours)."""
    try:
        doc = json.loads(path.read_text())
    except (OSError, ValueError):
        return False
    return isinstance(doc, dict) and str(doc.get("$id", "")).startswith(SCHEMA_ID_BASE)


def _prune_stale(output_dir: Path, written: set[Path]) -> list[Path]:
    """Delete every schema of ours under ``output_dir`` that this run did not write.

    Schemas/ is wholly generated, so a schema the generators no longer produce is
    stale: a spec renamed or removed, or a domain moved. Left in place it would
    keep being read by tools that glob the tree and, since ``git diff`` ignores
    files it has not been told about, never be flagged.

    "Ours" means a JSON file whose ``$id`` starts with ``SCHEMA_ID_BASE``. Any
    other file, and any directory that holds one, is left alone, so a preview into
    a directory with unrelated content (``--output-dir .``) cannot destroy it.
    Directories this pruning emptied are removed. Returns the files deleted.
    """
    removed: list[Path] = []
    touched: set[Path] = set()
    for path in sorted(output_dir.rglob("*.json")):
        if path.resolve() not in written and _is_ours(path):
            path.unlink()
            removed.append(path)
            touched.add(path.parent)
            print(f"  pruned stale {_display_path(path)}")
    for directory in sorted(touched, reverse=True):
        if directory != output_dir and not any(directory.iterdir()):
            directory.rmdir()
    return removed


def main(argv: list[str] | None = None) -> int:
    import argparse

    parser = argparse.ArgumentParser(description="Emit the TOPAS 3000A JSON Schemas.")
    parser.add_argument(
        "--phase", type=int, default=None,
        help=f"emit domains up to this phase (default: the live phase, {SCOPE.live_phase}, from scope.toml)",
    )
    parser.add_argument(
        "--output-dir", type=Path, default=None,
        help="write here instead of Schemas/ (required to emit a phase other than the live one)",
    )
    args = parser.parse_args(argv)
    phase = SCOPE.live_phase if args.phase is None else args.phase
    out = OUTPUT_DIR if args.output_dir is None else args.output_dir.resolve()
    print(f"Emitting TOPAS 3000A JSON Schemas for phase {phase}...")
    try:
        emit_all(phase, out)
    except ScopeError as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 2
    print(f"Done. {len(specs_for_phase(phase))} files written to {_display_path(out)}/.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
