$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
. (Join-Path $PSScriptRoot 'g1-c5-successor-route.ps1')
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json

if (!(Test-G1C5RecordedClosureRoute -Checkpoint $checkpoint)) {
    throw 'Canonical C5 recorded closure route was rejected.'
}
if ((& (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint) -ne $true) {
    throw 'Canonical C5 recorded closure route was rejected by the interruption guard.'
}

function Reject-Mutation([string]$Label, [scriptblock]$Mutate, [scriptblock]$Restore) {
    & $Mutate
    try {
        if (Test-G1C5RecordedClosureRoute -Checkpoint $checkpoint) {
            throw "Forged C5 recorded closure route was admitted: $Label"
        }
    } finally {
        & $Restore
    }
}

$value = $checkpoint.batch_id
Reject-Mutation 'batch' { $checkpoint.batch_id = 'FORGED' } { $checkpoint.batch_id = $value }
$value = $checkpoint.master_program_item
Reject-Mutation 'item' { $checkpoint.master_program_item = 'C6' } { $checkpoint.master_program_item = $value }
$value = $checkpoint.state
Reject-Mutation 'state' { $checkpoint.state = 'executing' } { $checkpoint.state = $value }
$value = $checkpoint.substage_id
Reject-Mutation 'substage' { $checkpoint.substage_id = 'c5-full-gate-route-reconciliation' } { $checkpoint.substage_id = $value }
$value = $checkpoint.authority_lane
Reject-Mutation 'legacy authority' { $checkpoint.authority_lane = 'Owner-authorized broad C5 significance/scheduler reconciliation and capability-free closure readiness only. Exact dependency C4. No C3B, C6, C7, broad G1 closure, runtime controllers, runtime executors, cache mutation, storage mutation, product weights, AI generation, rendering implementation, filesystem, network, process, Companion, Greenfield, visual assets or Kernel mutation.' } { $checkpoint.authority_lane = $value }
Reject-Mutation 'authority suffix' { $checkpoint.authority_lane = $value + ' forged' } { $checkpoint.authority_lane = $value }
Reject-Mutation 'authority omission' { $checkpoint.authority_lane = $value.Replace(' No C3B,', '') } { $checkpoint.authority_lane = $value }
Reject-Mutation 'array batch' { $checkpoint.batch_id = @('FORGED', 'G1-C5-SIGNIFICANCE-SCHEDULER-CLOSURE-V1') } { $checkpoint.batch_id = 'G1-C5-SIGNIFICANCE-SCHEDULER-CLOSURE-V1' }
Reject-Mutation 'array authority' { $checkpoint.authority_lane = @('FORGED', $value) } { $checkpoint.authority_lane = $value }

Write-Output 'C5 current recorded successor route verified: exact batch, item, state, substage and authority pair fail closed.'
