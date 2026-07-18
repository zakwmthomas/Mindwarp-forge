# G1 C3 Physical Region Partition Result

**Status:** corrected bounded v1 implemented and focused tests pass; C3 remains
open.

## Whole-plan fit

The implementation fits the master sequence as a C3 physical-world primitive:
it converts already validated physical fields into observer-independent
bounded regions before C6 can attach ecological meaning. It does not implement
biomes, organisms, a planet, runtime generation, or storage, and therefore
does not unlock C4, close C3, or select R1.

Two corrections were required before implementation. First, coordinate-free
regional source binding is now owned by `regional-environment-state` as
`RegionalFieldBindingV1`; the partition crate consumes it without duplicating
regional semantics. Second, signature availability is derived from the exact
`ClimateContract`: absorbed shortwave gates exposure and nested
surface-accessible liquid gates moisture. This prevents caller-authored
availability flags and distinguishes absent evidence from an available zero.

The owner's conditional approval was applied only to this corrected bounded
v1. It grants implementation authority, not approval or promotion authority.

## Implemented proof

- strict recipe, input, result, and upstream-binding replay;
- canonical closed dimension order and strict lower-bound cut validation;
- exact-value, binned, and unavailable signature states;
- exhaustive non-wrapping shared-edge connected components;
- stable component membership, boundary, input, and partition identities;
- a 65,536-cell proof ceiling checked before sampling;
- failure tests for reordered/duplicate dimensions, invalid cuts, unknown
  classifiers, noncanonical bytes, reconstruction mismatch, forged membership,
  identity and authority drift, disconnected equal-signature islands,
  traversal-order variation, and exact cut boundaries.

Focused verification passes 10 `physical-region-partition` tests and 10
`regional-environment-state` tests.

## Retained boundaries and next action

This result remains rectified bounded proof evidence. It is not spherical
geometry, a full planet, terrain, a watershed, biome semantics, ecological
occupancy, visibility, traversability, persistence, streaming, or runtime
generation.

The next C3 action is a post-partition closure reassessment against the
remaining second-platform, signal-propagation, visibility/traversability, and
physical-biome prerequisites. Named biome or C6 work remains gated.
