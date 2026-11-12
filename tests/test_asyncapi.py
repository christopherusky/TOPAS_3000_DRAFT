# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-03
# Version:     1.8
# Description: AsyncAPI document generation: channel naming across simple,
#              multi-word and compound domains, content types, channel counts,
#              exclusion of library schemas, dot-separated addresses,
#              component message references, envelope traits and JSON content
#              declarations.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for AsyncAPI document generation.

Validates the generated asyncapi.yaml against design decisions A1-A8.
"""

import yaml
import pytest
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

# Import from generators
import sys
sys.path.insert(0, str(ROOT))
from bin.emit import FILE_SPECS
from bin.emit_asyncapi import (
    generate_asyncapi,
    generate_channels,
    generate_messages,
    _schema_to_channel,
    _schema_to_content_type,
)


# ── Channel name derivation ──

def test_channel_name_simple_domain():
    assert _schema_to_channel("detector_measurement-v1.0.json") == "detector.measurement"


def test_channel_name_multi_word_domain():
    assert _schema_to_channel("car_park_definition-v1.0.json") == "car-park.definition"


def test_channel_name_multi_word_category():
    assert _schema_to_channel("detector_command_response-v1.0.json") == "detector.command-response"


def test_channel_name_traffic_event():
    assert _schema_to_channel("traffic_event_accident-v1.0.json") == "traffic-event.accident"


def test_channel_name_air_quality_enhanced():
    assert _schema_to_channel("air_quality_enhanced_measurement-v1.0.json") == "air-quality-enhanced.measurement"


def test_channel_name_network():
    assert _schema_to_channel("network_node-v1.0.json") == "network.node"


def test_channel_name_strategy():
    assert _schema_to_channel("strategy_command-v1.0.json") == "strategy.command"


# ── Content type derivation ──

def test_content_type_simple():
    assert _schema_to_content_type("detector_measurement-v1.0.json") == "detector.measurement-v1.0"


def test_content_type_multi_word():
    assert _schema_to_content_type("car_park_definition-v1.0.json") == "car-park.definition-v1.0"


# ── Channel generation ──

def test_channel_count():
    """Every non-library, non-reply FileSpec produces a channel.

    Replies are addressed to the requester's reply node (A11/A12), so they
    deliberately have no channel — one would describe an address that does
    not exist at runtime.
    """
    channels = generate_channels()
    publishable = [
        s for s in FILE_SPECS
        if s.canonical is not None and not s.reply_only
    ]
    assert len(channels) == len(publishable)


def test_library_schemas_excluded():
    """Library schemas (common, fault_support, traffic_event_common) have no channels."""
    channels = generate_channels()
    assert "common" not in channels
    assert "fault-support" not in channels
    assert "traffic-event.common" not in channels


def test_all_channels_dot_separated():
    """Every channel name uses dot-separated hierarchical naming (A8)."""
    channels = generate_channels()
    for name in channels:
        assert "." in name, f"Channel {name} is not dot-separated"
        assert "_" not in name, f"Channel {name} contains underscores"


def test_all_channels_reference_component_messages():
    """Every channel subscribe/publish uses a $ref to components/messages."""
    channels = generate_channels()
    for name, ch in channels.items():
        sub_ref = ch["subscribe"]["message"].get("$ref", "")
        pub_ref = ch["publish"]["message"].get("$ref", "")
        assert sub_ref.startswith("#/components/messages/"), \
            f"Channel {name} subscribe not referencing components/messages"
        assert pub_ref.startswith("#/components/messages/"), \
            f"Channel {name} publish not referencing components/messages"
        assert sub_ref == pub_ref, \
            f"Channel {name} subscribe and publish reference different messages"


def _is_request_or_reply(name: str) -> bool:
    return name.endswith(("Command", "CommandResponse", "SyncRequest", "SyncComplete"))


def test_data_messages_have_the_envelope_trait():
    """Data messages reference topasEnvelope, and only data messages do."""
    messages = generate_messages()
    for name, msg in messages.items():
        refs = [t.get("$ref", "") for t in msg.get("traits", [])]
        if _is_request_or_reply(name):
            continue
        assert refs == ["#/components/messageTraits/topasEnvelope"], \
            f"Data message {name} should carry exactly the data envelope, got {refs}"


def test_request_and_reply_messages_have_the_request_reply_trait_only():
    """Commands, responses, sync requests and SyncComplete are not data messages,
    so the data envelope's required headers must not apply to them."""
    messages = generate_messages()
    checked = 0
    for name, msg in messages.items():
        if not _is_request_or_reply(name):
            continue
        refs = [t.get("$ref", "") for t in msg.get("traits", [])]
        assert refs == ["#/components/messageTraits/topasRequestReply"], \
            f"{name} should carry exactly the request/reply trait, got {refs}"
        checked += 1
    assert checked >= 28


