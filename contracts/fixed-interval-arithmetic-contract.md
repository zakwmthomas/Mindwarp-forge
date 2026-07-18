# Fixed Interval Arithmetic Contract v1

This capability-free contract provides target-neutral checked signed-magnitude
512-bit values and explicitly scaled fixed intervals. It owns arithmetic
representation and directed rounding only.

`Signed512` has one canonical zero, total signed ordering, canonical decimal
parse/format, checked negate/add/subtract/multiply/shift, and mathematical
floor/ceiling division implemented from unsigned magnitudes. Native limbs,
word count, order and endianness are inaccessible.

`FixedInterval` requires ordered endpoints and one explicit fractional-bit
scale. Addition, subtraction, four-corner multiplication, intersection,
directed square root and outward precision projection are checked. Scale
mismatch, reversed or empty intervals, negative roots, zero division,
precision increase and storage overflow fail with typed arithmetic errors.

The crate owns no admitted precision list or consumer live-bit ceiling.
Physical and optical owners retain those policies and inspect magnitude bits
for their own receipts. It also owns no physical coordinates, paths, cells,
spectral bands, media, coefficients, domain identities, semantic codecs,
lineage, endpoint, visibility, collision, biome, persistence or runtime
meaning.

The only dependency is exactly `crypto-bigint 0.7.5` with default features
disabled. Arithmetic operations allocate no dynamic collections. Bounded
canonical decimal conversion may allocate its string representation. The
crate has no filesystem, network, process, UI or external capability.
