# P7b-1 Containment Readiness and Design Gate

**Status:** research/design and the capability-free P7b-1a policy reference are
verified. P7b-1b denial-canary and P7b-1c renderer compatibility remain
separately gated. No isolation feature, package, tool, renderer, format, or
asset was installed, enabled, selected, or executed.

## Decision summary

P7b-1 must be split into three separately gated packages:

1. **P7b-1a containment-profile reference:** a capability-free Rust validator
   for inert policy records and hostile synthetic profiles. It launches no
   process and touches no filesystem or network.
2. **P7b-1b denial canary:** only after explicit owner approval, launch a tiny
   harmless test executable inside one selected Windows boundary and prove that
   forbidden host reads/writes, network access, child escape, and resource
   overrun fail closed. This is not a renderer trial.
3. **P7b-1c renderer compatibility:** only after the denial canary passes and a
   new owner decision covers the exact tool, source, license, version, install,
   execution, and removal plan.

No stage silently authorizes the next. A green configuration validator proves
policy consistency, not operating-system containment. A green canary proves
only the tested boundary and host build, not arbitrary renderer safety.

## Local feasibility snapshot

The snapshot was collected read-only on 2026-07-13.

| Candidate | Local fact | Security conclusion | Decision |
|---|---|---|---|
| Windows Sandbox | Host is Windows 11 Home; Microsoft supports Sandbox on Pro, Enterprise, and Education, not Home. No Sandbox executable is present. | Its hardware boundary is attractive, but enabling it is not locally available without an edition change. Its defaults also expose network, clipboard, and vGPU unless explicitly disabled. | Infeasible now; do not upgrade or enable. |
| Hyper-V isolated Windows container | No Docker or Podman command is present; Windows client container prerequisites exclude Home. | Hypervisor isolation is robust; process-isolated containers are not an adequate hostile boundary. Hyper-V isolated Windows containers do not provide the desired GPU path. | Infeasible now and disproportionate. |
| WSL 2 | `wsl --status` reports WSL is not installed. | WSL is designed for integrated development: NAT networking, Windows interoperability, and drive automount are configurable integration surfaces, not a safe default for hostile rendering. | Reject as the first containment runner; do not install. |
| Job object plus ordinary process | Available Windows primitive. | Useful for time, memory, process-count limits, kill-tree recovery, and accounting, but not a security boundary by itself. | Required supporting control only. |
| AppContainer / LPAC | Windows 11 supports AppContainer; exact renderer/package compatibility is unproved. | Microsoft recognizes AppContainer as a security boundary. LPAC is more restrictive and access must be explicitly granted. Full-trust MSIX packaging alone is not containment. | Sole plausible local candidate for a later denial canary; not yet selected or executed. |

An optional-feature query was unavailable or denied, so no conclusion is drawn
from it. The edition, executable inventory, and WSL status are sufficient for
the immediate no-install decision.

## Primary-source reconciliation

Sources were checked on 2026-07-13. They constrain the design; they do not
endorse a tool.

