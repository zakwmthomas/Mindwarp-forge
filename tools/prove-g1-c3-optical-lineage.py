#!/usr/bin/env python3
"""Deterministic counterexample oracle for thin optical-lane lineage.

This models identity, ordering, exact-box propagation, explicit object-bundle
replay and typed termination only. It is not a production schema, composer,
optical kernel, cumulative-power calculation or endpoint proof.
"""
from copy import deepcopy
import hashlib
import json

BANDS = ("red", "green", "blue")
ZERO = "0" * 64
RECONSTRUCTION = "11" * 32
PROFILE = "22" * 32
DECLARED_SOURCE = "33" * 32


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":")).encode()


def digest(domain, value):
    return hashlib.sha256(domain.encode() + b"\0" + canonical(value)).hexdigest()


def object_id(kind, value):
    body = {key: item for key, item in value.items() if key != "id"}
    return digest("mindwarp.oracle.local." + kind + ".v1", body)


def add_object(bundle, kind, body):
    value = {"kind": kind, **body}
    value["id"] = object_id(kind, value)
    bundle.append(value)
    return value


def box(value, width=0):
    return [[str(value - width), str(value + width)] for _ in range(3)]


def derived_source(lane_id, ordinal, predecessor, role):
    return digest("mindwarp.oracle.derived-source.v1", {
        "lane_id": lane_id,
        "ordinal": ordinal,
        "predecessor": predecessor,
        "role": role,
    })


def build_lane(band, steps, lane_tag=0, terminal="outer_domain_exit"):
    assert band in BANDS and steps > 0
    bundle = []
    initial_point = box(1000 + lane_tag)
    initial_direction = box(10 + lane_tag)
    initial_body = {
        "reconstruction_id": RECONSTRUCTION,
        "profile_id": PROFILE,
        "state_source_id": DECLARED_SOURCE,
        "state_revision": 1,
        "current_cell": [0, lane_tag, 0],
        "point_q160": initial_point,
        "direction_q1_62": initial_direction,
    }
    initial_input = add_object(bundle, "cell_input", initial_body)
    lane_id = digest("mindwarp.optical-lane.v1", {
        "reconstruction_id": RECONSTRUCTION,
        "profile_id": PROFILE,
        "band": band,
        "initial_cell_step_input_id": initial_input["id"],
        "lane_source_id": DECLARED_SOURCE,
    })
    manifest_steps = []
    predecessor = None
    current_input = initial_input
    for ordinal in range(steps):
        if ordinal:
            expected_source = derived_source(lane_id, ordinal, predecessor, "cell_input")
            current_input = add_object(bundle, "cell_input", {
                "reconstruction_id": RECONSTRUCTION,
                "profile_id": PROFILE,
                "state_source_id": expected_source,
                "state_revision": ordinal + 1,
                "current_cell": [ordinal, lane_tag, 0],
                "point_q160": prior_hit,
                "direction_q1_62": prior_direction,
            })
        hit = box(2000 + ordinal * 10 + lane_tag, ordinal % 2)
        final = ordinal == steps - 1
        outcome = terminal if final else "known_neighbor"
        neighbor = None if outcome == "outer_domain_exit" else [ordinal + 1, lane_tag, 0]
        event = add_object(bundle, "cell_event", {
            "input_id": current_input["id"],
            "current_cell": current_input["current_cell"],
            "hit_point_q160": hit,
            "neighbor": neighbor,
            "outcome": outcome,
        })
        query = add_object(bundle, "bulk_query", {
            "profile_id": PROFILE,
            "band": band,
            "cell_input_id": current_input["id"],
            "cell_event_id": event["id"],
        })
        transfer = add_object(bundle, "bulk_transfer", {
            "query_id": query["id"],
            "cell_input_id": current_input["id"],
            "cell_event_id": event["id"],
            "band": band,
            "terminal": outcome,
        })
        interface_input = None
        interface_event = None
        selected_direction = current_input["direction_q1_62"]
        if not final and ordinal % 2 == 0:
            interface_input = add_object(bundle, "interface_input", {
                "reconstruction_id": RECONSTRUCTION,
                "incident_source_id": derived_source(lane_id, ordinal, predecessor, "interface_input"),
                "source_cell": current_input["current_cell"],
                "target_cell": neighbor,
                "incident_direction_q1_62": current_input["direction_q1_62"],
                "face_interaction_id": digest("face", [ordinal, lane_tag]),
            })
            directions = {
                candidate: box(100 + ordinal * 3 + index + lane_tag, ordinal % 2)
                for index, candidate in enumerate(BANDS)
            }
            interface_event = add_object(bundle, "interface_event", {
                "input_id": interface_input["id"],
                "outcomes": {candidate: "all_transmit" for candidate in BANDS},
                "transmitted_direction_q1_62": directions,
            })
            selected_direction = directions[band]
        step_body = {
            "lane_id": lane_id,
            "ordinal": ordinal,
            "predecessor_step_id": predecessor,
            "cell_input_id": current_input["id"],
            "cell_event_id": event["id"],
            "bulk_query_id": query["id"],
            "bulk_transfer_id": transfer["id"],
            "interface_input_id": interface_input and interface_input["id"],
            "interface_event_id": interface_event and interface_event["id"],
            "terminal": outcome,
        }
        step_id = digest("mindwarp.optical-lane-step.v1", step_body)
        manifest_steps.append({**step_body, "step_id": step_id})
        predecessor = step_id
        prior_hit = hit
        prior_direction = selected_direction
    bundle_receipt = make_bundle_receipt(bundle)
    manifest = {
        "schema_version": 1,
        "lane_id": lane_id,
        "reconstruction_id": RECONSTRUCTION,
        "profile_id": PROFILE,
        "band": band,
        "lane_source_id": DECLARED_SOURCE,
        "steps": manifest_steps,
        "bundle_receipt": bundle_receipt,
        "final_terminal": terminal,
    }
    manifest["transcript_id"] = digest("mindwarp.optical-lane-transcript.v1", manifest)
    return manifest, bundle


