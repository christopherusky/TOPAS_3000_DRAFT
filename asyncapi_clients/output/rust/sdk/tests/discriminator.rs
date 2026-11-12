/*
 * File:        sdk_stubs/rust/tests/discriminator.rs
 * Origin:      Hand-written regression test for the Rust SDK
 * Changes:     Hand-edit here, not in output/ — this file is copied into
 *              output/rust/sdk/tests/ by step_sdk_stubs
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-08-28
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 *
 * typify renders a discriminated union as `#[serde(untagged)]`, which can
 * only discriminate if each variant's `kind` is a type that rejects the
 * other variants' values. It renders JSON Schema `const` as a plain
 * String, which rejects nothing — and ResetDetectorCommand and
 * CalibrateDetectorCommand have identical property sets, so before the fix
 * a `calibrate` command silently deserialised as ResetDetectorCommand.
 *
 * The generator now presents `const` as a single-member `enum` on the copy
 * handed to typify, which produces a unit type per variant. This test is
 * what stops that regressing.
 */
use utmc_sdk::detector::DetectorCommand;

fn variant(c: &DetectorCommand) -> &'static str {
    match c {
        DetectorCommand::ResetDetectorCommand(_) => "reset",
        DetectorCommand::CalibrateDetectorCommand(_) => "calibrate",
        DetectorCommand::SetThresholdDetectorCommand(_) => "setThreshold",
    }
}

#[test]
fn discriminator_selects_the_right_variant() {
    // Reset and Calibrate have identical property sets; `kind` is the only
    // thing that separates them. Before the fix both deserialised as Reset.
    for kind in ["reset", "calibrate"] {
        let json = format!(
            r#"{{"kind":"{kind}","commandCorrelationId":7,"detectorId":"DET-1",
                 "requestTime":"2026-08-28T12:00:00Z"}}"#
        );
        let parsed: DetectorCommand = serde_json::from_str(&json)
            .unwrap_or_else(|e| panic!("{kind} failed to deserialise: {e}"));
        assert_eq!(variant(&parsed), kind, "wrong variant for kind={kind}");
    }
}

#[test]
fn a_bogus_discriminator_is_rejected() {
    let json = r#"{"kind":"explode","commandCorrelationId":7,"detectorId":"DET-1",
                   "requestTime":"2026-08-28T12:00:00Z"}"#;
    assert!(serde_json::from_str::<DetectorCommand>(json).is_err());
}
