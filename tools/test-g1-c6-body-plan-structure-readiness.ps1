$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
$temp=Join-Path ([IO.Path]::GetTempPath()) ('c6-body-plan-readiness-'+[guid]::NewGuid().ToString('N'))
try{
 New-Item -ItemType Directory -Path (Join-Path $temp 'docs/canonical-system'),(Join-Path $temp 'contracts'),(Join-Path $temp 'context/active') -Force|Out-Null
 Copy-Item (Join-Path $root 'docs/canonical-system/MASTER_PROGRAM.json') (Join-Path $temp 'docs/canonical-system/MASTER_PROGRAM.json')
 Copy-Item (Join-Path $root 'docs/canonical-system/G1_C6_BODY_PLAN_STRUCTURE_IMPLEMENTATION_READINESS.md') (Join-Path $temp 'docs/canonical-system/G1_C6_BODY_PLAN_STRUCTURE_IMPLEMENTATION_READINESS.md')
 Copy-Item (Join-Path $root 'contracts/body-plan-structure-contract.md') (Join-Path $temp 'contracts/body-plan-structure-contract.md')
 Copy-Item (Join-Path $root 'context/active/WORKER_BATCH_STATE.json') (Join-Path $temp 'context/active/WORKER_BATCH_STATE.json')
 $verify=Join-Path $root 'tools/verify-g1-c6-body-plan-structure-readiness.ps1'
 function Run { & $verify -ProgramPath (Join-Path $temp 'docs/canonical-system/MASTER_PROGRAM.json') -CheckpointPath (Join-Path $temp 'context/active/WORKER_BATCH_STATE.json') -ReadinessPath (Join-Path $temp 'docs/canonical-system/G1_C6_BODY_PLAN_STRUCTURE_IMPLEMENTATION_READINESS.md') -ContractPath (Join-Path $temp 'contracts/body-plan-structure-contract.md')|Out-Null }
 function Reset { Copy-Item (Join-Path $root 'docs/canonical-system/MASTER_PROGRAM.json') (Join-Path $temp 'docs/canonical-system/MASTER_PROGRAM.json') -Force;Copy-Item (Join-Path $root 'docs/canonical-system/G1_C6_BODY_PLAN_STRUCTURE_IMPLEMENTATION_READINESS.md') (Join-Path $temp 'docs/canonical-system/G1_C6_BODY_PLAN_STRUCTURE_IMPLEMENTATION_READINESS.md') -Force;Copy-Item (Join-Path $root 'contracts/body-plan-structure-contract.md') (Join-Path $temp 'contracts/body-plan-structure-contract.md') -Force;Copy-Item (Join-Path $root 'context/active/WORKER_BATCH_STATE.json') (Join-Path $temp 'context/active/WORKER_BATCH_STATE.json') -Force }
 function MustFail([string]$label,[scriptblock]$mutate){Reset;&$mutate;$failed=$false;try{Run}catch{$failed=$true};if(!$failed){throw "Body-plan readiness hostile passed: $label"}}
 Run
 MustFail 'authority drift' {$p=Join-Path $temp 'context/active/WORKER_BATCH_STATE.json';$j=Get-Content -Raw $p|ConvertFrom-Json;$j.authority_lane+=' forged';$j|ConvertTo-Json -Depth 100|Set-Content $p -Encoding utf8}
 MustFail 'expression identity substitution' {$p=Join-Path $temp 'docs/canonical-system/G1_C6_BODY_PLAN_STRUCTURE_IMPLEMENTATION_READINESS.md';(Get-Content -Raw $p).Replace('body_plan_ref` means family identity, not expression identity','body_plan_ref` means expression identity')|Set-Content $p -Encoding utf8}
 MustFail 'resource drift' {$p=Join-Path $temp 'contracts/body-plan-structure-contract.md';(Get-Content -Raw $p).Replace('relation instances | 512','relation instances | 513')|Set-Content $p -Encoding utf8}
 MustFail 'humanoid universalization' {$p=Join-Path $temp 'contracts/body-plan-structure-contract.md';(Get-Content -Raw $p).Replace('No universal pelvis','Universal pelvis')|Set-Content $p -Encoding utf8}
 Write-Output 'G1 C6 body-plan readiness hostiles verified: route, identity, resource and anti-humanoid boundaries fail closed.'
}finally{Remove-Item -LiteralPath $temp -Recurse -Force -ErrorAction SilentlyContinue}

