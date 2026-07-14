# Forge Operating Flow

```mermaid
flowchart LR
  E["Evidence"] --> C["Proposed candidate"] --> A["Explicit owner approval"] --> P["Promotion"] --> V["Verification"] --> W["Controlled application"]
  E -. "never grants authority" .-> A
```

Every work package follows: research → design → adversarial review → readiness
gate → authorized implementation → verification → promotion. Captured text,
assistant output, and summaries remain evidence; they are never authority.
