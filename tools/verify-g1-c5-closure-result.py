#!/usr/bin/env python3
"""Strict cross-record verifier for the recorded C5 closure holding state."""
from __future__ import annotations
import argparse, json
from pathlib import Path
from typing import Any

AUTHORITY = "Owner-authorized recorded C5 significance/scheduler capability-free closure evidence only. Exact dependency C4. C5 remains the sole master-program cursor pending a separate C6 transition. No C3B, C6 activation, C7, broad G1 closure, runtime controllers, runtime executors, cache mutation, storage mutation, product weights, AI generation, rendering implementation, filesystem, network, process, Companion, Greenfield, visual assets, promotion authority or Kernel mutation."
C6_AUTHORITY = "Owner-authorized C6 semantic/construction and organism-ecology reconciliation and capability-free readiness only. Exact dependencies verified C4 and C5. Retain corrected C6 prerequisite evidence as non-closure evidence. No C6 implementation source, C3B, C7, broad G1 closure, runtime, product ontology or vocabulary, solver or AI generation, geometry, assets, animation, renderer, visual-quality claim, physiology or content constants, filesystem, network, process, Companion, Greenfield, promotion authority or Kernel mutation."
C6_BODY_PLAN_AUTHORITY = "Owner-authorized capability-free C6 body-plan family/topology V1 test-first implementation only. Exact dependencies verified C4 and C5. Authorizes the new body-plan-structure crate, one additive macro-lineage-binding family-reference validator, exact tests, governance projections and verification for this package. No ecology realization, physiology, reproduction, heredity, development, sex or dimorphism applicability, caste, species, individual or population semantics, personhood, product ontology, solver or AI generation, geometry, proportions, pose, assets, animation, renderer, visual-quality claim, runtime, filesystem, network, process, Companion, Greenfield, C7, broad G1 closure, promotion authority or Kernel mutation."
C6_IDENTITY_READINESS_AUTHORITY = "Owner-routed code-free C6 package-3 identity readiness only. Authorizes reconciliation of stale body-plan projections and design, adversarial review, fixtures, verifier and governance records for distinct lineage, organism-form, species-candidate, individual and population identity envelopes plus exact C4 lifecycle/history consumption. No production crate or source implementation; no asserted species membership, population members/count/distribution, ancestry/evolution inference, ecology, physiology, reproduction, heredity, development, sex, dimorphism, culture, representation, runtime, Companion, Greenfield, C7, promotion authority or Kernel mutation."
C6_IDENTITY_IMPLEMENTATION_AUTHORITY = "Owner-authorized capability-free C6 organism subject identity V1 test-first implementation only. Exact dependencies verified C4, C5 and body-plan V1. Authorizes the organism-subject-identity crate, one additive person-form-eligibility bound-subject evaluator, exact 33-group implementation matrix, module/governance projections and verification. No asserted species membership, population members/count/distribution, ancestry/evolution inference, ecology, physiology, reproduction, heredity, development, sex, dimorphism, caste, culture, capacity truth, comparison, representation, runtime, filesystem, network, process, Companion, Greenfield, C7, broad G1 closure, promotion authority or Kernel mutation."
SOURCE = "9e48dd117c2b22b62bd31dba15c10c3a9bf4b100"
TREE = "cfc58943f96fed768f77ac2a6e3256aa13d59d6c0edbe24f13cd967315038636"
BOUNDED = "9430bc530ba39403803a05fd99a9bc5c257472c2f320921ca242b51344947ecb"
SUCCESSOR = "1e77df61675512c905688ae9edcc90e32e62ed4740c87148bfd16807390a6fc3"
SEMANTIC = "88e2be61586e728613fe2c7bf5b947074459fc5f63d6e5f13d4f4648e64624eb"
REQUEST = "28b24d548656874a3c4f6f6bba1a40a0a716ac0603e9e38c40318c7d932bc58f"
RESULT = "4dd77d3b16927644af2c9bb1b74f76e1dd7cc279a09a8297d10738a0efce1bf4"
RUN = "run-87b9301f9bb54b2d9b72767643c7ed9b"
POST_RUN = "run-8296afcac8e949cca8b6a3693d1dfc3f"

