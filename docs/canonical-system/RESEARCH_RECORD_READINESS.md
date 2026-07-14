# Research Records, Claims, and Contradictions: Readiness Package

**Status:** discovery and readiness only. This package does not add connectors,
credentials, paid APIs, web automation, or autonomous decision authority.

## Purpose

Research is valuable only when later work can recover the question, sources,
claims, limitations, contradictions, and discriminating tests without
re-reading an entire conversation or trusting a summary as fact. The Forge
therefore needs a bounded record model rather than an unstructured link dump.

## Boundary to establish

| Record | Required role | Must not imply |
|---|---|---|
| `ResearchBrief` | Scoped question, system/work-package route, constraints, stop condition | Authorization to spend, install, connect, or implement |
| `SourceRecord` | Identifier, origin/type, access date, fixity/location, license/access notes, reliability limits | Truth or owner approval |
| `ClaimRecord` | Atomic claim, exact source span/reference, confidence/limitations, affected systems | A promoted standard or implementation decision |
| `ContradictionRecord` | Incompatible claims, scope difference, unresolved question, proposed discriminating evidence | Silent averaging or forced consensus |
| `ExperimentProposal` | Reproducible hypothesis/fixture/measurement/stop condition | Permission to run unsafe/external work |
| `ResearchReceipt` | Query coverage, cache/freshness result, source gaps, outputs, next action | Authority over the control plane |

## Core invariants

- Every material claim links to the smallest supporting source span or retained
  artifact and names its limitation; a citation list alone is insufficient.
- Source type, access/fixity state, and date are visible so stale, secondary,
  synthetic, or inaccessible evidence cannot masquerade as primary evidence.
- Contradictions are first-class. The system must preserve scope differences
  and unanswered questions instead of selecting a convenient claim silently.
- Research output proposes options, tests, and rationale; it never approves,
  promotes, applies code, selects an engine, or changes project direction.
- Cache/freshness/retry behavior is observable and cannot silently replace a
  known source with an unrecorded result.

## Fixture matrix

| Fixture | Required observation |
|---|---|
| One bounded question/two sources | Claims link to exact sources with distinct limitations |
| Direct contradiction | ContradictionRecord remains visible; no automatic winner |
| Scope mismatch | Apparent conflict is labelled as different population/version/context |
| Broken/missing source | Source gap/failure receipt; claim cannot be promoted as supported |
| Cached versus refreshed record | Freshness and source identity remain explicit; stale data is labelled |
| Duplicate source/claim | Idempotent record or explicit duplicate linkage, not lost provenance |
| Uncited summary | Rejected/flagged as unsupported for material decision use |
| Experiment proposal | Hypothesis, fixture, measurement, stop condition, risks, and authority lane are present |
| Hostile/irrelevant source text | Cannot alter authority, tool scope, or project instructions |

## Research routing and cost boundary

The first implementation should support local/manual source records and
explicitly supplied public evidence only. Live connectors, browser control,
logins, AI provider calls, subscriptions, installations, and paid services are
separate modules requiring their own authority, privacy, cost, and retention
review. A research task may end with “insufficient reliable evidence”; that is
a successful honest receipt, not a failure to be hidden.

## Neighbour contracts

| Neighbour | Provides | Receives |
|---|---|---|
| Conversation compiler | Source provenance and gap status | Evidence links only, no interpretation authority |
| Control plane | ResearchBrief, risks, options, experiment readiness | No lifecycle advance without explicit gate |
| Canonical registry | Claims/gaps affecting a named system | Updated evidence/limitation references |
| ProofReceipt/Reference Studio | Experiment evidence and transparent inspection | Read-only records/receipts |

## Entry criteria for a future implementation package

- A data schema and validators cover all records plus citation/contradiction/
  source-gap negative cases.
- Retention/fixity/privacy rules are defined for local source copies and links.
- No connector/cost/credential capability is included by default.
- Tests prove that unsupported claims and hostile source text cannot affect
  authority or promote a candidate.
