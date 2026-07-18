# G1 C3 physical partition-policy design audit

Date: 2026-07-15

Status: **bounded design and adversarial review complete; implementation and
named ecology remain separately gated**.

## Decision

Retain **no partition** as the honest fallback. The smallest acceptable future
partition is not a distance-based cluster and not a neighbour-to-neighbour
tolerance flood. It is a two-stage deterministic reconstruction:

1. a versioned, content-authored, total classification recipe maps every
   validated cell to one canonical **physical signature**; then
2. Forge rebuilds the shared-edge connected components of cells with exactly
   equal signatures over the exact `spatial-domain` v1 topology.

This separates policy from topology. Classification decides which physical
values have the same authored category at a declared scale; connectivity only
separates spatially disconnected occurrences of that category. Callers may
submit neither region membership nor component edges as canonical facts.

The candidate is suitable for a separate implementation-readiness and owner
authorization gate. This audit does not authorize a crate, thresholds, named
regions, biomes, organisms, runtime terrain or promotion.

## Why the obvious tolerance algorithm fails

An edge rule such as `abs(a[d] - b[d]) <= tolerance[d]` is symmetric but is not
transitive. For adjacent values `0, 5, 10` with tolerance `5`, both edges pass,
so connected-component closure merges all three cells even though the endpoints
differ by `10`. A longer gradient can percolate across an arbitrarily large
range. The resulting component therefore does not satisfy the apparent
tolerance claim.

Adding a running mean, centroid or component minimum/maximum during growth does
not repair the contract: membership can then depend on traversal or merge order.
Sorting the traversal merely hides that semantic instability behind one chosen
algorithm.

If authored content needs a tolerance-like resolution, it must be expressed as
an anchored, exhaustive value partition with exact cut points. That creates a
true per-cell signature before connectivity. It should be called an authored
classification resolution, not a universal physical tolerance.

## Candidate comparison

| Candidate | Useful property | Failure | Disposition |
|---|---|---|---|
| no partition | exact, cheapest and honest | emits no physical regions | permanent fallback |
| exact `physical_regime_id`, then connectivity | deterministic and provenance-safe | continuous fields can fragment into singletons; equality is not similarity | diagnostic fixture only |
| per-edge per-dimension tolerance, then connectivity | locally intuitive | non-transitive chaining can violate the claimed end-to-end tolerance | reject |
| component centroid/range growth | can bound a completed component | merge and traversal order can change membership | reject |
| unconstrained clustering | may produce visually smooth groups | needs an objective, distance, seed, count and stopping policy; spatial connectivity is not guaranteed | reject |
| universal exposure/moisture bands | simple | transfers arbitrary thresholds as natural truth | reject |
| versioned total cell-signature recipe, then exact connectivity | deterministic, spatial, replayable and explicit about policy | authored cuts remain local policy and can be unstable across versions | select for readiness audit |

## Structural evidence and transfer limits

- Rosenfeld and Pfaltz define a connected subset through a path of neighbouring
  points and treat connected components as equivalence classes after the point
  property and neighbourhood are fixed. This supports property-first,
  connectivity-second reconstruction. It does not supply Forge classification
  thresholds. Source: Azriel Rosenfeld and John L. Pfaltz, *Sequential
  Operations in Digital Picture Processing*, Journal of the ACM 13(4), 1966,
  DOI `10.1145/321356.321357`, author-hosted copy accessed 2026-07-15,
  https://www.cs.virginia.edu/~jlp/66.sequential.op.pdf
- OGC coverage structure separates an explicit domain from the values attached
  to its positions. Forge already implements the relevant bounded domain seam;
  Earth coordinate systems and encodings do not transfer. Source: OGC
  09-146r8, *Coverage Implementation Schema 1.1*, accessed 2026-07-15,
  https://docs.ogc.org/is/09-146r8/09-146r8.html
- The US EPA ecoregion framework states that multiple biotic and abiotic
  characteristics contribute with importance that varies by region and level.
  This rejects a universal Forge weight or threshold and does not authorize EPA
  categories for another world. Source: US EPA, *Ecoregions*, accessed
  2026-07-15, https://www.epa.gov/eco-research/ecoregions

## Candidate contract boundary

### `PhysicalPartitionRecipeV1`

The recipe must contain only closed, canonical fields:

- `schema_version = 1`;
- a non-empty content provenance identifier and authored recipe version;
- an explicit applicability statement binding the supported spatial-domain,
  field and physical-evidence schema versions;
- an ordered, non-empty, duplicate-free list of supported C3 dimensions;
- for each dimension, either `exact_value` or strictly increasing authored
  lower-bound cuts over that dimension's full validated integer range;
- fixed classification semantics: a value's bin is the number of lower-bound
  cuts less than or equal to it;
- explicit handling for evidence that is physically unavailable rather than
  silently treating an unavailable measurement as zero;
- fixed `shared_edge_4` connectivity and `bounded_absent` edges inherited from
  the exact domain; and
- exact limitations and authority-negative claims.

Version 1 may admit only already validated integer C3 dimensions. At minimum,
regional exposure and regional moisture potential remain separate. Moisture
potential is physically available only when the exact upstream hydrological
evidence proves surface-accessible liquid; otherwise the signature carries a
distinct `unavailable` tag. Unlike dimensions are never summed, averaged or
collapsed into an unexplained scalar.

