# G1 GP3 encounter grammar readiness

Status: owner-authorized boundary frozen for adversarial review; source
implementation has not begun.

## Dependency and authority

GP0 owns the causal-explorer-maker promise and exactly five corrected authored
sessions. GP1 proves their deterministic base loop. GP2 proves typed
progression without granting encounters automatic progression authority. C4V
is separately closed and is not a GP3 dependency.

GP3 may add one strict, engine-neutral encounter-grammar record for each GP0
session. It may classify authored facets, reference exact GP0 facts and risks,
name explainable noncombat resolutions, and reference exact GP0 outcomes. It
may not generate or select content, assign weights or probabilities, execute a
runtime encounter, resolve combat, interpret progression, persist state, make
C3B claims, or start GP4.

## Frozen authored registry

The registry contains exactly these five situations and no caller-supplied
extension point:

| Situation | Required domain facets | Exact evidence | Exact risk | Exact approaches to GP0 outcomes | Threat contribution |
|---|---|---|---|---|---|
| `gp3.s1.colony-conduit` | environment, creature, society, construction | `s1.flow-loss`, `s1.colony-distress` | `conduit-failure` | `s1.approach.direct` -> `s1.direct`; `s1.approach.bypass` -> `s1.bypass`; `s1.approach.ration` -> `s1.ration`; `s1.approach.retreat` -> `s1.retreat` | none |
| `gp3.s2.storm-nest` | environment, creature, society, construction | `s2.exposure`, `s2.crystal-hazard` | `storm-arrival` | `s2.approach.relocate` -> `s2.relocate`; `s2.approach.harvest` -> `s2.harvest`; `s2.approach.retreat` -> `s2.retreat` | optional `predator` |
| `gp3.s3.memory-gate` | environment, society, construction | `s3.ledger`, `s3.testimony` | `channel-harm` | `s3.approach.charter` -> `s3.charter`; `s3.approach.force` -> `s3.force`; `s3.approach.alternate` -> `s3.alternate`; `s3.approach.retreat` -> `s3.retreat` | none |
| `gp3.s4.signal-anchor` | environment, creature, society, anomaly, construction | `s4.timing`, `s4.wire-scavengers`, `s4.event` | `anchor-collapse` | `s4.approach.temporary` -> `s4.temporary-rescue`; `s4.approach.permanent` -> `s4.permanent`; `s4.approach.long` -> `s4.long-route`; `s4.approach.retreat` -> `s4.retreat` | optional `wire-scavengers` |
| `gp3.s5.afterlight` | environment, creature, society, construction | `s5.history`, `s5.relocation` | `buffer-violation` | `s5.approach.nightway` -> `s5.nightway`; `s5.approach.dismantle` -> `s5.dismantle`; `s5.approach.retreat` -> `s5.retreat` | optional `food-scavengers` |

Every approach has a distinct ID, exact prepared tool or explicit absence,
ordered intervention steps, exact prerequisites, per-risk disposition, causal
explanation and a complete abstract consequence-reference set. Every listed
outcome remains owned by GP0. GP3 references every consequence element by
kind, ordinal and canonical digest exactly once; it does not copy, weaken,
reinterpret, or add payloads. The exact fixed values are frozen in
`G1_GP3_ENCOUNTER_GRAMMAR_FIXED_REGISTRY.md`.

## Frozen semantic invariants

- each situation carries its exact ordered `domain_facets`; each facet is a
  distinct tagged record with exact authored proposition and supporting GP0
  evidence references, and a situation cannot carry another domain's fields;
- every evidence reference binds fact ID, expected `FactKind`, exact
  `AuthoredGameplayNonC3B` class and canonical fact digest; mutated,
  `ObservedC3AOutput`, foreign and unsupported evidence is rejected;
- every situation binds its exact GP0 session digest and at least one exact GP0
  accessible risk digest;
