# G1 C3 physical-biome readiness audit

Date: 2026-07-15

Status: **not implementation-ready; spatial-domain and partition-policy dependencies identified**.

## Decision

The current C3 evidence cannot honestly emit physical biome regions yet.
It can deterministically evaluate exposure and moisture potential at one Q32.32
coordinate and build an exact opportunity graph for that place. It does not
define a spatial domain, finite extent, sampling resolution, neighbourhood
relation, connected-region identity, boundary behavior or partition policy.

The minimum viable future output is an observer-independent **physical region
partition**, not a named biome. It must be reconstructed from explicit spatial
and physical evidence, retain scale and policy provenance, and leave ecological
meaning to C6. No code increment is authorized by this audit because the two
required upstream contracts below do not yet exist.

## Local evidence

- `regional-environment-state` accepts one coordinate and returns two bounded
  point samples. A set of cells, neighbour relation, extent and resolution are
  absent.
- `niche-graph-binding` derives opportunities for one exact world packet. Its
  edges mean only coavailability among opportunity kinds, not adjacency among
  places.
- `physical_regime_id` hashes exact sorted opportunity values. It intentionally
  supplies equality, not similarity, grouping, connectedness or a boundary.
- Current downstream lineage evidence explicitly rejects biome classification
  and physical-similarity scoring.

## Primary-practice reconciliation

The Earth sources below constrain structure only. Their categories, variables,
weights, thresholds and biological conclusions are not Forge constants.

- The US EPA ecoregion framework identifies regions from patterns and
  composition across multiple biotic and abiotic phenomena, and states that the
  relative importance of each characteristic varies between regions and at any
  hierarchical level. This rejects one universal scalar or fixed global
  weighting rule. Source: US EPA, *Ecoregions*, accessed 2026-07-15,
  https://www.epa.gov/eco-research/ecoregions
- Omernik and Griffith describe a hierarchical spatial framework that is
  aggregated and subdivided into coarser and finer units. This makes declared
  scale part of the meaning of a region rather than an incidental storage
  choice. Source: USGS Publications Warehouse, *Ecoregions of the conterminous
  United States: Evolution of a hierarchical spatial framework* (2014),
  https://www.usgs.gov/publications/ecoregions-conterminous-united-states-evolution-a-hierarchical-spatial-framework
- The USDA Forest Service national hierarchy maps nested ecological units from
  broad to fine scales and distinguishes units using different combinations of
  climate, physiography, geomorphic process, topography and other evidence at
  different levels. This supports explicit domain, scale and multi-dimensional
  policy provenance; it does not supply transferable alien-world thresholds.
  Source: USDA Forest Service, *Terrestrial Ecological Unit Inventory Technical
  Guide*, accessed 2026-07-15,
  https://www.fs.usda.gov/emc/rig/documents/integrated_inventory/TEUI_guide.pdf

## Candidate comparison

| Candidate | Deterministic | Spatially meaningful | Decision |
|---|---:|---:|---|
| no partition | yes | no regions | retain as the honest baseline until dependencies exist |
| exact physical-regime equality | yes | no; equality can join remote points and fragment adjacent gradients | reject as a biome or partition |
| universal exposure/moisture bands | yes | policy is arbitrary and overfits the two currently available regional dimensions | reject |
| unconstrained clustering | potentially | result depends on unstated distance, scale, seed and stopping rules | reject |
| content-authored versioned partition recipe over an explicit spatial domain | potentially | yes, if its dimensions, topology, scale, boundary rule and provenance are explicit | minimum viable direction after dependencies |

## Required dependency contracts

### 1. Spatial domain

A future contract must bind:

- exact reconstruction/world identity;
- coordinate reference and dimensionality;
- finite extent and explicit resolution or level;
- canonical cells or sample sites;
- a deterministic neighbourhood relation;
- edge, wrap and out-of-domain behavior; and
- strict canonical codec, replay and identity.

Coordinates alone do not imply any of these choices.

### 2. Physical partition policy

A separate versioned recipe must declare:

- which validated C3 physical dimensions it consumes;
- dimension-specific comparison rules without collapsing unlike units into an
  unexplained score;
- scale and scope for which the recipe is valid;
- boundary and tie behavior;
- whether spatially disconnected components receive distinct place identities;
- recipe provenance and explicit non-universal status; and
- deterministic rebuild, canonical ordering and hostile-input rejection.

The policy may be content-authored, but caller-submitted regions cannot become
canonical facts. Validation must rebuild membership and connected components
from the exact domain, physical evidence and recipe.

## Required future output and nonclaims

The first valid output may identify versioned physical-region components and
their member cells, boundary relations, source evidence and partition recipe.
It must not emit biome names, vegetation, organisms, habitat suitability,
hazards, trophic roles, lineage occupancy, aesthetics, runtime terrain, weather
simulation, physical visibility or body-relative traversability.

## Adversarial readiness gate

Implementation becomes ready only when fixtures prove that it:

- rejects missing or mismatched domain, level, neighbour and recipe identity;
- distinguishes disconnected equal-valued areas;
- is invariant to input ordering and exact under replay;
- changes identity when extent, resolution, topology, evidence or recipe
  changes;
- handles ties and domain edges canonically;
- rejects fabricated membership, omitted cells and noncanonical bytes; and
- preserves the C3/C6 and visibility/traversability boundaries above.

## Exact next action

Design and adversarially review the smallest C3 spatial-domain contract before
any partition implementation. The design must compare a bounded rectangular
lattice, wrapped lattice and topology-agnostic declared graph, then select only
what current field sampling and downstream physical-region consumers can
justify. Partition-policy work follows that decision; named biome work does
not.

