Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$docPath = Join-Path $root 'docs\canonical-system\G1_C3_CROSS_BOUNDARY_ECOTONE_ORACLE_IMPLEMENTATION_READINESS.md'
if(!(Test-Path -LiteralPath $docPath)){throw 'C3 ecotone oracle implementation-readiness record is missing'}
$doc = Get-Content -LiteralPath $docPath -Raw
$design = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_CROSS_BOUNDARY_ECOTONE_MATHEMATICAL_DESIGN_AUDIT.md') -Raw
$derived = Get-Content -LiteralPath (Join-Path $root 'crates\derived-world-rules\src\lib.rs') -Raw
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$c3InterruptionRoute = & (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint
$cargo = Get-Content -LiteralPath (Join-Path $root 'Cargo.toml') -Raw

foreach($required in @(
  'ready for one exact disposable-oracle decision only','oracle result or production artifact',
  'prove-g1-c3-cross-boundary-ecotone.py','verify-g1-c3-cross-boundary-ecotone-oracle.ps1',
  'G1_C3_CROSS_BOUNDARY_ECOTONE_ORACLE_RESULT.md','repository proof harness',
  'no command-line arguments','canonical JSON object followed by one newline','-I -B',
  'may not import Forge modules','GPU execution is inappropriate','EcotoneFixtureV1',
  'synthetic_fixture_only','not a canonical 2D material-interface owner',
  'Fraction(product, 1_000_000_000)','10^12','40 bits',
  '[338,168,68]','[135,168,135]','[540,168,23]',
  'semantic_digest','audit_digest','entire receipt','refinement compares only',
  'mindwarp.disposable.ecotone.cell-result.v1','Do not invent hash pins before source exists',
  'Moisture is not a palette operand','Compound faults use this precedence',
  'Contradiction is dimension-local','missing_material_interface_join',
  '1 x 1','5 x 5','9 x 9','17 x 17','256 x 256','257 x 256',
  '65,536 cells','130,560 internal edges','seven enumeration modes',
  'Nineteen permanent hostile families','[0,1000,500]','375',
  '64 KiB','256 MiB','120','no recursion','at most four',
  'source and receipt hashes','Cargo files','module count','Rollback',
  'Claim ceiling','Exact owner decision','General continuation does not implement',
  'Nothing broader is locked in','One consumer first, reassess before expanding'
)) {
  if($doc -notlike "*$required*"){throw "Ecotone oracle readiness missing: $required"}
}
foreach($required in @('evidence-preserving typed-boundary witness selected','Hostile falsifier portfolio','no implementation authorized')) {
  if(($design + $doc) -notlike "*$required*"){throw "Ecotone design/readiness continuity drift: $required"}
}
if($derived -notmatch '\(product \+ 500_000_000\) / 1_000_000_000'){throw 'Derived-world palette rounding relation drift'}
if($cargo -like '*cross-boundary-ecotone*'){throw 'Disposable ecotone proof leaked into the Cargo workspace'}

$oraclePath = Join-Path $root 'tools\prove-g1-c3-cross-boundary-ecotone.py'
$resultPath = Join-Path $root 'docs\canonical-system\G1_C3_CROSS_BOUNDARY_ECOTONE_ORACLE_RESULT.md'
$readinessGate = $checkpoint.batch_id -eq 'G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-IMPLEMENTATION-READINESS-V1'
if($readinessGate -and ((Test-Path -LiteralPath $oraclePath) -or (Test-Path -LiteralPath $resultPath))){throw 'Ecotone oracle artifacts appeared during the code-free readiness stage'}

$c3 = @($program.items | Where-Object id -eq 'C3')
$readinessRoute = $c3.Count -eq 1 -and
  $c3[0].next_action -like '*ecotone*oracle*' -and
  $c3[0].next_action -like '*implementation*' -and
  $c3[0].proof -like '*evidence-preserving typed-boundary*semantic*digest*'
$resultRoute = $c3.Count -eq 1 -and
  $c3[0].next_action -like '*owner-approved disposable*ecotone oracle*complete Forge*' -and
  $c3[0].proof -like '*passes twice*semantic*provenance-sensitive audit*nineteen executable hostile*'
if(!$readinessRoute -and !$resultRoute){throw 'C3 ecotone oracle readiness route drift'}

$readinessActive = $readinessGate -and
  $checkpoint.substage_id -in @('c3-cross-boundary-ecotone-oracle-implementation-readiness','c3-cross-boundary-ecotone-oracle-owner-gate','c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and
  (($checkpoint.authority_lane -like '*code-free*C3 ecotone-oracle implementation-readiness*No Python oracle*crate*contract schema*dependency*production test*production source*downstream consumer*renderer*biome*organism*runtime*promotion*C3 closure*') -or
   ($checkpoint.authority_lane -like '*Serious owner gate*disposable*C3 ecotone oracle*No Python oracle*crate*contract schema*dependency*production test*production source*downstream consumer*renderer*biome*organism*runtime*promotion*C3 closure*'))
$implementationActive = $checkpoint.batch_id -eq 'G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-V1' -and
  $checkpoint.substage_id -in @('c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and
  $checkpoint.authority_lane -like '*Owner-approved disposable C3 ecotone oracle implementation only*No crate*contract schema*dependency*production test*production source*downstream consumer*renderer*biome*organism*runtime*promotion*C3 closure*'
if(!$readinessActive -and !$implementationActive -and !$c3InterruptionRoute){throw 'Ecotone oracle readiness checkpoint authority drift'}

$status = if($readinessActive){'no oracle exists'}else{'the exact owner-approved disposable package is active'}
Write-Output "C3 ecotone oracle readiness verified: exact independent proof package, digest semantics, hostiles, caps and deletion-only rollback are frozen; $status."
