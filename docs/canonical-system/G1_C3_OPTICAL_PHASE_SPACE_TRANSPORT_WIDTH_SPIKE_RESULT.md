# G1 / C3 optical phase-space transport width spike result

Date: 2026-07-17

Status: **the generic repeated-relinearization representation is rejected for
the existing checked 512-bit arithmetic. The transport subject remains open
only through an origin-anchored free-space representation audit. No production
artifact is authorized.**

## Pinned receipt

- Spike source SHA-256:
  `112fbb78356b38c0b2fad53a49c07040ea4812e484a54725b823b3f8c011d71d`
- Receipt SHA-256:
  `f7d2db26715b9a015918e3f48e25da98e9faaab2abc30ada0f4ce3801820c0c9`
- Checked storage shield: **512 bits**
- Candidate input caps: **16, 24, 32, 48, 64, 80, 96, 112, 128,
  160, 192, 256 and 368 bits**
- Ordered steps examined: **one, two and three**

The spike tracks both reduced stored values and the pre-reduction numerator,
denominator, cross-product and sum widths a fixed checked implementation must
survive. A separate symbolic pass assumes no favourable gcd cancellation.

## Falsification result

No tested candidate cap has a guaranteed one-step bound within 512 bits under
the generic reduced-rational interval algorithm. The symbolic pass finds only
a 5-bit one-step cap and no guaranteed two- or three-step cap even below the
listed production candidates. This is a representation failure, not evidence
that the mathematical enclosure is false.

Constructed near-cap cases are less pessimistic but still fail the required
repeated-step utility:

| Input cap | One plane raw / stored | Second plane raw / stored | Disposition |
|---:|---:|---:|---|
| 16 | 327 / 233 bits | 513 / 403 bits | second step exceeds shield |
| 24 | 488 / 353 bits | 778 / 613 bits | second step exceeds shield |
| 32 | 661 / 470 bits | not admitted | first step exceeds shield |
| 96 | 2,000 / 1,433 bits | not admitted | first step exceeds shield |
| 192 | 4,016 / 2,870 bits | not admitted | first step exceeds shield |
| 368 | 7,719 / 5,513 bits | not admitted | first step exceeds shield |

No constructed two-step cap remains within 512 bits. The 16-bit result misses
by one raw bit, but changing the shield to make one favourable fixture pass is
forbidden: the no-cancellation bound remains far larger, and valid 24-bit and
higher cases still fail.

## Cause

The generic representation stores independently reduced rational centre,
coefficient and residual endpoints, then reuses the widened result as the next
input. Extent addition multiplies unrelated denominators. Plane quotient,
gradient and conservative residual subtraction add further denominator
products. Reduction often helps concrete fixtures but provides no worst-case
guarantee.

Projection back to Q160/Q1.62 would cap growth only by erasing the exact shared
symbol proof. Adding arbitrary precision, copying private wide arithmetic or
raising the storage shield is outside authority. Therefore the direct
repeated-relinearization implementation candidate is rejected.

## Surviving origin-anchored question

Free-space direction is constant. For any later axis plane, the exact path
point can be derived directly from the original correlated forms as
`P_origin + ((h-P_origin,j)/V_origin,j) V_origin`, independent of earlier face
relinearizations. Ordered current-owner cell-step events can still certify the
sequence of faces and neighbour ownership. Keeping one immutable origin may
therefore make arithmetic width depend on the original cell and one final
plane rather than compound with path length.

That alternative is not silently accepted. A separate disposable audit must
prove:

- every topology step remains replayable from origin-derived projected boxes;
- order and positive progress are strict for the complete cell;
- final forms bind the complete ordered event chain;
- one-plane optimized common-denominator algebra has a useful guaranteed cap
  within 512 bits;
- residual widening does not make ordinary two- and three-face paths
  immediately unresolved; and
- any interface or direction change ends the origin-anchored free-space run.

If that audit fails, reject this transport-certificate route rather than add a
wider dependency or correlation-erasing approximation.

## Authority boundary

No crate, schema, dependency, production test or production source is
authorized. Existing cell, physical, arithmetic, interface, lineage,
cumulative and receiver owners remain unchanged. Arrival, power, visibility,
runtime, promotion and C3 closure remain excluded.

