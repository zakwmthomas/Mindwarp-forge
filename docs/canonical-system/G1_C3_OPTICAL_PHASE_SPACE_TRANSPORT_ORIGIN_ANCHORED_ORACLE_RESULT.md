# G1 / C3 origin-anchored optical phase-space transport oracle result

Date: 2026-07-17

Status: **the immutable-origin free-space candidate survives for a conservative
64-bit input subdomain and may return to a code-facing implementation-readiness
audit. No crate, schema or production source is authorized.**

## Pinned result

- Oracle source SHA-256:
  `97b287ec78d2d8f5031a3c7fbddbcd435db77a649e63ee4697519e1d6f66c156`
- Receipt SHA-256:
  `bbedc5a632b112b6eb633af57830034dfb99f98881f4f8968fbd44a42be93e76`
- Exact corner/interior equivalence falsifiers: **18**
- Hostile rejections and typed stops: **15**
- Base run identity:
  `dd944ff6567e83885d9013f2bf525c065632c5a5eba49815f0638010b7bb0e4c`

The optimized common-denominator result equals direct Python `Fraction`
evaluation at all 16 parameter-box corners, the centre and one deterministic
interior point. Residual endpoints are constructed from unreduced shared
denominators before canonical reduction; reduced favourable values are not
used to establish the arithmetic shield.

## Conservative width disposition

For input denominator and all signed numerators bounded by `B` bits, the
dominant no-cancellation Q160 projection intermediate is bounded by
`4B + 234` bits. The constants include a signed Q32.32 face numerator, its
`2^32` denominator, complete six-term affine extents, optimized residual
subtraction and the Q160 outward shift.

- Largest mathematically guaranteed cap under 512 bits: **69 bits**
- First over-cap bound: `B=70` gives **514 bits**
- Frozen readiness recommendation: `B=64` gives **490 bits**
- Remaining mathematical margin at 64 bits: **22 bits**

The recommendation deliberately uses the power-of-two 64-bit boundary rather
than the largest 69-bit proof. Constructed 70- through 88-bit cases sometimes
fit because of their values and reductions, but they have no universal shield
and grant no admission authority.

## Three-face utility

The 64-bit portfolio certifies the ordered faces `x+, y+, x+` from one
immutable origin:

- one face: complete;
- two faces: complete;
- three faces: complete;
- maximum observed raw / projection width: **408 bits**;
- maximum stored reduced width: **248 bits**; and
- three-step tracked operations: **978**.

Every face is derived directly from the original correlated forms. The width
does not compound with step count. The preceding face enclosure is used only
to select the next unique current-cell face; it is never promoted into the
next mathematical origin.

The constructed 96-bit case stops on its first face at 531 bits. The rejected
generic repeated-relinearization design remains worse: its 16-bit case reaches
513 bits on the second plane and its 24-bit case reaches 778.

## Hostile and terminal evidence

The oracle returns typed stops for mixed-sign direction, face tie and
non-forward progress. Medium or substance change returns
`interface_required`; outer and unavailable neighbours remain distinct
terminals. Cell, band/time, face order, missing face, resource ceiling,
authority and stale identity mutations change or invalidate the run identity.
The symbolic 70-bit candidate is rejected as
`arithmetic_shield_exceeded` even though a favourable constructed fixture may
fit.

Interface, reflection, refraction, TIR, scattering and direction mutation are
not supported. No hostile receives a favourable transport, arrival, power or
visibility result.

## What the result proves

It proves that an additive owner can, in principle, preserve exact original
correlation while binding a bounded sequence of current-cell free-space face
events, without arbitrary precision and without reusing widened output as a
new rational origin. It also proves a useful three-face portfolio above the
design falsifier's 32-bit floor.

It does not prove current Rust integration, codecs, allocation ceilings,
platform compilation, V1 identities or rollback. It does not classify receiver
coverage. Those remain code-facing readiness obligations.

## Next boundary

Rewrite the failed implementation-readiness record only if it freezes the
64-bit immutable-origin cap, exact Q32.32/Q160 algebra, current-owner replay,
typed terminals, step ceiling, codec and allocation limits, platform gates,
deletion-only rollback and one exact serious owner action. Stop at that owner
gate before adding a crate, contract, dependency, production test or source.

