# Forge Startup Duplicate ChatGPT Installer Repair

Date: 2026-07-16

Status: **root cause reproduced; permanent startup boundary repaired and
verified.**

## Failure

The owner reported that restarting Forge downloaded another copy of ChatGPT.
The Downloads directory contained the original `ChatGPT Installer.exe` plus 44
numbered copies before controlled testing. These are downloaded installer
bootstrap executables, not 45 separately installed applications.

Every inspected numbered file carries a `Zone.Identifier` host URL for the
official Microsoft Store web-installer endpoint for product `9PLM9XGG6VKS`.
Chrome's local download ledger records the same direct URL with no referrer.
The installed `OpenAI.Codex` AppX package is singular and reports healthy.

## Reproduction and cause

Two controlled Forge-only restarts each increased the installer count by one.
The first measured `45 -> 46`; the second added one more. Forge startup then
revealed the cause in `apps/forge-desktop/src-tauri/src/main.rs`: one second
after setup it searched the local Codex CLI cache and spawned
`codex app <project_root>`.

On this Windows installation that CLI route navigates Chrome to the Store
web-installer even when the packaged Codex application is already installed.
The capture scanner itself only reads existing local session evidence and does
not require the assistant application to be launched.

## Permanent repair

- Forge no longer launches Codex or any assistant client during setup.
- `ensure-context-current.ps1` runs a fail-closed startup-source verifier before
  it may launch Forge.
- The context gate rejects a Forge executable older than the desktop startup
  source, preventing a repaired source tree from launching stale behavior.
- The verifier rejects assistant CLI, browser-installer and known Store
  download tokens in both Forge startup and its context gate.
- The full Forge gate and focused launch-path fixture retain these shields.

The governing invariant is now: **Forge may launch only Forge.** AI client
lifecycle, installation and updates remain external to Forge.

## Cleanup boundary

The repair does not delete the existing installer files, modify Chrome history,
change browser policy, uninstall or alter the healthy Codex package, or rewrite
the recorded C3 route. Deleting the duplicates is a separate owner-authorized
cleanup action after their provenance and active-use state are known.

## Verification result

- The generated `forge-desktop` module front door was refreshed and its module
  context passed.
- The startup-idempotency and launch-path fixtures pass.
- All 41 `forge-desktop` tests pass.
- The first rebuild attempt exposed the ordinary Windows running-executable
  lock; the old process was closed and the repaired debug binary rebuilt.
- The controlled repaired restart held the installer count at `47 -> 47` and
  retained the same latest file, `ChatGPT Installer (46).exe` created at
  2026-07-16 11:04:03 local time.
- Bootstrap restarted capture successfully from the repaired executable.

Final governance and whitespace receipts are recorded in the active checkpoint.
