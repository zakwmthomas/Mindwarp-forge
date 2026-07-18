# Module Context Contract v1

Every module declared in `governance/module-boundaries.json` has exactly one
matching record in `governance/module-context-registry.json` and one generated
root `MODULE.md` file. The registry is canonical; the Markdown file is a
rebuildable first-read projection.

Each record states maturity, purpose, owned responsibilities, explicit
non-goals, primary entry points, invariants, verification commands and canonical
references. The generator adds the declared upstream dependencies, computed
downstream dependants and a deterministic source-tree fingerprint excluding the
generated `MODULE.md` itself and build outputs.

Workers read `MODULE.md` before module work. When a material change affects any
recorded field or neighbour boundary, they update the registry and regenerate
the projection. Verification fails for missing/extra records, missing roots,
unknown entry points or references, dependency mismatch, missing or hand-edited
front doors, or stale source fingerprints.

Module context explains boundaries; it does not grant implementation,
promotion, runtime, spending, publishing, security or owner authority.
