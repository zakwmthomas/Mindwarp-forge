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
| Blender Human Base Meshes v1.4.1 embedded previews | Official Blender download page identifies the Blender Studio/community bundle as CC0. The 50,643,039-byte archive from `https://download.blender.org/demo/asset-bundles/human-base-meshes/human-base-meshes-bundle-v1.4.1.zip` has SHA-256 `811f43accbb31a88266d932f8f5563b2d13586fca0ba2693aad1f5fe582b3515`. Its realistic and stylized male/female PNG previews were extracted to disposable storage and inspected at original scale. | Each body preview is only 256x256, single-view, and the subject is too small for reliable face, hand, foot, or subtle-anatomy judgment. The realistic pair is a credible source lead; the stylized pair has enlarged heads, eyes, hands, and feet beyond the mature target. | **Rejected** as the H5 baseline; source family retained. |
| High-resolution renders of exact Blender CC0 body assets | Wikimedia file records identify Blender Studio and the named artists, link back to Blender's bundle, and retain CC0. Disposable 1280x960 renders inspected: female realistic SHA-256 `009b3cbc1f5885ed66dd966306815effaaf529f33b4309060f2c7fee34a8441c`; male realistic `b0c0383e49d6db5f1bdfaF993d441ed58350ba67f7a361eb6c802d0dc3bfe04b`; female stylized `963fc7422d06cbd0a4a5e2a1151cd78fb6ea599c02c76dfcc4194b0177ac4ed4`; male stylized `534496a66eeb2207ed1b5d4b6a01571df8e8b0e69ca37a43029ebcc0a08a4be6`. | The larger pixels make overall form, hands, and feet visible, but only one elevated three-quarter view is rendered. Back and true side silhouettes are absent, and visible hip/thigh mesh transitions interrupt anatomical comparison. The stylized bodies are deliberately more caricatured than the owner target. | **Rejected** as the complete H5 baseline; realistic bodies retained only as bounded anatomical-direction evidence. |
| Generated mature-adult front direction v1 | Forge-requested generated preview retained at `evidence/visual-reference-intake/h5-mature-adult-front-direction-v1.png`; 1024x1536; SHA-256 `675e0d8c25d807501a588fd8a7567cc6931a94d95ebd8bd46d56a7f2af1ccbe2`; actual pixels inspected at original scale and on the phone-oriented single-subject layout | One coherent, clearly adult woman fills the frame in a neutral front A-pose. Head, eyes, hands, feet, and silhouette are legible. Its mature proportions remain useful direction evidence, but hair and training garments obscure and prematurely commit construction layers. It has no side/back view and proves no topology, shader, or cost. | **Rejected as the neutral construction baseline; retained as narrow mature-proportion evidence.** |

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

Blender Studio's Human Base Meshes bundle is the stronger second source family:
the official bundle is CC0, coherent, full-bodied, and includes separate
realistic and stylized assets. Its embedded previews and the available
high-resolution renders still fail H5 because they do not provide complete
comparison views and expose visible transition artifacts. The source screen is
therefore stopped after these two authoritative families; adding more sites
would currently cost more than the expected information gain. The archive was
inspected as data in disposable storage. Its `.blend` file was not executed.

## Current creative direction

The owner resolved the form target on 2026-07-14: semi-realistic anatomy and
proportions with mature, economical forms and materials, avoiding childlike
caricature. Shading is a replaceable presentation layer rather than a fixed
organism-form requirement. A restrained toon/cel-like profile may provide
cohesion, but different planets may select different shader profiles to create
distinct local feeling. Forge-authored form, material, composition, and quality
rules provide the broader analogous family resemblance. The same canonical
organism must support presentation tiers from constrained phone hardware
through high-end PC.

The owner then clarified the neutral construction target: bald, unclothed, and
free of baked genital anatomy; female bodies have no nipple surface detail.
Hair, grooming, garments, equipment, sex-specific surface details where a later
design actually needs them, materials, and world shaders are modular derived
layers. The baseline uses a continuous non-sexual mannequin surface so it can
support many characters without hair or clothing hiding comparison anatomy.

No candidate is yet `verified_fit`. A bounded generated comparison may now
clarify *how much* geometric simplification and graphic shading fits this
direction. Generated imagery remains visual-direction evidence only; it cannot
prove topology, shader implementation, animation, growth, or device cost.

The first large adult fixture proved phone-legible framing and a useful mature
proportion direction, but its hair and garments fail the newly clarified
neutral-base requirement. Shading is not part of the form gate. The next front
fixture must use the featureless mannequin surface; H5 then needs separately
legible side/back evidence. Generated view variants must preserve identity and
proportions or be rejected rather than averaged into a fictitious human.

The first generated three-panel fidelity sheet (SHA-256
`049b8dc095fe8d92834a89aa4388d4f672da9ee246d7947a27c4308e7494ff52`,
1717x916) was rejected after owner inspection on a phone: the three treatments
read as effectively the same at that display scale. Future comparisons must
show one large subject or stage per image and deliberately isolate material,
silhouette, age, or fidelity changes. A desktop-legible contact sheet is not a
valid phone review fixture merely because its source resolution is high.
