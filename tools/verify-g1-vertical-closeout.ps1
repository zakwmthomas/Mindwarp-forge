param([string]$ReceiptPath,[switch]$ComputeOnly)
$ErrorActionPreference='Stop';$root=Split-Path -Parent $PSScriptRoot;. (Join-Path $PSScriptRoot 'g1-c5-successor-route.ps1');. (Join-Path $PSScriptRoot 'g1-c6-successor-route.ps1');if([string]::IsNullOrWhiteSpace($ReceiptPath)){$ReceiptPath=Join-Path $root 'docs\canonical-system\G1_VERTICAL_CLOSEOUT_RECEIPT.json'}
$temp=Join-Path ([IO.Path]::GetTempPath()) ('forge-gp4-receipt-'+[guid]::NewGuid().ToString('N'));New-Item -ItemType Directory -Path (Join-Path $temp 'src') -Force|Out-Null;$priorForgeRoot=$env:FORGE_ROOT;$priorReceipt=$env:RECEIPT_PATH;$priorCompute=$env:COMPUTE_ONLY
try{$manifest=@"
[package]
name="gp4-receipt-proof"
version="0.1.0"
edition="2024"
[dependencies]
derived-world-rules={path="$($root.Replace('\','/'))/crates/derived-world-rules"}
mindwarp-gameplay-foundation={path="$($root.Replace('\','/'))/crates/mindwarp-gameplay-foundation"}
mindwarp-signal-anchor-vertical={path="$($root.Replace('\','/'))/crates/mindwarp-signal-anchor-vertical"}
mindwarp-vertical-persistence={path="$($root.Replace('\','/'))/crates/mindwarp-vertical-persistence"}
field-basis={path="$($root.Replace('\','/'))/crates/field-basis"}
serde={version="1",features=["derive"]}
serde_json="1"
sha2="0.10"
"@;Set-Content -LiteralPath (Join-Path $temp 'Cargo.toml') -Value $manifest -Encoding utf8;Copy-Item -LiteralPath (Join-Path $root 'tools\fixtures\gp4-signal-anchor-receipt\main.rs') -Destination (Join-Path $temp 'src\main.rs');$env:FORGE_ROOT=$root;$env:RECEIPT_PATH=$ReceiptPath;if($ComputeOnly){$env:COMPUTE_ONLY='1'}else{Remove-Item Env:COMPUTE_ONLY -ErrorAction SilentlyContinue};cargo run --quiet --offline --manifest-path (Join-Path $temp 'Cargo.toml');if($LASTEXITCODE-ne 0){throw 'Closeout receipt verification failed.'}
  if(-not $ComputeOnly){
    $program=Get-Content -Raw (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json')|ConvertFrom-Json
    $sha=[Security.Cryptography.SHA256]::Create()
    try {
      foreach($pin in @(@('G1-CLOSEOUT','0f568deda22a0901030948cb9c9355916cbfe2319e943267f1cd6135c5fe39a5'),@('R1','87ef69f50f4d4cd7fa297d19cae56e77db1cac53605f12849c3cc380156b9869'))){
        $item=$program.items|Where-Object id -eq $pin[0]
        if($null -eq $item){throw "Missing pinned $($pin[0]) item."}
        $bytes=[Text.Encoding]::UTF8.GetBytes(($item|ConvertTo-Json -Depth 20 -Compress))
        $actual=(($sha.ComputeHash($bytes)|ForEach-Object{$_.ToString('x2')})-join '')
        if($actual-ne $pin[1]){throw "$($pin[0]) changed during bounded closeout: $actual"}
        $hostile=$item|ConvertTo-Json -Depth 20|ConvertFrom-Json;$hostile.state='verified'
        $hostileBytes=[Text.Encoding]::UTF8.GetBytes(($hostile|ConvertTo-Json -Depth 20 -Compress))
        $hostileHash=(($sha.ComputeHash($hostileBytes)|ForEach-Object{$_.ToString('x2')})-join '')
        if($hostileHash-eq $pin[1]){throw "$($pin[0]) hostile mutation was not detected."}
      }
    } finally {$sha.Dispose()}
    $gp4=$program.items|Where-Object id -eq 'GP4';$closeout=$program.items|Where-Object id -eq 'G1-VERTICAL-CLOSEOUT';$c4=$program.items|Where-Object id -eq 'C4';$c5=$program.items|Where-Object id -eq 'C5'
    if($gp4.state-ne 'verified'-or$gp4.status-ne 'complete'){throw 'GP4 is not verified/complete.'}
    if((@($closeout.depends_on)-join ',')-ne 'C3A,C4V,GP0,GP1,GP2,GP3,GP4'){throw 'Bounded closeout dependency order changed.'}
    $checkpoint=Get-Content -Raw (Join-Path $root 'context\active\WORKER_BATCH_STATE.json')|ConvertFrom-Json
    $closeoutLive=$checkpoint.batch_id -eq 'G1-VERTICAL-CLOSEOUT-V1' -and $checkpoint.master_program_item -eq 'G1-VERTICAL-CLOSEOUT' -and $checkpoint.state -eq 'recorded' -and $checkpoint.substage_id -eq 'g1-vertical-closeout-recorded' -and $closeout.state -eq 'executing' -and $closeout.status -eq 'active'
    $c4Successor=$checkpoint.batch_id -eq 'G1-C4-HIERARCHY-HISTORY-CLOSURE-V1' -and $checkpoint.master_program_item -eq 'C4' -and $checkpoint.substage_id -in @('c4-reconciliation-readiness','c4-hierarchy-history-hardening','c4-verification','c4-verified-result','c4-independent-platform-gate') -and $closeout.state -eq 'verified' -and $closeout.status -eq 'complete' -and $c4.state -eq 'executing' -and $c4.status -eq 'active' -and (@($c4.depends_on)-join ',') -eq 'C2,C3A'
    $c4Run=[regex]::Match([string]$c4.proof,'run-[0-9a-f]{32}')
    $c6Route=Test-G1C6ReconciliationReadinessRoute -Checkpoint $checkpoint
    $c5Route=($checkpoint.batch_id -eq 'G1-C5-SIGNIFICANCE-SCHEDULER-CLOSURE-V1' -and $checkpoint.master_program_item -eq 'C5' -and $checkpoint.substage_id -eq 'c5-reconciliation-readiness' -and $checkpoint.authority_lane -eq 'Owner-authorized broad C5 significance/scheduler reconciliation and capability-free closure readiness only. Exact dependency C4. No C3B, C6, C7, broad G1 closure, runtime controllers, runtime executors, cache mutation, storage mutation, product weights, AI generation, rendering implementation, filesystem, network, process, Companion, Greenfield, visual assets or Kernel mutation.')-or(Test-G1C5FullGateReconciliationRoute -Checkpoint $checkpoint)-or(Test-G1C5RecordedClosureRoute -Checkpoint $checkpoint)-or$c6Route
    $c5Successor=$c5Route -and $closeout.state -eq 'verified' -and $closeout.status -eq 'complete' -and $c4.state -eq 'verified' -and $c4.status -eq 'complete' -and @($c4.sources) -contains 'G1_C4_CLOSURE_RESULT.md' -and $c4Run.Success -and @($checkpoint.verification_receipts) -contains "registered-full-gate:$($c4Run.Value):passed" -and @($checkpoint.verification_receipts) -contains 'receipt:G1-C4-CLOSURE:recorded' -and (($c6Route -and $c5.state -eq 'verified' -and $c5.status -eq 'complete')-or(!$c6Route -and $c5.state -eq 'executing' -and $c5.status -eq 'active')) -and (@($c5.depends_on)-join ',') -eq 'C4'
    if(!$closeoutLive -and !$c4Successor -and !$c5Successor){throw 'Bounded closeout is neither live nor retained by an exact C4/C5 successor.'}
    $active=@($program.items|Where-Object{$_.state-eq'executing'-and$_.status-eq'active'})
    $expectedActive=if($c6Route){'C6'}elseif($c5Successor){'C5'}elseif($c4Successor){'C4'}else{'G1-VERTICAL-CLOSEOUT'}
    if ($active.Count -ne 1 -or $active[0].id -ne $expectedActive) { throw 'The authenticated closeout route is not the sole executing/active program item.' }
    $metricPath=Join-Path $root '.local\forge-metrics\inbox\run-7e5c44dc8f48424a8cec42da756e3127.json'
    if(Test-Path -LiteralPath $metricPath){$metric=Get-Content -Raw $metricPath|ConvertFrom-Json
      if ($metric.id -ne 'run-7e5c44dc8f48424a8cec42da756e3127' -or $metric.outcome -ne 'passed' -or $metric.metric_name -ne 'wall_duration_ms' -or [uint64]$metric.metric_value -ne 590582 -or $metric.work_package_id -ne 'G1-GP4-SIGNAL-ANCHOR-VERTICAL-V1') { throw 'Registered GP4 metric evidence does not match the closeout receipt.' }
      if ((@($metric.evidence_ids) -join ',') -ne 'run-definition:forge-full-gate-v1,checkpoint-sha256:8427844116d40de75119565aadd056182c062273262c78d7fa509c3b7f47b93c') { throw 'Registered GP4 metric checkpoint binding changed.' }
    }
    if (@($checkpoint.verification_receipts) -notcontains 'registered-full-gate:run-7e5c44dc8f48424a8cec42da756e3127:passed' -or @($checkpoint.verification_receipts) -notcontains 'receipt:G1-VERTICAL-CLOSEOUT:recorded') { throw 'Canonical checkpoint lacks exact closeout receipts.' }
  }
}
finally{$env:FORGE_ROOT=$priorForgeRoot;$env:RECEIPT_PATH=$priorReceipt;$env:COMPUTE_ONLY=$priorCompute;Remove-Item -LiteralPath $temp -Recurse -Force -ErrorAction SilentlyContinue}
