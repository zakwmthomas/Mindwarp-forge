$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
$bundled='C:\Users\zakwm\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
$python=if(Test-Path -LiteralPath $bundled -PathType Leaf){$bundled}else{'python'}
$verifier=Join-Path $PSScriptRoot 'verify-g1-c5-closure-result.py'
$temp=Join-Path ([IO.Path]::GetTempPath()) ('forge-c5-closure-'+[guid]::NewGuid().ToString('N'))
$paths=@(
 'context/active/WORKER_BATCH_STATE.json','docs/canonical-system/MASTER_PROGRAM.json','docs/canonical-system/G1_C5_LOCAL_PLATFORM_OBSERVATIONS.json',
 'docs/canonical-system/G1_C5_INDEPENDENT_PLATFORM_EXECUTION.json','docs/canonical-system/system-registry.json','docs/canonical-system/G1_C5_CLOSURE_RESULT.md',
 'docs/canonical-system/MASTER_CLOSURE_REGISTER.md','docs/canonical-system/PROOF_MATRIX.md','docs/canonical-system/UNRESOLVED_GAPS.md','docs/project-atlas/ROADMAP.md')
function Reset-Fixture {
 foreach($relative in $paths){$target=Join-Path $temp $relative;New-Item -ItemType Directory -Path (Split-Path -Parent $target) -Force|Out-Null;Copy-Item -LiteralPath (Join-Path $root $relative) -Destination $target -Force}
}
function Expect-Failure([string]$Label,[scriptblock]$Mutate){
 Reset-Fixture;& $Mutate
 $saved=$ErrorActionPreference;$ErrorActionPreference='Continue'
 & $python $verifier --root $temp 2>&1|Out-Null
 $exit=$LASTEXITCODE;$ErrorActionPreference=$saved
 if($exit-eq0){throw "C5 closure hostile was admitted: $Label"}
}
try{
 Reset-Fixture;& $python $verifier --root $temp|Out-Null;if($LASTEXITCODE-ne0){throw 'Canonical C5 closure fixture was rejected.'}
 Expect-Failure 'C5 left active after successor' { $p=Get-Content (Join-Path $temp 'docs/canonical-system/MASTER_PROGRAM.json') -Raw|ConvertFrom-Json;$c=@($p.items|? id -eq C5)[0];$c.state='executing';$c.status='active';$p|ConvertTo-Json -Depth 100|Set-Content (Join-Path $temp 'docs/canonical-system/MASTER_PROGRAM.json') -Encoding utf8 }
 Expect-Failure 'C6 readiness deactivated' { $p=Get-Content (Join-Path $temp 'docs/canonical-system/MASTER_PROGRAM.json') -Raw|ConvertFrom-Json;$c=@($p.items|? id -eq C6)[0];$c.state='proposed';$c.status='gated';$p|ConvertTo-Json -Depth 100|Set-Content (Join-Path $temp 'docs/canonical-system/MASTER_PROGRAM.json') -Encoding utf8 }
 Expect-Failure 'authority suffix' { $p=Join-Path $temp 'context/active/WORKER_BATCH_STATE.json';$j=Get-Content $p -Raw|ConvertFrom-Json;$j.authority_lane+=' forged';$j|ConvertTo-Json -Depth 100|Set-Content $p -Encoding utf8 }
 Expect-Failure 'missing closure receipt' { $p=Join-Path $temp 'context/active/WORKER_BATCH_STATE.json';$j=Get-Content $p -Raw|ConvertFrom-Json;$j.verification_receipts=@($j.verification_receipts|?{$_-ne'receipt:G1-C5-CLOSURE:recorded'});$j|ConvertTo-Json -Depth 100|Set-Content $p -Encoding utf8 }
 Expect-Failure 'missing post-transition gate' { $p=Join-Path $temp 'context/active/WORKER_BATCH_STATE.json';$j=Get-Content $p -Raw|ConvertFrom-Json;$j.verification_receipts=@($j.verification_receipts|?{$_-ne'registered-full-gate:run-8296afcac8e949cca8b6a3693d1dfc3f:passed'});$j|ConvertTo-Json -Depth 100|Set-Content $p -Encoding utf8 }
 Expect-Failure 'missing C6 owner route' { $p=Join-Path $temp 'context/active/WORKER_BATCH_STATE.json';$j=Get-Content $p -Raw|ConvertFrom-Json;$j.verification_receipts=@($j.verification_receipts|?{$_-ne'owner-route:c6-reconciliation-readiness:authorized'});$j|ConvertTo-Json -Depth 100|Set-Content $p -Encoding utf8 }
 Expect-Failure 'promoted hosted authority' { $p=Join-Path $temp 'docs/canonical-system/G1_C5_INDEPENDENT_PLATFORM_EXECUTION.json';$j=Get-Content $p -Raw|ConvertFrom-Json;$j.authority.promotion_authority=$true;$j|ConvertTo-Json -Depth 20|Set-Content $p -Encoding utf8 }
 Expect-Failure 'Android execution overclaim' { $p=Join-Path $temp 'docs/canonical-system/G1_C5_CLOSURE_RESULT.md';(Get-Content $p -Raw).Replace('Android ARM64 is honestly classified compile-only','Android ARM64 executed')|Set-Content $p -Encoding utf8 }
 Expect-Failure 'successor manifest drift' { $p=Join-Path $temp 'docs/canonical-system/G1_C5_CLOSURE_RESULT.md';(Get-Content $p -Raw).Replace('5f934a61dff9effa0fec1a7c219f53e3a510faa715e1e4c58622fd7a5585715c',('f'*64))|Set-Content $p -Encoding utf8 }
 Expect-Failure 'closure status drift' { $p=Join-Path $temp 'docs/canonical-system/G1_C5_CLOSURE_RESULT.md';(Get-Content $p -Raw).Replace('Status: **verified, complete and recorded.**','Status: **draft.**')|Set-Content $p -Encoding utf8 }
 Expect-Failure 'duplicate checkpoint key' { $p=Join-Path $temp 'context/active/WORKER_BATCH_STATE.json';$raw=Get-Content $p -Raw;$raw=[regex]::Replace($raw,'"batch_id"\s*:','"batch_id":"forged","batch_id":',1);Set-Content $p $raw -Encoding utf8 }
 Write-Output 'G1 C5 closure result hostiles verified: crossed state, C6 activation, authority, receipts, overclaim and strict JSON fail closed.'
}finally{if(Test-Path $temp){Remove-Item -LiteralPath $temp -Recurse -Force}}
