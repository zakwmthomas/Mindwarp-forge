# P7b-1b LPAC Denial-Canary Trial 3 Result

**Result:** suspended-host verification passed, but the canary failed during
process initialization after resume; cleanup independently verified; no retry
and no denial pass.

## Bound result

- Run ID: `e6b479fd2ba785d6315dadd5c3ba1154`
- Profile moniker:
  `MindwarpForge.P7b1b.e6b479fd2ba785d6315dadd5c3ba1154`
- Receipt: `evidence/p7b1b/trial-3-owner-delegated.json`
- Receipt SHA-256:
  `dac8c40284507b0af09de0e7a8434221ed621fe1890e39567f38c49a084e464b`
- Runner exit: `1`
- Earliest retained failure: `unexpected canary exit 3221225794`
- Hexadecimal status: `0xC0000142` (`STATUS_DLL_INIT_FAILED`)
- Cleanup: `cleanup_ok=true`; zero cleanup errors.

Trial 3 was invoked exactly once after exact medium-integrity, zero-residue,
fresh-receipt, offline-build, five-test, PE32+ AppContainer, focused-P7,
worker-state, context-freshness, and full-Forge screening passed. The runner's
control flow reached and returned successfully from `verify_suspended` before
`ResumeThread`, so the corrected image-load check and all retained suspended
token, SID, zero-capability, low-integrity, job, LPAC-discriminator, mitigation,
and immutable-hash checks accepted this process. The failure occurred only
after resume while waiting for the fixed exit.

No report was admitted. The NTSTATUS value means a dynamic-link library failed
initialization and the process terminated abnormally; it does not identify the
DLL or prove why initialization failed. Therefore no sentinel-read,
child-spawn, loopback-connect, or report-write denial is credited and no
arbitrary-code containment claim is made.

## Independent cleanup and immutability

After runner exit, independent host checks found:

- zero temporary paths containing the exact run ID or moniker;
- zero `containment-*` processes;
- zero AppContainer loopback-exemption SID entries;
- zero registry mapping hits for the exact moniker;
- zero package-folder hits for the exact run ID or moniker;
- unchanged canary hash
  `550cec84a6f39d4fc6860cd342f7925a3e7b085f4fe404ff295ca6098e526865`;
- unchanged runner hash
  `5ea636687b99a99c5b66983370f18c1ff472ff4b054337ec85b5d4f9a0ea7be1`;
  and
- a parseable bounded receipt with the hash above.

No installation, elevation, external network, real credential or user content,
repository access by the canary, renderer, parser, asset, image, GPU,
engine/runtime, publishing, spending, promotion, or protected-Kernel mutation
occurred.

## Critical diagnosis

`dumpbin /dependents` shows direct imports from Windows system components plus
the MSVC/UCRT runtime, including `kernel32.dll`, `advapi32.dll`, `ntdll.dll`,
`ws2_32.dll`, `VCRUNTIME140.dll`, and API-set CRT contracts. Independent ACL
inspection found `ALL RESTRICTED APPLICATION PACKAGES` read/execute access on
the corresponding direct System32 DLLs checked, including `VCRUNTIME140.dll`
and `ucrtbase.dll`. This rejects a simple claim that one of those direct files
lacks the LPAC filesystem ACE, but it does not identify an indirect dependency,
initialization-time registry/object access, or another loader condition.

Microsoft states that LPAC needs explicit capabilities for resources a regular
AppContainer can access, and Chromium notes that `registryRead` is important
for registry access while every required file and registry location must have
the right ACL. That does **not** justify adding `registryRead`: Trial 3 produced
no evidence naming a registry key, and granting a capability would change the
zero-capability denial contract. A statically linked or smaller-runtime canary
is a design candidate, not a selected repair; it first needs a capability-free
dependency/import proof and a refreshed failure matrix.

No local loader trace was available from the Application log or Windows Error
Reporting, and no debugger or Process Monitor was installed or enabled. The
exact failing module is therefore unresolved rather than guessed.

## Prospective diagnostic repair

The runner now formats any future post-resume abnormal exit with both
hexadecimal and decimal status and the exact successful LPAC verification mode.
A sixth unit test fixes the expected `0xC0000142` representation. This is
diagnostic source evidence only; it was not used for another LPAC invocation
and does not alter Trial 3's receipt retroactively.

## Authority and next gate

The one Trial 3 invocation is consumed. There is no same-package retry and no
weaker regular-AppContainer or added-capability fallback. The next eligible
work is capability-free startup-compatibility design: compare a smaller/static
canary runtime and richer broker-side loader evidence without executing a new
LPAC process. Any later trial still needs a fresh identity, receipt, full
screening, and an exact one-run package under the owner's standing routine-test
delegation. P7b-1b denial behavior and P7b-1c renderer compatibility remain
unproved.

## Primary and practitioner references

- [MS-ERREF NTSTATUS values](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/596a1078-e883-4972-9bbc-49e60bebca55)
- [Launch an AppContainer](https://learn.microsoft.com/en-us/windows/win32/secauthz/implementing-an-appcontainer)
- [PROCESS_MITIGATION_IMAGE_LOAD_POLICY](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_image_load_policy)
- [Chromium Windows sandbox design](https://chromium.googlesource.com/chromium/src/+/refs/heads/main/docs/design/sandbox.md)
