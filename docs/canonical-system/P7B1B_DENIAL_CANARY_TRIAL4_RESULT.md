# P7b-1b Static-CRT Denial-Canary Trial 4 Result

**Result:** the fresh static-CRT candidate passed all suspended-host checks but
failed after resume with the same `0xC0000142` initialization status as Trial
3; cleanup independently verified; no retry and no denial pass.

## Bound result

- Run ID: `7fe469dbd720775e3075eba05fdc441f`
- Profile moniker: `MindwarpForge.P7b1b.7fe469dbd720775e3075eba05fdc441f`
- Receipt: `evidence/p7b1b/trial-4-static-crt.json`
- Receipt SHA-256: `b69d8c2467fe19d10ebf67a709d4c574ac23634d07aadf899cc883c6c43ec670`
- Runner exit: `1`
- Exact failure: `post-resume canary exit 0xC0000142 (3221225794) after access-check-after-class46-error87`
- Cleanup: `cleanup_ok=true`; zero cleanup errors.

The one invocation bound static canary SHA-256
`25109e5e9c4de262b4dc4020f2b75b6ba6515c3c6570fad575e131d7bf820856`
and runner SHA-256
`61167968fcaa251309d2140a7b07079dca24a782b2e33820e64e456772bfb350`.
Before invocation, the locked/offline proof reproduced the candidate, six
runner tests passed, all PE security markings remained present, the focused P7
and complete Forge gates passed, the receipt was fresh, the host was medium
integrity, and independent checks found zero owned residue.

The retained LPAC observation proves suspended token, SID, zero-capability,
low-integrity, job, mitigation, discriminator, and staged-hash checks accepted.
No admissible report followed resume, so no denial probe is credited.

## Cleanup and finding

Independent post-run checks found zero `containment-*` processes, exact temp
paths, loopback exemptions, profile mappings, or package folders; the bound
binaries were unchanged and the receipt is parseable.

Trial 4 removed `vcruntime140.dll` and all six observed direct
`api-ms-win-crt-*` imports. The identical failure therefore falsifies the
narrow hypothesis that merely removing those direct imports repairs startup.
It does not identify a DLL, registry key, object, transitive load, or LPAC
condition, and does not prove CRT code itself irrelevant.

The result does **not** justify `registryRead`, a regular-AppContainer fallback,
weaker checks, or another build permutation. No installation, capability,
renderer, engine, publishing, spending, promotion, or protected-Kernel action
occurred.

## Next boundary

Trial 4 is consumed and P7b-1b remains unproved. Next is a capability-free
loader-diagnosis design package evaluating existing local broker-side evidence
without another LPAC launch or installing/enabling a debugger, trace provider,
or external tool. Any later execution needs a new information-gain hypothesis,
separate selection, exact authority, and full preflight; no retry is permitted.
