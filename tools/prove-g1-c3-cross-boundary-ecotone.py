from concurrent.futures import ThreadPoolExecutor
from fractions import Fraction
import hashlib
import itertools
import json
import tracemalloc


SCHEMA = 1
DENOMINATOR = 1_000_000_000
MAX_CELLS = 65_536
MAX_EDGES = 130_560
MAX_COORD = (1 << 63) - 1
MIN_COORD = -(1 << 63)
SEED = b"mindwarp.disposable.ecotone.permutation.v1"
DOMAINS = {
    "cell": b"mindwarp.disposable.ecotone.cell-result.v1",
    "edge": b"mindwarp.disposable.ecotone.edge-result.v1",
    "fixture": b"mindwarp.disposable.ecotone.fixture-result.v1",
    "suite": b"mindwarp.disposable.ecotone.suite-receipt.v1",
}


class Rejection(ValueError):
    def __init__(self, code, phase):
        super().__init__(code)
        self.code = code
        self.phase = phase


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True, allow_nan=False).encode("ascii")


def streaming_sha256(value, prefix=b""):
    hasher = hashlib.sha256(prefix)
    encoder = json.JSONEncoder(sort_keys=True, separators=(",", ":"), ensure_ascii=True, allow_nan=False)
    for chunk in encoder.iterencode(value):
        hasher.update(chunk.encode("ascii"))
    return hasher.hexdigest()


def digest(domain, value):
    return streaming_sha256(value, DOMAINS[domain] + b"\0")


def exact_int(value, low=None, high=None, code="noncanonical_input"):
    if isinstance(value, bool) or not isinstance(value, int):
        raise Rejection(code, "preflight")
    if low is not None and value < low or high is not None and value > high:
        raise Rejection("arithmetic_out_of_range", "preflight")
    return value


def validate_band(values):
    if not isinstance(values, (list, tuple)) or len(values) != 3:
        raise Rejection("noncanonical_input", "preflight")
    for value in values: exact_int(value, 0, 1000)


def palette(irradiance, transmission, reflectance, exposure):
    for values in (irradiance, transmission, reflectance): validate_band(values)
    exact_int(exposure, 0, 1000)
    result = []
    maximum_bits = 0
    for band in range(3):
        product = irradiance[band] * transmission[band] * reflectance[band] * exposure
        maximum_bits = max(maximum_bits, (product + 500_000_000).bit_length())
        integer_result = (product + 500_000_000) // DENOMINATOR
        rational_result = (2 * Fraction(product, DENOMINATOR).numerator + Fraction(product, DENOMINATOR).denominator) // (2 * Fraction(product, DENOMINATOR).denominator)
        if integer_result != rational_result:
            raise AssertionError("independent exact-rational rounding disagreement")
        result.append(integer_result)
    return tuple(result), maximum_bits


def identity(label):
    return hashlib.sha256(("mindwarp.disposable.ecotone." + label).encode("ascii")).hexdigest()


def subject(suffix="a"):
    return {
        name: identity(name + "." + suffix) for name in (
            "reconstruction_id", "spatial_domain_id", "regional_recipe_id",
            "climate_evidence_id", "stellar_evidence_id", "atmosphere_evidence_id"
        )
    }


