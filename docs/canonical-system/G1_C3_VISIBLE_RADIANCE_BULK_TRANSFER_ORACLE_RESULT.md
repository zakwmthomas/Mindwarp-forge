# G1 C3 Visible-Radiance Bulk-Transfer Oracle Result

Date: 2026-07-15

Status: **disposable exact-arithmetic proof passed; implementation-readiness
audit is next; no consumer schema or implementation authorized.**

## Result

The selected direct-beam bulk-extinction candidate survived its first
arbitrary-precision counterexample portfolio. The proof supports:

- exact optical-depth accumulation before transmission conversion;
- maximal same-profile span merging before rounding;
- directed Q0.64 exponential interval arithmetic projected to Q0.48 output;
- explicit unavailable, ambiguous-boundary and interface-required states; and
- a conservative fixed-width target larger than `u128`.

The proof does not supply real substance coefficients, interface optics,
perception or implementation authority.

## Executed proof

`tools/prove-g1-c3-visible-radiance-math.py` uses Python arbitrary-precision
integers and `Fraction` arithmetic as its exact geometric/optical-depth oracle,
plus 120-digit `Decimal.exp()` as an independent transmission check. It compares
those results with a candidate directed fixed-point exponential enclosure:

1. reduce nonnegative optical depth by exact powers of two into `[0,1]`;
2. enclose the alternating exponential series in Q0.64 with directed integer
   multiply/divide operations;
3. restore the range with directed interval squaring; and
4. project the result outward to Q0.48.

The retained run passed:

- 1,570 total checks;
- 512 randomized exponential enclosures;
- 768 randomized three-band transfer enclosures;
- 256 randomized homogeneous-subdivision invariance checks;
- perfect-square and adjacent-nonsquare Q32.32 diagonal lengths;
- exact reversal and monotonicity in length/coefficient;
- vacuum identity, positive-length thin material and tangent point contact;
- stationary unavailable, unavailable interval, overlapping face-lane,
  undeclared interface and unique bulk-path classifications; and
- maximum-width arithmetic sizing.

The run completed in 267 ms on the current PC. Timing is diagnostic only.

## Numerical receipt

| Measure | Result |
|---|---:|
| worst returned transmission interval width | 1 Q0.48 unit |
| maximum three-axis squared-length width | 130 bits |
| worst three-axis rational common denominator | 192 bits |
| conservative exact accumulator product width | 337 bits |

The 130-bit result permanently rejects a plain `u128` sum of three maximum
Q32.32 squared endpoint deltas. The 337-bit bound follows from at most three
axis-derived denominator families, 64-bit Q16.48 coefficients, the 65,536-span
ceiling and a 65-bit directed length factor. A future readiness audit may select
a checked unsigned 384-bit local arithmetic surface, which leaves 47 bits of
headroom, or a smaller explicitly justified query ceiling. It may not silently
wrap, clamp or downgrade to float.

## Failure shields

- Same-substance cell subdivision canonicalizes to one maximal span before any
  rounding; adding cells alone cannot change transfer.
- A foreign tangent point has zero bulk length, while a positive-length thin
  material changes transfer.
- Reverse paths preserve optical depth exactly.
- Increasing length or extinction never increases the transmission upper
  bound.
- Multiple positive-length cells over one open parameter span return
  `ambiguous_boundary_lane`; no axis priority chooses physical ownership.
- Missing or unavailable interaction evidence returns unavailable.
- A change between distinct positive-length substances returns
  `interface_model_required`; reflectance or phase cannot fill the gap.
- Returned fixed transmission is an interval enclosing the high-precision
  oracle, never a falsely exact rounded scalar.

## Whole-plan fit

This result advances C3 observer-independent propagation evidence without
collapsing it into biological sight, rendering or gameplay line of sight. It
reuses exact path and substance identities and adds no second occupancy truth.
Generic probe passage remains a separate later design. No sphere, planet,
terrain, global topology, runtime, organism or biome meaning is introduced.

Biome continuity is unchanged: categorical cells and physical regions cannot
paint presentation seams. Continuous causal fields must still produce
deterministic ecotones, with sharp boundaries only where sharp physical causes
exist.

## Exact next action

Run a visible-radiance bulk-transfer implementation-readiness audit. It must
freeze the profile/query/result schemas, provenance and reconstruction chain,
checked wide arithmetic, fixed exponential algorithm, ceilings, canonical
codec, hostile tests, permanent verifier and rollback. It must resolve whether
the bounded reference embeds a small local U384 type or imposes a narrower
query ceiling. Stop before creating a consumer crate or coefficient catalogue.
