# Stage Quality Protocol

This protocol makes three owner-approved working principles mechanically
testable for every material Forge package. It does not grant approval,
promotion, external execution, or protected-Kernel authority.

## 1. Two-scale context refresh

Before implementation and whenever `substage_id` changes, update the sole
canonical Worker Batch State with `stage_context`:

- `stage_id` exactly equals the active `substage_id`;
- `macro_sources` and `macro_findings` cover the master objective, Atlas and
  dependency route, authority boundary, neighbouring contracts, and systemic
  risks;
- `micro_sources` and `micro_findings` cover exact inputs, identities,
  invariants, known failures, fixtures, tests, and exact next action.

Empty lists or a stale stage ID fail closed. Generated briefings remain views,
not a second authored context record.

Worker Batch State schema 3 binds every handoff-critical section through
`handoff_section_receipts`: objective, next action, two-scale context, visual
gate, simulation ladder, unresolved risks, evidence requirements, verification
plan, resume condition, evidence, verification receipts, and transition. One
shared integrity helper defines that section set for both enforcement and
fixtures. Each receipt names the active substage, hashes the section's canonical
JSON content, classifies it as `revised` or `carried_forward`, and records a
specific review note. Missing, unknown, stale, malformed, or content-mismatched
receipts fail bootstrap before generated handover views refresh. Hashes prove
fixity after attestation; they do not prove semantic comprehension, so bulk
retagging or dishonest review remains prohibited rather than falsely claimed
as mechanically solved.

## 2. Visual asset fitness gate

`visual_quality_gate.asset_use_intent` declares whether the current substage
will use a visual asset as a reference, comparison target, fixture, or
candidate. If false, status is `not_applicable` with a concrete reason. If
true, status is `required_pending` during bounded source/inspection work and
must set `dependent_implementation_blocked=true`. It becomes `passed` only
before dependent implementation and every admitted asset then has a canonical
receipt containing:

- content fingerprint, source/provenance, license or authority status;
- actual rendered views inspected at useful scale and inspection conditions;
- intended comparison and necessary accuracy;
- visible defects, occlusion, resolution, pose, view, lighting, and detail
  limits;
- disposition: `verified_fit`, `owner_check_required`, or `rejected`.

No receipt may pass from metadata, filename, dimensions, decode success, or
hash alone. Human references additionally require coherent visible anatomy and
proportions, sufficient completeness, an unobscured comparison region, and
appropriate pose, view, lighting, and detail. "Good quality" means fit for the
specific comparison, not an invented universal beauty score. Any uncertainty
or creative ambiguity becomes `owner_check_required`; dependent work pauses
and presents one labelled current-fixture comparison in chat.

## 3. Cheapest-sufficient proof ladder

The ordered tiers are:

1. `static_reasoning`
2. `typed_model`
3. `in_memory_fixture`
4. `disposable_simulation`
5. `bounded_integrated_pc`
6. `external_execution`

`simulation_ladder.cheapest_sufficient_tier` names the current tier and
`tiers_completed` retains results from all applicable cheaper tiers.
`expensive_execution_planned` is true for tiers 5 or 6 and then requires a
non-empty unresolved risk, expected information gain, estimated cost, retained
lower-tier result, regression guard, and stop condition. Simulation is an
early falsification tool; it never erases the package's final integration gate.

## Verification

`tools/verify-stage-quality-gates.ps1` validates the active record.
`tools/test-stage-quality-gates.ps1` proves stale context, metadata-only visual
admission, and unjustified expensive escalation fail closed.
`tools/test-worker-batch-state.ps1` proves old schemas, partial transitions,
post-review edits, and missing or unknown handoff receipts fail closed.
