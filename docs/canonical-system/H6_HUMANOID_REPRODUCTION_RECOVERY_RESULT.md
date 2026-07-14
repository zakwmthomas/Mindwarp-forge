# H6 humanoid reproduction and recovery result

Status: **verified engine-neutral recovery proof**. The complete Forge gate
passes.

H6 closes the continuity gap across the exact H1-H5 humanoid proof chain. The
implementation reuses the existing H1-H4 typed constructors, adds one strict
H5 owner-decision receipt, and binds all five ordered stages into a canonical
manifest without acquiring an asset or selecting a runtime.

Stable identities:

- H5 decision receipt:
  `5c4eb3041ced04e1c1a5cd0e011babafe1826a4d8caf420bf267c8bff0617520`;
- H1-H5 aggregate manifest:
  `a0eb0796a4a0edd800fcd937049eaa3ed7c65e695531daf0f754e835591ada2d`;
- approved visual-content fingerprint retained inside H5:
  `f3242d18b962103ec0b78fe424baf01db60c84d2c2a6f468c85d27818c145051`.

Five in-memory cases prove byte-identical clean rebuild and replay, published
H1-H4 fingerprints, strict H5 content and authority, nine hostile
mutation/truncation paths, and known-good recovery. A separate integration
harness stores all five stage receipts, the H5 decision, and the aggregate
manifest as seven content-addressed Kernel evidence objects. Its exact
authority-negative ProofReceipt and bytes survive verified SQLite online
backup plus live and backup reopen.

The first full-gate attempt correctly rejected an architectural mistake: a
test-only dependency made the protected `forge-kernel` package appear to depend
on the higher H6 proof module. The persistence test moved to the separate
`humanoid-proof-chain-integration` harness. The repeated modularity gate then
verified 21 modules with no forbidden imports or dependency cycles, and the
unchanged full Forge suite passed in 74.5 seconds.

Recovery never repairs hostile bytes in place and never trusts generated
projections as authority. Approval, promotion, asset import, runtime selection,
execution, and protected-Kernel mutation remain prohibited effects. H6 proves
evidence continuity only; production topology, rigging, deformation, shaders,
growth presentation, phone budgets, and the H7 promotion decision remain open.