def make_bundle_receipt(bundle):
    entries = []
    for value in sorted(bundle, key=lambda item: item["id"]):
        encoded = canonical(value)
        entries.append({"id": value["id"], "sha256": hashlib.sha256(encoded).hexdigest(), "bytes": len(encoded)})
    return {
        "object_count": len(entries),
        "canonical_bytes": sum(entry["bytes"] for entry in entries),
        "entries_sha256": digest("mindwarp.optical-lineage-bundle.v1", entries),
    }


class Invalid(Exception):
    pass


def require(condition, message):
    if not condition:
        raise Invalid(message)


def validate(manifest, bundle):
    require(manifest["schema_version"] == 1, "schema")
    require(manifest["band"] in BANDS, "band")
    require(len(bundle) == len({value["id"] for value in bundle}), "duplicate object")
    objects = {}
    for value in bundle:
        require(value["id"] == object_id(value["kind"], value), "local object hash")
        objects[value["id"]] = value
    require(manifest["bundle_receipt"] == make_bundle_receipt(bundle), "bundle receipt")
    steps = manifest["steps"]
    require(steps, "empty transcript")
    initial = objects.get(steps[0]["cell_input_id"])
    require(initial and initial["kind"] == "cell_input", "initial input")
    expected_lane = digest("mindwarp.optical-lane.v1", {
        "reconstruction_id": manifest["reconstruction_id"],
        "profile_id": manifest["profile_id"],
        "band": manifest["band"],
        "initial_cell_step_input_id": initial["id"],
        "lane_source_id": manifest["lane_source_id"],
    })
    require(manifest["lane_id"] == expected_lane, "lane identity")
    used = set()
    predecessor = None
    prior_event = None
    prior_interface = None
    for ordinal, step in enumerate(steps):
        require(step["ordinal"] == ordinal, "ordinal")
        require(step["lane_id"] == expected_lane, "foreign lane")
        require(step["predecessor_step_id"] == predecessor, "predecessor")
        body = {key: value for key, value in step.items() if key != "step_id"}
        require(step["step_id"] == digest("mindwarp.optical-lane-step.v1", body), "step identity")
        local = []
        for field, kind in (("cell_input_id", "cell_input"), ("cell_event_id", "cell_event"),
                            ("bulk_query_id", "bulk_query"), ("bulk_transfer_id", "bulk_transfer")):
            value = objects.get(step[field])
            require(value and value["kind"] == kind, field)
            local.append(value)
            used.add(value["id"])
        cell_input, cell_event, query, transfer = local
        require(cell_input["reconstruction_id"] == manifest["reconstruction_id"], "reconstruction")
        require(cell_input["profile_id"] == manifest["profile_id"], "profile")
        require(query["profile_id"] == manifest["profile_id"] and query["band"] == manifest["band"], "bulk binding")
        require(query["cell_input_id"] == cell_input["id"] and query["cell_event_id"] == cell_event["id"], "bulk nesting")
        require(transfer["query_id"] == query["id"] and transfer["band"] == manifest["band"], "transfer binding")
        require(transfer["cell_input_id"] == cell_input["id"] and transfer["cell_event_id"] == cell_event["id"], "transfer nesting")
        require(cell_event["input_id"] == cell_input["id"] and cell_event["current_cell"] == cell_input["current_cell"], "cell replay")
        if ordinal == 0:
            require(cell_input["state_source_id"] == manifest["lane_source_id"] and cell_input["state_revision"] == 1, "initial source")
        else:
            require(cell_input["state_source_id"] == derived_source(expected_lane, ordinal, predecessor, "cell_input"), "derived source")
            require(cell_input["state_revision"] == ordinal + 1, "revision")
            require(cell_input["current_cell"] == prior_event["neighbor"], "cell adjacency")
            require(cell_input["point_q160"] == prior_event["hit_point_q160"], "hit propagation")
            expected_direction = prior_interface["transmitted_direction_q1_62"][manifest["band"]] if prior_interface else prior_input["direction_q1_62"]
            require(cell_input["direction_q1_62"] == expected_direction, "direction propagation")
        has_interface = step["interface_input_id"] is not None or step["interface_event_id"] is not None
        require((step["interface_input_id"] is None) == (step["interface_event_id"] is None), "partial interface")
        current_interface = None
        if has_interface:
            interface_input = objects.get(step["interface_input_id"])
            current_interface = objects.get(step["interface_event_id"])
            require(interface_input and interface_input["kind"] == "interface_input", "interface input")
            require(current_interface and current_interface["kind"] == "interface_event", "interface event")
            used.update((interface_input["id"], current_interface["id"]))
            require(cell_event["outcome"] == "known_neighbor", "interface terminal")
            require(interface_input["incident_source_id"] == derived_source(expected_lane, ordinal, predecessor, "interface_input"), "interface source")
            require(interface_input["source_cell"] == cell_input["current_cell"] and interface_input["target_cell"] == cell_event["neighbor"], "interface cells")
            require(interface_input["incident_direction_q1_62"] == cell_input["direction_q1_62"], "incident direction")
            require(current_interface["input_id"] == interface_input["id"], "interface replay")
            require(current_interface["outcomes"][manifest["band"]] == "all_transmit", "interface branch")
        if ordinal + 1 < len(steps):
            require(cell_event["outcome"] == "known_neighbor", "early terminal")
        else:
            require(cell_event["outcome"] == manifest["final_terminal"], "final terminal")
        require(transfer["terminal"] == cell_event["outcome"] == step["terminal"], "terminal binding")
        predecessor = step["step_id"]
        prior_event, prior_interface, prior_input = cell_event, current_interface, cell_input
    require(used == set(objects), "unused or missing object")
    identity_body = {key: value for key, value in manifest.items() if key != "transcript_id"}
    require(manifest["transcript_id"] == digest("mindwarp.optical-lane-transcript.v1", identity_body), "transcript identity")
    return True


