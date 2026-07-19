$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
$temp=Join-Path ([IO.Path]::GetTempPath()) ('forge-c5-readiness-'+[guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $temp|Out-Null
try{
  $program=Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw|ConvertFrom-Json
  $checkpoint=Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw|ConvertFrom-Json
  $readiness=Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C5_CLOSURE_READINESS.md') -Raw
  $programPath=Join-Path $temp 'program.json';$checkpointPath=Join-Path $temp 'checkpoint.json';$readinessPath=Join-Path $temp 'readiness.md'
  function Save-Fixture{$program|ConvertTo-Json -Depth 100|Set-Content -LiteralPath $programPath -Encoding utf8;$checkpoint|ConvertTo-Json -Depth 100|Set-Content -LiteralPath $checkpointPath -Encoding utf8;Set-Content -LiteralPath $readinessPath -Value $readiness -Encoding utf8}
  function Assert-Rejected([scriptblock]$mutation,[string]$label){&$mutation;Save-Fixture;try{& (Join-Path $root 'tools\verify-g1-c5-closure-readiness.ps1') -ProgramPath $programPath -CheckpointPath $checkpointPath -ReadinessPath $readinessPath|Out-Null;throw "Forged C5 readiness admitted: $label"}catch{if($_.Exception.Message-eq"Forged C5 readiness admitted: $label"){throw}}}
  Save-Fixture;& (Join-Path $root 'tools\verify-g1-c5-closure-readiness.ps1') -ProgramPath $programPath -CheckpointPath $checkpointPath -ReadinessPath $readinessPath|Out-Null
  $saved=$checkpoint.substage_id;Assert-Rejected {$checkpoint.substage_id='forged'} 'substage';$checkpoint.substage_id=$saved
  $saved=$checkpoint.authority_lane;Assert-Rejected {$checkpoint.authority_lane=$checkpoint.authority_lane.Replace('cache mutation','')} 'authority';$checkpoint.authority_lane=$saved
  $c4=@($program.items|Where-Object id -eq 'C4')[0];$saved=$c4.status;Assert-Rejected {$c4.status='active'} 'C4 state';$c4.status=$saved
  $c5=@($program.items|Where-Object id -eq 'C5')[0];$saved=@($c5.depends_on);Assert-Rejected {$c5.depends_on=@('C4','C3B')} 'C5 dependency';$c5.depends_on=$saved
  $saved=$readiness;Assert-Rejected {$script:readiness=$script:readiness.Replace('`truth.packet-tier-forged`','`truth.packet-tier-missing`')} 'hostile registry';$readiness=$saved
  Write-Output 'G1 C5 readiness hostiles verified: forged route, authority, dependency and registry evidence fail closed.'
}finally{Remove-Item -LiteralPath $temp -Recurse -Force -ErrorAction SilentlyContinue}
