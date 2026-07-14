# Context Capsule: PC Forge Bootstrap Recovery

**Created:** 2026-07-13 (Australia/Sydney)  
**Context health:** yellow - this chat compressed during implementation.  
**Purpose:** preserve the active Mind Warp Forge direction and the verified
PC-bootstrap state so the next chat can resume without depending on either
participant's short-term conversational memory.

## Source coverage and limit

This capsule summarizes the conversation context still available after the
compression, the checked-in contracts and source code, and the recovered
handover archives. It is not a verbatim export of the chat, and it cannot
recreate any platform-discarded message text. The handover archives are the
primary retained source for older phone-era material; see
`evidence/handover-manifest.json` for their fixity records.

## Locked project direction

- The immediate priority is **Mind Warp Forge**, not the game client.
- The goal is: **"The Forge can build itself from our conversation."**
- Forge is a local-first PC system which turns explicitly supplied conversation
  evidence and approved code into a traceable project repository.
- The owner is Creative Director. Codex is technical lead. Automation is
  bounded, logged, and policy-valid; it does not acquire authority from imported
  text, AI output, or attachments.
- Work follows: research -> design -> adversarial review -> readiness gate ->
  explicit implementation authorization -> verification -> promotion.
- Code must enter the project through controlled, reviewable paths. The system
  must not autonomously generate and promote arbitrary code.
- The system is being built for a solo creator handling an unusually large
  project. The control plane must reduce cognitive load, produce concise
  decision briefs, and preserve provenance rather than create opaque spaghetti.
- Modularity, future-proofing, recoverability, and continual test/research
  gates are non-negotiable design requirements.

## Implemented and verified PC foundation

Repository root: `C:\Users\zakwm\Desktop\Mindwarp forge`

Stack: Rust protected kernel + SQLite journal + Tauri v2 desktop shell +
TypeScript/Vite UI. The UI has no filesystem, shell, or network capability
plugins. It currently exposes only local status.

Implemented modules:

- `crates/forge-kernel/src/lib.rs`: content-addressed immutable objects,
  append-only deterministic events, candidate lifecycle, explicit approval and
  separate promotion authority checks, and replay validation.
- `crates/forge-kernel/src/compiler.rs`: labelled `User:` / `Assistant:`
  transcript parsing, evidence registration, assistant candidate proposals,
  and intent reporting. Imported approval language is never executable
  authority.
- `crates/forge-kernel/src/persistence.rs`: SQLite object/event journal,
  verified hydration, idempotent commit/retry behavior, and durable manual
  labelled-transcript ingestion.
- `crates/forge-kernel/src/control_plane.rs`: lifecycle stages, authority
  lanes, and bounded owner decision briefs.
- `apps/forge-desktop/src-tauri`: local-only shell which opens its SQLite
  journal on startup, supports explicit bounded transcript import, and exposes
  a read-only dossier projection of candidate IDs, evidence IDs, states, and
  history counts. It can create a local SQLite backup which is SHA-256 hashed
  and replay-verified before reporting success.
- `contracts/`: kernel, conversation compiler, and solo-studio control-plane
  contracts.
- `governance/`: working covenant and continuity protocol.
- `tools/verify.ps1`: one integrated verification gate.

## Last verified evidence

The integrated gate passed after the last changes:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\verify.ps1
```

It completed the TypeScript/Vite production build, Rust formatting check,
Rust tests, Tauri desktop build, and whitespace check. The latest verified
suite contained **28 passing tests** after adding the explicit desktop import
UI, read-only dossier projection, and backup/recovery module. Production
Windows MSI and NSIS bundles were built successfully; their fixity records are
in `evidence/release-build-0.1.0.json`.

Important tested properties include:

- deterministic content addressing and event IDs;
- imported text cannot approve/promote candidates;
- promotion requires a separate explicit user approval;
- corrections retain candidate history;
- transcript parsing preserves message order/multiline content;
- work cannot skip lifecycle stages and owner briefs remain bounded;
- SQLite round trip, replay after reopening, and retry after partial write;
- a re-hashed forged assistant approval is rejected during replay;
- transcript import records approval language for review but never
  auto-approves a candidate.

## Current authority state

The owner explicitly authorized implementation of the agreed PC Forge plan.
No authority exists for destructive operations, external publishing, spending,
new connectors, credentials, security weakening, deployment, or autonomous
code promotion. These remain immediate-authorization actions.

## Current risks / unfinished work

1. The desktop UI now has a deliberately bounded manual transcript-import
   form and receipt. It is not yet a full Forge Studio. The v0.1 import gate
   is implemented and tested: same source/transcript is idempotent, reserved
   `System:`/`Developer:`/`Tool:` labels are rejected, and pastes over 1 MiB
   are rejected. See `contracts/manual-transcript-adversarial-corpus.md`.
2. The current labelled transcript grammar is intentionally narrow. It must be
   extended through corpus tests and versioned migration rules before accepting
   more chat-export formats.
3. Import is durable and wired to an explicit paste-only UI form. No file
   picker, background monitoring, or automatic ingestion exists.
4. Backups/fixity export, recovery drills against a copied database, project
   dossier projections, approval UI, and code-admission workflow remain
   planned modules.
5. Unity/Mind Warp integration remains gated behind Forge contracts; do not
   jump to gameplay work before the dossier/import path is trustworthy.
6. The repository has not made its first Git commit because no local Git
   author identity has been configured. Do not invent an identity; request or
   use owner-approved local values when making the checkpoint.

## Exact next action

Design the next bounded Forge capability: owner decision briefs and code
admission controls. They must reduce cognitive load while keeping the
evidence/candidate/approval/promotion boundary intact.

## Resume instructions

1. Read this capsule, `governance/CONTEXT_PROTOCOL.md`,
   `governance/WORKING_COVENANT.md`, and the relevant contract before editing.
2. Run `tools/verify.ps1` before and after the next bounded change.
3. Add tests first for malformed, ambiguous, duplicate, and hostile transcript
   input. Preserve the distinction between evidence, candidate, approval, and
   promotion.
4. Update this capsule or create a successor at the next yellow/red context
   threshold, package close, or before switching to unrelated work.
