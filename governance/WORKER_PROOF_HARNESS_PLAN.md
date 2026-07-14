# W1 Worker Proof Harness

The worker cannot be trusted as effective until deterministic fixtures prove:

1. changed policy, batch, metric, and closure records appear in its regenerated feedback brief;
2. stale feedback/source hashes are rejected before package selection;
3. worker selection follows the active master-program dependency graph and skips gated work;
4. a completed package records evidence, verification, metrics, and next action in Forge records;
5. micro-edits, repeated summaries, and partial tests do not count as meaningful batches;
6. unavailable token/time data remains unknown rather than fabricated;
7. three failed optimization audits produce one bounded owner escalation, while a successful correction avoids escalation; and
8. worker/telemetry/inspector failures cannot grant authority or mutate protected Kernel state.

W1 passes only when fixtures are automated, reproducible, and included in the
full Forge verification gate.

## Executable proof map

| Obligation | Executable fixture |
|---|---|
| Policy, batch, metric, and closure feedback propagation | `tools/test-worker-feedback.ps1` |
| Stale feedback rejection | `tools/verify-worker-feedback-freshness.ps1` |
| Dependency selection and gated/stale-route exclusion | `tools/test-worker-selector.ps1` |
| Completion evidence, verification, metrics, transition, and next action | `tools/test-worker-batch-state.ps1` |
| Repeated/no-progress rejection and interrupted resume | `tools/test-worker-progress.ps1`, `tools/test-worker-batch-state.ps1` |
| Unknown cost/time integrity | `tools/test-worker-batch-state.ps1` |
| Three-failure escalation and corrected-path suppression | `tools/test-worker-escalation.ps1` |
| No authority or protected-state grant | `tools/test-worker-batch-state.ps1` |

All fixtures above are invoked by `tools/verify.ps1`; W1 cannot close on a
focused or partial run.

## Durable batch state machine

Heartbeats are wake signals, never progress units. Each batch is one durable
record with states `ready`, `executing`, `verifying`, `recorded`, `complete`,
or `blocked`. A transition requires new evidence: changed artifact, test
receipt, recorded blocker, or verified package result. Interrupted work resumes
from the recorded state and exact next action; a wake-up without a transition is
silent. Fixtures must cover interrupted resume, false-progress rejection, and
empty-wake-up measurement.
