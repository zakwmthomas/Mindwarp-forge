param([string]$ReceiptPath,[switch]$ComputeOnly)
$ErrorActionPreference='Stop';$root=Split-Path -Parent $PSScriptRoot;if([string]::IsNullOrWhiteSpace($ReceiptPath)){$ReceiptPath=Join-Path $root 'docs\canonical-system\G1_VERTICAL_CLOSEOUT_RECEIPT.json'}
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
    $gp4=$program.items|Where-Object id -eq 'GP4';$closeout=$program.items|Where-Object id -eq 'G1-VERTICAL-CLOSEOUT'
    if($gp4.state-ne 'verified'-or$gp4.status-ne 'complete'){throw 'GP4 is not verified/complete.'}
    if($closeout.state-ne 'executing'-or$closeout.status-ne 'active'){throw 'Bounded closeout is not executing/active.'}
    if((@($closeout.depends_on)-join ',')-ne 'C3A,C4V,GP0,GP1,GP2,GP3,GP4'){throw 'Bounded closeout dependency order changed.'}
    $active=@($program.items|Where-Object{$_.state-eq'executing'-and$_.status-eq'active'})
    if ($active.Count -ne 1 -or $active[0].id -ne 'G1-VERTICAL-CLOSEOUT') { throw 'Bounded closeout is not the sole executing/active program item.' }
    $metricPath=Join-Path $root '.local\forge-metrics\inbox\run-7e5c44dc8f48424a8cec42da756e3127.json'
    if(Test-Path -LiteralPath $metricPath){$metric=Get-Content -Raw $metricPath|ConvertFrom-Json
      if ($metric.id -ne 'run-7e5c44dc8f48424a8cec42da756e3127' -or $metric.outcome -ne 'passed' -or $metric.metric_name -ne 'wall_duration_ms' -or [uint64]$metric.metric_value -ne 590582 -or $metric.work_package_id -ne 'G1-GP4-SIGNAL-ANCHOR-VERTICAL-V1') { throw 'Registered GP4 metric evidence does not match the closeout receipt.' }
      if ((@($metric.evidence_ids) -join ',') -ne 'run-definition:forge-full-gate-v1,checkpoint-sha256:8427844116d40de75119565aadd056182c062273262c78d7fa509c3b7f47b93c') { throw 'Registered GP4 metric checkpoint binding changed.' }
    }
    $checkpoint=Get-Content -Raw (Join-Path $root 'context\active\WORKER_BATCH_STATE.json')|ConvertFrom-Json
    if ($checkpoint.batch_id -ne 'G1-VERTICAL-CLOSEOUT-V1' -or $checkpoint.master_program_item -ne 'G1-VERTICAL-CLOSEOUT' -or $checkpoint.state -ne 'recorded' -or $checkpoint.substage_id -ne 'g1-vertical-closeout-recorded') { throw 'Canonical checkpoint is not the recorded closeout successor.' }
    if (@($checkpoint.verification_receipts) -notcontains 'registered-full-gate:run-7e5c44dc8f48424a8cec42da756e3127:passed' -or @($checkpoint.verification_receipts) -notcontains 'receipt:G1-VERTICAL-CLOSEOUT:recorded') { throw 'Canonical checkpoint lacks exact closeout receipts.' }
  }
}
finally{$env:FORGE_ROOT=$priorForgeRoot;$env:RECEIPT_PATH=$priorReceipt;$env:COMPUTE_ONLY=$priorCompute;Remove-Item -LiteralPath $temp -Recurse -Force -ErrorAction SilentlyContinue}
