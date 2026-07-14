# P7b-1b Capability-Free Loader Diagnosis Design Gate

## Decision

Trials 3 and 4 establish a repeated post-resume `0xC0000142`
(`STATUS_DLL_INIT_FAILED`) failure after the exact suspended LPAC checks passed.
They do not identify the DLL, initialization callback, registry key, object,
transitive dependency, or denied access. The selected next step is therefore
an **offline PE loader-surface proof**, not another LPAC launch, capability,
build permutation, debugger, trace provider, dump capture, or containment
change.

The proposed proof may read only the already bound dynamic/static candidate
bytes and emit a bounded receipt covering import symbols, delay-load entries,
TLS callback presence/count, load-configuration identity, entry point, and
embedded manifest/resource identity. It must not create a profile, launch a
process, alter an ACL or registry key, inspect unrelated memory, or claim a
runtime cause. Implementation requires a separate owner confirmation because
the active package grants research and design only.

## Exact retained observations

| Observation | What it proves | What it does not prove |
|---|---|---|
| Trial 3 dynamic candidate exited `0xC0000142` after `verify_suspended` | The configured suspended process passed token, SID, zero-capability, low-integrity, job, mitigation, discriminator, and hash checks before resume | Which module or resource failed after resume |
| Trial 4 static candidate exited the same status after `access-check-after-class46-error87` | Removing `vcruntime140.dll` and six direct API-set CRT imports was not sufficient to repair startup | That CRT code is irrelevant or that a shared system DLL is at fault |
| Both receipts have `cleanup_ok=true`, no cleanup errors, and unchanged bound hashes | The trials terminated and cleaned their owned state without admitted residue | Containment-denial behavior, because no canary report existed |
| Static candidate retains five direct imports: `advapi32.dll`, `api-ms-win-core-synch-l1-2-0.dll`, `kernel32.dll`, `ntdll.dll`, `ws2_32.dll` | These names remain in the eager direct-import surface recorded by the existing proof | Imported functions, forwarded/transitive modules, delay imports, TLS callbacks, or the failing DLL |
| Runner calls `GetExitCodeProcess` only after the process signals | The retained value is the terminated process status | A module name, loader event sequence, stack, exception record, or denied object |
| Canary source can exit only `73`, `74`, or non-Windows `75` through its explicit paths | `0xC0000142` is not an intentional canary return code | Whether failure occurred in a DLL entry point, TLS callback, runtime startup, or an unhandled pre-main exception |

Microsoft defines `CREATE_SUSPENDED` to keep the primary thread from running
until `ResumeThread`; therefore suspended-host verification is pre-user-code
evidence, not proof that loader initialization or `main` completed. Microsoft
defines `0xC0000142` as failed dynamic-link-library initialization and states
that a `DllMain` failure during process initialization terminates the process.
`GetExitCodeProcess` can retain an explicit exit value, a `main` return, or an
unhandled-exception value, so the broker's status alone is not a module trace.

## Competing hypotheses

| ID | Hypothesis | Current support | Differentiating observation | Falsifier |
|---|---|---|---|---|
| H1 | A shared eager or transitive system DLL fails process-attach initialization under the zero-capability LPAC context | **Medium.** Both variants retain the same five direct system imports and fail identically; Microsoft binds the status to DLL initialization | Offline import-symbol/forwarder surface, followed only if necessary by a separately authorized module-specific runtime observation | A trustworthy runtime record names a non-DLL cause, or a future image with the implicated eager surface absent reaches user code |
| H2 | The deliberately minimal environment, profile current directory, or LPAC object namespace denies a resource used during shared initialization | **Medium-low.** These inputs are identical across Trials 3/4; LPAC cannot read registry keys without a capability | A module-specific record plus the exact denied object, or a predeclared environment-only comparison under separate authority | The same exact context reaches user code with no resource grant, or evidence names an unrelated image defect |
| H3 | Unrecorded image structure—delay imports, TLS callbacks, load configuration, entry point, or manifest/resource data—changes the real startup surface | **Low-medium because the current receipt omits these fields.** Microsoft PE/COFF defines each as loader-relevant image data | The selected offline PE loader-surface receipt | Strict parsing proves no delay import/TLS callback surprise and binds the remaining structures without anomaly |
| H4 | The retained mitigation or job configuration interacts with initialization | **Low.** The checks accept before resume, and no evidence names a mitigation or job failure | A future, separately authorized diagnostic that preserves containment while producing a specific policy failure | Module/object evidence identifies another cause; blind removal of a shield is not a valid test |
| H5 | Canary/Rust user startup intentionally returns or throws `0xC0000142` before the report | **Low.** Explicit canary exits are 73/74 and the status has DLL-initialization meaning, but pre-main runtime behavior is not observed | A trustworthy user-code phase marker or exception record under a future exact diagnostic design | A module initialization record precedes entry to user code |

No hypothesis justifies adding `registryRead`. Microsoft documents that LPAC
cannot open registry keys without that capability, but the trials name no key
and granting it would change the zero-capability denial contract. Chromium's
LPAC practice confirms that required filesystem and registry locations need
appropriate access; it is practitioner evidence for what must be proved, not
evidence that this canary needs a particular capability.

## Information-gain comparison

