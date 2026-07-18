$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent $PSScriptRoot
$recordPath = Join-Path $root 'docs\canonical-system\G1_PRODUCT_PORTFOLIO_AND_C3_DEPENDENCY_REBASELINE.md'
if (!(Test-Path -LiteralPath $recordPath -PathType Leaf)) { throw 'G1 product-portfolio adjudication record is missing.' }
$record = Get-Content -LiteralPath $recordPath -Raw
foreach ($token in @(
    'Forge Core -> proven reusable capabilities -> domain creative suites -> isolated product shells',
    'vertical-then-extract',
    'Yard Atlas remains an independent product',
    'no repository, database, UI, authentication, billing, or release authority merge',
    'C3B does not block the first playable vertical'
)) { if (!$record.Contains($token)) { throw "Product-portfolio adjudication is missing: $token" } }

$atlas = Get-Content -LiteralPath (Join-Path $root 'docs\project-atlas\project-model.json') -Raw | ConvertFrom-Json
$systemIds = @($atlas.systems.id)
foreach ($id in @('forge-kernel','reusable-capabilities','domain-creative-suites','mindwarp-game','mindwarp-companion','greenfield-yard-atlas')) {
    if ($systemIds -notcontains $id) { throw "Project Atlas lacks product topology system: $id" }
}
$d1 = @($atlas.decisions | Where-Object id -eq 'D1')
if ($d1.Count -ne 1 -or $d1[0].rationale -notlike '*vertical-then-extract*') { throw 'D1 does not bind product-driven vertical-then-extract delivery.' }

$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
function One([string]$id) { @($program.items | Where-Object id -eq $id) }
$c3a = One 'C3A'; $c3b = One 'C3B'; $c3 = One 'C3'; $c4 = One 'C4'
if (@($c3a).Count -ne 1 -or $c3a.state -ne 'promoted' -or $c3a.proof -notlike '*WorldGenerationInput*CausalWorldPacket*v1*nested identity*provenance*') { throw 'C3A is not the exact promoted dependency-sufficient seam.' }
if (@($c3b).Count -ne 1 -or $c3b.state -ne 'blocked' -or $c3b.next_action -notlike '*physical scale*coefficient*applicability*visibility*presentation fidelity*') { throw 'C3B is not visibly evidence-blocked.' }
if (@($c3).Count -ne 1 -or $c3.state -eq 'executing' -or $c3.proof -notlike '*does not claim full C3 closure*') { throw 'The umbrella C3 item still conflates foundation readiness and physical fidelity.' }
if (@($c4).Count -ne 1 -or @($c4.depends_on).Count -ne 2 -or @($c4.depends_on) -notcontains 'C2' -or @($c4.depends_on) -notcontains 'C3A') { throw 'C4 does not depend exactly on C2 and C3A.' }
foreach ($id in @('GP0','GP1','GP2','GP3','GP4')) { if (@(One $id).Count -ne 1) { throw "Master program lacks gameplay item: $id" } }
$executing = @($program.items | Where-Object state -eq 'executing')
if ($executing.Count -ne 1 -or $executing[0].id -ne 'GP2') { throw 'GP2 is not the unique executing design-intake item.' }
$gp0 = One 'GP0'
if ($gp0.state -ne 'promoted' -or $gp0.proof -notlike '*structural player-promise package only*') { throw 'GP0 is not structurally closed with its no-system-promotion boundary.' }
$gp1 = One 'GP1'
if ($gp1.state -ne 'promoted' -or $gp1.status -ne 'complete' -or $gp1.proof -notlike '*capability-free fixed-loop proof*GP2 remain excluded*') { throw 'GP1 is not structurally closed at the bounded fixed-loop proof.' }
$gp2 = One 'GP2'
if ($gp2.state -ne 'executing' -or $gp2.status -ne 'active' -or $gp2.proof -notlike '*readiness verifier freezes five distinct typed lanes*no source implementation*universal currency*') { throw 'GP2 is not bounded to accepted design and pre-source readiness.' }

$binding = Get-Content -LiteralPath (Join-Path $root 'crates\addressable-world-binding\src\lib.rs') -Raw
if ($binding -notmatch 'use derived_world_rules::\{CausalWorldPacket, WorldGenerationInput, validate_world_packet\};') { throw 'C3A lacks the exact C4 code-facing import seam.' }
foreach ($forbidden in @('physical_path_substrate::','visible_radiance_','optical_phase_space_')) { if ($binding.Contains($forbidden)) { throw "C4 imports deferred C3B implementation: $forbidden" } }

Write-Output 'G1 portfolio rebaseline verified: product topology, C3A/C3B split, C4 seam, GP0-GP1 structural closure and active design-first GP2 route are exact.'