def obj(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    out: dict[str, Any] = {}
    for key, value in pairs:
        if key in out: raise ValueError(f"duplicate JSON key: {key}")
        out[key] = value
    return out
def bad_float(_: str) -> Any: raise ValueError("floating-point JSON is forbidden")
def load(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8-sig"), object_pairs_hook=obj, parse_float=bad_float, parse_constant=bad_float)
    if type(value) is not dict: raise ValueError(f"{path.name} must contain one object")
    return value
def one(items: list[Any], ident: str) -> dict[str, Any]:
    found = [x for x in items if type(x) is dict and x.get("id") == ident]
    if len(found) != 1: raise ValueError(f"expected exactly one {ident}")
    return found[0]
def strings(value: Any, label: str) -> list[str]:
    if type(value) is not list or any(type(x) is not str for x in value) or len(value) != len(set(value)):
        raise ValueError(f"{label} must be a unique string array")
    return value

def verify(root: Path) -> None:
    checkpoint = load(root / "context/active/WORKER_BATCH_STATE.json")
    program = load(root / "docs/canonical-system/MASTER_PROGRAM.json")
    local = load(root / "docs/canonical-system/G1_C5_LOCAL_PLATFORM_OBSERVATIONS.json")
    external = load(root / "docs/canonical-system/G1_C5_INDEPENDENT_PLATFORM_EXECUTION.json")
    registry = load(root / "docs/canonical-system/system-registry.json")
    c6_readiness = checkpoint.get("batch_id") == "G1-C6-SEMANTIC-CONSTRUCTION-ORGANISM-ECOLOGY-READINESS-V1"
    c6_body_plan = checkpoint.get("batch_id") == "G1-C6-BODY-PLAN-STRUCTURE-IMPLEMENTATION-V1"
    c6_identity_readiness = checkpoint.get("batch_id") == "G1-C6-ORGANISM-IDENTITY-READINESS-V1"
    c6_identity_implementation = checkpoint.get("batch_id") == "G1-C6-ORGANISM-SUBJECT-IDENTITY-IMPLEMENTATION-V1"
    successor = c6_readiness or c6_body_plan or c6_identity_readiness or c6_identity_implementation
    expected = ({
        "batch_id":"G1-C6-SEMANTIC-CONSTRUCTION-ORGANISM-ECOLOGY-READINESS-V1", "master_program_item":"C6",
        "state":"executing", "previous_state":"ready", "substage_id":"c6-reconciliation-readiness",
        "authority_lane":C6_AUTHORITY,
    } if c6_readiness else {
        "batch_id":"G1-C6-BODY-PLAN-STRUCTURE-IMPLEMENTATION-V1", "master_program_item":"C6",
        "state":"executing", "previous_state":"ready", "substage_id":"c6-body-plan-structure-test-first-implementation",
        "authority_lane":C6_BODY_PLAN_AUTHORITY,
    } if c6_body_plan else {
        "batch_id":"G1-C6-ORGANISM-IDENTITY-READINESS-V1", "master_program_item":"C6",
        "state":"executing", "previous_state":"ready", "substage_id":"c6-organism-identity-readiness",
        "authority_lane":C6_IDENTITY_READINESS_AUTHORITY,
    } if c6_identity_readiness else {
        "batch_id":"G1-C6-ORGANISM-SUBJECT-IDENTITY-IMPLEMENTATION-V1", "master_program_item":"C6",
        "state":"executing", "previous_state":"ready", "substage_id":"c6-organism-subject-identity-test-first-implementation",
        "authority_lane":C6_IDENTITY_IMPLEMENTATION_AUTHORITY,
    } if c6_identity_implementation else {
        "batch_id":"G1-C5-SIGNIFICANCE-SCHEDULER-CLOSURE-V1", "master_program_item":"C5",
        "state":"recorded", "previous_state":"verifying", "substage_id":"c5-registered-closure-recorded",
        "authority_lane":AUTHORITY,
    })
    for key, wanted in expected.items():
        if type(checkpoint.get(key)) is not str or checkpoint[key] != wanted: raise ValueError(f"checkpoint {key} drifted")
    receipts = strings(checkpoint.get("verification_receipts"), "checkpoint receipts")
    for receipt in ("receipt:G1-C5-CLOSURE:recorded", f"registered-full-gate:{RUN}:passed", f"registered-full-gate:{POST_RUN}:passed", "independent-review:c5-portability-receipt:accepted", "independent-review:c5-proofreceipt-integration:accepted"):
        if receipt not in receipts: raise ValueError(f"missing closure receipt: {receipt}")
    if successor:
        for receipt in ("owner-route:c6-reconciliation-readiness:authorized", "independent-review:c5-c6-readiness-transition:accepted", "focused:c5-c6-successor-route-hostiles:passed", "transition:c5-verified-c6-readiness-activated:recorded"):
            if receipt not in receipts: raise ValueError(f"missing successor receipt: {receipt}")
    if c6_body_plan and "owner-authorization:c6-body-plan-structure-v1:released" not in receipts: raise ValueError("missing body-plan owner authorization")
    if c6_identity_readiness:
        for receipt in ("receipt:G1-C6-BODY-PLAN-STRUCTURE-V1:recorded", "owner-route:c6-organism-identity-readiness:authorized"):
            if receipt not in receipts: raise ValueError(f"missing identity-readiness receipt: {receipt}")
    if c6_identity_implementation:
        for receipt in ("receipt:G1-C6-BODY-PLAN-STRUCTURE-V1:recorded", "owner-route:c6-organism-identity-readiness:authorized", "owner-authorization:c6-organism-subject-identity-v1:released"):
            if receipt not in receipts: raise ValueError(f"missing identity-implementation receipt: {receipt}")
    criteria = checkpoint.get("exit_criteria")
    if type(criteria) is not list or any(type(x) is not dict for x in criteria): raise ValueError("checkpoint exit criteria are malformed")
    if not successor and any(x.get("status") != "verified" for x in criteria): raise ValueError("C5 exit criteria are not all verified")
    items = program.get("items")
    if type(items) is not list: raise ValueError("program items are malformed")
    c5, c6 = one(items,"C5"), one(items,"C6")
    expected_c5 = ("verified","complete","recorded") if successor else ("executing","active","recorded")
    if (c5.get("state"),c5.get("status"),c5.get("gate")) != expected_c5: raise ValueError("C5 closure state drifted")
    if strings(c5.get("depends_on"),"C5 dependencies") != ["C4"] or "G1_C5_CLOSURE_RESULT.md" not in strings(c5.get("sources"),"C5 sources"): raise ValueError("C5 closure binding drifted")
    expected_c6 = ("executing","active") if successor else ("proposed","gated")
    if (c6.get("state"),c6.get("status")) != expected_c6 or strings(c6.get("depends_on"),"C6 dependencies") != ["C4","C5"]: raise ValueError("C6 state or dependency drifted")
    if successor and c6.get("gate") != ("implementation" if (c6_body_plan or c6_identity_implementation) else "design"): raise ValueError("C6 successor gate drifted")
    active = [x.get("id") for x in items if type(x) is dict and x.get("state") == "executing" and x.get("status") == "active"]
    if active != (["C6"] if successor else ["C5"]): raise ValueError("sole active cursor drifted")
    local_expected = {"source_commit":SOURCE,"tracked_tree_manifest_sha256":TREE,"bounded_source_manifest_sha256":BOUNDED,"semantic_receipt_sha256":SEMANTIC,"independent_second_platform_execution":False,"promotion_authority":False,"c6_authority":False}
    for key,wanted in local_expected.items():
        if type(local.get(key)) is not type(wanted) or local.get(key) != wanted: raise ValueError(f"local receipt {key} drifted")
    external_expected = {"source_commit":SOURCE,"semantic_receipt_sha256":SEMANTIC,"request_sha256":REQUEST,"result_sha256":RESULT,"status":"independence_verified","hosted_runner":True}
    for key,wanted in external_expected.items():
        if type(external.get(key)) is not type(wanted) or external.get(key) != wanted: raise ValueError(f"independent receipt {key} drifted")
    if external.get("authority") != {"activate_c6":False,"evidence_only":True,"promotion_authority":False,"repository_mutation":False}: raise ValueError("independent authority boundary drifted")
    systems = registry.get("systems")
    if type(systems) is not list: raise ValueError("system registry is malformed")
    for ident in ("significance-system","streaming-scheduler"):
        system = one(systems,ident)
        if system.get("status") != "reference_proven" or "docs/canonical-system/G1_C5_CLOSURE_RESULT.md" not in strings(system.get("references"),f"{ident} references"): raise ValueError(f"{ident} closure status drifted")
    docs = [
        root/"docs/canonical-system/G1_C5_CLOSURE_RESULT.md", root/"docs/canonical-system/MASTER_CLOSURE_REGISTER.md",
        root/"docs/canonical-system/PROOF_MATRIX.md", root/"docs/canonical-system/UNRESOLVED_GAPS.md",
        root/"docs/project-atlas/ROADMAP.md",
    ]
    result_text = docs[0].read_text(encoding="utf-8-sig")
    for token in ("Status: **verified, complete and recorded.**","Android ARM64 is honestly classified compile-only","not Forge","C5 grants no C6 authority",SUCCESSOR):
        if token not in result_text: raise ValueError(f"closure result is missing exact boundary: {token}")
    text = "\n".join(p.read_text(encoding="utf-8-sig") for p in docs)
    for token in (SOURCE,TREE,BOUNDED,SUCCESSOR,SEMANTIC,REQUEST,RESULT,RUN,POST_RUN,"not Forge","C5 grants no C6 authority"):
        if token not in text: raise ValueError(f"closure records are missing exact token: {token}")

def main() -> None:
    parser=argparse.ArgumentParser(); parser.add_argument("--root",type=Path,required=True); args=parser.parse_args()
    verify(args.root.resolve()); print("G1 C5 closure result verified: immutable evidence and exact current successor route agree.")
if __name__ == "__main__": main()
