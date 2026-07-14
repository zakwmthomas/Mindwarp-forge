# Mind Warp Forge

Local-first PC Forge for preserving Mind Warp project truth, compiling approved conversation evidence, and running bounded studio workflows.

## Current implementation increment

The repository contains a tested Rust Kernel, SQLite persistence and recovery,
local conversation capture, generated context projections, bounded proof
harnesses, and a Tauri desktop interface. Runtime/game-engine adapters remain
separately gated and have not been selected.

The recovered handover material remains untouched in `forge documents from gpt handover/` and is treated as source evidence, not production code.

## Verification

Run `powershell -NoProfile -ExecutionPolicy Bypass -File tools\verify.ps1` from the repository root. The verifier builds the TypeScript UI, checks Rust formatting, runs the Rust workspace tests, builds the Tauri desktop shell, and checks whitespace errors.

## Safety model

- Evidence and events are immutable.
- Assistant/imported content cannot approve or promote work.
- Approval and promotion are separate explicit steps.
- Future UI and tools must call the Kernel command boundary; they cannot write truth directly.
