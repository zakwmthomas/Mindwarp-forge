# G1-VERTICAL-CLOSEOUT receipt

> Human-readable summary only. The canonical typed, hashed receipt is
> `G1_VERTICAL_CLOSEOUT_RECEIPT.json`.

Status: recorded bounded evidence receipt; the closeout item remains
`executing/active` until an owner-authorized successor is activated.

| Field | Exact value |
|---|---|
| `receipt_id` | `G1-VERTICAL-CLOSEOUT` |
| `schema_version` | `1` |
| `bundle_id` | `gp4.signal-anchor.bundle-v1` |
| `registered_full_gate` | `run-7e5c44dc8f48424a8cec42da756e3127` |
| `registered_duration_ms` | `590582` |
| `broad_g1` | `false` |
| `runtime_selected` | `false` |
| `runtime_containment_pending` | `true` |
| `evidence_only` | `true` |
| `promotion_authority` | `false` |

## Exact dependency receipt set

- C3A: `causal-world-packet-seam` and the exact fixed C3A input/packet bytes in
  `SignalAnchorBundleV1`;
- C4V: registered proof `run-fa6334a300e04d409dd5cddb4f22542e`;
- GP0: `G1_GP0_GAMEPLAY_FOUNDATION_RESULT.md` exact S4 session;
- GP1: `G1_GP1_FIXED_BASE_LOOP_RESULT.md` exact five-action trace;
- GP2: registered proof `run-2dc3db644adc416a8ef56461dbb771b6`;
- GP3: registered proof `run-50a8c78043eb46c483f1f655d3793f9b`;
- GP4: registered proof `run-7e5c44dc8f48424a8cec42da756e3127`.

The receipt closes only this engine-neutral vertical evidence slice. It does
not close broad `G1-CLOSEOUT`, change R1, select an engine, prove runtime
containment, authorize C3B, broaden C4, or grant Companion/Greenfield,
filesystem, network, process, Kernel, procedural-generation or publishing
authority.
