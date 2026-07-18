# Mindwarp Signal Anchor vertical contract

This module owns one fixed, runtime-independent GP4 proof bundle. It composes
exact GP0-GP3, C3A and C4V evidence for the Signal Anchor temporary-rescue
route; it does not select, host or promote a runtime.

The public boundary is `SignalAnchorBundleV1`, its strict canonical codec,
`build_signal_anchor_bundle`, and evidence-only inspection. Callers must supply
the exact expected `WorldGenerationInput` and `CausalWorldPacket`. Validation
checks the 8 MiB ceiling before top-level parse, checks every nested cap, then
strictly decodes and replays received dependency evidence before fixed-registry
equality.

The C3A-backed terminal state and authority-lowering authored shadow must match
all thirteen `BaseLoopStateV1` fields except `world_context`, including both
ledger byte sets. C4V is exactly four parent-bound batches containing five GP1
actions, with verified revision-three and revision-four restarts. GP2 consumes
only the authored shadow. Its six ordered emissions and six ordered world
transitions are exact, and the selected threat never enters progression.

The twenty-five semantic slots are neutral inspectable facts. The twenty-nine
adapter requirements remain typed `Unmeasured`. Neither collection proves
visual quality, accessibility conformance, performance, engine suitability or
runtime containment.

Filesystem, network, process, Kernel, Companion, Greenfield, broad C4, C3B,
procedural generation and publishing authority are outside this contract.
