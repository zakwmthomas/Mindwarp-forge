# Physical Region Partition Contract v1

This capability-free contract binds an exact validated `SpatialDomain`, the
upstream-owned coordinate-free `RegionalFieldBindingV1`, an exact
`ClimateContract`, and one versioned partition recipe. It reconstructs every
bounded domain cell, creates a total physical signature, and then computes
deterministic shared-edge connected components.

V1 has two closed dimensions: regional exposure and regional moisture
potential. A dimension is either exact-valued or classified by strictly
increasing lower-bound cuts. Dimension order is canonical. Exposure is
available only when the exact climate contract has absorbed shortwave energy;
moisture is available only when its nested hydrological state has
surface-accessible liquid. Unavailable evidence is a distinct signature value,
never a caller-supplied sentinel and never numeric zero.

Components join only cells with identical full signatures that are connected
through the spatial domain's bounded shared edges. Edges never wrap.
Disconnected islands with the same signature remain distinct. Membership,
cell identities, component identities, boundaries, partition identity,
limitations, and authority claims are reconstructed and validated rather than
trusted from serialized output.

The proof is capped at 65,536 cells. That ceiling is a verification resource
bound, not a production world size, storage layout, streaming policy, terrain,
watershed, sphere, planet surface, biome, habitat, visibility, traversability,
ecology, runtime map, approval, promotion, persistence, or external-capability
claim.
