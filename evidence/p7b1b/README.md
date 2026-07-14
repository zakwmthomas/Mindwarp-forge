# P7b-1b denial-canary evidence

This directory admits only bounded host-generated JSON receipts from explicitly
owner-authorized LPAC denial trials. Quarantine content, executables, profiles,
staging files, and unparsed child output must never be retained here.

Trial receipts prove only the exact host, binary, profile, configuration, and
synthetic probes named by the receipt. They do not authorize retries, tools,
renderers, assets, runtime selection, promotion, or protected-Kernel changes.

- `trial-1-owner-authorized.json`: failed before resume on an ambiguously
  labelled token query; cleanup succeeded.
- `trial-2-owner-authorized.json`: failed before resume on the image-load
  mitigation verifier; cleanup succeeded. The retained result and root-cause
  analysis are in `P7B1B_DENIAL_CANARY_TRIAL2_RESULT.md`.
- `trial-3-owner-delegated.json`: passed suspended-host verification, then
  failed after resume with `0xC0000142`; cleanup succeeded and no denial report
  was admitted. The retained result is in
  `P7B1B_DENIAL_CANARY_TRIAL3_RESULT.md`.