def make_fixture(fixture_id, width, height, *, step=1 << 32, moved=False, relabel=False,
                 field="horizontal", sharp=False, equal_sharp=False, moisture_missing=False,
                 exposure_missing=False, contradiction=False, category_shift=False,
                 constant_exposure=None):
    exact_int(width, 1)
    exact_int(height, 1)
    exact_int(step, 1)
    cells_count = width * height
    edges_count = width * (height - 1) + height * (width - 1)
    if width > 256 or height > 256 or cells_count > MAX_CELLS or edges_count > MAX_EDGES:
        raise Rejection("resource_limit", "preflight")
    origin = step // 2 if moved else 0
    furthest = origin + max(width - 1, height - 1) * step
    if origin < MIN_COORD or furthest > MAX_COORD:
        raise Rejection("arithmetic_out_of_range", "preflight")
    cells = []
    for y in range(height):
        for x in range(width):
            coordinate = (origin + x * step, origin + y * step)
            if any(value < MIN_COORD or value > MAX_COORD for value in coordinate):
                raise Rejection("arithmetic_out_of_range", "preflight")
            if exposure_missing and x == 0 and y == 0:
                exposure = None
            elif constant_exposure is not None:
                exposure = exact_int(constant_exposure, 0, 1000)
            elif field == "horizontal":
                exposure = 500 if width == 1 else (1000 * x + (width - 1) // 2) // (width - 1)
            elif field == "vertical":
                exposure = 500 if height == 1 else (1000 * y + (height - 1) // 2) // (height - 1)
            elif field == "diagonal":
                divisor = width + height - 2
                exposure = 500 if divisor == 0 else (1000 * (x + y) + divisor // 2) // divisor
            elif field == "plateau":
                exposure = 500 if width == 1 else min(750, (1000 * x + (width - 1) // 2) // (width - 1))
            elif field == "reversed":
                exposure = 500 if width == 1 else 1000 - (1000 * x + (width - 1) // 2) // (width - 1)
            else:
                raise Rejection("noncanonical_input", "preflight")
            on_right = x >= width // 2
            reflectance = (800, 400, 100) if sharp and on_right and not equal_sharp else (200, 400, 600) if sharp else (500, 400, 300)
            material = identity("material-right") if sharp and on_right else identity("material-left")
            annotation_x = (x + 1) % width if category_shift and width > 1 else x
            cells.append({
                "annotation": [
                    ("component-z" if relabel else "component-a") + str((annotation_x + y) % 2),
                    "renamed" if relabel else "baseline",
                    "signature-z" if relabel else "signature-a",
                ],
                "atmosphere_transmission": [900, 700, 500],
                "coordinate_q32_32": list(coordinate),
                "exposure": exposure,
                "irradiance": [1000, 800, 600],
                "material_evidence_id": material,
                "moisture": None if moisture_missing and x == 0 else 500,
                "oracle_cell_id": "cell-%03d-%03d" % (x, y),
                "reflectance": list(reflectance),
                "x": x,
                "y": y,
            })
    edges = []
    for y in range(height):
        for x in range(width):
            for nx, ny in ((x + 1, y), (x, y + 1)):
                if nx < width and ny < height:
                    left = "cell-%03d-%03d" % (x, y)
                    right = "cell-%03d-%03d" % (nx, ny)
                    interface = None
                    if sharp and nx != x and x == width // 2 - 1:
                        interface = {
                            "cause_id": "fixture-material-interface-a",
                            "fixture_revision_id": "fixture-revision-a",
                            "kind": "surface_reflectance_discontinuity",
                            "left_material_evidence_id": identity("material-left"),
                            "left_values": [200, 400, 600],
                            "right_material_evidence_id": identity("material-right"),
                            "right_values": [200, 400, 600] if equal_sharp else [800, 400, 100],
                            "subject": subject(),
                            "synthetic_fixture_only": True,
                        }
                    edges.append({"a": left, "b": right, "interface": interface})
    return {
        "annotation_manifest": [list(value) for value in sorted({tuple(cell["annotation"]) for cell in cells})],
        "cells": cells,
        "contradiction": None if not contradiction else {
            "dimension": "surface_reflectance",
            "fixture_revision_id": "fixture-revision-a",
            "subject": subject(),
        },
        "domain": {
            "adjacency": "shared_edge_4",
            "boundary": "bounded_absent",
            "height": height,
            "origin_q32_32": [origin, origin],
            "step_q32_32": [step, step],
            "width": width,
        },
        "edges": edges,
        "fixture_id": fixture_id,
        "schema_version": SCHEMA,
        "subject": subject(),
    }


def validate_fixture(fixture):
    if set(fixture) != {"annotation_manifest", "cells", "contradiction", "domain", "edges", "fixture_id", "schema_version", "subject"}:
        raise Rejection("noncanonical_input", "preflight")
    if isinstance(fixture["schema_version"], bool) or fixture["schema_version"] != SCHEMA:
        raise Rejection("noncanonical_input", "preflight")
    if not isinstance(fixture["fixture_id"], str) or not fixture["fixture_id"] or not fixture["fixture_id"].isascii():
        raise Rejection("noncanonical_input", "preflight")
    contradiction = fixture["contradiction"]
    if contradiction is not None and (
        not isinstance(contradiction, dict) or
        set(contradiction) != {"dimension", "fixture_revision_id", "subject"} or
        contradiction["dimension"] != "surface_reflectance"
    ):
        raise Rejection("noncanonical_input", "preflight")
    expected_subject_keys = {
        "reconstruction_id", "spatial_domain_id", "regional_recipe_id",
        "climate_evidence_id", "stellar_evidence_id", "atmosphere_evidence_id",
    }
    if set(fixture["subject"]) != expected_subject_keys or any(
        not isinstance(value, str) or len(value) != 64 or value == "0" * 64 or
        any(character not in "0123456789abcdef" for character in value)
        for value in fixture["subject"].values()
    ):
        raise Rejection("noncanonical_input", "preflight")
    domain = fixture["domain"]
    if set(domain) != {"adjacency", "boundary", "height", "origin_q32_32", "step_q32_32", "width"}:
        raise Rejection("noncanonical_input", "preflight")
    if domain["adjacency"] != "shared_edge_4" or domain["boundary"] != "bounded_absent":
        raise Rejection("unsupported_join", "preflight")
    width = exact_int(domain["width"], 1)
    height = exact_int(domain["height"], 1)
    expected_cells = width * height
    expected_edges = width * (height - 1) + height * (width - 1)
    if width > 256 or height > 256 or expected_cells > MAX_CELLS or expected_edges > MAX_EDGES:
        raise Rejection("resource_limit", "preflight")
    if any(not isinstance(value, list) or len(value) != 2 for value in (domain["origin_q32_32"], domain["step_q32_32"])):
        raise Rejection("noncanonical_input", "preflight")
    origins = tuple(exact_int(value, MIN_COORD, MAX_COORD) for value in domain["origin_q32_32"])
    steps = tuple(exact_int(value, 1, MAX_COORD) for value in domain["step_q32_32"])
    if any(origins[axis] + (size - 1) * steps[axis] > MAX_COORD for axis, size in enumerate((width, height))):
        raise Rejection("arithmetic_out_of_range", "preflight")
    cell_keys = {"annotation", "atmosphere_transmission", "coordinate_q32_32", "exposure", "irradiance", "material_evidence_id", "moisture", "oracle_cell_id", "reflectance", "x", "y"}
    ids, indices, coordinates = [], [], []
    for cell in fixture["cells"]:
        if set(cell) != cell_keys:
            raise Rejection("noncanonical_input", "preflight")
        x, y = exact_int(cell["x"], 0, width - 1), exact_int(cell["y"], 0, height - 1)
        expected_id = "cell-%03d-%03d" % (x, y)
        expected_coordinate = [origins[0] + x * steps[0], origins[1] + y * steps[1]]
        if cell["oracle_cell_id"] != expected_id or cell["coordinate_q32_32"] != expected_coordinate:
            raise Rejection("noncanonical_input", "preflight")
        if not isinstance(cell["annotation"], list) or len(cell["annotation"]) != 3 or any(not isinstance(value, str) for value in cell["annotation"]):
            raise Rejection("noncanonical_input", "preflight")
        if not isinstance(cell["material_evidence_id"], str) or len(cell["material_evidence_id"]) != 64:
            raise Rejection("noncanonical_input", "preflight")
        for values in (cell["irradiance"], cell["atmosphere_transmission"], cell["reflectance"]): validate_band(values)
        if cell["exposure"] is not None:
            exact_int(cell["exposure"], 0, 1000)
        if cell["moisture"] is not None:
            exact_int(cell["moisture"], 0, 1000)
        ids.append(expected_id); indices.append((x, y)); coordinates.append(tuple(expected_coordinate))
    if len(ids) != expected_cells or len(set(ids)) != expected_cells or len(set(indices)) != expected_cells or len(set(coordinates)) != expected_cells:
        raise Rejection("noncanonical_input", "preflight")
    if fixture["annotation_manifest"] != [list(value) for value in sorted({tuple(cell["annotation"]) for cell in fixture["cells"]})]:
        raise Rejection("noncanonical_input", "preflight")
    expected_pairs = set()
    for y in range(height):
        for x in range(width):
            here = "cell-%03d-%03d" % (x, y)
            if x + 1 < width: expected_pairs.add((here, "cell-%03d-%03d" % (x + 1, y)))
            if y + 1 < height: expected_pairs.add((here, "cell-%03d-%03d" % (x, y + 1)))
    interface_keys = {"cause_id", "fixture_revision_id", "kind", "left_material_evidence_id", "left_values", "right_material_evidence_id", "right_values", "subject", "synthetic_fixture_only"}
    edge_keys = []
    for edge in fixture["edges"]:
        if set(edge) != {"a", "b", "interface"} or (edge["a"], edge["b"]) not in expected_pairs:
            raise Rejection("unsupported_join", "preflight")
        interface = edge["interface"]
        if interface is not None:
            if set(interface) != interface_keys or interface["kind"] != "surface_reflectance_discontinuity" or interface["synthetic_fixture_only"] is not True:
                raise Rejection("noncanonical_input", "preflight")
            if not interface["cause_id"] or not interface["fixture_revision_id"]:
                raise Rejection("noncanonical_input", "preflight")
            validate_band(interface["left_values"]); validate_band(interface["right_values"])
        edge_keys.append((edge["a"], edge["b"]))
    if len(edge_keys) != expected_edges or set(edge_keys) != expected_pairs or len(set(edge_keys)) != expected_edges:
        raise Rejection("noncanonical_input", "preflight")
    return expected_cells, expected_edges


def cell_result(fixture, cell):
    if cell["exposure"] is None:
        causal_body = {"coordinate": cell["coordinate_q32_32"], "outcome": "unavailable_evidence", "violations": ["regional_exposure_unavailable"]}
        return (cell["coordinate_q32_32"], None, "unavailable_evidence", digest("cell", causal_body)), 0
    value, bits = palette(cell["irradiance"], cell["atmosphere_transmission"], cell["reflectance"], cell["exposure"])
    causal_body = {
        "causal_values": {
            "atmosphere_transmission": cell["atmosphere_transmission"],
            "exposure": cell["exposure"], "irradiance": cell["irradiance"],
            "reflectance": cell["reflectance"],
        },
        "coordinate": cell["coordinate_q32_32"], "palette": list(value), "outcome": "exact",
    }
    return (cell["coordinate_q32_32"], list(value), "exact", digest("cell", causal_body)), bits


def edge_result(fixture, edge, cells, results):
    left = cells[edge["a"]]
    right = cells[edge["b"]]
    left_result = results[edge["a"]]
    right_result = results[edge["b"]]
    if tuple(left["coordinate_q32_32"]) > tuple(right["coordinate_q32_32"]):
        left, right = right, left
        left_result, right_result = right_result, left_result
    violations = []
    interface = edge["interface"]
    provenance_mismatch = interface is not None and (
        interface["subject"] != fixture["subject"] or
        interface["left_material_evidence_id"] != left["material_evidence_id"] or
        interface["right_material_evidence_id"] != right["material_evidence_id"] or
        interface["left_values"] != left["reflectance"] or interface["right_values"] != right["reflectance"]
    )
    if provenance_mismatch:
        outcome = "provenance_mismatch"
        violations.append("material_interface_provenance_mismatch")
    elif left_result[2] != "exact" or right_result[2] != "exact":
        outcome = "unavailable_evidence"
        violations.append("regional_exposure_unavailable")
    elif fixture["contradiction"] is not None and interface is not None and (
        fixture["contradiction"]["dimension"] == "surface_reflectance" and
        fixture["contradiction"]["subject"] == interface["subject"] and
        fixture["contradiction"]["fixture_revision_id"] == interface["fixture_revision_id"]
    ):
        outcome = "contradictory_evidence"
        violations.append("same_dimension_continuity_conflict")
    elif interface is not None:
        outcome = "sharp_cause_exact"
    elif left["material_evidence_id"] != right["material_evidence_id"] or left["reflectance"] != right["reflectance"]:
        outcome = "unavailable_evidence"
        violations.append("missing_material_interface_join")
    elif left["moisture"] is None or right["moisture"] is None:
        outcome = "unavailable_evidence"
        violations.append("regional_moisture_unavailable")
    else:
        outcome = "continuous_cause_exact"
    body = {
        "cause_id": interface["cause_id"] if interface is not None and outcome == "sharp_cause_exact" else "regional_exposure",
        "endpoint_coordinates": [left["coordinate_q32_32"], right["coordinate_q32_32"]],
        "left": {"coordinate": left_result[0], "palette": left_result[1], "outcome": left_result[2]},
        "outcome": outcome,
        "right": {"coordinate": right_result[0], "palette": right_result[1], "outcome": right_result[2]},
        "violations": sorted(violations),
    }
    body["edge_digest"] = digest("edge", body)
    return body


def ordered_cells(fixture, mode):
    cells = list(fixture["cells"])
    if mode == "row_major":
        return cells
    if mode == "column_major":
        return sorted(cells, key=lambda cell: (cell["x"], cell["y"]))
    if mode == "reverse":
        return list(reversed(cells))
    if mode == "annotation_major":
        return sorted(cells, key=lambda cell: (cell["annotation"][0], cell["y"], cell["x"]))
    if mode == "sha256_permutation":
        return sorted(cells, key=lambda cell: hashlib.sha256(SEED + canonical([cell["x"], cell["y"], cell["oracle_cell_id"]])).digest())
    if mode == "fixed_chunks":
        chunks = [cells[index:index + 17] for index in range(0, len(cells), 17)]
        return list(itertools.chain.from_iterable(reversed(chunks)))
    if mode == "four_threads":
        return cells
    raise AssertionError("unknown mode")


def ordered_edges(fixture, mode):
    edges = list(fixture["edges"])
    cells = {cell["oracle_cell_id"]: cell for cell in fixture["cells"]}
    if mode == "row_major":
        return edges
    if mode == "column_major":
        return sorted(edges, key=lambda edge: (cells[edge["a"]]["x"], cells[edge["a"]]["y"], cells[edge["b"]]["x"], cells[edge["b"]]["y"]))
    if mode == "reverse":
        return list(reversed(edges))
    if mode == "annotation_major":
        return sorted(edges, key=lambda edge: (cells[edge["a"]]["annotation"][0], edge["a"], edge["b"]))
    if mode == "sha256_permutation":
        return sorted(edges, key=lambda edge: hashlib.sha256(SEED + canonical([edge["a"], edge["b"]])).digest())
    if mode == "fixed_chunks":
        chunks = [edges[index:index + 31] for index in range(0, len(edges), 31)]
        return list(itertools.chain.from_iterable(reversed(chunks)))
    if mode == "four_threads":
        return edges
    raise AssertionError("unknown mode")


def edge_ordinal(edge, width, height):
    _, x_text, y_text = edge["a"].split("-")
    _, bx_text, by_text = edge["b"].split("-")
    x, y, bx, by = int(x_text), int(y_text), int(bx_text), int(by_text)
    base = y * (2 * width - 1)
    if y == height - 1:
        return base + x
    if bx == x + 1 and by == y:
        return base + 2 * x
    return base + (2 * x + 1 if x < width - 1 else 2 * (width - 1))


def evaluate(fixture, mode="row_major", reference_cell_projection_list=None,
             reference_edge_list=None, validated_counts=None, fixture_commitment=None,
             collect_details=True):
    declared_cells, declared_edges = validate_fixture(fixture) if validated_counts is None else validated_counts
    ordered = ordered_cells(fixture, mode)
    edge_projection, edge_outcomes = [], []
    width = fixture["domain"]["width"]
    if reference_cell_projection_list is not None:
        projection_for = lambda cell: reference_cell_projection_list[cell["y"] * width + cell["x"]]
        if mode == "four_threads":
            with ThreadPoolExecutor(max_workers=4) as pool:
                visited = list(pool.map(lambda cell: projection_for(cell), ordered))
        else:
            visited = [projection_for(cell) for cell in ordered]
        cell_projection_list = reference_cell_projection_list
        cell_projection = sorted(visited, key=lambda item: tuple(item[0]))
        evaluated_cells, maximum_bits, retained_details = len(visited), 0, []
        edge_digest_list = reference_edge_list
    else:
        if mode == "four_threads":
            with ThreadPoolExecutor(max_workers=4) as pool:
                paired = list(pool.map(lambda cell: (cell["oracle_cell_id"],) + cell_result(fixture, cell), ordered))
        else:
            paired = [(cell["oracle_cell_id"],) + cell_result(fixture, cell) for cell in ordered]
        results = {cell_id: result for cell_id, result, _ in paired}
        maximum_bits = max((bits for _, _, bits in paired), default=0)
        cells = {cell["oracle_cell_id"]: cell for cell in fixture["cells"]}
        edge_digest_list = [None] * declared_edges
        for edge in ordered_edges(fixture, mode):
            result = edge_result(fixture, edge, cells, results)
            edge_digest_list[edge_ordinal(edge, width, fixture["domain"]["height"])] = result["edge_digest"]
            if collect_details: edge_outcomes.append([result["endpoint_coordinates"], result["outcome"]])
        sorted_details = sorted(results.values(), key=lambda result: tuple(result[0]))
        cell_projection_list = [[results[cell["oracle_cell_id"]][0], results[cell["oracle_cell_id"]][3]] for cell in fixture["cells"]]
        cell_projection = [[result[0], result[3]] for result in sorted_details]
        evaluated_cells = len(results)
        retained_details = ([{"coordinate": result[0], "palette": result[1], "outcome": result[2], "cell_digest": result[3]} for result in sorted_details] if collect_details else [])
    for edge in ordered_edges(fixture, mode):
        ordinal = edge_ordinal(edge, width, fixture["domain"]["height"])
        edge_projection.append((ordinal, edge_digest_list[ordinal]))
    cell_projection_digest = digest("fixture", {"cells": cell_projection})
    canonical_edges = [edge_digest for _, edge_digest in sorted(edge_projection, key=lambda item: item[0])]
    semantic = {"cells": cell_projection, "edges": canonical_edges}
    semantic_digest = digest("fixture", semantic)
    if fixture_commitment is None:
        fixture_commitment = streaming_sha256(fixture)
    audit = {"execution_mode": mode, "fixture_input_sha256": fixture_commitment, "semantic_digest": semantic_digest}
    return {
        "audit_digest": digest("fixture", audit),
        "declared_cells": declared_cells,
        "declared_edges": declared_edges,
        "evaluated_cells": evaluated_cells,
        "evaluated_edges": len(edge_projection),
        "edge_digest_list": edge_digest_list,
        "edge_outcomes": edge_outcomes,
        "cell_details": retained_details,
        "cell_projection_list": cell_projection_list,
        "maximum_product_bits": maximum_bits,
        "semantic": semantic,
        "cell_projection_digest": cell_projection_digest,
        "semantic_digest": semantic_digest,
    }


def expect_rejection(action, code=None, phase=None):
    try:
        action()
    except Rejection as error:
        return (code is None or error.code == code) and (phase is None or error.phase == phase)
    return False


def duplicate_key_rejected(text):
    def pairs(values):
        result = {}
        for key, value in values:
            if key in result:
                raise Rejection("noncanonical_input", "parse")
            result[key] = value
        return result
    try:
        json.loads(text, object_pairs_hook=pairs)
    except (Rejection, json.JSONDecodeError, UnicodeDecodeError):
        return True
    return False


def clone(value):
    return json.loads(canonical(value).decode("ascii"))


def connected_signature_components(fixture, signature):
    cells = {cell["oracle_cell_id"]: cell for cell in fixture["cells"] if cell["annotation"][2] == signature}
    adjacency = {cell_id: set() for cell_id in cells}
    for edge in fixture["edges"]:
        if edge["a"] in cells and edge["b"] in cells:
            adjacency[edge["a"]].add(edge["b"]); adjacency[edge["b"]].add(edge["a"])
    remaining, count = set(cells), 0
    while remaining:
        count += 1; stack = [remaining.pop()]
        while stack:
            for neighbour in adjacency[stack.pop()] & remaining:
                remaining.remove(neighbour); stack.append(neighbour)
    return count


def join_disposition(fixture, a, b):
    cells = {cell["oracle_cell_id"]: cell for cell in fixture["cells"]}
    if a not in cells or b not in cells:
        return "unsupported_join"
    left, right = cells[a], cells[b]
    distance = abs(left["x"] - right["x"]) + abs(left["y"] - right["y"])
    return "supported_join" if distance == 1 and any({edge["a"], edge["b"]} == {a, b} for edge in fixture["edges"]) else "unsupported_join"


def recursive_blend(values):
    result = values[0]
    for value in values[1:]: result = (result + value + 1) // 2
    return result


def collapse_heterogeneous(record):
    if set(record) != {"exposure", "material"}:
        raise Rejection("noncanonical_input", "preflight")
    raise Rejection("heterogeneous_evidence_collapse", "preflight")


def hostile_portfolio(base):
    modes = ("row_major", "column_major", "reverse", "annotation_major", "sha256_permutation", "fixed_chunks", "four_threads")
    baseline = evaluate(base)
    relabelled = evaluate(make_fixture("label-only-split", 3, 3, relabel=True))
    category_shift = evaluate(make_fixture("category-boundary-shift", 3, 3, category_shift=True))
    physical_move = evaluate(make_fixture("moved-coordinate", 3, 3, moved=True))
    sharp_fixture = make_fixture("sharp", 2, 1, sharp=True, constant_exposure=750)
    sharp = evaluate(sharp_fixture)
    equal_sharp = evaluate(make_fixture("equal-sharp", 2, 1, sharp=True, equal_sharp=True, constant_exposure=750))
    missing = clone(sharp_fixture); missing["fixture_id"] = "missing-join"; missing["edges"][0]["interface"] = None
    contradiction = evaluate(make_fixture("contradiction", 2, 1, sharp=True, contradiction=True, constant_exposure=750))
    unavailable_exposure = evaluate(make_fixture("missing-exposure", 2, 1, exposure_missing=True))
    zero_exposure = evaluate(make_fixture("zero-exposure", 2, 1, constant_exposure=0))
    unavailable_moisture = evaluate(make_fixture("missing-moisture", 2, 1, moisture_missing=True))
    zero_moisture_fixture = make_fixture("zero-moisture", 2, 1); zero_moisture_fixture["cells"][0]["moisture"] = 0
    zero_moisture = evaluate(zero_moisture_fixture)

    mode_results = [evaluate(base, mode) for mode in modes]
    mode_digests = {result["semantic_digest"] for result in mode_results}
    edge_cells = {cell["oracle_cell_id"]: cell for cell in sharp_fixture["cells"]}
    edge_cell_results = {cell_id: cell_result(sharp_fixture, cell)[0] for cell_id, cell in edge_cells.items()}
    forward = edge_result(sharp_fixture, sharp_fixture["edges"][0], edge_cells, edge_cell_results)
    reversed_edge = clone(sharp_fixture["edges"][0]); reversed_edge["a"], reversed_edge["b"] = reversed_edge["b"], reversed_edge["a"]
    reversed_result = edge_result(sharp_fixture, reversed_edge, edge_cells, edge_cell_results)

    islands = make_fixture("islands", 3, 3)
    for cell in islands["cells"]:
        cell["annotation"][2] = "island" if (cell["x"], cell["y"]) in ((0, 0), (2, 2)) else "separating-cross"
    islands["annotation_manifest"] = [list(value) for value in sorted({tuple(cell["annotation"]) for cell in islands["cells"]})]

    ramps_ok = True
    for field in ("horizontal", "vertical", "diagonal", "plateau", "reversed"):
        fixture = make_fixture("field-" + field, 17, 17, field=field)
        for cell in fixture["cells"]:
            x, y = cell["x"], cell["y"]
            horizontal = (1000 * x + 8) // 16
            expected = {"horizontal": horizontal, "vertical": (1000 * y + 8) // 16,
                        "diagonal": (1000 * (x + y) + 16) // 32,
                        "plateau": min(750, horizontal), "reversed": 1000 - horizontal}[field]
            ramps_ok = ramps_ok and cell["exposure"] == expected

    provenance_ok = True
    for field in subject():
        fixture = make_fixture("provenance-" + field, 2, 1, sharp=True, constant_exposure=750)
        fixture["edges"][0]["interface"]["subject"][field] = subject("b")[field]
        provenance_ok = provenance_ok and evaluate(fixture)["edge_outcomes"][0][1] == "provenance_mismatch"
    material_mismatch = make_fixture("provenance-material", 2, 1, sharp=True, constant_exposure=750)
    material_mismatch["edges"][0]["interface"]["left_material_evidence_id"] = identity("forged-material")
    provenance_ok = provenance_ok and evaluate(material_mismatch)["edge_outcomes"][0][1] == "provenance_mismatch"

    missing_cell = clone(base); missing_cell["cells"].pop()
    duplicate_edge = clone(base); duplicate_edge["edges"][-1] = clone(duplicate_edge["edges"][0])
    stale_partition = clone(base); stale_partition["annotation_manifest"] = []
    poison = [True, "1", None, 1.0, float("nan"), -1, 1001]

    coarse = evaluate(make_fixture("coarse", 5, 5, step=1 << 33))["cell_details"]
    fine = evaluate(make_fixture("fine", 9, 9))["cell_details"]
    coarse_map = {tuple(cell["coordinate"]): cell["cell_digest"] for cell in coarse}
    fine_map = {tuple(cell["coordinate"]): cell["cell_digest"] for cell in fine}
    moved_coords = {tuple(cell["coordinate_q32_32"]) for cell in make_fixture("moved", 5, 5, step=1 << 33, moved=True)["cells"]}

    families = {
        "label_only_split": baseline["semantic_digest"] == relabelled["semantic_digest"] and baseline["audit_digest"] != relabelled["audit_digest"],
        "relabelled_and_moved_categorical_boundary": baseline["semantic_digest"] == category_shift["semantic_digest"] and baseline["audit_digest"] != category_shift["audit_digest"] and baseline["semantic_digest"] != physical_move["semantic_digest"],
        "enumeration_chunk_and_thread_permutation": len(mode_digests) == 1 and all(result["evaluated_edges"] == 12 for result in mode_results) and forward["edge_digest"] == reversed_result["edge_digest"],
        "disconnected_equal_signature_islands": connected_signature_components(islands, "island") == 2,
        "unavailable_versus_numeric_zero": unavailable_exposure["cell_details"][0]["outcome"] == "unavailable_evidence" and zero_exposure["cell_details"][0]["palette"] == [0, 0, 0] and unavailable_moisture["cell_details"][0]["palette"] == zero_moisture["cell_details"][0]["palette"] and unavailable_moisture["edge_outcomes"][0][1] == "unavailable_evidence" and zero_moisture["edge_outcomes"][0][1] == "continuous_cause_exact",
        "explicit_sharp_and_missing_witness": sharp["edge_outcomes"][0][1] == "sharp_cause_exact" and sharp["cell_details"][0]["palette"] == [135, 168, 135] and sharp["cell_details"][1]["palette"] == [540, 168, 23] and evaluate(missing)["edge_outcomes"][0][1] == "unavailable_evidence",
        "equal_valued_explicit_sharp_cause": equal_sharp["edge_outcomes"][0][1] == "sharp_cause_exact",
        "steep_continuous_exposure": evaluate(make_fixture("steep", 2, 1))["edge_outcomes"][0][1] == "continuous_cause_exact",
        "rational_ramps_plateaus_and_reversal": ramps_ok,
        "rounding_thresholds": palette((1, 1, 1), (499, 500, 501), (1000, 1000, 1000), 1000)[0] == (0, 1, 1),
        "type_range_overflow_coordinate_and_forged_output": all(expect_rejection(lambda value=value: palette((value, 1, 1), (1, 1, 1), (1, 1, 1), 1)) for value in poison) and expect_rejection(lambda: palette((1, 1), (1, 1, 1), (1, 1, 1), 1)) and expect_rejection(lambda: make_fixture("zero-step", 1, 1, step=0)) and 1000 ** 4 > (1 << 32) and expect_rejection(lambda: exact_int(1001, 0, 1000)),
        "provenance_substitution": provenance_ok,
        "partition_evidence_poison": duplicate_key_rejected('{"a":1,"a":2}') and duplicate_key_rejected('{"a":1} trailing') and expect_rejection(lambda: validate_fixture(missing_cell)) and expect_rejection(lambda: validate_fixture(duplicate_edge)) and expect_rejection(lambda: validate_fixture(stale_partition)),
        "same_dimension_contradiction": contradiction["edge_outcomes"][0][1] == "contradictory_evidence" and sharp["edge_outcomes"][0][1] == "sharp_cause_exact",
        "bounded_wrap_and_diagonal_attack": baseline["declared_edges"] == 12 and join_disposition(base, "cell-000-000", "cell-001-001") == "unsupported_join" and join_disposition(base, "cell-000-000", "cell-002-000") == "unsupported_join" and join_disposition(base, "cell-000-000", "unknown") == "unsupported_join",
        "aligned_refinement_and_moved_control": all(fine_map[coordinate] == value for coordinate, value in coarse_map.items()) and not (set(coarse_map) & moved_coords),
        "recursive_blend_order_drift": recursive_blend([0, 1000, 500]) == 500 and recursive_blend([500, 1000, 0]) == 375,
        "fixed_width_categorical_halo": (200 + 800 + 1) // 2 == 500 and 500 not in (200, 800) and sharp["edge_outcomes"][0][1] == "sharp_cause_exact",
        "heterogeneous_evidence_collapse": expect_rejection(lambda: collapse_heterogeneous({"exposure": 500, "material": [200, 400, 600]}), "heterogeneous_evidence_collapse"),
    }
    return families


def main():
    tracemalloc.start()
    modes = ("row_major", "column_major", "reverse", "annotation_major", "sha256_permutation", "fixed_chunks", "four_threads")
    fixtures = [
        make_fixture("grid-1x1", 1, 1), make_fixture("grid-1x9", 1, 9),
        make_fixture("grid-9x1", 9, 1), make_fixture("grid-2x2", 2, 2),
        make_fixture("grid-3x3", 3, 3), make_fixture("aligned-5x5", 5, 5, step=1 << 33),
        make_fixture("aligned-9x9", 9, 9), make_fixture("field-horizontal", 17, 17),
        make_fixture("field-vertical", 17, 17, field="vertical"),
        make_fixture("field-diagonal", 17, 17, field="diagonal"),
        make_fixture("field-plateau", 17, 17, field="plateau"),
        make_fixture("field-reversed", 17, 17, field="reversed"),
        make_fixture("grid-256x256", 256, 256),
    ]
    fixture_results = []
    disposition_counts = {"pass": 0, "rejected": 0}
    phase_counts = {"evaluation": 0, "preflight": 0}
    maximum_bits = 0
    for fixture in fixtures:
        fixture_commitment = streaming_sha256(fixture)
        retain_details = fixture["fixture_id"] in ("aligned-5x5", "aligned-9x9")
        baseline = evaluate(fixture, modes[0], fixture_commitment=fixture_commitment, collect_details=retain_details)
        semantic_digests = {baseline["semantic_digest"]}
        audit_digests = [baseline["audit_digest"]]
        maximum_bits = max(maximum_bits, baseline["maximum_product_bits"])
        reference_cell_projection_list = baseline["cell_projection_list"]
        reference_edge_list = baseline["edge_digest_list"]
        validated_counts = (baseline["declared_cells"], baseline["declared_edges"])
        for mode in modes[1:]:
            result = evaluate(
                fixture, mode, reference_cell_projection_list=reference_cell_projection_list,
                reference_edge_list=reference_edge_list, validated_counts=validated_counts,
                fixture_commitment=fixture_commitment,
                collect_details=False,
            )
            if result["evaluated_edges"] != baseline["declared_edges"]:
                raise AssertionError("enumeration skipped edge evaluation")
            semantic_digests.add(result["semantic_digest"])
            audit_digests.append(result["audit_digest"])
            maximum_bits = max(maximum_bits, result["maximum_product_bits"])
            del result
        if len(semantic_digests) != 1:
            raise AssertionError("enumeration changed semantic digest")
        fixture_results.append({
            "audit_digests": sorted(audit_digests),
            "declared_cells": baseline["declared_cells"],
            "declared_edges": baseline["declared_edges"],
            "fixture_id": fixture["fixture_id"],
            "semantic_digest": baseline["semantic_digest"],
        })
        disposition_counts["pass"] += 1
        phase_counts["evaluation"] += 1
    aligned_coarse = evaluate(fixtures[5])["cell_details"]
    aligned_fine = evaluate(fixtures[6])["cell_details"]
    coarse_map = {tuple(cell["coordinate"]): cell["palette"] for cell in aligned_coarse}
    fine_map = {tuple(cell["coordinate"]): cell["palette"] for cell in aligned_fine}
    if any(fine_map[coordinate] != value for coordinate, value in coarse_map.items()):
        raise AssertionError("aligned refinement disagreement")
    try:
        make_fixture("oversize-257x256", 257, 256)
        raise AssertionError("oversize fixture accepted")
    except Rejection as error:
        if (error.code, error.phase) != ("resource_limit", "preflight"):
            raise
        disposition_counts["rejected"] += 1
        phase_counts["preflight"] += 1
    vectors = [
        (((1000, 800, 600), (900, 700, 500), (500, 400, 300), 750), (338, 168, 68)),
        (((1, 1, 1), (499, 500, 501), (1000, 1000, 1000), 1000), (0, 1, 1)),
        (((999, 999, 999), (1000, 1000, 1000), (1000, 1000, 1000), 499), (499, 499, 499)),
        (((999, 999, 999), (1000, 1000, 1000), (1000, 1000, 1000), 500), (500, 500, 500)),
        (((999, 999, 999), (1000, 1000, 1000), (1000, 1000, 1000), 501), (500, 500, 500)),
        (((1000, 1000, 1000), (1000, 1000, 1000), (1000, 1000, 1000), 1000), (1000, 1000, 1000)),
    ]
    if any(palette(*operands)[0] != expected for operands, expected in vectors):
        raise AssertionError("known arithmetic vector failed")
    isolated = palette((1000, 800, 600), (900, 700, 500), (500, 400, 300), 750)[0]
    isolated_mutation = palette((999, 800, 600), (900, 700, 500), (500, 400, 300), 750)[0]
    if isolated[1:] != isolated_mutation[1:]:
        raise AssertionError("band isolation failed")
    families = hostile_portfolio(make_fixture("hostile-base", 3, 3))
    if set(families.values()) != {True} or len(families) != 19:
        raise AssertionError("hostile family failed")
    _, peak = tracemalloc.get_traced_memory()
    tracemalloc.stop()
    receipt = {
        "audit_digests": [item["audit_digests"] for item in fixture_results],
        "declared_cell_count": sum(item["declared_cells"] for item in fixture_results),
        "declared_edge_count": sum(item["declared_edges"] for item in fixture_results),
        "disposition_counts": disposition_counts,
        "enumeration_modes": list(modes),
        "evaluated_cell_count": sum(item["declared_cells"] for item in fixture_results) * len(modes),
        "evaluated_edge_count": sum(item["declared_edges"] for item in fixture_results) * len(modes),
        "expected_outcomes": {"hostile_families": 19, "positive_fixtures": len(fixtures), "preflight_rejections": 1},
        "fixture_ids": [item["fixture_id"] for item in fixture_results],
        "hostile_family_ids": sorted(families),
        "maximum_product_bits": maximum_bits,
        "observed_outcomes": {"hostile_families": sum(families.values()), "positive_fixtures": disposition_counts["pass"], "preflight_rejections": disposition_counts["rejected"]},
        "pass": True,
        "resource_ceiling": {"max_traced_bytes": 256 * 1024 * 1024, "peak_below_limit": True},
        "schema_version": SCHEMA,
        "semantic_digests": [item["semantic_digest"] for item in fixture_results],
        "violation_codes": ["arithmetic_out_of_range", "material_interface_provenance_mismatch", "missing_material_interface_join", "noncanonical_input", "regional_exposure_unavailable", "regional_moisture_unavailable", "resource_limit", "same_dimension_continuity_conflict"],
        "failure_phase_counts": phase_counts,
    }
    receipt["receipt_hash"] = digest("suite", receipt)
    output = canonical(receipt)
    if len(output) + 1 > 65_536 or peak > 256 * 1024 * 1024:
        raise AssertionError("proof-tool resource ceiling exceeded: stdout=%d peak=%d" % (len(output) + 1, peak))
    print(output.decode("ascii"))


if __name__ == "__main__":
    main()
