# G1 C5 independent platform execution protocol

Status: **independent hosted result imported and cryptographically replayed; portability only**.

This protocol proves only the independent-platform portion of C5 portability. It grants no integration, promotion, C5 closure, C6 activation, repository mutation, runtime, cache, storage, network, rendering, AI-generation or publishing authority.

This is not a Forge application port. Forge remains the PC-first factory and verification environment; the game is the product intended to consume portable canonical contracts. Only the isolated, capability-free C5 significance/scheduler proof fixture is compiled or executed on the named targets. No Forge desktop binary, control plane, authoring UI or repository workflow is being made into a game or mobile runtime.

## Trust boundary

The Windows observation permanently distinguishes native x64 execution, same-host i686 execution and Android ARM64 compile-only evidence. It records `independent_second_platform_execution=false`, `promotion_authority=false` and `c6_authority=false`. A foreign JSON result or flipped boolean cannot change those classifications.

Independent evidence is accepted only when all of these pass:

1. a fresh 256-bit challenge binds one request to a clean committed source tree;
2. the tracked-tree digest, self-binding bounded path/blob manifest, fixture manifest, fixture lock and dependency graph match exactly;
3. a GitHub-hosted, non-self-hosted Linux or macOS runner builds the isolated C5 receipt fixture with the pinned toolchain and `--locked --offline`, then directly runs the same absolute executable in two distinct overlapping processes;
4. both executions produce the exact semantic receipt bytes already observed locally, including eight domain rows, ten pressure transcripts, the 92-ID hostile registry and ten authority-negative flags;
5. raw stdout/stderr, executable identity, toolchain, native target, clean-tree state, workflow identity and negative-authority claims validate exactly; and
6. local `gh attestation verify` authenticates the result, repository, exact source digest, signer workflow and hosted-runner boundary from the retained offline bundle.

Compile-only, emulated, same-host, same-platform remote, self-hosted, self-asserted, unattested, stale-challenge, dirty-source, changed-lock, changed-workflow, changed artifact, single-process, non-overlapping, stdout-drift or authority-positive evidence fails closed. Android compilation and Windows i686 execution remain useful observations but never count as the independent platform.

## Credential separation

The execution job receives read-only repository access and no OIDC or attestation permission. It transfers unsigned request/result files to a separate dependent job. Only that second job receives the minimum OIDC and attestation permissions needed to sign the exact result. The result is evidence only and cannot mutate the repository.

## Bounded workflow

After the complete C5 portability tooling surface and local observation are committed cleanly:

1. Generate a challenge-bound request outside the repository with `tools/generate-g1-c5-external-request.py`.
2. Base64 the request and manually dispatch `.github/workflows/g1-c5-independent-platform.yml` at the exact bound commit. GitHub requires a manually dispatched workflow to be registered on the default branch before another ref can be selected.
3. Download the request, result and offline attestation bundle through GitHub's authenticated artifact channel.
4. Import them with `tools/verify-g1-c5-external-receipt.py`. The fixed create-only output is `docs/canonical-system/G1_C5_INDEPENDENT_PLATFORM_EXECUTION.json`, retaining exact request, result and bundle bytes in one self-hashed record.
5. Replay the retained receipt with `tools/verify-g1-c5-independent-platform-result.ps1`. Missing GitHub CLI, missing evidence, altered embedded bytes or failed cryptographic replay must fail the exact `c5-independent-platform-gate`; unrelated substages do not acquire a permanent GitHub CLI dependency.

The public repository registered the workflow on the default branch. GitHub-hosted run `29678602236` executed exact source `9e48dd1` on Linux x86_64, produced two overlapping byte-identical semantic receipts, separated unsigned execution from attestation authority, and was imported only after local cryptographic replay passed. The retained result is `G1_C5_INDEPENDENT_PLATFORM_EXECUTION.json`; it remains evidence-only.

## Remaining gates

Passing this protocol completes neither C5 nor G1. Read-only ProofReceipt integration, dependent regressions, fresh independent review and one registered full Forge gate remain required. C6 is never activated automatically.
