Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$source = Join-Path $root 'tools\prove-g1-c3-whole-cell-receiver-coupling-width.py'
$result = Join-Path $root 'docs\canonical-system\G1_C3_WHOLE_CELL_RECEIVER_COUPLING_WIDTH_SPIKE_RESULT.md'
$readinessPath = Join-Path $root 'docs\canonical-system\G1_C3_WHOLE_CELL_RECEIVER_COUPLING_IMPLEMENTATION_READINESS.md'
foreach ($path in @($source,$result,$readinessPath)) { if (-not (Test-Path -LiteralPath $path)) { throw "Missing receiver-coupling readiness artifact: $path" } }
if ((Get-FileHash -LiteralPath $source -Algorithm SHA256).Hash.ToLowerInvariant() -ne '173d8e45c3c3f7944c7cae43698c722df3df679066c5ce5be0429dddccc57292') { throw 'Receiver-coupling width source drifted.' }
$python = Join-Path $env:USERPROFILE '.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
if (-not (Test-Path -LiteralPath $python)) { $python = 'python' }
$receipt = (& $python $source | Out-String | ConvertFrom-Json)
if ($LASTEXITCODE -ne 0 -or $receipt.status -ne 'pass' -or $receipt.maximum_live_bits -ne 391 -or $receipt.storage_margin_bits -ne 121 -or $receipt.public_reduced_form_product_bits -ne 980 -or $receipt.classifier_checks -ne 1020 -or $receipt.checksum -ne '71c6e716283348cd690887e00b265cb33997a62dc22c0b6367f68a172d969ea6') { throw 'Receiver-coupling width receipt drifted.' }
$resultText = Get-Content -LiteralPath $result -Raw
foreach ($required in @('391-bit','980-bit','121 bits','1,020','production')) { if ($resultText -notlike "*$required*") { throw "Receiver-coupling width result drift: $required" } }
$readiness = Get-Content -LiteralPath $readinessPath -Raw
foreach ($required in @('owner-approved on 2026-07-17','optical-phase-space-receiver-coupling','OriginAnchoredTransportInputV1','OriginAnchoredTransportCertificateV1','physical-path-substrate','ReceiverAabbV1','CertifiedFullBeforeFace','CertifiedZeroBeforeFace','UnresolvedReceiverCoupling','mindwarp.optical-phase-space.receiver-coupling.input.v1','mindwarp.optical-phase-space.receiver-coupling.result.v1','391 bits','16,384','4,096','40 MiB','256 KiB','64 MiB','i686-pc-windows-msvc','aarch64-linux-android','deletion-only','Exact serious owner action','delegated mathematical and engineering decisions')) { if ($readiness -notlike "*$required*") { throw "Receiver-coupling readiness drift: $required" } }
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$c3 = @($program.items | Where-Object id -eq 'C3')
if ($c3.Count -ne 1 -or -not (& (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint -C3 $c3[0])) { throw 'Receiver-coupling readiness is not aligned to the preserved C3 route.' }
Write-Output 'Whole-cell receiver-coupling readiness verified: origin replay, 391-bit shield, conservative outcomes, exact measure, resource/platform gates, deletion rollback and serious owner action are frozen.'
