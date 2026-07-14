# Reference Intake Contract v1

Reference intake is a capability-free evidence boundary. It accepts typed
metadata supplied by a separate inspection tool and cannot read files, open
archives, access a network, execute content, grant authority, select a
baseline, set an acceptance threshold, approve an artifact, or promote a
candidate.

Every `ReferenceTarget` binds a safe locator, exact lowercase SHA-256, byte
length, provenance class, content class, authority class, executable-content
flag, permitted uses, forbidden uses, claims with bases and limitations, and
target-level limitations. Recovered reports are `evidence_only`; their claims
remain `declared_legacy_claim` and cannot masquerade as deterministic
observations. Forge-owned synthetic fixtures may retain deterministic structural
claims, but still cannot imply perceptual approval or production suitability.

Every target must forbid canonical-baseline use, production import, runtime
execution, perceptual approval, and numeric acceptance thresholds. Rooted,
drive-prefixed, UNC, traversal, network, markup-shaped, empty, executable, and
unknown-version inputs fail closed.

A `ReferenceSuite` requires at least one recovered target and one Forge-owned
synthetic target. Target IDs and content hashes are unique. Byte-identical
copies in another archive do not create independent evidence. The suite has a
plain selection rule but no weights, aggregate score, similarity threshold, or
promotion effect. Canonical JSON bytes and the resulting suite fingerprint are
deterministic.

The initial H1 suite deliberately contains only:

1. the recovered `one_button_humanoid_blueprint.json` as a declared structural
   challenge with no asset, quality, timing, or generator authority; and
2. Forge's deterministic v3 typed T-pose scene as a structural test fixture,
   not an approved humanoid baseline.

The vertical-slice result and category table remain retained source evidence
outside the minimal suite. They may be used to construct adversarial tests but
cannot establish quality, category architecture, timings, automation success,
or engine readiness.
