# G1 C3 Same-Platform Second-Language Field Receipt Result

Date: 2026-07-15

## Result

The canonical field-basis v1 lane now has a permanent same-platform,
second-language determinism receipt. Rust emits a committed JSON vector set and
an independent Python standard-library implementation reproduces:

- the Random123-compatible Philox4x32-10 zero vector;
- one Forge-mapped key/counter vector;
- the strict canonical CBOR bytes for the fixed receipt recipe;
- the recipe fingerprint and reconstruction/domain-bound cache key; and
- four Q32.32 coordinate samples, including negative and exact-cell inputs,
  with Q16.48 ties-to-even arithmetic.

The F5 field-basis gate first requires the live Rust emitter to match the
committed fixture, then launches Python as a separate process and requires its
independent implementation to match the same bytes and integers. A drift in
the Rust mapping, fixture, hash binding, fixed-point rules or Python oracle now
fails the retained gate.

## Root-cause protection

The verifier resolves the bundled Codex Python runtime directly before trying
an external `python3`. This avoids accepting the Windows Store `python` alias,
which can exist as a command but fail when launched. A missing real interpreter
fails explicitly rather than silently skipping the cross-language proof.

The receipt fixture itself must retain all three limitations:

- `same Windows host`;
- `not a second-platform receipt`; and
- `not reference_proven`.

The field-basis module remains `prototype_tested`. This package supplies a
second-language and fresh-process implementation check only; the distinct
second-platform, scientific-validation and production-performance risks remain
open.

## Independent-platform availability audit

On 2026-07-15 a read-only host audit looked for an already-installed lane that
could supply the still-required independent second-platform replay without
installation or machine configuration.

- `wsl.exe` is present only as the Windows system stub. `wsl.exe --status` and
  `wsl.exe --list --verbose` both report that Windows Subsystem for Linux is not
  installed, so no Linux distribution or interpreter exists behind that
  command.
- Command discovery found no Docker, Podman, nerdctl, Multipass, VirtualBox,
  VMware, QEMU, Wasmtime, Android emulator or other candidate runner.
- Installed-product, service and conventional-install-directory inspection
  found no matching container, VM, Linux or emulator platform. The Hyper-V
  `Get-VM` command was also absent.

Therefore no qualifying independent platform was available and no replay was
attempted. Installing WSL, a container engine, VM, emulator or interpreter was
outside the audit authority and was not performed. Command presence alone is
not counted as platform evidence. The second-platform and `reference_proven`
gaps remain open; this is a concrete local-availability blocker, not a failed
determinism result.

## Visibility and traversability selection boundary

`G1_C3_VISIBILITY_TRAVERSABILITY_CONSUMER_AUDIT.md` records why this portability
risk was selected next. Current consumers do not yet expose the path geometry,
terrain continuity, body envelope or locomotion constraints needed to define
physical visibility and traversability honestly. Those C3 obligations remain
open; no universal distance or access scalar was added.

## Verification

- `cargo test -p field-basis`
- `cargo run --quiet -p field-basis --example second_language_vectors`
- bundled Python `tools/verify-field-basis-second-language.py`
- `tools/verify-f5-field-basis-readiness.ps1`
- module-context and canonical-system gates
- the complete repository gate through all workspace tests and UI build; its
  ordinary final desktop build reached the known live-executable Windows lock
- an isolated `CARGO_TARGET_DIR` desktop build with `RUSTFLAGS=-D warnings`
  passes

No promotion, runtime selection, cross-platform equivalence, visibility,
traversability, biome or scientific-sufficiency claim is made.