def test_messages_declare_json_schema_format():
    """Payloads declare JSON Schema format so AsyncAPI validator treats
    them as JSON Schema rather than AsyncAPI-native schemas."""
    messages = generate_messages()
    for name, msg in messages.items():
        assert msg.get("schemaFormat") == "application/schema+json;version=draft-07", \
            f"Message {name} missing or wrong schemaFormat"
        ref = msg.get("payload", {}).get("$ref", "")
        assert ref, f"Message {name} missing payload $ref"


def test_all_messages_have_payload_ref():
    """Every component message has a payload $ref to a schema file."""
    messages = generate_messages()
    for name, msg in messages.items():
        payload = msg.get("payload", {})
        ref = payload.get("$ref", "")
        assert ref.startswith("Schemas_asyncapi/"), f"Message {name} payload ref: {ref}"
        assert ref.endswith("-v1.0.json"), f"Message {name} payload ref: {ref}"


def test_payload_refs_resolve():
    """Every payload $ref points to an existing schema file."""
    messages = generate_messages()
    for name, msg in messages.items():
        ref = msg["payload"]["$ref"]
        schema_path = ROOT / ref
        assert schema_path.exists(), f"Message {name}: {ref} does not exist"


def test_messages_exceed_channels_by_the_reply_count():
    """Replies keep a components/messages entry but lose their channel."""
    channels = generate_channels()
    messages = generate_messages()
    replies = [s for s in FILE_SPECS if s.canonical is not None and s.reply_only]
    assert replies, "expected reply-only specs"
    assert len(messages) == len(channels) + len(replies)


def test_wildcard_pattern_detector():
    """detector.* should match all detector channels."""
    channels = generate_channels()
    detector_channels = [n for n in channels if n.startswith("detector.")]
    # def, config, meas, cmd, fault, type, history, sync-request.
    # cmd-resp and sync-complete are replies and have no channel (A11/A12).
    assert len(detector_channels) == 8


def test_wildcard_pattern_all_faults():
    """*.fault should match fault channels from all device domains."""
    channels = generate_channels()
    fault_channels = [n for n in channels if n.endswith(".fault")]
    assert len(fault_channels) >= 5  # every live device domain has faults


def test_no_reply_channels():
    """Command responses and sync completions are addressed, not published."""
    channels = generate_channels()
    assert [n for n in channels if n.endswith(".command-response")] == []
    assert [n for n in channels if n.endswith(".sync-complete")] == []


def test_request_channels_carry_the_reply_binding():
    """Each request channel describes its reply in the x- extension, since
    AsyncAPI 2.6 has no request/reply primitive (A11/A12)."""
    channels = generate_channels()
    cmd = channels["detector.command"]["x-amqp1-request-reply"]
    assert cmd["fanout"] is False
    assert cmd["replyAddress"]["requestProperty"] == "reply-to"
    assert cmd["replyAddress"]["required"] is True
    assert cmd["responseDestination"]["valueFrom"] == "request.reply-to"
    assert cmd["transportCorrelation"] == {
        "requestProperty": "message-id",
        "responseProperty": "correlation-id",
    }
    assert cmd["applicationCorrelation"]["payloadProperty"] == "commandCorrelationId"
    assert cmd["responseMessage"]["$ref"].endswith("DetectorCommandResponse")

    sync = channels["detector.sync-request"]["x-amqp1-request-reply"]
    assert sync["applicationCorrelation"]["payloadProperty"] == "syncCorrelationId"
    assert sync["responseMessage"]["$ref"].endswith("DetectorSyncComplete")


# ── Full document structure ──

def test_asyncapi_version():
    doc = generate_asyncapi()
    assert doc["asyncapi"] == "2.6.0"


def test_asyncapi_has_server():
    doc = generate_asyncapi()
    assert "production" in doc["servers"]
    assert doc["servers"]["production"]["protocol"] == "amqp"


def test_asyncapi_has_security():
    doc = generate_asyncapi()
    assert "mtls" in doc["components"]["securitySchemes"]
    assert doc["components"]["securitySchemes"]["mtls"]["type"] == "X509"


def test_asyncapi_has_envelope_trait():
    doc = generate_asyncapi()
    trait = doc["components"]["messageTraits"]["topasEnvelope"]
    headers = trait["headers"]["properties"]
    assert "x-message-mode" in headers
    assert "x-content-type" in headers
    assert "x-geo-tile" in headers
    assert "x-geo-tile-system" in headers
    assert "x-sync-id" in headers
    assert "x-sync-sequence" in headers
    assert "x-sync-total" in headers
    # x-originator belongs to request/reply messages, not data messages.
    assert "x-originator" not in headers


def test_request_reply_trait_carries_only_the_audit_label():
    doc = generate_asyncapi()
    trait = doc["components"]["messageTraits"]["topasRequestReply"]
    assert list(trait["headers"]["properties"]) == ["x-originator"]
    assert "required" not in trait["headers"]


