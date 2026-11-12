/*
 * File:        output/typescript/sdk/fault_support.ts
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
export interface AllTypes {
    AcknowledgementState?: AcknowledgementState;
    FaultCategory?:        FaultCategory;
    FaultStatus?:          FaultStatus;
    FaultTypeDefinition?:  FaultTypeDefinition;
    [property: string]: any;
}

/**
 * Acknowledgement state of a device fault.
 *
 * Maps from legacy XSD integer values:
 * - 0 -> notAcknowledged
 * - 1 -> acknowledged
 * - 2 -> deferred
 * - 3 -> recordedForAction
 */
export enum AcknowledgementState {
    Acknowledged = "acknowledged",
    Deferred = "deferred",
    NotAcknowledged = "notAcknowledged",
    RecordedForAction = "recordedForAction",
}

/**
 * Coarse grouping for catalogue entries.
 *
 * Deliberately short. It exists so an operator can filter "show me the
 * communications faults" without the catalogue having to agree on a
 * taxonomy first; the precise meaning of a fault is its catalogue entry,
 * not its category.
 *
 * Coarse grouping, for filtering rather than semantics.
 */
export enum FaultCategory {
    Communications = "communications",
    Environmental = "environmental",
    Hardware = "hardware",
    Other = "other",
    SignalTimings = "signalTimings",
}

/**
 * Whether a fault is still active or has been cleared.
 *
 * This is the only fault lifecycle the payload carries. Record archival
 * — distinguishing a live record from a retired one — is not modelled
 * here and is a matter for the publishing system.
 */
export enum FaultStatus {
    Active = "active",
    Cleared = "cleared",
}

/**
 * Catalogue entry describing one kind of fault that can be reported.
 */
export interface FaultTypeDefinition {
    /**
     * Coarse grouping, for filtering rather than semantics.
     */
    category?: FaultCategory;
    /**
     * The identifier this fault carries in its source document or vendor documentation — 'A1',
     * 'F3', 'W12'. Distinct from faultTypeId, which is this catalogue's key: a deployment may
     * hold two entries with the same code from different vendors.
     */
    code?: string;
    /**
     * Human-readable description of the fault type.
     */
    description: string;
    /**
     * Vendor or local extensions. Open-content object.
     */
    extensions?: { [key: string]: any };
    /**
     * Identifier of this fault type.
     */
    faultTypeId: string;
    /**
     * Human-readable message with positional placeholders, e.g. 'Minimum green not met: stage
     * {0} ran {1}s against a minimum of {2}s'. A fault instance supplies the values in its
     * parameters field. This is what lets a catalogue carry faults whose meaning depends on
     * runtime values — the stuck-stage and interstage timing faults that a closed enum cannot
     * express.
     */
    messageTemplate?: string;
    /**
     * Names for the positional placeholders in messageTemplate, in order — ['stageNumber',
     * 'actualSeconds', 'minimumSeconds']. Lets a consumer label the values rather than only
     * interpolate them. Length SHOULD match the number of placeholders.
     */
    parameterNames?: string[];
    /**
     * Numeric severity (higher = more severe). Optional.
     */
    severity?: number;
    /**
     * Reference to a SubsystemDefinition entry: the subsystem this fault type belongs to.
     */
    subsystemTypeId?: string;
}
