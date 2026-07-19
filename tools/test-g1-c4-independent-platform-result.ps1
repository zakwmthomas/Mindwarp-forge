$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$bundled = 'C:\Users\zakwm\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
$python = if (Test-Path -LiteralPath $bundled -PathType Leaf) { $bundled } else { 'python3' }
& $python (Join-Path $PSScriptRoot 'test-g1-c4-independent-platform-result.py')
if ($LASTEXITCODE -ne 0) { throw 'C4 retained independent-platform hostile fixtures failed.' }
$temporary = Join-Path ([IO.Path]::GetTempPath()) ('forge-c4-replay-scope-' + [guid]::NewGuid().ToString('N'))
try {
    New-Item -ItemType Directory -Path $temporary -Force | Out-Null
    $checkpoint = Join-Path $temporary 'checkpoint.json'
    $missing = Join-Path $temporary 'missing-receipt.json'
    @{batch_id='UNRELATED-V1';master_program_item='B4';state='executing';substage_id='unrelated-stage'} | ConvertTo-Json | Set-Content -LiteralPath $checkpoint -Encoding utf8
    & powershell -NoProfile -ExecutionPolicy Bypass -File (Join-Path $PSScriptRoot 'verify-g1-c4-independent-platform-result.ps1') -CheckpointPath $checkpoint -ReceiptPath $missing | Out-Null
    if ($LASTEXITCODE -ne 0) { throw 'Unrelated checkpoint did not skip C4 replay.' }
    @{batch_id='G1-C4-HIERARCHY-HISTORY-CLOSURE-V1';master_program_item='C4';state='executing';substage_id='c4-independent-platform-gate'} | ConvertTo-Json | Set-Content -LiteralPath $checkpoint -Encoding utf8
    $saved = $ErrorActionPreference; $ErrorActionPreference = 'Continue'
    & powershell -NoProfile -ExecutionPolicy Bypass -File (Join-Path $PSScriptRoot 'verify-g1-c4-independent-platform-result.ps1') -CheckpointPath $checkpoint -ReceiptPath $missing 2>&1 | Out-Null
    $targetExit = $LASTEXITCODE; $ErrorActionPreference = $saved
    if ($targetExit -eq 0) { throw 'Target C4 checkpoint admitted a missing retained receipt.' }
    @{batch_id='G1-C5-SIGNIFICANCE-SCHEDULER-CLOSURE-V1';master_program_item='C5';state='executing';substage_id='c5-reconciliation-readiness'}|ConvertTo-Json|Set-Content -LiteralPath $checkpoint -Encoding utf8
    $saved=$ErrorActionPreference;$ErrorActionPreference='Continue'; & powershell -NoProfile -ExecutionPolicy Bypass -File (Join-Path $PSScriptRoot 'verify-g1-c4-independent-platform-result.ps1') -CheckpointPath $checkpoint -ReceiptPath $missing 2>&1|Out-Null;$c5Exit=$LASTEXITCODE;$ErrorActionPreference=$saved
    if($c5Exit-eq0){throw 'Exact C5 successor admitted a missing retained C4 receipt.'}
    @{batch_id='G1-C5-SIGNIFICANCE-SCHEDULER-CLOSURE-V1';master_program_item='C5';state='executing';substage_id='forged'}|ConvertTo-Json|Set-Content -LiteralPath $checkpoint -Encoding utf8
    $saved=$ErrorActionPreference;$ErrorActionPreference='Continue'; & powershell -NoProfile -ExecutionPolicy Bypass -File (Join-Path $PSScriptRoot 'verify-g1-c4-independent-platform-result.ps1') -CheckpointPath $checkpoint -ReceiptPath $missing 2>&1|Out-Null;$forgedExit=$LASTEXITCODE;$ErrorActionPreference=$saved
    if($forgedExit-eq0){throw 'Malformed C5 successor replay route was admitted.'}

    $program=Get-Content -Raw (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json')|ConvertFrom-Json
    $state=Get-Content -Raw (Join-Path $root 'context\active\WORKER_BATCH_STATE.json')|ConvertFrom-Json
    $programPath=Join-Path $temporary 'program.json';$statePath=Join-Path $temporary 'state.json'
    function Save-Records{$program|ConvertTo-Json -Depth 100|Set-Content $programPath -Encoding utf8;$state|ConvertTo-Json -Depth 100|Set-Content $statePath -Encoding utf8}
    Save-Records
    & (Join-Path $PSScriptRoot 'verify-g1-c4-record-consistency.ps1') -ProgramPath $programPath -CheckpointPath $statePath|Out-Null
    $c4=@($program.items|Where-Object id -eq 'C4')[0];$c5=@($program.items|Where-Object id -eq 'C5')[0]
    $c4.state='verified';$c4.status='complete';$c4.proof+=' Registered complete gate run-0123456789abcdef0123456789abcdef.';$c4.sources=@($c4.sources)+'G1_C4_CLOSURE_RESULT.md';$c5.state='executing';$c5.status='active'
    $state.batch_id='G1-C5-SIGNIFICANCE-SCHEDULER-CLOSURE-V1';$state.master_program_item='C5';$state.state='executing';$state.substage_id='c5-reconciliation-readiness';$state.authority_lane='Owner-authorized broad C5 significance/scheduler reconciliation and capability-free closure readiness only. Exact dependency C4. No C3B, C6, C7, broad G1 closure, runtime controllers, runtime executors, cache mutation, storage mutation, product weights, AI generation, rendering implementation, filesystem, network, process, Companion, Greenfield, visual assets or Kernel mutation.';$state.verification_receipts=@($state.verification_receipts)+'registered-full-gate:run-0123456789abcdef0123456789abcdef:passed'+'receipt:G1-C4-CLOSURE:recorded'
    Save-Records
    & (Join-Path $PSScriptRoot 'verify-g1-c4-record-consistency.ps1') -ProgramPath $programPath -CheckpointPath $statePath|Out-Null
    function Reject-Record([string]$label,[scriptblock]$mutate,[scriptblock]$restore){&$mutate;Save-Records;try{& (Join-Path $PSScriptRoot 'verify-g1-c4-record-consistency.ps1') -ProgramPath $programPath -CheckpointPath $statePath|Out-Null;throw "$label admitted"}catch{if($_.Exception.Message-eq"$label admitted"){throw}};&$restore}
    $s=$state.substage_id;Reject-Record 'forged C5 substage' {$state.substage_id='forged'} {$state.substage_id=$s}
    $r=@($state.verification_receipts);Reject-Record 'missing C4 closure receipt' {$state.verification_receipts=@($r|Where-Object{$_-ne'receipt:G1-C4-CLOSURE:recorded'})} {$state.verification_receipts=$r}
    $d=@($c5.depends_on);Reject-Record 'altered C5 dependency' {$c5.depends_on=@()} {$c5.depends_on=$d}
    $a=$state.authority_lane;Reject-Record 'forged C5 authority' {$state.authority_lane='forged'} {$state.authority_lane=$a}
} finally {
    if (Test-Path -LiteralPath $temporary) { Remove-Item -LiteralPath $temporary -Recurse -Force }
}
Write-Output 'C4 independent-platform replay scope verified: unrelated stages skip; C4 and exact C5 successor stages fail closed.'
