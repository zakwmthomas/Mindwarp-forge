$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
. (Join-Path $PSScriptRoot 'g1-c5-successor-route.ps1')
. (Join-Path $PSScriptRoot 'g1-c6-successor-route.ps1')
$canonical = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json

if (!(Test-G1C6AuthorizedCurrentRoute -Checkpoint $canonical)) { throw 'Canonical authorized C6 route was rejected.' }
if ((& (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $canonical) -ne $true) { throw 'Canonical C6 readiness route was rejected by the interruption guard.' }

$c5 = $canonical | ConvertTo-Json -Depth 100 | ConvertFrom-Json
$c5.batch_id = 'G1-C5-SIGNIFICANCE-SCHEDULER-CLOSURE-V1'
$c5.master_program_item = 'C5'
$c5.state = 'recorded'
$c5.previous_state = 'verifying'
$c5.substage_id = 'c5-registered-closure-recorded'
$c5.authority_lane = 'Owner-authorized recorded C5 significance/scheduler capability-free closure evidence only. Exact dependency C4. C5 remains the sole master-program cursor pending a separate C6 transition. No C3B, C6 activation, C7, broad G1 closure, runtime controllers, runtime executors, cache mutation, storage mutation, product weights, AI generation, rendering implementation, filesystem, network, process, Companion, Greenfield, visual assets, promotion authority or Kernel mutation.'
if (!(Test-G1C5RecordedClosureRoute -Checkpoint $c5)) { throw 'Synthetic historical C5 recorded route was rejected.' }

function Reject-C6Mutation([string]$Label, [scriptblock]$Mutate, [scriptblock]$Restore) {
    & $Mutate
    try { if (Test-G1C6AuthorizedCurrentRoute -Checkpoint $canonical) { throw "Forged C6 route admitted: $Label" } }
    finally { & $Restore }
}
$value=$canonical.batch_id; Reject-C6Mutation 'batch' {$canonical.batch_id='FORGED'} {$canonical.batch_id=$value}
$value=$canonical.master_program_item; Reject-C6Mutation 'item' {$canonical.master_program_item='C5'} {$canonical.master_program_item=$value}
$value=$canonical.state; Reject-C6Mutation 'state' {$canonical.state='recorded'} {$canonical.state=$value}
$value=$canonical.substage_id; Reject-C6Mutation 'substage' {$canonical.substage_id='c6-implementation'} {$canonical.substage_id=$value}
$value=$canonical.authority_lane; Reject-C6Mutation 'authority suffix' {$canonical.authority_lane=$value+' forged'} {$canonical.authority_lane=$value}
if(Test-G1C6ReconciliationReadinessRoute -Checkpoint $canonical){
  Reject-C6Mutation 'source authority omission' {$canonical.authority_lane=$value.Replace('No C6 implementation source, ','')} {$canonical.authority_lane=$value}
}else{
  Reject-C6Mutation 'body-plan scope omission' {$canonical.authority_lane=$value.Replace('No ecology realization, ','')} {$canonical.authority_lane=$value}
}
Reject-C6Mutation 'array authority' {$canonical.authority_lane=@($value,'FORGED')} {$canonical.authority_lane=$value}

Write-Output 'C5 historical and C6 current successor routes verified: exact tuples and authority boundaries fail closed.'
