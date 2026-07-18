# G1 / C3 source-distribution measure oracle result

Date: 2026-07-17

Status: **passed as an abstract additive source-quantity measure; physical
quantity basis and schema remain unresolved, so implementation is not
authorized.**

## Result

The deterministic exact-rational oracle passed twice with byte-identical
receipts. It preserved both the existing geometric cell measure and the
independent source quantity across nonuniform refinement to 4, 16 and 64
leaves, including zero-quantity children. Parent quantity always equaled the
sum of both atomically produced children.

The oracle rejected all three unsafe alternatives:

- copying ambient absolute quantity to each leaf doubled quantity on split;
- holding density constant while rescaling uncalibrated abstract measure
  doubled the inferred quantity; and
- the current subject lacked every physical calibration field required to
  justify an SI-radiance label.

It also retained zero source, positive source with zero coupling, and
unresolved source as distinct typed cases. A positive exact product that
projected to zero at Q0.48 remained numerical underflow rather than physical
zero. Adding an extra geometric-spreading multiplier changed an already
geometrically coupled result and remained rejected as unproved double
counting.

## Pinned receipt

- Oracle: `tools/prove-g1-c3-source-distribution-measure.py`
- Oracle SHA-256:
  `a1eea7cead874d74d7c2bcd87f7020577bbe58469dd9d899ab61cc8aec73090a`
- Receipt checksum:
  `8bbde156aefe052dd473a149fc897da5595b7674e942b8126b102360b0570a25`
- Portfolios: `10`
- Hostile rejections: `21`
- Geometric conservation checks: `3`
- Source-quantity conservation checks: `3`
- Subdivision leaf counts: `4`, `16`, `64`
- Complete Forge gate: passed in `423.5` seconds across `2,206` output lines;
  `787` durable files were classified and all `50` module front doors remained
  current.

## Surviving claim

Only this claim survives: an exact nonnegative additive source-quantity
measure can be bound independently to the already owned exact cell algebra
without dividing by its abstract geometric measure. The candidate remains
subdivision-safe and representation-stable at the tested boundary.

This is not yet a physical source contract. `quantity_basis_id` is only an
opaque identity. The current RGB and time-basis identities do not define
wavelength integration, duration, joules, watts, photon count or radiance.
The existing dimensionless transfer contributes no missing source units.

## Decision and next action

Advance one bounded step to a **code-facing source-quantity-basis and schema
gap audit**. That audit must inspect exact ownership, canonical codecs,
nonnegative rational representation, atomic split receipts, band/time
calibration requirements, resource ceilings, platform behavior and
deletion-only rollback. It must first decide whether a physical quantity basis
exists or needs a separate mathematical design.

Do not proceed directly to implementation. Add no crate, contract schema,
dependency, production test or production source without a later exact owner
action. Multi-interface whole-cell transport, aggregation, scattering,
detector response, visibility, runtime, promotion and C3 closure remain later
or excluded authority.

Nothing broader is locked in. One consumer first, reassess before expanding.
