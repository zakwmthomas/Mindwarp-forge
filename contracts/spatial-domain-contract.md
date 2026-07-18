# Spatial Domain Contract v1

This capability-free C3 contract binds exact logical-world and reconstruction
identity to one finite two-dimensional rectified sampling domain over the
existing signed Q32.32 field coordinate frame.

V1 fixes a cell-centre origin, positive per-axis step, finite cell counts,
zero-based indices, shared-edge four-neighbour adjacency and `bounded_absent`
edges. Opposite edges never become neighbours. Coordinates, domain and cell
identities, and canonical neighbour lists are reconstructed from the exact
descriptor and index; caller-authored cell coordinates, IDs or edge lists are
never accepted as canonical facts. Checked `i128` arithmetic proves the
furthest sample coordinate fits `i64` before a domain is admitted.

The one-million-cell proof ceiling bounds validation and test cost. It is not a
production world-size claim, storage layout or eager-materialization rule.
Changing origin, step, count, axis order, coordinate frame, adjacency or
boundary semantics changes domain and cell identity. There is no ambient
latest schema and no silent migration to wrap, hierarchy, graph or globe
semantics.

This is a bounded sampling domain, not a planet, sphere, projection, terrain,
physical-region partition, biome, habitat, visibility, traversability,
simulation or runtime map. It grants no persistence, approval, promotion,
filesystem, process, network, engine or application authority.