def must_reject(name, manifest, bundle, mutate):
    candidate_manifest, candidate_bundle = deepcopy(manifest), deepcopy(bundle)
    mutate(candidate_manifest, candidate_bundle)
    try:
        validate(candidate_manifest, candidate_bundle)
    except (Invalid, KeyError, TypeError):
        return name
    raise AssertionError("counterexample admitted: " + name)


def reseal(manifest, bundle):
    """Recompute all attacker-controlled hashes after a semantic mutation."""
    replacements = {}
    reference_fields = ("input_id", "cell_input_id", "cell_event_id", "query_id")
    for value in bundle:
        old = value["id"]
        for field in reference_fields:
            if field in value and value[field] in replacements:
                value[field] = replacements[value[field]]
        value["id"] = object_id(value["kind"], value)
        replacements[old] = value["id"]
    predecessor = None
    for step in manifest["steps"]:
        for field in ("cell_input_id", "cell_event_id", "bulk_query_id", "bulk_transfer_id", "interface_input_id", "interface_event_id"):
            if step[field] in replacements:
                step[field] = replacements[step[field]]
        step["predecessor_step_id"] = predecessor
        body = {key: value for key, value in step.items() if key != "step_id"}
        step["step_id"] = digest("mindwarp.optical-lane-step.v1", body)
        predecessor = step["step_id"]
    manifest["bundle_receipt"] = make_bundle_receipt(bundle)
    identity_body = {key: value for key, value in manifest.items() if key != "transcript_id"}
    manifest["transcript_id"] = digest("mindwarp.optical-lane-transcript.v1", identity_body)


