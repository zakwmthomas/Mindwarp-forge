# Containment Profile Contract v1

The `containment-profile` reference is the capability-free P7b-1a policy lane.
It validates inert `ToolIdentity`, `BoundaryProfile`, `InputPolicy`,
`OutputPolicy`, `ResourceBudget`, `RecoveryPlan`, and
`ContainmentReadinessReceipt` records. It launches no process and has no
filesystem, network, installer, package-manager, renderer, image, GPU, runtime,
engine, approval, promotion, spending, credential, publishing, or protected-
Kernel capability.

A valid receipt means only `policy_ready_not_executed`. It cannot claim that an
operating-system boundary, denial canary, renderer, parser, or tool has run or
is safe. AppContainer, LPAC, and hypervisor boundaries may be recorded as
security-boundary candidates; ordinary processes, restricted directories,
full-trust packages, WSL distributions, process-isolated containers, and job
objects cannot. Job-object limits and full-tree termination remain mandatory
supporting controls, never the security boundary itself.

In short, job objects cannot satisfy the containment-boundary field.

Policy-only records require an unselected tool class, future official-source,
signature, license, hash, update, and removal requirements, and no bound binary
or dependency. Inputs are fresh, content-addressed, read-only synthetic data.
The hostile matrix rejects traversal, absolute/UNC/device/reserved paths,
environment expansion, alternate streams, reparse points, external references,
archives, native projects, and active content.

Outputs go only to a fresh per-run quarantine outside the repository and
existing user content. The runner must lose access before host admission;
path/reparse, count/size/depth, signature/type, hash, manifest, and bounded
format checks occur in that order. Direct preview and durable writes reject.
All resource dimensions are finite. Recovery retains failures, kills the full
tree, disposes the boundary, revokes output access, proves project immutability,
and assigns a new identity to a retry.

Nineteen independent/adversarial tests cover canonical encoding, schema and
budget failure, tool-selection and supply-chain overreach, weak-boundary claims,
host/prerequisite evidence, complete denial sets, A3/reparse and active-content
hazards, repository/user/mutable inputs, quarantine reuse and preview, admission
order, output allowlists, resource bounds, tree termination, recovery, receipt
recomputation, runtime-containment overclaim, determinism, and authority-negative
evidence. Forge Desktop may store the serialized evidence as a read-only
`ProofReceipt`; doing so changes no Kernel object, event, candidate, or owner
authority state.
