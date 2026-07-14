# P7b-1b LPAC Denial-Canary Trial 2 Result

**Result:** failed safely before canary resume; cleanup independently verified;
no retry authorized and no denial pass.

## Bound result

- Run ID: `c6a42d8af4bf79d860f95a7f8e101d22`
- Profile moniker:
  `MindwarpForge.P7b1b.c6a42d8af4bf79d860f95a7f8e101d22`
- Receipt: `evidence/p7b1b/trial-2-owner-authorized.json`
- Receipt SHA-256:
  `86653b5573ea54eedc332e385cc9f63a6681c567c47922c27dbf2b7d66d925cf`
- Runner exit: `1`
- Earliest failure: `image-load mitigations missing`
- Cleanup: `cleanup_ok=true`; zero cleanup errors.

Trial 2 was invoked exactly once after the offline build, exact medium-integrity
host check, zero-residue/loopback check, PE inspection, four runner tests,
focused P7 checks, and full Forge gate passed. The failure occurred inside
`verify_suspended` before `ResumeThread`. No sentinel read, child-spawn,
loopback-connect, or report-write probe ran, so Trial 2 provides no denial pass
and no arbitrary-code containment claim.

## Independent cleanup and immutability

After runner exit, the host independently found:

- zero temporary paths containing the exact run ID;
- zero `containment-*` processes;
- zero AppContainer loopback-exemption SIDs;
- zero registry mapping hits for the exact moniker;
- zero package-folder hits for the exact run ID;
- unchanged canary hash
  `8e3231eadf0a9b6768f4fe14616d1f261eca4d6d0044ab8851b51c98883238d5`;
- unchanged runner hash
  `37d8117a282f90d3aff29a3cd7dded649e28db7fbc322ced2997090cb349e1f8`;
  and
- a parseable bounded receipt with the hash above.

No installation, elevation, external network, real credential/user content,
repository access by the canary, renderer, parser, asset, image, GPU,
engine/runtime, publishing, spending, promotion, or protected-Kernel mutation
occurred.

## Root cause and prospective repair

The runner requested the three image-load creation policies for no remote
images, no low-mandatory-label images, and preferring System32 images. Microsoft
documents those as the first three one-bit fields of
`PROCESS_MITIGATION_IMAGE_LOAD_POLICY`, so their required runtime mask is
`0x1 | 0x2 | 0x4 = 0x7`.

The Trial 2 verifier incorrectly required mask `0xb` (`0x1 | 0x2 | 0x8`). Bit
`0x8` is the audit field, while `0xb` omits the required
`PreferSystem32Images` bit `0x4`. The check therefore could not accept the
correct requested three-bit value. The Trial 2 receipt did not record the observed flag word,
so it is not retroactively assigned a value; the source and
public structure layout nevertheless identify the verifier-mask defect.

The prospective code now centralizes the required mask as `0x7`, emits the
observed and required masks on failure, and has a fifth unit test proving that
`0x7`/`0xf` pass while `0x3` and the old `0xb` fail. This repair was compiled
and tested only. It was not executed against another LPAC process and does not
convert Trial 2 into a pass.

## Authority and next gate

The one Trial 2 authorization was consumed. There is no automatic retry or
weaker fallback. Because two authorized trials have now failed before resume,
any future attempt requires a new owner decision after retaining this second
failure, the exact mask regression, full preflight, a fresh identity/receipt,
and the same cleanup/no-retry rules. P7b-1b denial behavior and P7b-1c renderer
compatibility remain unproved.

## Primary references

- [PROCESS_MITIGATION_IMAGE_LOAD_POLICY](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_image_load_policy)
- [GetProcessMitigationPolicy](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessmitigationpolicy)
- [UpdateProcThreadAttribute](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute)
