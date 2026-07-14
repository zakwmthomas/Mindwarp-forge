# P7b-1b Repaired Observer Validation Result

## Exact result

The owner selected the one repaired-observer validation. After the required
locked/offline build and preflight, it ran exactly once as run ID
`c09a15ae38d7bbfac9a7f86cd14b5c40` and produced a complete, automatically
cleaned diagnostic trace.

- Receipt: `evidence/p7b1b/repaired-observer-validation.json`
- Receipt SHA-256:
  `36af2a8ec422cbcf0cced35d61b560f4fce518773f50534b9a472337e274c3b8`
- Executed runner SHA-256:
  `7cbf42965342d085721c94db3eff946a6ef73106a9af49034fecc1639879bc94`
- Exact static candidate SHA-256:
  `25109e5e9c4de262b4dc4020f2b75b6ba6515c3c6570fad575e131d7bf820856`
- Status: `diagnostic_completed`; `cleanup_ok=true`; event count `7`.
- Terminal status: `0xC0000142` (`3221225794`) after
  `access-check-after-class46-error87` suspended-host verification.

`diagnostic_completed` means only that the bounded event trace reached an exit
event and cleanup succeeded. It is not a containment pass.

## Ordered retained events

The receipt retains this exact order:

1. `create_process` for the exact staged static candidate;
2. `load_dll` for `C:\Windows\System32\ntdll.dll`;
3. `load_dll` for `C:\Windows\System32\kernel32.dll`;
4. `load_dll` for `C:\Windows\System32\KernelBase.dll`;
5. two `unload_dll` events without file-path evidence;
6. `exit_process` with `0xC0000142`.

The trace validates the repaired resume-before-wait ordering and observes three
core system DLLs, but it does not identify a failing DLL, denied object,
exception, registry key, or runtime cause. The two unload events carry no path
and cannot be attributed. The receipt therefore retains
`runtime_cause_proved=false` and `denial_proved=false`.

## Preflight and independent cleanup

Before execution, the runner was rebuilt offline in an isolated target,
the exact candidate hash was retained, all eight runner tests passed, three
fixed-command negative fixtures rejected, the focused P7 and full Forge gates
passed, medium integrity was confirmed as `S-1-16-8192`, and process, temp,
package, profile-mapping, and Forge loopback inventories were all zero.

The runner reported `cleanup_ok=true` with no cleanup errors. Independent
post-run inspection found zero containment processes, exact or other owned
stage directories, package folders, profile mappings, and Forge loopback
exemptions. No manual cleanup was needed. The separate cleanup receipt is
`evidence/p7b1b/repaired-observer-independent-cleanup.json`.

## Terminal claim and authority boundary

The P10 stop rule has fired. The repaired validation succeeded as an observer
validation but did not identify a runtime cause or prove any denial. This
diagnostic family is terminal: no retry, external debugger, dump, trace
provider, different canary, capability addition, weakened containment, or
further static build permutation follows.

P7b-1b remains blocked and unproved. P7b-1c, F5 completion, G1, and R1 remain
gated. No renderer, engine, publishing, spending, credential, promotion, or
protected-Kernel authority was exercised or granted.
