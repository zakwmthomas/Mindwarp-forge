# F5 Owner Gate

**State:** Satisfied 2026-07-13. The owner delegated the F4-to-F5 transition,
research, storage-lane selection, failure review, and bounded repair.

The owner gate required both:

1. approval to change the Atlas milestone from F4 to F5; and
2. selection of protected Kernel-object or versioned-projection ProofReceipt
   storage.

The versioned-projection lane was selected after adversarial review because it
keeps protected Kernel authority unchanged while permitting strict evidence
linkage, atomic writes, additive migration, verified backup, and read-only
inspection. The decision and its failure matrix are retained in
`F5_PROOF_RECEIPT_DECISION.md`.

The approval is narrow. It activates only the bounded engine-neutral F5
ProofReceipt/inspector package. It does not select an engine/runtime or expand
approval, promotion, spending, credential, publishing, or protected-Kernel
authority.
