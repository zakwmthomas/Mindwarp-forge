$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
$verifier=Join-Path $PSScriptRoot 'verify-g1-c6-ecological-niche-design-readiness.ps1'
&$verifier|Out-Null
$temp=Join-Path ([IO.Path]::GetTempPath()) ('forge-c6-ecology-design-'+[guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $temp|Out-Null
try{
 foreach($dir in @('docs\canonical-system','contracts','context\active','crates\organism-niche-binding\src')){New-Item -ItemType Directory -Path (Join-Path $temp $dir)-Force|Out-Null}
 foreach($relative in @(
  'docs\canonical-system\G1_C6_ECOLOGICAL_NICHE_SEMANTICS_MATHEMATICAL_DESIGN_AUDIT.md',
  'docs\canonical-system\G1_C6_ECOLOGICAL_NICHE_SEMANTICS_IMPLEMENTATION_READINESS.md',
  'contracts\ecological-niche-semantics-contract.md','docs\canonical-system\MASTER_PROGRAM.json',
  'context\active\WORKER_BATCH_STATE.json','Cargo.toml','crates\organism-niche-binding\Cargo.toml','crates\organism-niche-binding\src\lib.rs'
 )){Copy-Item (Join-Path $root $relative) (Join-Path $temp $relative)}
 $designPath=Join-Path $temp 'docs\canonical-system\G1_C6_ECOLOGICAL_NICHE_SEMANTICS_MATHEMATICAL_DESIGN_AUDIT.md'
 $contractPath=Join-Path $temp 'contracts\ecological-niche-semantics-contract.md'
 $readinessPath=Join-Path $temp 'docs\canonical-system\G1_C6_ECOLOGICAL_NICHE_SEMANTICS_IMPLEMENTATION_READINESS.md'
 function Expect-Failure([string]$label,[scriptblock]$mutate,[scriptblock]$restore){
  &$mutate;$failed=$false;try{&$verifier -RootPath $temp|Out-Null}catch{$failed=$true};&$restore
  if(!$failed){throw "C6 ecology design/readiness hostile passed: $label"}
 }
 $design=Get-Content -Raw $designPath;$contract=Get-Content -Raw $contractPath;$ready=Get-Content -Raw $readinessPath
 Expect-Failure 'supported-to-suitable' {Set-Content $contractPath $contract.Replace('does not mean ecologically suitable','means ecologically suitable')} {Set-Content $contractPath $contract}
 Expect-Failure 'prospective-to-realized' {Set-Content $designPath ($design+[Environment]::NewLine+'Normative override: prospective candidate proves realized occupancy.')} {Set-Content $designPath $design}
 Expect-Failure 'missing-to-zero' {Set-Content $designPath $design.Replace('Missing evidence is not false or zero','Missing evidence is zero')} {Set-Content $designPath $design}
 Expect-Failure 'relocated-hostiles' {Set-Content $designPath ($design.Replace('- `C6-H103`: labels or coavailability presented as trophic flow;','- relocated trophic hostile;')+[Environment]::NewLine+'C6-H103')} {Set-Content $designPath $design}
 Expect-Failure 'cap-drift' {Set-Content $contractPath $contract.Replace('maximum examinations: `128`','maximum examinations: `256`')} {Set-Content $contractPath $contract}
 Expect-Failure 'dimorphism-laundering' {Set-Content $designPath ($design+[Environment]::NewLine+'Body-plan expression proves dimorphism.')} {Set-Content $designPath $design}
 Expect-Failure 'production-source-authorization' {
  $p=Get-Content -Raw (Join-Path $temp 'context\active\WORKER_BATCH_STATE.json')|ConvertFrom-Json;$p.verification_receipts+=@('source-authorization:c6-ecological-niche:test');$p|ConvertTo-Json -Depth 100|Set-Content (Join-Path $temp 'context\active\WORKER_BATCH_STATE.json')
 } {Copy-Item (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') (Join-Path $temp 'context\active\WORKER_BATCH_STATE.json') -Force}
 Expect-Failure 'implementation-gate' {
  $p=Get-Content -Raw (Join-Path $temp 'docs\canonical-system\MASTER_PROGRAM.json')|ConvertFrom-Json;($p.items|Where-Object id -eq C6).gate='implementation';$p|ConvertTo-Json -Depth 100|Set-Content (Join-Path $temp 'docs\canonical-system\MASTER_PROGRAM.json')
 } {Copy-Item (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') (Join-Path $temp 'docs\canonical-system\MASTER_PROGRAM.json') -Force}
 $crate=Join-Path $temp 'crates\ecological-niche-semantics';New-Item -ItemType Directory $crate|Out-Null
 $failed=$false;try{&$verifier -RootPath $temp|Out-Null}catch{$failed=$true};if(!$failed){throw 'C6 ecology production crate passed.'};Remove-Item $crate -Recurse -Force
 Add-Content (Join-Path $temp 'Cargo.toml') ([Environment]::NewLine+'ecological-niche-semantics = { path = "crates/ecological-niche-semantics" }')
 $failed=$false;try{&$verifier -RootPath $temp|Out-Null}catch{$failed=$true};if(!$failed){throw 'C6 ecology Cargo dependency passed.'};Copy-Item (Join-Path $root 'Cargo.toml') (Join-Path $temp 'Cargo.toml') -Force
 Add-Content (Join-Path $temp 'crates\organism-niche-binding\src\lib.rs') ([Environment]::NewLine+'// ecological-niche-semantics premature consumer')
 $failed=$false;try{&$verifier -RootPath $temp|Out-Null}catch{$failed=$true};if(!$failed){throw 'C6 ecology premature consumer source passed.'}
}finally{Remove-Item -LiteralPath $temp -Recurse -Force}
Write-Output 'G1 C6 ecological-niche design/readiness hostiles verified: truth laundering, relocated hostiles, cap/authority/gate/source/dependency/consumer crossings fail closed.'