- every situation exposes at least two noncombat, threat-free causal
  resolutions plus exactly one retreat;
- `s3.force` is the only `force_partial` resolution, references the GP0 outcome
  whose `resolves_core_tension` flag is false, and cannot be described as
  complete;
- a threat contribution is optional and nonterminal, may appear only in S2,
  S4, and S5, and references the exact GP0 `ThreatContribution`; it is never a
  resolution or an outcome prerequisite;
- all 18 GP0 outcomes are referenced exactly once, with a trigger matching the
  GP3 approach kind and complete consequence references covering every exact
  mutation, fixture-owned opportunity cost, memory, grant and named decision;
  when optional threat composition is selected, its world-contribution elements
  are separately referenced exactly once as nonterminal `threat_contribution`
  elements and never enter the outcome set;
- every causal explanation is a closed chain from admitted facet evidence,
  through its ordered intervention steps, to the complete exact outcome
  elements and per-risk disposition, with an explicit limitation; motive,
  unseen evidence and erased liability cannot enter the chain;
- every S5 approach requires the exact latest prior S1 outcome, admits only
  direct, bypass or ration, and rejects missing, stale, reordered or retreat
  predecessors inside GP3 validation;
- the registry is fixed authored data: no seed, weight, score, probability,
  random choice, procedural rule, generated candidate, or caller extension;
- GP3 imports no GP2 record or mapping and emits no progression result.

## Codec and resource bounds

The registry and each public situation use strict canonical JSON with
`deny_unknown_fields` on every nested public struct and tagged enum and
byte-for-byte round-trip equality. Decode rejects before allocation-intensive
parsing above 131,072 bytes for the complete registry or 32,768 bytes for one
situation. Strings are at most 1,024 bytes; IDs are at most 96 bytes; facets,
evidence, risks, approaches, steps, prerequisites, risk dispositions and
consequence references are each capped at 32 elements before semantic use.
After decoding, every situation must equal the entire corresponding fixed
authored situation, including order and authored facet, step, explanation and
limitation text. Registry order and all exact counts are fixed.

## Adversarial matrix required before implementation acceptance

Red tests must reject:

- zero, duplicate, sixth, reordered, foreign-session, missing/mutated
  session-digest, domain-swapped, missing-facet, extra-facet or reordered-facet
  situations;
- missing, duplicated, foreign, or invented evidence, risk, outcome, retreat,
  or threat references;
- fewer than two threat-free noncombat resolutions, a second retreat, combat
  as a resolution, or any threat contribution used as a terminal route;
- `s3.force` marked complete, another force route, or a force reference to a
  core-tension-resolving outcome;
- wrong approach ID, prepared tool, step order, prerequisite, risk disposition,
  explanation, limitation, consequence kind/ordinal/digest, incomplete or
  duplicated consequence coverage, motive, unseen evidence, or erased cost;
- missing, stale, reordered or retreat S5 predecessor;
- copied or caller-supplied consequences, GP2 mapping, weights, seeds,
  probabilities, generation fields, runtime authority, or C3B authority;
- unknown JSON fields, trailing bytes, noncanonical field order, oversized
  payloads, schema drift, malformed identifiers, and mutated fixed registry
  bytes.

Positive tests must prove the required multi-domain shapes are materially
different but explainable, every approach closes its evidence-to-intervention-
to-consequence chain against the exact GP0 outcome, all consequence elements
are referenced exactly once, both required threat-free routes and retreat exist
per situation, exact latest S1 predecessor semantics hold for S5, threat
contribution composes optionally without terminal authority, and strict codecs
round-trip deterministically.

## Readiness decision

The proposed grammar is bounded, authored-only, reversible, game-first, and
falsifiable. Independent pre-source review must accept this readiness record
and `G1_GP3_ENCOUNTER_GRAMMAR_DESIGN.md` before Rust source is added. Stop after
the recorded GP3 result; GP4 and runtime remain forbidden.
