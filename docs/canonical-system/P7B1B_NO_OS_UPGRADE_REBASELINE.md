# P7b-1b No-OS-Upgrade Rebaseline

> **Superseded for Forge-owned visual stimuli:** the owner subsequently chose
> the built-in Forge reference viewport. External-executable containment is no
> longer an F5 prerequisite. Retain this record only for a future R1 adapter
> that actually runs untrusted executable content. See
> `P7B_BUILTIN_REFERENCE_VIEWPORT_DECISION.md`.

## Owner constraint

The owner explicitly rejected upgrading the computer's operating-system
edition as a containment prerequisite. That route is closed. Forge must not
recommend, purchase, or perform a Windows edition upgrade to rescue P7b-1b.

This record does not waive containment. It repairs when the executable proof
is required.

## Root dependency error

F5 is the **engine-neutral proof** milestone. G1 promotes those neutral proof
packs. R1 is the first milestone that selects and qualifies a concrete runtime
adapter from clean-import, build, profiling, identifier, replay, cost, and
maintenance evidence.

P7a, P7b-0, and P7b-1a fit F5: they prove representation lineage, controlled-
perception records, and containment-policy invariants without selecting or
running a renderer. P7b-1b and P7b-1c do not: an actual denial canary and
renderer compatibility test necessarily depend on a host, executable format,
tool/runtime, operating-system family, device path, and output channel.

Requiring that host-specific executable proof to finish F5 created a circular
dependency: Forge was being asked to select a sandbox before the runtime/tool
that determines the sandbox requirements could be selected.

The repair is to move **execution**, not evidence quality, to R1.

## Rebased completion boundary

### F5 may prove

- capability-free P7a representation decisions, artifact lineage, material and
  articulation plans, temporal mappings, repairs, and review conditions;
- capability-free P7b-0 controlled-perception protocol and receipt validation;
- capability-free P7b-1a containment-policy records and adversarial rejection;
- a durable `runtime_containment_pending` blocker that names the missing
  P7b-1b denial proof and P7b-1c tool compatibility proof; and
- Reference Studio read-only visibility of that blocker.

F5 completion cannot claim rendered recognisability, visual quality, arbitrary-
code safety, tool compatibility, GPU safety, or runtime containment.

### G1 may promote

G1 may promote only the capability-free neutral contracts and their negative
claims. Promotion must preserve `runtime_containment_pending`; it cannot turn a
specified asset factory, procedural animation path, renderer, or executable
adapter into a production candidate.

### R1 must prove before execution

After the owner selects one exact runtime/tool and host family, R1 must:

1. select a containment boundary that matches that exact executable and host;
2. bind tool source/provenance, binary, dependencies, input/output mappings,
   network/device/GPU exposure, resource limits, teardown, and recovery;
3. run a separately authorized denial canary in that boundary;
4. prove cleanup and host/repository/user-state immutability;
5. run a separately authorized exact-tool compatibility trial; and
6. reject runtime import if either proof is absent, stale, incompatible, or
   overclaims its host/tool scope.

No renderer, DCC, parser, imported asset, model, plugin, or engine executable
may run merely because F5 or G1 closes.

## No-upgrade technical options retained for R1

Sources were checked on 2026-07-13. These are future tool-dependent options,
not selected programs and not installation authority.

| Route | Evidence and limitation | R1 treatment |
|---|---|---|
| **WSL 2 utility VM** | [Microsoft states WSL 2 is available on Windows Home](https://learn.microsoft.com/en-us/windows/wsl/faq), uses the Virtual Machine Platform subset of Hyper-V, and can disable Windows-drive automount and Windows interop. [Installation](https://learn.microsoft.com/en-us/windows/wsl/install) still enables features, installs a Linux distribution, requires administrator action/restart, and changes the executable/tool OS. | Candidate only for an explicitly selected Linux-native headless tool. It needs a hardened no-automount/no-interop/no-network design and a fresh owner gate. |
| **Oracle VirtualBox or another full VM** | A separate guest can avoid a Windows edition upgrade, but it installs host drivers/programs and requires a trusted guest image, patching, licensing, storage, snapshot, device, and teardown policy. | Candidate when the selected tool needs a full guest and WSL is unsuitable. Different-program approval and supply-chain review required. |
| **Sandboxie Plus** | Its own documentation describes restricted-token/driver/syscall-hook isolation, configuration-dependent compatibility, and a separately licensed hardened mode. It is not a separate kernel and weakening settings can remove security isolation. | Do not adopt as the default arbitrary-tool boundary. Tool-specific research would need to prove it is not weaker than the required threat model. |
| **Remote/cloud worker** | Can provide a separate machine boundary but introduces provider trust, credentials, network transfer, retention, possible spending, and publishing/data-location concerns. | Separate owner program only. |
| **Windows Sandbox/Hyper-V on this host** | Requires an edition/feature route the owner rejected. | Closed unless the owner independently changes the constraint in the future. |

The correct R1 choice may also be **no executable adapter** if no boundary fits
the selected tool. Forge must prefer an explicit blocker over a weak sandbox.

## Adversarial safeguards

| Failure | Required result |
|---|---|
| F5/G1 receipt implies containment passed | Reject; `runtime_containment_pending` must remain visible. |
| A renderer/tool appears before R1 tool selection | Reject import and execution. |
| R1 selects a sandbox before binding the executable/host | Reject as premature and potentially incompatible. |
| WSL route leaves Windows drives, interop, network, or host sockets ambient | Fail the future design before launch. |
| VM route lacks image provenance, patch/license state, teardown, or device rules | Fail readiness. |
| Sandboxie compatibility requires a documented security weakening | Reject the route. |
| Remote route needs unapproved credentials, spending, upload, or retention | Stop at owner gate. |
| A runtime pass is reused for another tool, build, host, GPU, or boundary | Reject as stale/incompatible evidence. |

## P10 boundary

- **Baseline:** substantial time was spent forcing one Windows Home LPAC canary
  through a host-specific startup path before a renderer/runtime was selected.
- **Expected gain:** restore engine-neutral progress and postpone containment
  selection until the exact executable makes its requirements knowable.
- **Implementation cost:** one dependency/claim rebaseline and blocker fixture;
  no system change.
- **Future operating cost:** one tool-specific containment design and proof at
  R1, which was required regardless of the generic canary result.
- **Uncertainty:** a future tool may still need WSL, a full VM, remote execution,
  or may be rejected entirely.
- **Regression guard:** F5/G1 tests must reject any cleared or absent
  `runtime_containment_pending` blocker and any executable capability.
- **Stop/refocus:** if rebaselining would silently mark asset/perception/runtime
  proof complete, reject it. Only neutral contract closure is eligible.

## Exact owner decision

Recommended response:
**`Move executable containment to R1 and keep the hard blocker`**.

This authorizes a bounded dependency rebaseline, blocker contract/fixtures,
canonical state updates, and verification. It does not authorize F5/G1 to
claim runtime or visual proof; select or execute a tool; install/enable WSL,
VirtualBox, Sandboxie, a VM, container, renderer, or engine; change Windows;
use network/GPU/credentials; spend, publish, promote executable code; or mutate
the protected Kernel.

Alternative response: **`Keep F5 blocked`**.
