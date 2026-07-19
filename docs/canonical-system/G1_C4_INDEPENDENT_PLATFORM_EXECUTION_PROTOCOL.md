# G1 C4 independent platform execution protocol

Status: **readiness implementation; no independent result imported**.

This protocol closes only the remaining C4 portability obligation. It grants
no promotion, C5 activation, repository mutation, runtime, storage, network or
publishing authority.

## Trust boundary

The existing local observation remains Windows-only and permanently records
`independent_second_platform_execution=false`. A foreign JSON file or changed
boolean cannot promote C4. Independence is derived locally only after all of
these pass:

1. a fresh 256-bit challenge binds one request to the clean source commit;
2. the complete tracked-tree digest, bounded path/blob manifest, retained
   fixture manifest and retained fixture lock match;
3. a GitHub-hosted, non-self-hosted Linux or macOS runner builds with
   `--locked`, then runs the absolute native executable twice with distinct
   overlapping launches;
4. raw stdout and stderr bytes, executable hashes, toolchain, runtime target,
   workflow identity and authority-negative claims validate exactly. Cargo is
   offline during proof execution; the execution job has no OIDC or attestation
   permission, and signing occurs only in a separate dependent job;
5. `gh attestation verify` authenticates the result, repository, source digest,
   workflow provenance and hosted-runner boundary.

Compile-only, emulated, same-Windows-platform, self-hosted, self-asserted,
unattested, replayed, dirty-source, changed-lock, changed-workflow, changed
artifact, single-launch, stdout-drift or authority-positive evidence fails
closed. Failed evidence is never relabelled as an independent receipt.

## Bounded workflow

After the proof/tooling source commit and refreshed local observation are
cleanly retained:

1. Generate a request outside the repository:
   `C:\Users\zakwm\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe tools/generate-g1-c4-external-request.py --github-repository <owner/repository> --output <request.json>`
   on this Windows host, or `python3` on Linux/macOS.
2. Base64 the request and dispatch
   `.github/workflows/g1-c4-independent-platform.yml` at that exact source
   commit. The job has read-only repository permission plus only the OIDC and
   attestation permissions required for its result.
3. Download the request, result and attestation bundle through GitHub's
   authenticated artifact channel.
4. Run the same bundled Windows Python with
   `tools/verify-g1-c4-external-receipt.py --request <request.json>
   --result <result.json> --bundle <bundle.jsonl>`. The canonical receipt path
   is fixed as
   `docs/canonical-system/G1_C4_INDEPENDENT_PLATFORM_EXECUTION.json`. It retains
   the exact request bytes, signed result bytes and attestation bundle in one
   self-hashed record, so an identical result cannot be imported under multiple
   names or reduced to an unreplayable summary.
5. Only that `independence_verified` receipt permits the registered measured
   full gate. While the checkpoint is exactly
   `c4-independent-platform-gate`, the full gate decodes the retained bytes
   before Atlas or other expensive work, revalidates the exact
   semantic/source/authority package and reruns `gh attestation verify` against
   the retained offline bundle with the frozen repository, source digest,
   signer workflow and hosted-runner policy. Missing GitHub CLI, missing
   evidence, tampering or failed replay stops this gate; unrelated substages do
   not acquire a permanent GitHub CLI dependency. The full gate, not the
   foreign result, is the remaining prerequisite for C4 promotion. C5 requires
   a separate activation transition.

The repository currently has no Git remote. Therefore this readiness package
can be fully tested locally, but actual independent execution remains an
external prerequisite until a repository and hosted runner are connected.
GitHub attestations work for public repositories on current plans; private or
internal repository attestations require GitHub Enterprise Cloud. Repository
visibility and plan eligibility must therefore be confirmed before dispatch.

## Primary references

- GitHub CLI `gh attestation verify` manual:
  <https://cli.github.com/manual/gh_attestation_verify>
- GitHub artifact-attestation security model:
  <https://docs.github.com/en/actions/concepts/security/artifact-attestations>
- GitHub's attestation action, output bundle and plan boundary:
  <https://github.com/actions/attest>
- GitHub offline attestation verification:
  <https://docs.github.com/en/enterprise-cloud@latest/actions/how-tos/secure-your-work/use-artifact-attestations/verify-attestations-offline>

These references establish signature, signer-workflow, source-digest,
hosted-runner and offline-bundle verification capabilities. They do not prove
the semantic result; the local importer independently validates that content.
