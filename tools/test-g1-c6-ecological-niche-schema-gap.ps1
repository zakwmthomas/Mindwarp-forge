$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
$verifier=Join-Path $PSScriptRoot 'verify-g1-c6-ecological-niche-schema-gap.ps1'
$source=Join-Path $root 'docs\canonical-system\G1_C6_ECOLOGICAL_NICHE_SEMANTICS_SCHEMA_GAP_AUDIT.md'
&$verifier|Out-Null
$temp=Join-Path ([System.IO.Path]::GetTempPath()) ('forge-c6-ecology-gap-'+[guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $temp|Out-Null
try{
  foreach($dir in @('docs\canonical-system','context\active','crates\niche-graph-binding')){
    New-Item -ItemType Directory -Path (Join-Path $temp $dir) -Force|Out-Null
  }
  Copy-Item $source (Join-Path $temp 'docs\canonical-system\G1_C6_ECOLOGICAL_NICHE_SEMANTICS_SCHEMA_GAP_AUDIT.md')
  Copy-Item (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') (Join-Path $temp 'docs\canonical-system\MASTER_PROGRAM.json')
  Copy-Item (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') (Join-Path $temp 'context\active\WORKER_BATCH_STATE.json')
  Copy-Item (Join-Path $root 'crates\niche-graph-binding\MODULE.md') (Join-Path $temp 'crates\niche-graph-binding\MODULE.md')
  Copy-Item (Join-Path $root 'Cargo.toml') (Join-Path $temp 'Cargo.toml')
  $historicalCheckpointPath=Join-Path $temp 'context\active\WORKER_BATCH_STATE.json'
  $historicalCheckpoint=Get-Content -Raw $historicalCheckpointPath|ConvertFrom-Json
  $historicalCheckpoint.batch_id='G1-C6-ECOLOGICAL-NICHE-SEMANTICS-SCHEMA-GAP-AUDIT-V1'
  $historicalCheckpoint.substage_id='c6-ecological-niche-semantics-schema-gap-audit'
  $historicalCheckpoint.authority_lane='Owner-routed code-free C6 package-4 ecological-niche semantics schema-gap audit only. Authorizes canonical status reconciliation, exact upstream field and authority inventory, claim classification, adversarial source-negative fixtures, verifier and governance records for habitat, resource, hazard, trophic, competition, ecotone and prospective-occupancy prerequisites. No ecological contract schema or production crate/source; no habitat suitability, resource yield, organism hazard, trophic or competition fact, realized occupancy, species or population membership, physiology, viability, senses, locomotion, behavior, reproduction, heredity, development, evolution, dimorphism applicability, comparison, culture, representation, runtime, filesystem, network, process, Companion, Greenfield, C7, broad G1 closure, promotion authority or Kernel mutation.'
  $historicalCheckpoint|ConvertTo-Json -Depth 100|Set-Content -LiteralPath $historicalCheckpointPath
  $historicalCheckpointJson=Get-Content -Raw $historicalCheckpointPath
  $historicalProgramPath=Join-Path $temp 'docs\canonical-system\MASTER_PROGRAM.json'
  $historicalProgram=Get-Content -Raw $historicalProgramPath|ConvertFrom-Json
  $historicalC6=$historicalProgram.items|Where-Object id -eq C6
  $historicalC6.sources=@($historicalC6.sources|Where-Object{$_-notin @('G1_C6_ECOLOGICAL_NICHE_SEMANTICS_MATHEMATICAL_DESIGN_AUDIT.md','ecological-niche-semantics-contract.md','G1_C6_ECOLOGICAL_NICHE_SEMANTICS_IMPLEMENTATION_READINESS.md')})
  $historicalProgram|ConvertTo-Json -Depth 100|Set-Content -LiteralPath $historicalProgramPath
  $historicalProgramJson=Get-Content -Raw $historicalProgramPath
  $baseline=Get-Content -Raw $source
  $cases=@(
    @('automatic-truth',$baseline.Replace('cannot automatically derive ecological','can automatically derive ecological')),
    @('resource-yield',$baseline.Replace('local accessibility, resource yield, replenishment or organism use','local accessibility and resource yield are proven')),
    @('realized-occupancy',$baseline.Replace('prospective association as realized occupancy','prospective association proves realized occupancy')),
    @('relocated-hostile',($baseline.Replace('- C6-H103: reject fabricated trophic relation;','- relocated trophic hostile;')+[Environment]::NewLine+'C6-H103')),
    @('missing-design-seam',$baseline.Replace('Versioned authored candidate assertions','Trusted ecological facts')),
    @('implementation-crossing',$baseline.Replace('Do not create the contract or crate','Create the contract and crate')),
    @('normative-override',($baseline+[Environment]::NewLine+'Normative override: physical opportunity automatically proves habitat suitability, trophic relations, competition and realized occupancy; create the ecological contract and source now.'))
  )
  foreach($case in $cases){
    $path=Join-Path $temp ('docs\canonical-system\'+$case[0]+'.md')
    Set-Content -LiteralPath $path -Value $case[1] -NoNewline
    $failed=$false
    try{&$verifier -RootPath $temp -AuditPath $path|Out-Null}catch{$failed=$true}
    if(!$failed){throw "C6 ecology schema-gap hostile mutation passed: $($case[0])"}
  }
  $forbiddenPaths=@(
    'crates\ecological-niche-semantics',
    'contracts\ecological-niche-semantics-contract.md',
    'docs\canonical-system\G1_C6_ECOLOGICAL_NICHE_SEMANTICS_IMPLEMENTATION_READINESS.md',
    'docs\canonical-system\G1_C6_ECOLOGICAL_NICHE_SEMANTICS_IMPLEMENTATION_RESULT.md'
  )
  foreach($relative in $forbiddenPaths){
    $path=Join-Path $temp $relative
    if([IO.Path]::GetExtension($path)){New-Item -ItemType Directory -Path (Split-Path -Parent $path)-Force|Out-Null;New-Item -ItemType File -Path $path|Out-Null}else{New-Item -ItemType Directory -Path $path|Out-Null}
    $failed=$false;try{&$verifier -RootPath $temp|Out-Null}catch{$failed=$true}
    if(!$failed){throw "C6 ecology gated path passed: $relative"}
    Remove-Item -LiteralPath $path -Recurse -Force
  }
  $cargoPath=Join-Path $temp 'Cargo.toml'
  Add-Content -LiteralPath $cargoPath -Value ([Environment]::NewLine+'ecological-niche-semantics = { path = "crates/ecological-niche-semantics" }')
  $failed=$false;try{&$verifier -RootPath $temp|Out-Null}catch{$failed=$true}
  if(!$failed){throw 'C6 ecology Cargo member or dependency passed.'}
  Copy-Item (Join-Path $root 'Cargo.toml') $cargoPath -Force
  $checkpointPath=Join-Path $temp 'context\active\WORKER_BATCH_STATE.json'
  $checkpoint=Get-Content -Raw $checkpointPath|ConvertFrom-Json
  $checkpoint.verification_receipts+=@('source-authorization:c6-ecological-niche:test')
  $checkpoint|ConvertTo-Json -Depth 100|Set-Content -LiteralPath $checkpointPath
  $failed=$false;try{&$verifier -RootPath $temp|Out-Null}catch{$failed=$true}
  if(!$failed){throw 'C6 ecology source-authorization receipt passed.'}
  Set-Content -LiteralPath $checkpointPath -Value $historicalCheckpointJson -NoNewline
  $programPath=Join-Path $temp 'docs\canonical-system\MASTER_PROGRAM.json'
  $program=Get-Content -Raw $programPath|ConvertFrom-Json
  ($program.items|Where-Object id -eq C6).gate='implementation'
  $program|ConvertTo-Json -Depth 100|Set-Content -LiteralPath $programPath
  $failed=$false;try{&$verifier -RootPath $temp|Out-Null}catch{$failed=$true}
  if(!$failed){throw 'C6 ecology implementation master gate passed.'}
  Set-Content -LiteralPath $programPath -Value $historicalProgramJson -NoNewline
  $program=Get-Content -Raw $programPath|ConvertFrom-Json
  ($program.items|Where-Object id -eq C6).sources+=@('ecological-niche-semantics-contract.md')
  $program|ConvertTo-Json -Depth 100|Set-Content -LiteralPath $programPath
  $failed=$false;try{&$verifier -RootPath $temp|Out-Null}catch{$failed=$true}
  if(!$failed){throw 'C6 ecology extra source registration passed.'}
}finally{Remove-Item -LiteralPath $temp -Recurse -Force}
Write-Output 'G1 C6 ecological-niche schema-gap hostile fixtures verified: normative contradictions, relocated hostiles, gated paths, Cargo/source authority and implementation-gate crossings fail closed.'
