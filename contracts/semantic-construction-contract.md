# Semantic and construction reference contract

Status: `prototype_tested`. This is a capability-free synthetic proof boundary,
not product vocabulary, AI generation, a solver, geometry, representation,
physical validation, runtime behavior, or engine integration.

## Canonical layers

- Stable `ConceptId` values are canonical. Preferred, alternate, localized,
  hostile, and ambiguous labels are metadata and cannot change semantic identity.
- Claims are explicitly observed, derived, declared, or hypothetical and retain
  evidence references. The justification proof graph is acyclic and typed.
- Every role and mechanism traces to grounded claims. Unsupported causal leaps
  fail closed.
- Material decisions retain mechanism-distinct feasible alternatives or an
  explicit single-feasible-family receipt. Hard feasibility precedes named
  trade-vector comparison; there is no universal scalar score.
- Capability validation uses a closed versioned registry. Unknown, missing,
  duplicate, stale, and conflicting capabilities are violations.
- `PartRoleGraph` contains only functional nodes, typed sockets, relations, and
  capability references. It contains no mesh, voxel, rig, shader, physics body,
  executable code, runtime system, or engine object.
- `ConstructionRecipe` applies ordered typed graph operations against exact
  pre-state fingerprints. A stale or invalid operation returns no partial result;
  dangling references, incompatible sockets, invalid cardinality, disconnected
  graphs, and unexpected final fingerprints fail.

## Encoding, budget, and evidence

The reference uses compact canonical JSON with strict schemas and byte-exact
round trips. Unknown fields and noncanonical encodings are rejected. Semantic
fingerprints exclude lexical labels; full package fingerprints retain them as
evidence. Graph fingerprints canonicalize set-like collections while recipe
operation order remains explicit.

Validation consumes a bounded integer work allowance. Exhaustion returns
`indeterminate_budget`, never false impossibility or an automatic best answer.
Measurements are simulated counts only.

Forge Desktop may persist serialized evidence in a read-only `ProofReceipt`.
Recording or viewing that receipt must not alter Kernel objects, events,
candidates, authority, approval, or promotion state.

## Retained gates

The tiny fixture IDs, labels, claims, capabilities, mechanisms, trade values,
part roles, socket types, and recipe operations are discriminating test data,
not Mind Warp content grammar. P7 alone may decide representation, assets,
materials, animation, perception, and runtime adaptation after separate gates.
