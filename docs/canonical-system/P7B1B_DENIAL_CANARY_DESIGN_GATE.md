# P7b-1b LPAC Denial-Canary Design Gate

**Status:** research/design complete; authorized Trials 1 and 2 failed safely
before canary resume, and delegated routine-test Trial 3 passed suspended-host
verification but failed during canary initialization after resume. All three
cleaned successfully. See
`P7B1B_DENIAL_CANARY_RESULT.md` and
`P7B1B_DENIAL_CANARY_TRIAL2_RESULT.md` and
`P7B1B_DENIAL_CANARY_TRIAL3_RESULT.md`. No same-package retry is authorized.

The design package itself created nothing. Its later, separately authorized
implementation and one-run trial are recorded independently in the result.

## Decision summary

The first denial trial should use one unique, zero-capability **Less Privileged
AppContainer (LPAC)** launched through the stable Win32 AppContainer APIs. It
should run one locally built, source-reviewed canary that is linker-marked to
run only in an AppContainer. It is not a renderer, parser, imported asset, or
hostile executable.

The canary is a falsification probe, not proof of arbitrary-code safety. The
host independently inspects the suspended token, job membership, and process
mitigations before allowing the canary to run. The canary then attempts only
synthetic denied operations and writes a bounded report into its disposable
per-run profile. Any unexpected success, timeout, cleanup failure, or ambiguous
observation is a hard failure. There is no automatic retry and no fallback to a
regular AppContainer or weaker process boundary.

## Primary-source findings

Sources were checked on 2026-07-13.

