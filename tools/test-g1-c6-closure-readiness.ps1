$ErrorActionPreference='Stop';$root=Split-Path -Parent $PSScriptRoot;$verify=Join-Path $PSScriptRoot 'verify-g1-c6-closure-readiness.ps1'
$temp=Join-Path ([IO.Path]::GetTempPath()) ('forge-c6-ready-'+[guid]::NewGuid().ToString('N'));New-Item -ItemType Directory $temp|Out-Null
$program=Join-Path $temp 'program.json';$checkpoint=Join-Path $temp 'checkpoint.json';$readiness=Join-Path $temp 'readiness.md'
function Reset-Fixture{Copy-Item (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') $program -Force;Copy-Item (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') $checkpoint -Force;Copy-Item (Join-Path $root 'docs\canonical-system\G1_C6_CLOSURE_READINESS.md') $readiness -Force}
function Must-Fail([string]$label,[scriptblock]$mutate){Reset-Fixture;&$mutate;$saved=$ErrorActionPreference;$ErrorActionPreference='Continue';& powershell.exe -NoProfile -ExecutionPolicy Bypass -File $verify -ProgramPath $program -CheckpointPath $checkpoint -ReadinessPath $readiness 2>&1|Out-Null;$exit=$LASTEXITCODE;$ErrorActionPreference=$saved;if($exit-eq0){throw "C6 readiness hostile admitted: $label"}}
try{Reset-Fixture;&$verify -ProgramPath $program -CheckpointPath $checkpoint -ReadinessPath $readiness|Out-Null
Must-Fail 'C5 not complete' {$p=Get-Content $program -Raw|ConvertFrom-Json;$x=@($p.items|? id -eq C5)[0];$x.state='executing';$x.status='active';$p|ConvertTo-Json -Depth 100|Set-Content $program -Encoding utf8}
Must-Fail 'C6 dependency drift' {$p=Get-Content $program -Raw|ConvertFrom-Json;$x=@($p.items|? id -eq C6)[0];$x.depends_on=@('C4');$p|ConvertTo-Json -Depth 100|Set-Content $program -Encoding utf8}
Must-Fail 'implementation authority' {$p=Get-Content $checkpoint -Raw|ConvertFrom-Json;$p.authority_lane=$p.authority_lane.Replace('No C6 implementation source, ','');$p|ConvertTo-Json -Depth 100|Set-Content $checkpoint -Encoding utf8}
Must-Fail 'missing hostile domain' {(Get-Content $readiness -Raw).Replace('C6-H1100..1108','C6-H1100')|Set-Content $readiness -Encoding utf8}
Must-Fail 'missing exact hostile identity' {(Get-Content $readiness -Raw).Replace('`C6-H1209`','`C6-H1208`')|Set-Content $readiness -Encoding utf8}
Must-Fail 'test count laundering' {(Get-Content $readiness -Raw).Replace('`organism-niche-binding` 5','`organism-niche-binding` 4')|Set-Content $readiness -Encoding utf8}
Write-Output 'G1 C6 closure readiness hostiles verified: dependency, authority, registry and evidence-count drift fail closed.'}
finally{Remove-Item -LiteralPath $temp -Recurse -Force -ErrorAction SilentlyContinue}
