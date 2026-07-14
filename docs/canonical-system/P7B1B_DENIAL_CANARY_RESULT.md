# P7b-1b LPAC Denial-Canary Trial 1 Result

**Result:** failed safely before canary resume; cleanup independently verified;
no retry authorized.

## Bound result

- Run ID: `196f4519aac9bec367cbfee0cdc70c22`
- Profile moniker reserved for the attempt:
  `MindwarpForge.P7b1b.196f4519aac9bec367cbfee0cdc70c22`
- Admitted receipt: `evidence/p7b1b/trial-1-owner-authorized.json`
- Receipt SHA-256:
  `f5670b94f82ba0728ae57d827250da58d94a4b2937d617bb655a410d2a3b449a`
- Runner exit: `1`
- Earliest recorded failure: `GetTokenInformation` returned
  `ERROR_INVALID_PARAMETER` (`87`) for a DWORD token query during the host
  verification path.
- Cleanup receipt: `cleanup_ok=true`; no cleanup errors.

The runner was invoked exactly once. Its control flow performs every child
token query while the process is suspended and calls `ResumeThread` only after
all token, SID, capability, integrity, job, and mitigation checks pass. The
failure therefore occurred before canary resume. No sentinel read, child-spawn,
loopback-connect, or report-write probe ran, so this trial provides **no denial
pass** and no arbitrary-code containment claim.

## Independent cleanup checks

After the runner exited, the host independently observed:

- zero `mindwarp-forge-lpac-*` staging or sentinel directories under the host
  temporary directory;
- zero running `containment-*` processes;
- zero AppContainer loopback exemptions;
- zero registry mapping hits for the exact moniker;
- zero package-folder hits for the run identity; and
- the bounded receipt remained parseable and content-hashed.

No installation, elevation, firewall/feature/loopback-exemption change,
external network, real credential or user content, repository access by the
canary, renderer, parser, asset, image, GPU, engine/runtime, publishing,
spending, promotion, or protected-Kernel mutation occurred.

## Failure analysis and repair retained

The exact first trial receipt used one generic `token u32` error label, which is
insufficient to prove which of the four ordered queries the host rejected. The
implementation labels parent elevation, parent AppContainer, child
AppContainer, and child LPAC queries separately for any future package. That
repair has not been executed and does not reinterpret this receipt.

Post-trial review corrected an overstatement: Microsoft's public enum page
defines class 46 and LPAC semantics but does not explicitly name class 46's
output buffer type. No buffer-shape guess is accepted. The reviewed prospective
compatibility path retains the class-46 query and additionally requires an
in-memory suspended-token `AccessCheck` that grants exactly the LPAC marker
mask, never the regular-AppContainer mask. See
`P7B1B_DENIAL_CANARY_FAILURE_ANALYSIS.md`.

The trial also retains the pre-launch critical repairs: job completion evidence
must observe exactly one process, and staging DACL restoration must match the
original ACL bytes and protection state. These safeguards passed compilation,
unit, modularity, focused-P7, and pre-launch full-Forge verification; they do
not convert the failed OS trial into a pass.

## Authority and next gate

The original authorization was consumed by this one invocation. There is no
automatic retry, regular-AppContainer fallback, omitted LPAC check, or relaxed
ACL/mitigation path. A future attempt requires:

1. a fresh run identity and receipt path;
2. new explicit owner authorization for a second trial; and
3. the reviewed class-46 plus exact access-discriminator rule, with no weaker
   fallback or automatic retry.

Until then, P7b-1b denial behavior remains unproved and P7b-1c renderer
compatibility remains gated.

## Primary references

- [GetTokenInformation](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)
- [TOKEN_INFORMATION_CLASS](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-token_information_class)
- [Launch an AppContainer](https://learn.microsoft.com/en-us/windows/win32/secauthz/implementing-an-appcontainer)
