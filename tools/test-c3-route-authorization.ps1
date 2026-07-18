$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent $PSScriptRoot
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$c3 = @($program.items | Where-Object id -eq 'C3')
if ($c3.Count -ne 1) { throw 'Canonical C3 item is missing or ambiguous.' }

$accepted = & (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint -C3 $c3[0]
if (!$accepted) { throw 'The canonical checkpoint is not accepted by the shared C3 route policy.' }

$forged = $checkpoint | ConvertTo-Json -Depth 20 | ConvertFrom-Json
$forged.batch_id = 'UNREGISTERED-CROSS-CUTTING-ROUTE'
$rejected = & (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $forged -C3 $c3[0]
if ($rejected) { throw 'The shared C3 route policy accepted an unregistered package.' }

$consumers = @(rg -l 'test-c3-federated-interruption\.ps1' $PSScriptRoot -g 'verify-g1-c3-*.ps1')
if ($consumers.Count -lt 30) { throw "Shared C3 route policy has too few verifier consumers: $($consumers.Count)" }

Write-Output "C3 route authorization verified: canonical route accepted, unknown route rejected, $($consumers.Count) verifier consumers centralized."
