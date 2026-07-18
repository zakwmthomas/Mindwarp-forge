# G1 GP2 progression result

Status: bounded engine-neutral implementation independently accepted and
registered-complete verified.

The gameplay foundation now contains a private registry of exactly eighteen
session/outcome progression rules (4/3/4/4/3 across S1-S5). Each application
requires an authored-fixture `BaseLoopStateV1`, strict replay round-trip,
byte-exact canonical session record, exact prior `BaseLoopLedgerV1` history,
and domain-separated terminal, session, registry, and ledger digests. Each
receipt retains canonical terminal-state bytes so decode can strict-replay the
source and reconstruct every historical lane emission, digest, order, and
final external history. The caller cannot provide or alter the rule registry.

The ledger keeps knowledge, access, relationship events, construction,
capabilities, and named unique assets as distinct typed records. Knowledge is
not projected into access; mutations are not generically projected into
construction. Services are fulfilled and inactive. Only the named S3 and S5
rights remain active with open obligations. S1 capabilities require the exact
successful outcome and exact allowlisted tool. Recovery and caller-authored
failure costs remain attempt-local.
Fixture-owned outcome costs persist as exact typed liabilities. Caller-authored
failure costs do not enter the ledger.

There are zero conversion and zero reset rules. Three deterministic strategies
each cover S1-S5, validate the exact admitted S1-to-S5 predecessor, and derive
exact affordances, reachable decisions, meaningful rule-owned comparison
categories, and fixture-owned liabilities from private rules and fixed
outcomes. Formal dominance requires an
affordance and decision superset plus a liability subset with at least one
strict relation; the three exact results are pairwise incomparable.

The focused proof covers strict codec/history binding, idempotency, fabricated
source rejection, non-authored context rejection, wrong tools, retreat,
attempt-local recovery, exact rule coverage, exact strategy sets, and absence
of magnitude, spending, currency, XP, or level surfaces. This result grants no
runtime, persistence, network, monetization, Greenfield, C3B, GP3, or GP4
authority.

## Closure evidence

Focused verification passes seven GP2 tests, seven retained GP1 tests and
fourteen retained GP0 tests. Independent review accepted the result with no
remaining blocker. Registered complete run
`run-2dc3db644adc416a8ef56461dbb771b6` passed in 547129 ms.

Two earlier registered attempts failed closed on continuity-verifier drift:
`run-d9c82bd7f6944457a5dd92f520b323c9` exposed a missing exact GP2
implementation route, and `run-0a84d622a10f42e0ba50467cb0838b51`
exposed a stale GP1 sentence token after GP2 legitimately updated the shared
contract. Both narrow verifiers were repaired and focused-tested. Exact-token
continuity assertions remain recorded engineering debt; future verifiers
should prefer stage-aware invariant assertions.
