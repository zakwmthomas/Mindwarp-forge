param([string]$Only)
$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
$registry=Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_GP4_SIGNAL_ANCHOR_FIXED_REGISTRY.md') -Raw
$design=Join-Path $root 'docs\canonical-system\G1_GP4_SIGNAL_ANCHOR_DESIGN.md'
$verifier=Join-Path $root 'tools\verify-g1-gp4-signal-anchor-computation.ps1'
$temporary=Join-Path ([IO.Path]::GetTempPath()) ('forge-gp4-hostile-'+[guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $temporary|Out-Null
try {
  $id1='6fa6a6d429003d91fb4f577486a34ff4bf174e16e659c966ccf3327e8dd2cc15'
  $id2='287ccf55549997ecd90f3fd8fc202bd7c92be386923e1954a9324554b343fda1'
  $swapped=$registry.Replace($id1,'SWAP-TEMP').Replace($id2,$id1).Replace('SWAP-TEMP',$id2)
  $swappedPath=Join-Path $temporary 'swapped.md';$swapped|Set-Content -LiteralPath $swappedPath -Encoding utf8
  $question='Does the adapter preserve strict bundle bytes and every digest?'
  $arbitrary=$registry.Replace($question,'Arbitrary nonempty replacement question?')
  $arbitraryPath=Join-Path $temporary 'arbitrary.md';$arbitrary|Set-Content -LiteralPath $arbitraryPath -Encoding utf8
  $schemaDesign=Get-Content -LiteralPath $design -Raw
  $schemaDesign=$schemaDesign.Replace('`schema_version: u16` — exactly `1`','`schema_version: u16` — exactly `2`')
  $schemaPath=Join-Path $temporary 'schema.md';$schemaDesign|Set-Content -LiteralPath $schemaPath -Encoding utf8
  $emit1='1. `knowledge.s4-temporary-rescue`';$emit2='2. `knowledge.s4-temporary-rescue.grant-0`'
  $emissionSwap=$registry.Replace($emit1,'EMIT-TEMP').Replace($emit2,'2. `knowledge.s4-temporary-rescue`').Replace('EMIT-TEMP','1. `knowledge.s4-temporary-rescue.grant-0`')
  $emissionPath=Join-Path $temporary 'emission.md';$emissionSwap|Set-Content -LiteralPath $emissionPath -Encoding utf8
  $threatLeak=$registry.Replace('1. `anchor.brace.temporary`','1. `work-area.state.safe`')
  $threatPath=Join-Path $temporary 'threat.md';$threatLeak|Set-Content -LiteralPath $threatPath -Encoding utf8
  $cases=@(@($swappedPath,$design,'Typed command row drift','command'),@($arbitraryPath,$design,'Typed adapter row drift','requirement'),@((Join-Path $root 'docs\canonical-system\G1_GP4_SIGNAL_ANCHOR_FIXED_REGISTRY.md'),$schemaPath,'schema drift','schema'),@($emissionPath,$design,'GP2 output order','emission'),@($threatPath,$design,'GP2 output order','threat'))
  if($Only){$cases=@($cases|Where-Object{$_[3]-eq$Only})}
  foreach($case in $cases){
    $old=$ErrorActionPreference;$ErrorActionPreference='Continue'
    $output=& powershell -NoProfile -ExecutionPolicy Bypass -File $verifier -RegistryPath $case[0] -DesignPath $case[1] 2>&1;$exit=$LASTEXITCODE
    $ErrorActionPreference=$old
    if($exit-eq0-or($output-join"`n")-notlike("*"+$case[2]+"*")){throw "GP4 hostile readiness mutation did not fail exactly: $($case[3]) expected=$($case[2]) actual=$($output-join' | ')"}
  }
  Write-Output 'GP4 hostile readiness verified: command order, requirement text, schema tuples, GP2 emission order, and forbidden GP2 threat injection all fail exact computation.'
} finally {if(Test-Path -LiteralPath $temporary){Remove-Item -LiteralPath $temporary -Recurse -Force}}
