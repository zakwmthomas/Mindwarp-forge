# Worker Metric Registry

No metric is valid until it has all fields below.

| Metric | Unit | Denominator | Exclusions | Minimum sample | Goodhart guard |
|---|---|---|---|---:|---|
| estimate calibration error | percent | completed batches with estimate and actual duration | blocked/external-wait batches | 5 | Never reward lower estimate by itself |
| verified closure rate | percent | completed meaningful batches | partial/micro updates | 5 | Pair with rework and verification failures |
| repair-loop rate | count per verified batch | verified batches | formatting-only retries tracked separately | 5 | Lower is not always better if defects are being found earlier |
| idle recovery delay | duration | batches awaiting continuation | active uninterrupted batches | 5 | Balance against interruption/context waste |
| rework ratio | percent | verified outputs | planned-only work | 5 | Do not penalize legitimate discovery/refinement |
| marginal verified gain | local outcome units per total cost | comparable completed trials for one local objective | blocked, incomparable, and insufficient-sample trials | 3 | Never aggregate unlike modules; pair with regression and uncertainty |
| improvement cost | context, token, tool, machine, and owner-attention units | bounded improvement trials | normal delivery work not caused by the trial | 3 | Do not treat lower cost as better when verification coverage falls |
| stop/refocus adherence | percent | improvement trials reaching a predeclared decision | trials stopped by external authority | 3 | A high score is meaningless without retained baseline and outcome evidence |
| verification coverage | percent | applicable declared verification gates | gates not applicable by contract | 3 | Never increase coverage by splitting one test into many names |
| regression escape rate | percent | verified changes with later retained regression evidence | unrelated later failures | 5 | Do not suppress regression reporting to improve the rate |
| rollback recovery rate | percent | triggered rollback cases | cases without a safe rollback route | 3 | A high rate does not excuse frequent unsafe changes |
| cost-data completeness | percent | measurable cost fields for meaningful batches | inherently unavailable provider fields | 5 | Unknown is preferable to fabricated zero cost |
| transfer success rate | percent | target-local trials of one explicitly scoped reusable method | incomparable targets and insufficient-sample trials | 3 | Pair with target-local regressions; never use a global average alone |
| negative-transfer incidence | percent | target-local transfer trials | trials without valid local baseline | 3 | Reporting more failures is preferable to hiding incompatible reuse |
| owner-wait fallback compliance | percent | owner gates reaching five unchanged heartbeat wakes | owner-explicit current-wait exemptions | 3 | Never improve the score by inferring approval, crossing the gate, or selecting a descendant or dependency-incomplete item |

All metrics are advisory, carry `insufficient_sample` until their threshold,
and link to event references rather than raw prompts, paths, or free text.