| Source | Retained lesson |
|---|---|
| [Windows Sandbox](https://learn.microsoft.com/en-us/windows/security/application-security/application-isolation/windows-sandbox/) | Sandbox uses hardware virtualization but is unavailable on Home. Ephemeral disposal is useful, not a substitute for restrictive configuration. |
| [Windows Sandbox configuration](https://learn.microsoft.com/en-us/windows/security/application-security/application-isolation/windows-sandbox/windows-sandbox-configure-using-wsb-file) | Network, clipboard, audio, vGPU, mapped folders, and protected-client mode are separate controls. Writable host mappings can expose the host; vGPU expands attack surface. |
| [Windows application isolation](https://learn.microsoft.com/en-us/windows/security/book/application-security-application-isolation) | AppContainer is a Windows security boundary and Win32 isolation uses least privilege plus explicit access. |
| [Launch an AppContainer](https://learn.microsoft.com/en-us/windows/win32/secauthz/implementing-an-appcontainer) | AppContainer and LPAC restrict files, registry, network, devices, processes, windows, and credentials; LPAC requires still narrower capabilities. |
| [MSIX containerization](https://learn.microsoft.com/en-us/windows/msix/msix-containerization-overview) | Converted Win32/MSIX applications are full trust by default. Package identity and virtualization must never be confused with AppContainer isolation. |
| [Windows container security](https://learn.microsoft.com/en-us/virtualization/windowscontainers/manage-containers/container-security) | Microsoft treats hypervisor-isolated containers as robust; process isolation is insufficient for hostile elevated workloads. |
| [WSL configuration](https://learn.microsoft.com/en-us/windows/wsl/wsl-config) and [networking](https://learn.microsoft.com/en-us/windows/wsl/networking) | Automount, Windows interoperability, NAT/mirrored networking, DNS, and proxy integration are normal WSL features. A special hardening configuration would need independent proof. |
| [Windows job objects](https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects) | Job objects bound a process tree and resources and support unit termination; modern per-process security still needs a real isolation token/boundary. |
| [Blender scripting and security](https://docs.blender.org/manual/en/latest/advanced/scripting/security.html) | Native scene files can carry scripts and drivers; disabling automatic execution does not erase all executable paths. Native project files, plugins, scripts, drivers, and macros remain forbidden in the first tool trial. |
| [OWASP File Upload Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/File_Upload_Cheat_Sheet.html) | Extensions and media types are spoofable; parsers face active content, overwrites, and resource bombs. Inputs and outputs remain hostile bytes until independently admitted. |

## P7b-1a containment profile

The next safe reference should validate serialized records only:

`ToolIdentity + BoundaryProfile + InputPolicy + OutputPolicy + ResourceBudget + RecoveryPlan -> ContainmentReadinessReceipt`

- `ToolIdentity` records a placeholder tool class, official-source rule,
  publisher/signature requirement, exact binary and dependency hashes, license
  evidence, version, update policy, and removal plan. It selects no product.
- `BoundaryProfile` names the claimed boundary and host prerequisites, denies
  network, credentials, clipboard, devices, host UI, GPU, plugins, scripting,
  package-manager access, and undeclared capabilities, and distinguishes a
  security boundary from a resource limiter.
- `InputPolicy` permits only tiny generated synthetic fixtures through a fresh
  read-only content-addressed directory. Repository paths, user directories,
  native project files, archives, external references, environment expansion,
  UNC/device paths, alternate streams, and reparse traversal reject.
- `OutputPolicy` permits only a newly created per-run quarantine. It declares
  byte/file/depth/type budgets and requires host-side admission after the
  process has stopped and access is revoked.
- `ResourceBudget` bounds wall/CPU time, memory, process count, output bytes and
  files, nesting, dimensions, geometry, textures, and frames. Job-object kill
  and timeout receipts are mandatory supporting controls.
- `RecoveryPlan` requires retained failure evidence, full process-tree
  termination, boundary disposal, quarantine retention or safe discard, project
  immutability proof, and a clean retry with a new run identity.

The validator must reject claims that an ordinary process, restricted path,
job object, package identity, full-trust MSIX, WSL distribution, or
process-isolated container is independently a robust hostile-code boundary.

## Output quarantine and admission

The runner, if later authorized, may write only to a fresh directory that is
not inside the repository or an existing user content directory. The directory
is granted to the isolated identity for one run and never reused.

After termination, the host revokes runner access before it examines output.
Admission proceeds from cheap inert checks to narrower parsers: path and
reparse validation, count/size/depth limits, file signature and declared-type
checks, content hashing, manifest completeness, and only then a format-specific
validator inside another bounded lane. No previewer, shell extension, DCC, or
project importer opens unadmitted output. Accepted evidence is copied; the
runner never writes directly to durable evidence or project state.

## Hostile fixture matrix

| Fixture family | Required result |
|---|---|
| `..`, absolute, UNC, device, reserved-name, environment-expanded, alternate-stream path | Reject before launch. |
| Symlink, junction, mount point, or other reparse ancestor/target | Reject; reuse the A3 Windows junction failure as a mandatory regression. |
| External URI/file reference, hidden dependency, archive, or native project file | Reject from the first trial. |
| Script, driver, macro, plugin, startup hook, shell command, or child-process request | Reject or deny; retain the attempt. |
| Network, DNS, proxy, localhost, credential, clipboard, device, host-window, or registry access | Deny and record without granting a capability to observe success from inside the workload. |
| Decompression bomb, excessive nesting/count/dimensions/geometry/frames, NaN/infinity, malformed/truncated buffer | Reject or terminate within the declared budget. |
| Repository or user-directory write, rename, hard link, or reparse output | Hard containment failure; stop the program. |
| Timeout, crash, hang, child survivor, partial write, locked output | Kill the entire job, retain failure evidence, prove no project mutation, and retry only with a new run identity. |
| Stale, duplicate, missing, unexpected, polyglot, executable, or active-metadata output | Quarantine; never preview or promote. |
| Same canary repeated | Equivalent denial and disposal receipt; unexplained drift is a failure. |

## P7b-1b denial-canary entry criteria

A later canary requires one explicit owner decision naming the boundary and the
exact harmless executable. Before launch, Forge must prove:

- P7b-1a profile and hostile synthetic tests pass;
- the executable is locally built from reviewed source or obtained from an
  official pinned source with signature/hash/license evidence;
- no installation, elevation, OS edition purchase, optional-feature change,
  firewall change, credential use, or package-manager action is implicit;
- no network, host UI, clipboard, credentials, devices, GPU, plugins, scripts,
  repository access, or existing user-directory write capability is granted;
- a job object bounds and terminates the entire process tree;
- a fresh read-only synthetic input and fresh write-only quarantine are the
  only deliberate data paths;
- host-side negative observations verify denied reads/writes/network and clean
  disposal. The workload is never asked to handle real hostile code.

AppContainer/LPAC is the current candidate because it is the only researched
native security boundary plausibly available on this host. Compatibility,
launch mechanics, ACL correctness, child inheritance, and output revocation
remain falsification targets, not assumptions.

## Whole-system alignment

| Boundary | Alignment |
|---|---|
| P2 identity | Tool, run, input, output, profile, and receipt identities are derivatives; none becomes universe truth. |
| P3 fields | A contained render cannot infer or rewrite world law. |
| P4 history | Attempts and failures append; retries get new identities. Temporary runner state is disposable. |
| P5 scheduler | Resource budgets are containment limits, not runtime significance or production performance claims. |
| P6 construction | Only already-validated synthetic recipes may later enter. Visual output cannot repair invalid semantics. |
| P7a representation | Exact artifact and derivative lineage remain prerequisites; containment does not validate representation. |
| P7b-0 protocol | Environment and stimulus receipts bind the exact containment receipt. Review cannot hide a security failure. |
| Reference Studio | Displays admitted receipts and stimuli read-only; it does not launch, browse, scan, approve, or promote. |
| Controlled application | A3 hostile path/junction, environment, network, process, crash, and rollback evidence is reused as a lower-bound regression, not treated as arbitrary-code sandbox proof. |
| Protected Kernel and authority | Policy records are serialized evidence only. Runner code cannot access or mutate Kernel, approval, spending, credential, publishing, or promotion state. |

## Alternatives rejected or deferred

- **Upgrade Windows to obtain Sandbox:** deferred because it changes the OS
  edition and may spend money; no such authority exists.
- **Install WSL/Docker/Podman:** rejected for the current package. Installation
  and integration surfaces add risk without proving a stronger local boundary.
- **Use a job object or restricted working directory alone:** rejected because
  recovery and resource limits are not a security boundary.
- **Treat MSIX packaging as containment:** rejected unless the process is
  explicitly AppContainer/LPAC and capabilities are independently verified.
- **Use GPU rendering first:** rejected. Shared-device access adds driver and
  reproducibility surfaces; any GPU trial needs a separate gate.
- **Choose a renderer now:** deferred until the denial boundary passes. Tool
  popularity cannot compensate for a failed containment model.
- **Copy outputs directly into the repository:** rejected. Quarantine and
  independent admission are permanent boundaries.

## Exact next confirmation (satisfied)

Authorize Codex to implement and test **P7b-1a only**: a capability-free Rust
containment-profile and readiness-receipt validator using inert synthetic
records. It will launch no process, install or enable nothing, access no file or
network, create no image, select no renderer/format, and grant no authority.

This authorization would **not** cover the AppContainer/LPAC denial canary,
tool installation or execution, renderer selection, assets, images, GPU,
runtime, engine, spending, credentials, publishing, promotion, or
protected-Kernel mutation. Those remain separately gated.

In plain language: **may Codex build and test only the checklist that a future
sandbox must pass, without running the sandbox or any graphics tool?**

The owner explicitly approved P7b-1a subject to the usual research,
dependency-alignment, adversarial-review, repair, and full-verification
screening. That approval did not authorize P7b-1b or P7b-1c.

## Verified P7b-1a result

- The capability-free `containment-profile` crate validates strict
  `ToolIdentity`, `BoundaryProfile`, `InputPolicy`, `OutputPolicy`,
  `ResourceBudget`, `RecoveryPlan`, and recomputed
  `ContainmentReadinessReceipt` records. A valid status is only
  `policy_ready_not_executed`.
- Nineteen independent/adversarial tests reject tool selection at the policy
  stage, incomplete supply-chain rules, weak boundary claims, missing or
  duplicate host evidence, incomplete capability denials, A3/reparse and
  active-content gaps, mutable/repository/user input, unsafe quarantine,
  reordered admission, unbounded resources, incomplete recovery, stale
  receipts, runtime overclaim, schema drift, and budget exhaustion.
- Critical review after the first 18-test pass found that a job object labelled
  as a supporting control could still make the package valid. The validator was
  repaired so every valid package requires an actual security-boundary
  candidate; job-object limits remain mandatory supporting controls only.
- One Forge Desktop test stores serialized policy evidence as a read-only
  ProofReceipt without changing Kernel object, event, candidate, or authority
  state. Eleven-module boundaries bar Kernel, desktop, filesystem, process, and
  network capability from the crate.
- The full Forge gate passes with 19 P7b-1a tests, 23 desktop tests, 64 Kernel
  tests, all earlier reference suites, UI/Rust builds, formatting, worker
  proofs, and selector/checkpoint verification.
- P7b-1a proves policy consistency only. It did not install, enable, select, or
  execute an isolation feature, process, canary, renderer, parser, tool, file,
  network path, GPU, asset, or image. P7b-1b and P7b-1c remain unproved.

The exact stable-LPAC denial-canary, cleanup, and owner boundary are researched
in `P7B1B_DENIAL_CANARY_DESIGN_GATE.md`. That document grants no execution.
