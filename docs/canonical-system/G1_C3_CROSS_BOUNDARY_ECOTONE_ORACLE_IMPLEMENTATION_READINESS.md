# G1 C3 cross-boundary ecotone oracle implementation readiness

Date: 2026-07-18

Status: **ready for one exact disposable-oracle decision only; no Python
oracle, oracle result or production artifact has been created.**

## Bounded candidate

The only admitted implementation candidate is one independent disposable
Python proof tool. It reproduces the selected evidence-preserving
typed-boundary model with exact arithmetic, runs the frozen grid and hostile
portfolio, and emits one canonical summary receipt.

The later additive package is limited to:

1. `tools/prove-g1-c3-cross-boundary-ecotone.py`;
2. `tools/verify-g1-c3-cross-boundary-ecotone-oracle.ps1`; and
3. `docs/canonical-system/G1_C3_CROSS_BOUNDARY_ECOTONE_ORACLE_RESULT.md`.

Route, checkpoint, generated-context, README and verifier-invocation updates
may reference those three artifacts. No other source artifact is admitted.
The tool is a repository proof harness, not a declared Forge module or
production schema.

## Independence boundary

The script accepts no command-line arguments, stdin, external fixture file,
network data or environment-derived semantics. It writes exactly one compact
canonical JSON object followed by one newline to stdout and writes no file.
It runs with the bundled Python in isolated no-bytecode mode (`-I -B`).

Only Python standard-library exact and deterministic facilities are admitted:
`fractions`, `hashlib`, `json`, `itertools`, `tracemalloc`, and bounded
`concurrent.futures`. The implementation may not import Forge modules, call a
production helper, read a production artifact, execute a compiled binary, use
FFI, subprocess, filesystem discovery, clock values, locale, network,
randomness, floats, `Decimal`, NumPy, Torch, CuPy or a GPU API.

The equation is embedded independently. The focused PowerShell verifier may
inspect the frozen Rust relation, but the Python tool may not read or execute
that source. GPU execution is inappropriate because the proof is exact,
small-integer, branch-heavy and dominated by canonical provenance checks.

## Oracle-only fixture input

`EcotoneFixtureV1` is an internal disposable test model, explicitly not a
production record. It contains exactly:

- `schema_version`, exactly `1`;
- an ASCII `fixture_id` from a closed manifest;
- `subject`: nonzero 32-byte lowercase-hex reconstruction, spatial-domain,
  regional-recipe, climate, stellar and atmosphere evidence IDs;
- `domain`: signed Q32.32 origin pair, positive Q32.32 step pair, positive
  width and height, `shared_edge_4` adjacency and `bounded_absent` boundary;
- a total unique cell list;
- a canonical shared-edge evidence list; and
- a total unique annotation manifest.

Each cell contains its integer `(x,y)` index, fixture-local oracle cell ID,
checked signed-Q32.32 coordinate, and separately typed causal evidence:

- three-band stellar irradiance permille;
- three-band atmosphere transmission permille;
- three-band surface reflectance permille plus material evidence ID;
- exact or unavailable regional exposure; and
- exact or unavailable regional moisture.

Every exact permille component is an integer in `[0,1000]`; Python booleans are
not integers for this contract. Causal provenance binds the actual owner
identities. Region signature, component ID and display label are annotation
only and are forbidden from causal calculation and the semantic digest.

Every undirected edge is declared once in canonical endpoint order. Optional
fixture-local interface evidence contains an exact edge, cause ID,
`surface_reflectance_discontinuity` kind, subject, left and right material
evidence IDs, three-band values and a fixture revision ID. It is always marked
`synthetic_fixture_only`. It is not a canonical 2D material-interface owner.

## Exact arithmetic and pinned vectors

For each cell and band, validate the four operands before multiplication and
compute with Python arbitrary integers:

```text
product = irradiance * transmission * reflectance * exposure
palette = (product + 500_000_000) // 1_000_000_000
```

Independently require equality with `Fraction(product, 1_000_000_000)` rounded
half-up by `(2 * numerator + denominator) // (2 * denominator)`. `round()`,
float conversion, saturation and wrapping are forbidden. The maximum valid
product is `10^12`; the post-bias magnitude requires at most 40 bits.

Permanent known vectors include:

| Fixture | Expected palette/result |
|---|---|
| baseline `[1000,800,600]`, `[900,700,500]`, `[500,400,300]`, exposure `750` | `[338,168,68]` |
| `1 * 499 * 1000 * 1000` | `0` |
| `1 * 500 * 1000 * 1000` | `1` |
| `1 * 501 * 1000 * 1000` | `1` |
| `999 * 1000 * 1000 * 499` | `499` |
| `999 * 1000 * 1000 * 500` | `500` |
| `999 * 1000 * 1000 * 501` | `500` |
| all factors `1000` | `1000` |
| sharp left reflectance `[200,400,600]` with baseline other factors | `[135,168,135]` |
| sharp right reflectance `[800,400,100]` with baseline other factors | `[540,168,23]` |

