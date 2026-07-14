# P7b-1b Trial 1 Failure Analysis and Compatibility Decision

**Decision:** retain Trial 1 as failed and unproved; for any separately
authorized future run, require an independent suspended-token access
discriminator in addition to the class-46 query. Tolerate
`ERROR_INVALID_PARAMETER` only from that one child LPAC query and only when the
access discriminator proves exact LPAC behavior. No trial or canary was launched
for this analysis.

## What Trial 1 can and cannot establish

The original receipt contains the generic error `token u32: The parameter is
incorrect. (os error 87)`. At that revision the four ordered calls were parent
`TokenElevation`, parent `TokenIsAppContainer`, child `TokenIsAppContainer`, and
child `TokenIsLessPrivilegedAppContainer`. The receipt therefore proves only
that one of those calls failed before `ResumeThread`; it cannot identify which
call and must not be reinterpreted after the labels were improved.

The host currently reports 64-bit Windows build `10.0.26200.8655`. The loaded
`advapi32.dll` file reports component version `10.0.26100.3624`. The pinned
`windows-sys 0.61.2` bindings map
`TokenIsLessPrivilegedAppContainer` to information class `46`, matching the
Windows SDK enumeration. The differing OS and component build numbers are an
observable compatibility fact, not evidence that the component caused error
87.

## Documentation correction

Microsoft's public `TOKEN_INFORMATION_CLASS` page defines class 46 and LPAC as
an AppContainer that disregards `ALL_APPLICATION_PACKAGES`, but unlike several
other enum members it does not state the concrete output buffer type for class
46. Therefore the Trial 1 result's earlier sentence that Microsoft documents a
DWORD result was too strong. The four-byte shape is consistent with current
SDK/community descriptions, but changing the buffer shape without a public
contract would be guesswork and is rejected.

## Alternatives reviewed

| Option | Decision | Reason |
|---|---|---|
| Retry the same query or widen its buffer | Reject | Consumes authority and guesses at an undocumented shape without explaining Trial 1. |
| Omit class 46 or accept regular AppContainer | Reject | Permanently weakens the boundary because regular AppContainer can use resources granted to `ALL_APPLICATION_PACKAGES`. |
| Trust only the creation attribute | Reject | Proves host intent, not the suspended token's effective access behavior. |
| Query `WIN://NOALLAPPPKG`/token security attributes | Reject | Chromium explicitly avoids this because the attribute is undocumented and may change; Microsoft's enum marks `TokenSecurityAttributes` reserved. |
| Inspect token group layout | Reject as primary proof | Group presence/attributes are a representation detail and do not directly prove the resulting access decision. |
| Synthetic in-memory `AccessCheck` discriminator | **Select** | Directly observes the defining LPAC behavior on the suspended token, uses documented access-check primitives, mutates no object ACL, and is independently used by Chromium's Windows sandbox tests. |

## Selected fail-closed rule

For a future owner-authorized run, while the child is still suspended:

1. Require `TokenIsAppContainer == 1`, the exact expected AppContainer SID,
   zero capabilities, low integrity, the expected job, and all mitigations as
   before.
2. Query class 46 and retain its exact result/error as evidence.
3. Duplicate the child token for an in-memory `AccessCheck` against a synthetic
   descriptor whose marker rights distinguish `ALL_APPLICATION_PACKAGES`
   (`S-1-15-2-1`) from `ALL_RESTRICTED_APPLICATION_PACKAGES`
   (`S-1-15-2-2`).
4. Require the granted marker mask to be exactly `0x2`. Mask `0x3` is regular
   AppContainer behavior; `0`, `0x1`, an access-check error, or any extra or
   ambiguous result fails closed.
5. Accept class 46 only when it returns `1`, or when it returns exact Windows
   error `87` and the independent discriminator is exactly `0x2`. Any other
   value or error fails closed. A successful class-46 result does not bypass
   the independent discriminator.

This is not a fallback to weaker isolation: it tests the security property that
LPAC is meant to enforce. The creation opt-out attribute remains mandatory and
the class-46 query remains present. The implementation records which of the two
reviewed modes supplied the class-46 side of the evidence:
`class46-and-access-check` or `access-check-after-class46-error87`.

## Capability-free implementation evidence

The runner now contains the prospective in-memory access discriminator and a
pure reconciliation function. Four unit tests pass, including rejection of
regular-AppContainer mask `0x3`, AAP-only mask `0x1`, no access, class-46 false,
and non-87 query errors. The Windows runner compiles against the pinned offline
dependency, and the complete Forge regression gate passes. No runner command,
child process, AppContainer profile, job, ACL
mutation, socket probe, or denial canary was executed.

Trial 2 preflight review additionally found that duplicating the child token
requires the source handle to include `TOKEN_DUPLICATE`. The child handle now
requests exactly `TOKEN_QUERY | TOKEN_DUPLICATE`; the parent remains
query-only. The focused verifier retains this least-additional-right shield.

This compilation proves source/API consistency only. The access discriminator
has not been observed against a new LPAC token on this host, so P7b-1b denial
behavior remains unproved.

## Authority and next gate

The failure-analysis package is complete. A second trial may use this candidate
only after fresh owner authorization, a fresh run identity and receipt, and the
same pre-launch, cleanup, immutability, and no-retry gates. Unexpected class-46
behavior, an access mask other than exact `0x2`, disagreement between the two
signals, or any cleanup residue terminates the attempt without resume or retry.

## Research record

Accessed 2026-07-13:

- [GetTokenInformation](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation) — public query contract and error handling; it does not specialize class 46's buffer type.
- [TOKEN_INFORMATION_CLASS](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-token_information_class) — class-46 identity and the defining LPAC/`ALL_APPLICATION_PACKAGES` behavior.
- [Launch an AppContainer](https://learn.microsoft.com/en-us/windows/win32/secauthz/implementing-an-appcontainer) — Microsoft's imperative LPAC creation pattern and opt-out attribute.
- [AccessCheck](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-accesscheck) — documented access-token/security-descriptor decision primitive.
- [SECURITY_CAPABILITIES](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-security_capabilities) — AppContainer SID and explicit capability-count contract.
- [Chromium LPAC test](https://chromium.googlesource.com/chromium/src/+/refs/tags/133.0.6909.0/sandbox/win/src/app_container_test.cc) — independent production-browser test pattern; uses an access check because the security-attribute alternative is undocumented and change-prone. This is practitioner evidence, not a Microsoft contract.
- [Chromium Windows sandbox design](https://chromium.googlesource.com/chromium/src/+/refs/heads/main/docs/design/sandbox.md) — operational distinction between regular AppContainer and LPAC.
