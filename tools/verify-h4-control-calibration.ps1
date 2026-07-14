$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$cargo = 'C:\Users\zakwm\.cargo\bin\cargo.exe'
$output = & $cargo run -q -p control-calibration --example h4_receipt 2>&1
if ($LASTEXITCODE -ne 0) { throw 'H4 receipt example failed.' }
$receipt = $output -join "`n"
foreach ($required in @(
  'calibration_fingerprint=774a790aa963bb7ed329394d869fda4f5530697cce4d4d029a23d31e6d575f4d',
  'control=BrokenConnection;fingerprint=cb1cd360a0e3a57fdfe2a86b6ee025609597b601a4c7afa1a385d979884af33a;edge_deficit=1;rest_span_loss=0;hand_vertical_displacement=0',
  'control=SilhouetteCollapse;fingerprint=f1a3693ae5ecb962fa278af953b29cb9f3acabef2cd19a062ff8b1b42beefed2;edge_deficit=0;rest_span_loss=480;hand_vertical_displacement=0',
  'control=ArticulationDrift;fingerprint=cc03ed8673a508af921021df69e546347a87c2342171f945eb770eabfa832b08;edge_deficit=0;rest_span_loss=0;hand_vertical_displacement=480'
)) { if (!$receipt.Contains($required)) { throw "H4 calibration drifted: $required" } }

$source=Get-Content (Join-Path $root 'crates\control-calibration\src\lib.rs') -Raw
$contract=Get-Content (Join-Path $root 'contracts\control-calibration-contract.md') -Raw
$result=Get-Content (Join-Path $root 'docs\canonical-system\H4_HUMANOID_FUNCTIONAL_CONTROLS_RESULT.md') -Raw
foreach($required in @('does_not_detect','exact_integer_difference_from_bound_reference','score_above_0.8','not a verified good-quality visual human','Actual-pixel inspection')) {
  if(!$source.Contains($required) -and !$contract.Contains($required) -and !$result.Contains($required)){throw "H4 claim limit missing: $required"}
}
Write-Output 'H4 controls verified: exact orthogonal integer matrix, fixed H3/reference/control bindings, explicit non-detections, and no visual-quality inference retained.'