For a `0..1000` dimension, lower-bound cuts must be unique and inside
`1..=1000`. For example, cuts `[250, 750]` canonically produce `[0,249]`,
`[250,749]` and `[750,1000]`. This fixed rule eliminates gaps, overlap,
inclusive-edge ambiguity and caller-authored tie handling.

Canonical recipe bytes produce `physical_partition_recipe_id`. Recipe identity
must change when dimension order, cuts, availability semantics, applicability,
provenance, limitations or schema changes. There is no ambient latest recipe.

### `PhysicalPartitionInputV1`

The input must bind:

- the exact validated spatial-domain descriptor and identity;
- exact logical-world and reconstruction identities;
- exact replayable upstream physical evidence and regional field recipes needed
  to reconstruct every cell's admitted C3 values;
- the exact canonical partition recipe bytes and identity; and
- an explicit proof-resource ceiling that is not a production world-size claim.

Forge reconstructs every spatial cell and every selected physical value. A
caller-provided array of values, signatures, region labels, memberships or
edges is evidence at most and cannot enter the canonical input.

### Deterministic reconstruction

1. Validate the domain, recipe and upstream source evidence before output.
2. Rebuild every domain cell in ascending `(x_index, y_index)` order.
3. Rebuild the exact selected physical evidence for that cell.
4. Map each selected dimension to its exact value or authored bin and create the
   ordered cell signature.
5. Create an undirected relation only between reconstructed shared-edge
   neighbours with byte-identical signatures.
6. Compute the relation's connected components. Implementation traversal may
   vary, but final membership must be identical.
7. Sort member cells by `(x_index, y_index)` and components by their smallest
   member index.
8. Prove every domain cell appears in exactly one component and every component
   is non-empty and connected.

Component identity binds the partition-run identity, signature bytes and
canonical member cell identities. Partition-run identity binds the domain,
reconstruction, exact source evidence and recipe identities. Any change to
domain extent/resolution/topology, physical evidence, availability, recipe or
membership therefore rekeys the affected result. Logical-world identity alone
does not imply reusable partition identity.

Validation must fully rebuild the result and compare canonical bytes. It must
not maintain an independent permissive validator that can drift from the
builder.

## Adversarial readiness matrix

A future implementation package is not ready until its proposed fixtures prove:

| Failure class | Required falsifier |
|---|---|
| tolerance chaining | `0,5,10` with apparent tolerance `5` demonstrates why local tolerance mode is unavailable |
| disconnected equality | two separated equal-signature islands receive different component identities |
| smooth exact values | exact-value mode may produce singleton components without being relabelled a biome |
| order attack | all cell, edge and source permutations rebuild byte-identical output |
| cut boundary | values immediately below, exactly at and above every cut use the fixed lower-bound rule |
| malformed recipe | empty/duplicate dimensions, duplicate/descending/out-of-range cuts and unsupported schemas fail before output |
| availability confusion | unavailable moisture is distinct from numeric zero and cannot bypass hydrological evidence |
| membership forgery | omitted, duplicated, out-of-domain or cross-signature member cells fail full rebuild |
| topology forgery | diagonal, wrapped, asymmetric, duplicate or caller-invented edges fail |
| identity drift | domain, source, recipe, cut, availability or member changes rekey the appropriate identities |
| pathological domains | one-cell, one-row, uniform, checkerboard and maximum proof-size domains remain exact and bounded |
| codec drift | whitespace, unknown fields, noncanonical order and legacy schema bytes fail closed |
| authority overclaim | biome, habitat, organism, spherical, runtime, approval and promotion claims fail validation |

## Failure engineering retained beyond code

- No default recipe is inferred merely because the engine supports recipes.
- A recipe is content policy with provenance, not a discovered law of nature.
- Passing deterministic tests proves replay and contract integrity, not that the
  authored regions are scientifically or creatively useful.
- Changing recipe version is a new partition reconstruction, never an in-place
  reinterpretation of old component identities.
- Cached output, parallel execution and lazy iteration may optimize a future
  implementation only if a full canonical rebuild produces the same result.
- A future sphere, wrapped surface, hierarchy or irregular graph requires a new
  spatial schema and migration evidence before any partition recipe can target
  it.

## Explicit nonclaims

The candidate emits observer-independent physical classification components
only. It is not a planet shape, continent, terrain mesh, watershed, weather
cell, biome, habitat, hazard map, resource quality, vegetation class, niche,
organism, lineage, aesthetic region, visibility field, traversability map,
storage shard, streaming unit, runtime simulation or engine authority.

## Readiness and exact owner gate

The design survives static adversarial review because the selected classifier
is total before connectivity and has no tolerance-chaining or traversal-order
semantics. Implementation remains gated until a separate readiness receipt
confirms the exact closed dimension enum, source-reconstruction seam, bounded
resource policy, strict codec surface, hostile fixtures, module boundary and
permanent verification shield.

That bounded receipt is now recorded in
`G1_C3_PHYSICAL_PARTITION_IMPLEMENTATION_READINESS.md`. No crate or named
partition content should be created until the owner explicitly approves its
exact candidate boundary.
