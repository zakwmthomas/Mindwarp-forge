# H5 visual reference intake

H5 is a fail-closed source and pixel-fitness intake. It does not approve a
humanoid, import a production asset, select an engine, or alter the verified
H2-H4 structural evidence. Dependent visual implementation remains blocked
until an actual human reference has a `verified_fit` receipt and any creative
ambiguity has been resolved by the owner.

## Fitness target

The required baseline must show a coherent human at useful scale, with enough
unobscured body coverage and relevant views to judge silhouette, anatomy,
proportions, articulation landmarks, hands, and feet. Pose, projection,
lighting, clothing, annotations, and crop must not conceal the comparison.
Technical validity, source reputation, licensing, file dimensions, and hashes
are necessary evidence but do not establish visual fitness.

## Pixel-inspection receipts

| Candidate | Provenance and identity | Actual-pixel finding | Disposition |
|---|---|---|---|
| Existing Forge pose capture | Forge-owned `artifacts/chat-screenshots/v2-pose-reference.png`; SHA-256 `c85653f3329d164ea07f0dca334de05879faab4d17e65ac00ab1e9eed764840e`; original 1116x799 capture inspected | Clean Forge UI capture, but the human reference is a tiny front/side/top stick skeleton. It has no surface silhouette, face, hands, musculature, or detailed proportions. | **Rejected** for human-quality comparison; retained only for verified H2-H4 structural proofs. |
| MakeHuman labelled main-window image | Official MakeHuman documentation, `https://static.makehumancommunity.org/makehuman/docs/main_labelled.png`; SHA-256 `80f312ced3cfd22aa29a196b89a479290fb7e57c1c2c753d6ff5d0fd4766d75f`; downloaded to disposable temp storage and inspected at original scale | Plausible rendered upper body in the MakeHuman UI, but clothing and annotations obscure the subject and the lower body, hands, and feet are not available for comparison. | **Rejected** as the H5 baseline. |
| MakeHuman age image | Official MakeHuman documentation, `https://static.makehumancommunity.org/makehuman/docs/age_0.png`; SHA-256 `2b45690e716cf725927222087ac76e05cf68f2e1522e9c5fd7f74334577cf233`; downloaded to disposable temp storage and inspected at original scale | Four head-and-bust variants. No full bodies, neutral articulation view, hands, or feet. | **Rejected** as the H5 baseline. |
| MakeHuman legs-modelling image | Official MakeHuman documentation, `https://static.makehumancommunity.org/makehuman/docs/legs_modelling_2_0.png`; SHA-256 `69cfc6220d7754adef469bf9423bcb9b5c9fbd1f603ae500426f1a88e3bccc8c`; downloaded to disposable temp storage and inspected at original scale | Cropped torso and legs; arms are clipped, face and feet are absent, and a large white censor block obscures anatomy. | **Rejected** as the H5 baseline. |

## Source-family assessment

MakeHuman remains a promising *source family*, not a verified visual asset.
Its official licensing material says the core assets and exported models are
CC0, and its official repository provides traceable project provenance:

- `https://static.makehumancommunity.org/about/license.html`
- `https://static.makehumancommunity.org/makehuman/faq/can_i_sell_models_created_with_makehuman.html`
- `https://github.com/makehumancommunity/makehuman`
- `https://github.com/makehumancommunity/makehuman-assets`

The inspected documentation screenshots do not satisfy the H5 visual target.
Creating a fresh, neutral, full-body multi-view candidate with the external
application would be a later escalation, not a justified substitute for this
cheap source-and-pixel screen. No executable was downloaded or run.

## Current creative direction

The owner resolved the broad target on 2026-07-14: semi-realistic anatomy and
proportions with a mature stylized treatment, economical forms/materials, and
a restrained toon/cel-like shader. It must avoid both photorealistic production
cost and childlike caricature. The same canonical organism must support
presentation tiers from constrained phone hardware through high-end PC.

No candidate is yet `verified_fit`. A bounded generated comparison may now
clarify *how much* geometric simplification and graphic shading fits this
direction. Generated imagery remains visual-direction evidence only; it cannot
prove topology, shader implementation, animation, growth, or device cost.

The first generated three-panel fidelity sheet (SHA-256
`049b8dc095fe8d92834a89aa4388d4f672da9ee246d7947a27c4308e7494ff52`,
1717x916) was rejected after owner inspection on a phone: the three treatments
read as effectively the same at that display scale. Future comparisons must
show one large subject or stage per image and deliberately isolate material,
silhouette, age, or fidelity changes. A desktop-legible contact sheet is not a
valid phone review fixture merely because its source resolution is high.