def mutate_and_reseal(manifest, bundle, mutation):
    mutation(manifest, bundle)
    reseal(manifest, bundle)


def hostile_cases():
    manifest, bundle = build_lane("red", 4)
    validate(manifest, bundle)
    cases = []
    cases.append(must_reject("gap_ordinal", manifest, bundle, lambda m, b: m["steps"][2].__setitem__("ordinal", 7)))
    cases.append(must_reject("duplicate_step", manifest, bundle, lambda m, b: m["steps"].insert(2, deepcopy(m["steps"][1]))))
    cases.append(must_reject("cycle_predecessor", manifest, bundle, lambda m, b: m["steps"][1].__setitem__("predecessor_step_id", m["steps"][3]["step_id"])))
    cases.append(must_reject("foreign_band", manifest, bundle, lambda m, b: m.__setitem__("band", "green")))
    cases.append(must_reject("foreign_profile", manifest, bundle, lambda m, b: m.__setitem__("profile_id", "44" * 32)))
    cases.append(must_reject("foreign_reconstruction", manifest, bundle, lambda m, b: m.__setitem__("reconstruction_id", "55" * 32)))
    cases.append(must_reject("step_id_bit", manifest, bundle, lambda m, b: m["steps"][0].__setitem__("step_id", "0" + m["steps"][0]["step_id"][1:])))
    cases.append(must_reject("bundle_receipt", manifest, bundle, lambda m, b: m["bundle_receipt"].__setitem__("canonical_bytes", 0)))
    cases.append(must_reject("local_object_mutation", manifest, bundle, lambda m, b: b[0].__setitem__("state_revision", 9)))
    cases.append(must_reject("duplicate_object", manifest, bundle, lambda m, b: b.append(deepcopy(b[0]))))
    cases.append(must_reject("unused_object", manifest, bundle, lambda m, b: b.append({**deepcopy(b[0]), "id": "66" * 32})))
    cases.append(must_reject("hit_narrowing", manifest, bundle, lambda m, b: next(x for x in b if x["id"] == m["steps"][1]["cell_input_id"])["point_q160"][0].__setitem__(0, "999")))
    cases.append(must_reject("direction_recoding", manifest, bundle, lambda m, b: next(x for x in b if x["id"] == m["steps"][1]["cell_input_id"])["direction_q1_62"][0].__setitem__(0, "+100")))
    cases.append(must_reject("caller_source_alias", manifest, bundle, lambda m, b: next(x for x in b if x["id"] == m["steps"][1]["cell_input_id"]).__setitem__("state_source_id", "77" * 32)))
    cases.append(must_reject("partial_interface", manifest, bundle, lambda m, b: m["steps"][0].__setitem__("interface_event_id", None)))
    cases.append(must_reject("wrong_interface_cells", manifest, bundle, lambda m, b: next(x for x in b if x["kind"] == "interface_input").__setitem__("target_cell", [9, 9, 9])))
    green, green_bundle = build_lane("green", 4)
    cases.append(must_reject("cross_lane_event", manifest, bundle, lambda m, b: (b.extend(deepcopy(green_bundle)), m["steps"][0].__setitem__("interface_event_id", green["steps"][0]["interface_event_id"]))))
    cases.append(must_reject("early_terminal", manifest, bundle, lambda m, b: next(x for x in b if x["id"] == m["steps"][1]["cell_event_id"]).__setitem__("outcome", "outer_domain_exit")))
    cases.append(must_reject("wrong_final_terminal", manifest, bundle, lambda m, b: m.__setitem__("final_terminal", "unavailable_neighbor")))
    cases.append(must_reject("transcript_id", manifest, bundle, lambda m, b: m.__setitem__("transcript_id", ZERO)))
    short, short_bundle = build_lane("red", 2)
    cases.append(must_reject("resealed_hit_narrowing", short, short_bundle, lambda m, b: mutate_and_reseal(m, b, lambda mm, bb: next(x for x in bb if x["id"] == mm["steps"][1]["cell_input_id"])["point_q160"][0].__setitem__(0, "999"))))
    cases.append(must_reject("resealed_source_alias", short, short_bundle, lambda m, b: mutate_and_reseal(m, b, lambda mm, bb: next(x for x in bb if x["id"] == mm["steps"][1]["cell_input_id"]).__setitem__("state_source_id", "88" * 32))))
    cases.append(must_reject("resealed_wrong_interface_cells", short, short_bundle, lambda m, b: mutate_and_reseal(m, b, lambda mm, bb: next(x for x in bb if x["kind"] == "interface_input").__setitem__("target_cell", [9, 9, 9]))))
    cases.append(must_reject("resealed_incident_direction", short, short_bundle, lambda m, b: mutate_and_reseal(m, b, lambda mm, bb: next(x for x in bb if x["kind"] == "interface_input").__setitem__("incident_direction_q1_62", box(777)))))
    cases.append(must_reject("resealed_cross_band_direction", short, short_bundle, lambda m, b: mutate_and_reseal(m, b, lambda mm, bb: next(x for x in bb if x["id"] == mm["steps"][1]["cell_input_id"]).__setitem__("direction_q1_62", next(x for x in bb if x["kind"] == "interface_event")["transmitted_direction_q1_62"]["green"]))))
    cases.append(must_reject("resealed_early_terminal", short, short_bundle, lambda m, b: mutate_and_reseal(m, b, lambda mm, bb: next(x for x in bb if x["id"] == mm["steps"][0]["cell_event_id"]).__setitem__("outcome", "outer_domain_exit"))))
    return cases


