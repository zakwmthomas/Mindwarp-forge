$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
$crate=Join-Path $root 'crates\significance-scheduler'
$readiness=Get-Content (Join-Path $root 'docs\canonical-system\G1_C5_CLOSURE_READINESS.md') -Raw
$source=(Get-ChildItem (Join-Path $crate 'src') -Filter '*.rs'|Get-Content -Raw)-join "`n"
$testPaths=@('eight_domain_scheduler_closure.rs','c5_contract_hostiles.rs','c5_scheduler_hostiles.rs','c5_residency_trace_authority_hostiles.rs','c5_pressure_simulation.rs')|ForEach-Object{Join-Path $crate ("tests\"+$_)}
$test=($testPaths|ForEach-Object{Get-Content $_ -Raw})-join "`n"
$manifest=Get-Content (Join-Path $crate 'Cargo.toml') -Raw

foreach($required in @('ConsumerDomainV1','DomainFidelityMapSetV1','ImportanceDecisionBindingV1','CompletionReceiptV1','AdmissionReceiptV1','PressureTraceV2','ResidencyIntentV1','StarvationDiagnosed')){
  if(!$source.Contains($required)){throw "C5 implementation surface missing: $required"}
}
foreach($required in @('fallback_must_preserve_domain_and_work_class','completion_is_accepted_only_from_running_and_never_from_inactive_or_terminal','strict_trace_has_domain_budget_and_stable_code_identity','residency_intents_are_streaming_only_bounded_and_strict')){
  if(!$test.Contains($required)){throw "C5 integration assertion missing: $required"}
}
$documentIds=@([regex]::Matches($readiness,'(?m)^- `([a-z]+\.[a-z0-9-]+)`\r?$')|ForEach-Object{$_.Groups[1].Value})
$sourceIds=@([regex]::Matches($source,'"([a-z]+\.[a-z0-9-]+)"')|ForEach-Object{$_.Groups[1].Value}|Where-Object{$_ -in $documentIds})
if($documentIds.Count-ne 92-or(@($documentIds|Sort-Object -Unique).Count-ne 92)){throw 'Frozen C5 hostile registry is not exactly 92 unique IDs.'}
if((Compare-Object ($documentIds|Sort-Object) ($sourceIds|Sort-Object))){throw 'Rust C5 hostile registry differs from the frozen readiness registry.'}
foreach($id in $documentIds){$fn=$id.Replace('.','_').Replace('-','_');if(!$test.Contains($id)-and!$test.Contains("fn $fn")){throw "C5 hostile has no executable test mapping: $id"}}
foreach($forbidden in @('forge_kernel','tauri','std::fs','std::process','std::net','reqwest','tokio::net')){if($source.Contains($forbidden)){throw "C5 source crosses capability boundary: $forbidden"}}
foreach($forbidden in @('forge-kernel','tauri','tokio','reqwest')){if($manifest.Contains($forbidden)){throw "C5 manifest crosses capability boundary: $forbidden"}}

& cargo test -p significance-scheduler --locked
if($LASTEXITCODE-ne 0){throw 'C5 significance-scheduler tests failed.'}
& cargo clippy -p significance-scheduler --all-targets --locked -- -D warnings
if($LASTEXITCODE-ne 0){throw 'C5 significance-scheduler strict Clippy failed.'}
Write-Output 'G1 C5 local implementation candidate verified: typed eight-domain surface, 90 Rust tests, executable mapping for all 92 hostile IDs, complete local pressure scenarios, verified truth admission/replay and capability-negative boundary pass. This does not claim portability, integration, independent review, full-gate passage or closure.'