Band-isolation fixtures mutate one operand in one band and require the other
two output bands to remain unchanged. Equal products with different factor
roles are not provenance-equivalent.

## Canonical result projections

The oracle freezes two deliberately different projections.

`semantic_digest` covers only canonical coordinate-keyed palette values,
typed shared-edge outcomes, exact cause identity and exact one-sided values.
It excludes fixture name, region signature, component ID, display label,
enumeration order, chunking and thread completion order.

`audit_digest` covers the complete fixture input, causal provenance,
partition/domain identities and execution manifest. A relabelled, moved or
refined fixture may therefore have a different audit digest while preserving
the comparison-specific semantic projection.

No test may claim that the entire receipt is byte-identical across fixtures
whose input or provenance changed. Label and enumeration tests compare the
full `semantic_digest`; refinement compares only coordinate-keyed causal
values at exactly coincident coordinates.

Canonical JSON is UTF-8 with ASCII-only identifiers, sorted object keys,
compact separators, no NaN and no insignificant whitespace. Cells sort by
spatial-domain ID, `x`, `y`, and oracle cell ID. Edges normalize endpoints and
sort by domain and endpoint keys. Fixed permutation order is SHA-256 of a
constant seed plus the canonical cell key; Python runtime hashes and PRNGs are
forbidden.

Hashes use SHA-256 over an ASCII disposable domain, one NUL byte, and canonical
bytes. Domains are exactly:

- `mindwarp.disposable.ecotone.cell-result.v1`;
- `mindwarp.disposable.ecotone.edge-result.v1`;
- `mindwarp.disposable.ecotone.fixture-result.v1`; and
- `mindwarp.disposable.ecotone.suite-receipt.v1`.

The suite receipt contains schema version, closed fixture and transformation
IDs, declared and evaluated cell/edge counts, disposition and failure-phase
counts, sorted violation codes, semantic and audit digests, expected/observed
outcomes, pass/fail, maximum product bits, peak traced bytes, and the receipt
hash computed without its own hash field.

Do not invent hash pins before source exists. The oracle result must pin the
raw Python source SHA-256 and canonical stdout SHA-256 only after two isolated
runs are byte-identical.

## Typed edge rules

Palette calculation and complete cross-boundary evidence remain separate.
Moisture is not a palette operand: unavailable moisture cannot erase an
otherwise exact palette. It may make a complete continuous-source edge witness
`unavailable_evidence(regional_moisture_unavailable)`.

Every edge emits one top-level outcome with a canonically sorted `violations`
list. Compound faults use this precedence:

1. `noncanonical_input`;
2. `arithmetic_out_of_range` or resource limit;
3. `provenance_mismatch`;
4. `unavailable_evidence`;
5. `unsupported_join`;
6. `contradictory_evidence`;
7. `sharp_cause_exact`; and
8. `continuous_cause_exact`.

Contradiction is dimension-local. Continuous exposure plus an explicit sharp
material interface is compatible and yields `sharp_cause_exact`. Only
incompatible continuity and discontinuity claims about the same causal
dimension, subject, edge and fixture revision yield
`contradictory_evidence`. Different material values without the explicit
fixture-local interface witness yield
`unavailable_evidence(missing_material_interface_join)`, never inferred
sharpness. Diagonal, wrapped or unknown-cause joins yield `unsupported_join`.

Equal values do not erase an explicit interface. Equal numeric values never
repair provenance mismatch. Every reason is a closed snake-case code, never
free text.

## Frozen positive grid portfolio

- `1 x 1`: one cell, zero internal edges and four bounded-absent sides;
- `1 x 9` and `9 x 1`: eight internal edges and no wrap;
- `2 x 2`: four internal edges and no diagonal edge;
- `3 x 3`: a separating cross and disconnected equal-signature corners;
- aligned `5 x 5` step `2^33` and `9 x 9` step `2^32`: compare only the 25
  exactly coincident coordinates;
- `17 x 17`: horizontal, vertical, diagonal, plateau and reversed rational
  exposure fields;
- `256 x 256`: exactly 65,536 cells and 130,560 internal edges; and
- `257 x 256`: rejected before coordinate construction, allocation or
  evaluation with zero evaluated cells and edges.

Synthetic exposure before one half-up quantization is horizontal
`1000*x/(w-1)`, vertical `1000*y/(h-1)`, and diagonal
`1000*(x+y)/(w+h-2)`, with explicit singleton-axis rules. Moving a coordinate
is not a refinement match.

Each admissible fixture runs row-major, column-major, reverse,
annotation/component-major, SHA-256 permutation, bounded fixed chunks and a
maximum four-thread evaluation. Every mode canonically merges before hashing;
scheduling never enters a receipt.

## Nineteen permanent hostile families

The implementation must preserve each design family as separately named cases:

1. label-only split;
2. relabelled and moved categorical boundary;
3. enumeration, chunk and thread permutation;
4. disconnected equal-signature islands;
5. unavailable versus numeric-zero exposure and moisture;
6. explicit sharp material interface and missing-witness control;
7. equal-valued explicit sharp cause;
8. steep continuous exposure without a sharp cause;
9. rational ramps, plateaus and reversal;
10. below-half, exact-half and above-half rounding;
11. type, range, overflow, coordinate and forged-output poison;
12. reconstruction, regional source, moisture source, recipe, domain, cell,
    climate, material and interface-subject provenance substitution;
