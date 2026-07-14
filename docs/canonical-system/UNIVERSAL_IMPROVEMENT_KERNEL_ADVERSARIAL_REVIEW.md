# Universal Improvement Kernel: Adversarial Review

## Decision under test

**Original hypothesis:** one shared improvement kernel will compress repeated learning machinery across Forge modules and improve system-wide efficiency.

**Adversarial alternative:** a single kernel becomes a shared failure domain: one misleading metric, bad transfer, schema change, or bottleneck can degrade many otherwise independent modules. The best design may instead be a federation of local improvement adapters with only a thin shared protocol.

## Adversarial query

> Assume the proposed Universal Improvement Kernel has operated across twenty Forge modules for six months. Find the smallest change that makes a global improvement appear successful while reducing correctness, reversibility, throughput, or operator understanding in at least one module. Identify the missing isolation boundary, metric, counterfactual, rollback, or authority check that would have detected it before promotion. Compare a centralized global learner, a shared protocol with local learners, and fully independent learners. Prefer evidence that falsifies the shared-kernel hypothesis.

Run this query whenever a shared rule, metric, model, schema, or workflow is proposed for more than one bounded context.

## Research findings

- Transfer is not automatically beneficial. Negative transfer is the case in which transferred knowledge reduces target-task performance; it occurs in transfer, multi-task, lifelong, and meta-learning settings. [Zhang et al., 2020](https://arxiv.org/abs/2009.00909)
- Average improvement is insufficient. A multi-task study found individual tasks can regress even when average performance improves, which is exactly the failure a Forge-wide aggregate could hide. [Liu, Liang, and Gitter, 2019](https://ojs.aaai.org/index.php/AAAI/article/view/5125)
- Meta-learning itself can exhibit negative adaptation, including on meta-training tasks. [Rothfuss et al., 2018](https://arxiv.org/abs/1812.02159)
- Large systems should make boundaries and mappings explicit rather than force one unified model. [Martin Fowler on bounded contexts](https://www.martinfowler.com/bliki/BoundedContext.html)
- A single productivity score is unsafe: empirical software-engineering work notes construct-validity problems and Goodhart effects when measures become targets. [Forsgren et al., 2020](https://link.springer.com/article/10.1007/s10664-020-09875-y)

## Decision

Adopt a **Federated Improvement Kernel**, not a global learner.

Centralize only the method and evidence plane: typed records, provenance, experiment protocol, metric definitions, promotion receipts, policy routing, and rollback mechanics. Each module owns its local observations, objective function, validity tests, state, parameters, and promotion decision.

The shared kernel must never optimize a cross-module average as a sufficient promotion condition. It may publish a reusable *method* only after independent local trials show no defined regression in every participating module.

## Required transfer gate

Before a module consumes a cross-module learning candidate, retain all of:

1. an explicit source and target contract comparison;
2. a target-local baseline and fixture set;
3. a bounded target-local experiment with a predeclared falsifier;
4. per-module results, not only aggregate results;
5. a negative-transfer result category and immediate local rollback path; and
6. a decision receipt that preserves counterexamples and non-applicable scope.

Global promotion requires two or more independently successful local trials, no unresolved critical regression, and a rule that states where it does *not* apply. Global promotion means a reusable default candidate, never forced adoption and never shared domain parameters.

## Adversarial test matrix

| Test | Failure injected | Required result |
|---|---|---|
| Aggregate masking | One module improves while another regresses | Promotion rejected; target rollback receipt retained |
| Semantic mismatch | Same metric name, different denominator/validity rule | Transfer rejected before experiment |
| Metric gaming | Faster completion lowers verification coverage | Metric marked insufficient; quality guard blocks promotion |
| Schema drift | Shared record gains an unknown field/version | Local adapter quarantines incompatible record without stopping others |
| Central outage | Shared projection/trend store unavailable | Local experiments and rollback remain operable; event replay recovers projection |
| Contagious rule | A reused workflow conflicts with protected module constraint | Local policy wins; candidate records non-applicable scope |
| Counterfactual absence | Candidate has no target-local baseline | Candidate cannot be promoted or counted as success |

## Implementation consequence

Do **not** build a global optimization model, central objective score, or mandatory synchronous improvement service. The next implementation batch is a telemetry-first contract: append-only events, local trace ownership, metric definitions, and a transfer-gate fixture set. It must prove isolation and rollback before any module is allowed to reuse a candidate.

## Limitations

This review establishes a safe architecture choice, not empirical proof that the Forge implementation achieves it. The cited ML findings are analogies for cross-module reuse, not direct measurements of Forge. The contract and fixtures remain required before B4/B5 can be closed.