| Source | Retained design requirement |
|---|---|
| [Launch an AppContainer](https://learn.microsoft.com/en-us/windows/win32/secauthz/implementing-an-appcontainer) | Use `STARTUPINFOEX` with `PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES`; use zero capability SIDs; add `PROCESS_CREATION_ALL_APPLICATION_PACKAGES_OPT_OUT` for LPAC. Access is the intersection of user and AppContainer DACL grants. LPAC further removes common registry/COM/file access. |
| [CreateAppContainerProfile](https://learn.microsoft.com/en-us/windows/win32/api/userenv/nf-userenv-createappcontainerprofile) | Profile creation is per-user and creates writable per-app filesystem/registry storage. The unique profile must therefore be treated as disposable quarantine, not ignored ambient state. |
| [DeleteAppContainerProfile](https://learn.microsoft.com/en-us/windows/win32/api/userenv/nf-userenv-deleteappcontainerprofile) | Close every process and profile-storage handle before deletion. A failed deletion leaves profile state undetermined and must be retried; persistent failure blocks further trials. |
| [UpdateProcThreadAttribute](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute) | Apply security capabilities, LPAC opt-out, restricted-child policy, job list, and process mitigations at creation. Child restriction is effective only with a real sandbox that denies privileged process handles. |
| [Token information classes](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-token_information_class) | While suspended, the host can verify `TokenIsAppContainer`, `TokenAppContainerSid`, `TokenCapabilities`, and integrity information instead of trusting the child report. |
| [Job objects](https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects) | Children join the parent job by default; no breakaway flags may be set. `KILL_ON_JOB_CLOSE`, active-process and resource limits, accounting, explicit termination, and wait-for-empty are required. Job security is supporting control, not the isolation boundary. |
| [CreateProcessW](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw) | Use an absolute application path, explicit current directory and minimal Unicode environment, `bInheritHandles=FALSE`, `EXTENDED_STARTUPINFO_PRESENT`, `CREATE_SUSPENDED`, and `CREATE_NO_WINDOW`. Never inherit the caller environment, current directory, console, or handles. |
| [Process mitigation query](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessmitigationpolicy) | The host verifies applied DEP/ASLR, extension-point, dynamic-code, Win32k, and image-load mitigations before resume; an unsupported required mitigation fails closed. |
| [ACL modification](https://learn.microsoft.com/en-us/windows/win32/secauthz/modifying-the-acls-of-an-object-in-c--) | Grant the unique Package SID read/execute only to a fresh staging directory by merging a named ACE, retain the original DACL, and restore it during cleanup. Never modify repository ACLs. |
| [AppContainer networking](https://learn.microsoft.com/en-us/windows/apps/develop/networking/networking-basics) | With no network capability, connection attempts must fail. The first probe uses only a host loopback listener and does not add a loopback exemption, firewall rule, proxy, or external route. |
| [Experimental Create Process in Sandbox](https://learn.microsoft.com/en-us/windows/win32/secauthz/createprocessinsandbox) | Deferred: the API is explicitly experimental, has no public header, uses a versioned FlatBuffer specification, creates/opens profiles by identity, and can inherit caller environment/current directory when omitted. It is not the stable first proof. |

## Exact implementation boundary

If separately authorized, add two Windows-only modules that no Kernel, desktop,
UI, canonical reference, or runtime module may import:

1. `containment-denial-canary`: a tiny Rust binary with no dependencies beyond
   the standard library/Win32 bindings, built with the linker `/APPCONTAINER`
   flag so accidental ordinary execution fails.
2. `containment-canary-runner`: a narrow local host executable depending only
   on the capability-free `containment-profile` contract and pinned
   `windows-sys` already present in `Cargo.lock`. If Cargo requires any network
   download or version change, stop before continuing.

The runner is an intentional filesystem/process/local-loopback capability
module. Governance must forbid dependencies on `forge-kernel`, Tauri, desktop
UI, canonical production modules, credentials, HTTP clients, package managers,
and external network APIs. Its integration test is ignored by default and can
run only through the exact owner-authorized command. The normal full gate may
compile it but must never execute the canary implicitly.

## Per-run state machine

`preflight -> staged -> profile_created -> process_suspended -> host_verified -> resumed -> stopped -> access_revoked -> admitted_or_quarantined -> profile_deleted -> receipt_recorded`

Every transition is append-only. A failure transitions to `terminating`, then
performs the same cleanup path and records the earliest failure. Later cleanup
failures are appended; they never replace the original cause.

### 1. Preflight and staging

- Bind the exact host build, source tree hash, canary source/binary hash,
  contract version, `windows-sys` version, limits, and intended Win32 calls.
- Reject elevation, impersonation, Developer Mode bypass assumptions, existing
  loopback exemptions, stale profile names, dirty prior quarantine ownership,
  or a runner already inside AppContainer.
- Generate a random per-run moniker under 64 permitted characters. Never derive
  and reuse an existing profile after `ERROR_ALREADY_EXISTS`; collision blocks.
- Build the canary locally, verify `/APPCONTAINER`, copy it into a newly created
  staging directory outside the repository, and hash the copy. The runner never
  executes from `target`, the repository, PATH, a shell, or a relative path.
- Create only inert synthetic sentinels. The canary receives no repository
  path, user-content path, credential, token, secret, imported file, or network
  address other than one random loopback port.

### 2. Profile, ACL, job, and launch

- Call `CreateAppContainerProfile` with zero capabilities. Treat the entire
  unique profile filesystem and registry storage as hostile, disposable
  quarantine. Its redirected `LOCALAPPDATA`, `TEMP`, and `TMP` are not trusted
  output channels.
- Grant the Package SID only read/execute on the fresh staging directory. Save
  the original DACL. Do not add any ACE to the repository or user content.
- Create an unnamed job before process creation with no breakaway flags,
  `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`, active-process limit 1, bounded process
  and job memory, CPU/accounting limits, and an associated completion port.
- Use a creation attribute list containing: zero security capabilities, LPAC
  All Application Packages opt-out, restricted child-process policy, atomic job
  assignment, and declared mitigations. Prefer atomic `JOB_LIST` assignment to
  avoid a process-running-before-job race.
- Launch by explicit absolute path, suspended and without a window. Pass no
  inheritable handles, shell, parent environment, parent current directory,
  standard streams, or command expansion.

### 3. Host verification before resume

The host, not the canary, verifies:

- token is AppContainer and its SID exactly matches the fresh profile;
- capability count is zero and integrity is Low;
- LPAC opt-out and child restriction were requested in the immutable creation
  attributes;
- process is in the expected job and active-process limit is 1;
- required mitigations are active; unsupported or missing required mitigation
  terminates before resume;
- staged binary and synthetic input hashes are unchanged; and
- no profile collision, unexpected child, inherited handle, or output exists.

### 4. Bounded denial probes

After resume, the canary performs this fixed sequence once:

1. Confirm its own token observations for comparison with the host record.
2. Read a fresh synthetic sentinel deliberately lacking a Package-SID ACE;
   expected result: access denied.
3. Attempt to spawn its own staged binary with a `--child` marker; expected
   result: child creation denied and job active count remains one.
4. Attempt one TCP connection to the host's random `127.0.0.1` listener;
   expected result: denied/no accept. No loopback exemption, firewall change,
   DNS, proxy, LAN, or Internet endpoint is used.
5. Write one final bounded report to an explicit path inside its own profile
   quarantine; expected result: success. No other output path is supplied.
6. Exit with a fixed status. The host enforces a short wall timeout regardless
   of the child report.

The first canary deliberately does not touch the repository, Credential
Manager, real credentials, clipboard, registry, devices, GPU, GUI, external
network, renderer, parser, native project, or imported asset. Absence of those
probes is a retained limitation, not a pass claim.

### 5. Independent observation and cleanup

- The host records listener non-acceptance, job accounting/process count, exit
  status, timeout state, and immutable synthetic sentinel hashes independently.
- Terminate the whole job on every exit path, wait until it is empty, close
  process/thread/job/listener handles, and only then inspect quarantine.
- Admit at most one report under a tiny byte limit using strict UTF-8 and
  deny-unknown-fields JSON. Record every unexpected path, file, stream, reparse
  point, size, or schema as hostile metadata; never preview it.
- Copy only the admitted receipt bytes and hashes into durable evidence. Do not
  retain or parse other profile content.
- Restore the staging DACL, remove staging, and call
  `DeleteAppContainerProfile`. Retry deletion as documented after rechecking
  all handles. If deletion still fails, record the exact moniker/path and block
  all later trials; do not hide, reuse, or broadly delete state.
- Verify the repository inventory, approved user sentinels, loopback exemption
  state, and process inventory are unchanged. A retry always needs a new run
  identity and a fresh owner-authorized package after failure analysis.

## Failure matrix

| Failure | Required outcome |
|---|---|
| Existing profile/moniker collision | Stop; never derive/reuse it. |
| LPAC cannot load the locally built canary | Record incompatibility; do not fall back to regular AppContainer or relax ACLs. |
| Token SID, capability count, integrity, job, or mitigation mismatch | Terminate before resume. |
| Parent environment/current directory/handle is inherited | Hard harness failure. |
| Synthetic denied read succeeds | Hard boundary failure; terminate and retain evidence. |
| Child starts or job count exceeds one | Hard escape/control failure. |
| Loopback connection is accepted | Hard network-isolation failure. |
| Canary cannot write its profile report | Compatibility failure, not permission to add a host writable mapping. |
| Timeout, crash, unexpected exit, or missing report | Failed trial; terminate, clean, no same-run retry. |
| Unexpected profile output, reparse point, stream, executable, or oversized report | Quarantine metadata only; no preview/admission. |
| DACL restoration or profile deletion fails | Block all later trials and report exact retained state. |
| Any repository/user/credential/firewall/feature/installation mutation | Stop program; rollback only the exact owned reversible artifact and escalate. |
| All probes pass | Proves only this canary and exact host/profile/configuration; renderer/tool containment remains unproved. |

## Whole-system alignment

| Boundary | Alignment |
|---|---|
| P2 identity/P4 history | Run moniker, source, binary, profile, input, report, failure, and cleanup receipts are content-bound; collisions never reuse history. |
| P5 scheduler | Job budgets are containment limits only, not runtime performance or significance evidence. |
| P6/P7a | The canary has no construction or representation payload. A containment pass cannot validate semantics or an artifact. |
| P7b-0/P7b-1a | The exact containment receipt can later bind an environment/stimulus; policy validation remains distinct from denial evidence. |
| A3 controlled application | Reuses fail-closed path, reparse, process, crash, and rollback thinking without claiming A3 is an arbitrary-code sandbox. |
| Reference Studio | May display the admitted receipt read-only; it cannot launch, clean, retry, approve, or promote. |
| Kernel/authority | Runner and canary cannot import or call protected Kernel, approval, spending, credential, publishing, or promotion paths. |

## Alternatives rejected or deferred

- **Experimental composable sandbox API:** deferred despite useful default-deny
  fields because it is explicitly experimental, its public contract is not
  stable, and its FlatBuffer/header toolchain is not publicly packaged.
- **Regular AppContainer fallback:** rejected because silent weakening would
  invalidate the LPAC claim.
- **MSIX/package deployment:** rejected for the canary; package identity is not
  required for stable manual launch and adds install/signing lifecycle.
- **PowerShell, shell, system utility, or downloaded executable canary:**
  rejected due interpreter, ambient capability, provenance, and update surface.
- **Pipes or inherited standard handles:** rejected; they enlarge the handle
  boundary. A bounded profile report is simpler to revoke and inspect.
- **External-network or real-credential probe:** rejected as unnecessary and
  disproportionate. First evidence uses loopback and synthetic sentinels only.
- **Repository write probe:** rejected from the first trial. The canary is not
  given a repository path; host inventory proves no mutation.
- **Automatic retry after failure:** rejected because it can hide nondeterminism
  or cleanup residue.

## Exact owner confirmation

Authorize one **P7b-1b LPAC denial-canary implementation and trial** under the
exact boundary above. This would permit Codex to:

- add and compile the two narrow Windows-only Rust modules;
- create one fresh staging directory, synthetic sentinel, unique per-user LPAC
  profile, Package-SID ACL, unnamed job, and local loopback listener;
- launch the locally built `/APPCONTAINER` canary once with zero capabilities;
- perform only the fixed synthetic denial probes;
- terminate, inspect a bounded report, restore ACLs, delete all owned temporary
  state and the profile, verify immutability, and retain a read-only receipt.

This does **not** authorize elevation, OS/feature/firewall/loopback-exemption
changes, package installation, dependency downloads, external network, DNS,
proxy, real credentials or user content, repository access by the canary,
renderer/DCC/parser execution, imported assets, images, GPU, engine/runtime,
spending, publishing, promotion, or protected-Kernel mutation. Any unexpected
success or cleanup residue stops the trial; no weakening or retry is implied.

In plain language: **may Codex build one tiny test program, run it once inside a
fresh Windows LPAC sandbox, verify that synthetic file/child/loopback actions
are blocked, then remove the sandbox and report exactly what happened?**