13. stale, duplicate, missing, forged and noncanonical partition evidence;
14. same-dimension sharp/continuous contradiction;
15. bounded-edge wrap and diagonal attack;
16. aligned refinement and moved-coordinate negative control;
17. recursive blend order drift;
18. fixed-width categorical halo and sharp-interface blur; and
19. heterogeneous-evidence collapse into one score or weight.

The recursive negative control uses half-up sequential averaging from the
first value. `[0,1000,500]` yields `500`; its reversal `[500,1000,0]` yields
`375`, proving order dependence. The fixed-width negative control uses a
one-cell symmetric half-up average across a categorical split, demonstrating
both an invented label-correlated midpoint and an averaged explicit sharp
interface.

Raw poison cases include booleans, floats, NaN/infinity, numeric strings,
missing factors, unknown and duplicate JSON keys, invalid UTF-8 and trailing
bytes. Duplicate and missing cells or edges are detected before a map or set
could erase them.

## Resource and platform gates

- width and height are positive and at most `256`;
- evaluated cells are at most `65,536`;
- undirected internal edges are at most `130,560`, using
  `w*(h-1)+h*(w-1)`;
- signed Q32.32 coordinates use positive steps and checked Python-integer
  construction before signed-`i64` admission;
- fixtures run sequentially with `O(N+E)` work, no recursion and at most four
  worker threads;
- canonical stdout is at most `64 KiB`;
- `tracemalloc` peak is at most `256 MiB`; and
- the complete bundled-Windows-Python run has a `120` second proof budget.

Timing and memory are proof-tool budgets, not semantic truth or production
performance claims. A failed budget stops and returns to readiness revision;
ceilings may not be silently raised. A second available Python 3.11-or-newer
platform may be used only as extra evidence and must emit the same canonical
receipt bytes. No platform semantic fork is authorized.

## Implementation and verification gates

An explicitly released implementation must:

1. add only the frozen three-artifact package and route references;
2. run the Python source twice with `-I -B` and require identical stdout;
3. pin source and receipt hashes only from those successful runs;
4. pass every positive grid, seven enumeration modes, nineteen hostile
   families, known arithmetic vector and early-rejection count;
5. prove source scans contain no production import/helper, external input,
   capability, float, PRNG or GPU path;
6. prove Cargo files, all crates/contracts, module registries, module count and
   production bytes are unchanged;
7. run parser, record-role, module-context, modularity and focused historical
   C3 shields; and
8. pass the complete Forge gate.

Any requirement for a production owner, material-cell join, external fixture,
new dependency or existing-source change invalidates this deletion-only route
and returns to a serious owner decision.

## Rollback

On any arithmetic, digest, fixture, cap, independence, platform or complete
gate failure, delete only the Python oracle, focused oracle verifier, oracle
result and their route/invocation references, then refresh generated context.
Retain this readiness record and the verified mathematical-design audit as
evidence. No data migration exists.

The readiness stage itself changes no Cargo file, crate, contract, production
test/source, module registry/boundary or record-role rule. `tools/**` and
`docs/canonical-system/**` already have canonical record roles.

## Claim ceiling

Passing the later oracle may establish exact causal replay, category-label
independence, bounded order/reversal/refinement behavior, synthetic
sharp-interface retention and typed failure in the frozen fixtures only.

It cannot prove a continuous interpolant, real spatial material interface,
metres, physical calibration, received energy, physical or perceptual
visibility, visible seamlessness, biome/ecology/organism behavior, renderer
quality, production performance, runtime integration, promotion or C3 closure.

## Exact owner decision

Approve or reject one test-first disposable exact-rational oracle package with
the exact three artifacts, schemas, digests, arithmetic, typed outcomes,
positive grids, nineteen hostile families, caps, isolated stdlib execution,
verification gates and deletion-only rollback above.

Approval authorizes only that repository proof tool. Any production schema,
crate, dependency, canonical 2D material-interface owner, blend kernel or
cause scale, physical-evidence acquisition, renderer/visibility behavior,
downstream consumer, promotion or C3 closure remains a separate serious owner
decision.

General continuation does not implement the oracle in this stage. Stop here
after readiness verification. Nothing broader is locked in. One consumer first, reassess before expanding.

## Verification receipt

- `tools/verify.ps1`: exit `0` on 2026-07-18.
- Measured wall time: `303.5` seconds.
- Captured verification output: `2,436` lines.
- Record-role verification: `849` durable files classified by `32` ordered
  rules.
- Modularity verification: `52` modules with no forbidden imports or
  dependency cycles.
- The complete parser sweep, 20 readiness-compatible C3 route shields,
  checkpoint, bootstrap, worker-feedback freshness, generated context,
  module context, UI, Rust workspace and isolated desktop build all passed.