def test_envelope_required_headers():
    doc = generate_asyncapi()
    required = doc["components"]["messageTraits"]["topasEnvelope"]["headers"]["required"]
    assert "x-message-mode" in required
    assert "x-content-type" in required


def test_message_mode_enum():
    doc = generate_asyncapi()
    mode = doc["components"]["messageTraits"]["topasEnvelope"]["headers"]["properties"]["x-message-mode"]
    assert mode["enum"] == ["sync", "delta"]


def test_geo_tile_system_enum():
    doc = generate_asyncapi()
    system = doc["components"]["messageTraits"]["topasEnvelope"]["headers"]["properties"]["x-geo-tile-system"]
    assert set(system["enum"]) == {"geohash", "s2", "intercor"}


# ── Idempotency ──

def test_generation_is_idempotent():
    """Running generate_asyncapi twice produces identical output."""
    first = yaml.dump(generate_asyncapi(), sort_keys=False)
    second = yaml.dump(generate_asyncapi(), sort_keys=False)
    assert first == second


# ── Conditional header rules (envelope) ──

import jsonschema


def _envelope_validator():
    headers = generate_asyncapi()["components"]["messageTraits"]["topasEnvelope"]["headers"]
    return jsonschema.Draft7Validator(headers)


_SYNC_IDS = {
    "x-sync-id": "3f2a8c1e-0000-4000-8000-000000000001",
    "x-sync-sequence": 1,
    "x-sync-total": 3,
}


def test_delta_message_needs_no_sync_headers():
    _envelope_validator().validate(
        {"x-message-mode": "delta", "x-content-type": "detector.measurement-v1.0"}
    )


def test_sync_message_with_its_batch_headers_is_valid():
    _envelope_validator().validate(
        {"x-message-mode": "sync", "x-content-type": "detector.measurement-v1.0", **_SYNC_IDS}
    )


@pytest.mark.parametrize("missing", ["x-sync-id", "x-sync-sequence", "x-sync-total"])
def test_sync_message_missing_a_batch_header_is_rejected(missing):
    headers = {"x-message-mode": "sync", "x-content-type": "detector.measurement-v1.0", **_SYNC_IDS}
    del headers[missing]
    with pytest.raises(jsonschema.ValidationError):
        _envelope_validator().validate(headers)


def test_geo_tile_without_a_system_is_rejected():
    base = {"x-message-mode": "delta", "x-content-type": "detector.measurement-v1.0"}
    with pytest.raises(jsonschema.ValidationError):
        _envelope_validator().validate({**base, "x-geo-tile": "gcpvj0"})
    _envelope_validator().validate({**base, "x-geo-tile": "gcpvj0", "x-geo-tile-system": "geohash"})


def test_data_message_without_mode_or_content_type_is_rejected():
    with pytest.raises(jsonschema.ValidationError):
        _envelope_validator().validate({"x-content-type": "detector.measurement-v1.0"})
    with pytest.raises(jsonschema.ValidationError):
        _envelope_validator().validate({"x-message-mode": "delta"})


# ── Sender role ──


def test_every_channel_names_its_sending_role():
    """Stated from the channel names, not from the condition the generator uses."""
    channels = generate_asyncapi()["channels"]
    assert channels, "no channels generated"
    for name, chan in channels.items():
        expected = (
            "data-subscriber"
            if name.endswith((".command", ".sync-request"))
            else "data-publisher"
        )
        assert chan["x-sender"] == expected, name
        assert ("x-amqp1-request-reply" in chan) == (expected == "data-subscriber"), name


def test_message_traits_agree_with_the_file_spec_flags():
    """Each message's trait follows the spec flags, and so does the name-based split."""
    from bin.emit import FILE_SPECS
    from bin.emit_asyncapi import _pascal_to_message_name

    messages = generate_messages()
    for spec in FILE_SPECS:
        if spec.canonical is None:
            continue
        name = _pascal_to_message_name(spec.title)
        is_exchange = bool(spec.reply_only or spec.reply_filename)
        assert _is_request_or_reply(name) == is_exchange, name
        expected = "topasRequestReply" if is_exchange else "topasEnvelope"
        assert messages[name]["traits"] == [{"$ref": f"#/components/messageTraits/{expected}"}], name


def test_transformed_schemas_refuse_a_stale_schemas_directory(tmp_path, monkeypatch):
    """asyncapi.yaml is built from the live registry; Schemas_asyncapi/ from disk.
    If Schemas/ does not match the registry the generator must stop, not emit both."""
    import bin.emit_asyncapi as ea

    (tmp_path / "detector").mkdir()
    (tmp_path / "detector" / "detector_measurement-v1.0.json").write_text("{}")
    monkeypatch.setattr(ea, "CANONICAL_SCHEMAS_DIR", tmp_path)
    monkeypatch.setattr(ea, "TRANSFORMED_SCHEMAS_DIR", tmp_path.parent / "transformed_unused")
    with pytest.raises(SystemExit, match="make schemas"):
        ea.generate_transformed_schemas()
