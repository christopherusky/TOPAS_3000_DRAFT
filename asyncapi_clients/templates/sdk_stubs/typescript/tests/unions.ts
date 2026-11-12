/*
 * File:        sdk_stubs/typescript/tests/unions.ts
 * Origin:      Hand-written regression test for the TypeScript SDK
 * Changes:     Hand-edit here, not in output/ — copied into
 *              output/typescript/sdk/tests/ by step_sdk_stubs
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-08-28
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 *
 * A type-level test: it asserts nothing at runtime and exists to be
 * compiled. `tsc --noEmit` failing IS the failure.
 *
 * quicktype used to flatten a discriminated `oneOf` into one interface
 * carrying every variant's fields with the extras optional, so nothing
 * distinguished a reset command from a calibrate one and a caller could
 * build a "reset" that also carried threshold values. The union is now
 * emitted as a real discriminated union, which gives back both narrowing
 * and exhaustiveness checking.
 */
import type {
  DetectorCommand,
  ResetDetectorCommand,
  CalibrateDetectorCommand,
  SetThresholdDetectorCommand,
} from "../detector";
import {
  CalibrateDetectorCommandKind,
  ResetDetectorCommandKind,
  SetThresholdDetectorCommandKind,
} from "../detector";

/** Narrowing: inside each branch the variant type is known. */
export function describe(command: DetectorCommand): string {
  // Switched on the generated enum members rather than string literals:
  // quicktype types `kind` as a single-member enum per variant, and
  // TypeScript does not narrow an enum-typed value against a bare string.
  switch (command.kind) {
    case ResetDetectorCommandKind.Reset: {
      const reset: ResetDetectorCommand = command;
      return `reset ${reset.detectorId}`;
    }
    case CalibrateDetectorCommandKind.Calibrate: {
      const calibrate: CalibrateDetectorCommand = command;
      return `calibrate ${calibrate.detectorId}`;
    }
    case SetThresholdDetectorCommandKind.SetThreshold: {
      const threshold: SetThresholdDetectorCommand = command;
      // Only this variant carries the threshold fields — under the old
      // flattened interface they were optional on every command.
      return `threshold ${threshold.thresholdUp}/${threshold.thresholdDown}`;
    }
    default:
      // Exhaustiveness: adding a variant without handling it here makes
      // this assignment a compile error.
      return assertNever(command);
  }
}

function assertNever(value: never): never {
  throw new Error(`unhandled variant: ${JSON.stringify(value)}`);
}