| Candidate | Expected verified gain | Cost and uncertainty | Decision |
|---|---|---|---|
| A. Offline PE loader-surface proof over the two already bound images | Closes the current import-symbol, delay-import, TLS, load-config, entry-point, and manifest/resource blind spots; can reject H3 or produce a specific structural lead | Low implementation/operating cost; read-only; high uncertainty that static structure will name the runtime cause | **Selected for owner confirmation** |
| B. Broker-side debug events, loader snaps, ETW, or process snapshot | Potentially names module/event order and sharply separates H1/H2/H5 | High information but changes launch/observation semantics, requires new execution and possibly tracing/debug authority | Gated; not selected or authorized |
| C. WER LocalDumps or `MiniDumpWriteDump` | May retain module/thread/memory evidence | WER requires administrative registry configuration; minidumps require broader process access, create potentially sensitive artifacts, and still require a new failing execution | Rejected in this package |
| D. Add `registryRead`, use regular AppContainer, remove mitigations/job limits, or try more blind build permutations | A pass/fail result might change, but it would conflate cause with weakened containment or another uncontrolled variable | High regression risk and poor causal value | Rejected |
| E. New phase-split or subsystem-minimal canary | Could distinguish whether user code starts and isolate network/token/report subsystems | Requires code implementation plus another separately screened execution; useful only after the offline surface proof defines the smallest discriminating change | Deferred |

### P10 baseline and stop rule

- **Baseline:** two post-resume failures with the same status; five retained
  static direct-DLL names; zero retained import-symbol, delay-load, TLS-callback,
  load-config, entry-point, manifest, module-order, stack, exception, or denied-
  object evidence.
- **Expected gain:** replace six structural unknowns with a deterministic
  receipt and either reject H3 or identify one exact image-level discriminator.
- **Implementation cost:** one bounded PE32+ read-only parser/proof path, fixed
  fixtures, receipt binding, and verifier integration; no recurring process or
  profile cost.
- **Uncertainty:** high that static evidence alone will reveal H1/H2; the proof
  must retain this limit.
- **Stop/refocus condition:** if both images have no surprising delay import or
  TLS callback and the remaining structures do not distinguish a plausible
  cause, stop static optimization. Do not create more build variants. Prepare a
  separate owner choice between a narrowly defined dynamic module observation
  and leaving P7b-1b blocked.

## Adversarial requirements for the selected proof

1. Accept only the two exact candidate hashes already recorded; reject path
   aliases, reparse points, prefix confusion, PE32 images, truncation, overlap,
   out-of-range RVAs, unknown required layouts, and receipt overwrite.
2. Parse bytes directly from Microsoft PE/COFF structures; do not shell out to
   `dumpbin`, PowerShell inspection helpers, a debugger, or a downloaded tool.
3. Bind parser source, input hashes, section table, import DLL/function pairs,
   delay-import DLL/function pairs, TLS callback count, load-config size/hash,
   entry point, and manifest/resource identity into canonical JSON.
4. State `canary_executed=false`, `profile_created=false`,
   `registry_modified=false`, `acl_modified=false`, `capability_added=false`,
   `runtime_cause_proved=false`, and `denial_proved=false`.
5. Fail closed on ambiguity. A parser limitation is a recorded unknown, not a
   zero value or proof of absence.

## Whole-system and authority boundary

This design changes no P7a/P7b contract, containment policy, runner, canary,
profile, job, mitigation, ACL, capability, renderer, engine, runtime adapter,
publishing path, spending authority, or protected Kernel. It does not authorize
the selected proof's implementation, any dynamic diagnostic, or another trial.

## Exact owner confirmation

**Approve implementing the bounded offline PE loader-surface proof exactly as
specified above?** Approval permits only repository code/tests that read the
two exact existing candidate images and produce a claim-limited receipt. It
does not permit any process launch, LPAC profile, debugger/trace/dump action,
registry or ACL mutation, capability addition, containment weakening, renderer,
or future trial.

## Primary and implementation references

Accessed 2026-07-13:

- [Microsoft MS-ERREF: NTSTATUS values](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/596a1078-e883-4972-9bbc-49e60bebca55) — defines `0xC0000142`; does not identify the runtime module for these trials.
- [Microsoft DllMain entry point](https://learn.microsoft.com/en-us/windows/win32/dlls/dllmain) — describes process-attach initialization and termination on failure; does not prove this failure was a particular `DllMain`.
- [Microsoft Process Creation Flags](https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags) — defines `CREATE_SUSPENDED` and debug flags; does not expose module order in the current receipt.
- [Microsoft GetExitCodeProcess](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodeprocess) — defines the termination-status observation and its ambiguity.
- [Microsoft PE/COFF format](https://learn.microsoft.com/en-us/windows/win32/debug/pe-format) — specifies imports, delay imports, TLS callbacks, and load configuration; explicitly notes the specification is not guaranteed complete in all respects.
- [Microsoft Launch an AppContainer](https://learn.microsoft.com/en-us/windows/win32/secauthz/implementing-an-appcontainer) — defines LPAC resource/capability behavior; does not prove a denied registry access occurred here.
- [Microsoft MiniDumpWriteDump](https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/nf-minidumpapiset-minidumpwritedump) and [WER LocalDumps](https://learn.microsoft.com/en-us/windows/win32/wer/collecting-user-mode-dumps) — establish access, artifact, registry, and execution costs that keep dump capture outside this package.
- [Chromium Windows sandbox design](https://chromium.googlesource.com/chromium/src/+/refs/heads/main/docs/design/sandbox.md) — practitioner evidence on LPAC ACL/capability needs; not evidence of this canary's missing resource.