def terminal_cases():
    terminals = ("outer_domain_exit", "unavailable_neighbor", "unavailable_current", "ambiguous_next_face", "no_forward_progress", "all_tir", "ambiguous_interface_branch", "nonconvergent_interface", "unsupported_interface_model", "work_exhausted")
    for terminal in terminals:
        manifest, bundle = build_lane("blue", 1, terminal=terminal)
        validate(manifest, bundle)
    return list(terminals)


def portfolio(lanes, steps):
    manifests, bundle = [], []
    for index in range(lanes):
        manifest, objects = build_lane(BANDS[index], steps, lane_tag=index)
        validate(manifest, objects)
        manifests.append(manifest)
        bundle.extend(objects)
    manifest_bytes = len(canonical(manifests))
    bundle_bytes = sum(len(canonical(value)) for value in bundle)
    maximum_object_bytes = max(len(canonical(value)) for value in bundle)
    conservative_peak = manifest_bytes + bundle_bytes + maximum_object_bytes
    return {
        "lanes": lanes,
        "steps_per_lane": steps,
        "total_steps": lanes * steps,
        "manifest_bytes": manifest_bytes,
        "bundle_canonical_bytes": bundle_bytes,
        "maximum_object_bytes": maximum_object_bytes,
        "conservative_validation_bytes": conservative_peak,
        "manifest_under_1_mib": manifest_bytes <= 1 << 20,
        "bundle_under_48_mib": bundle_bytes <= 48 << 20,
        "conservative_validation_under_64_mib": conservative_peak <= 64 << 20,
    }


def main():
    hostile = hostile_cases()
    terminals = terminal_cases()
    portfolios = {
        "one_step": portfolio(1, 1),
        "three_lanes_one_step": portfolio(3, 1),
        "one_lane_64_steps": portfolio(1, 64),
        "three_lanes_64_steps": portfolio(3, 64),
    }
    receipt = {
        "schema_version": 1,
        "oracle_kind": "thin_per_band_manifest_with_explicit_replayed_object_bundle",
        "candidate": "accepted_for_code_facing_readiness_audit_only",
        "hostile_rejections": hostile,
        "hostile_rejection_count": len(hostile),
        "typed_terminals": terminals,
        "typed_terminal_count": len(terminals),
        "portfolios": portfolios,
        "identity_rules": {
            "ambient_lookup": "forbidden",
            "unused_or_duplicate_objects": "rejected",
            "derived_successor_sources": "domain_separated_from_lane_ordinal_predecessor_and_role",
            "point_propagation": "exact_q160_endpoint_strings",
            "direction_propagation": "matching_band_exact_q1_62_endpoint_strings",
        },
        "nonclaims": ["no production schema", "no cumulative power", "no receiver arrival", "no visibility", "no authority effect"],
    }
    receipt_hash = hashlib.sha256(canonical(receipt)).hexdigest()
    print(json.dumps({**receipt, "receipt_sha256": receipt_hash}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
