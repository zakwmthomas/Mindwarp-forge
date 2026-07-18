# Forge Module Boundary Contract v1

Forge module dependencies are declared in
`governance/module-boundaries.json` and verified independently of runtime
behavior. The protected kernel has no Forge-module dependency and cannot
import desktop/UI or network capabilities. The desktop backend may depend on
the kernel but cannot import frontend source. The frontend cannot bypass the
bounded desktop command surface or create filesystem, process, or direct
network paths.

`tools/verify-modularity.ps1` fails closed on unknown modules, missing roots,
undeclared Cargo workspace dependencies, forbidden imports, or dependency
cycles. It reports every detected module violation before failing so one
broken module cannot hide a neighbour failure.

Every declared module also has a generated root `MODULE.md` front door sourced
from `governance/module-context-registry.json`. It explains purpose, ownership,
non-goals, entry points, invariants, direct upstream and downstream neighbours,
verification and canonical references. The front door includes a deterministic
source-tree fingerprint and is read before module work. It is regenerated, not
hand-edited, whenever the module boundary or context changes.

`tools/test-modularity.ps1` proves the live graph, two simultaneous forbidden
imports, dependency-cycle rejection, and retained multi-module diagnostics.
The boundary grants no execution, approval, promotion, credential, spending,
publishing, or protected-Kernel mutation authority.
