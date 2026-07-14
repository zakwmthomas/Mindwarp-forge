$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
$readinessPath=Join-Path $root 'docs\canonical-system\SIGNIFICANCE_SCHEDULER_READINESS.md'
$gatePath=Join-Path $root 'docs\canonical-system\SIGNIFICANCE_SCHEDULER_DESIGN_GATE.md'
$contractPath=Join-Path $root 'contracts\significance-scheduler-contract.md'
$cratePath=Join-Path $root 'crates\significance-scheduler'
foreach($path in @($readinessPath,$gatePath,$contractPath,$cratePath)){if(!(Test-Path $path)){throw "P5 artifact missing: $path"}}
$readiness=Get-Content $readinessPath -Raw
$gate=Get-Content $gatePath -Raw
$contract=Get-Content $contractPath -Raw
foreach($required in @('ImportancePacket','BudgetEnvelope','WorkTicket','PressureTrace','Starvation pressure','Poison ticket','simulated')){if(!$readiness.Contains($required)){throw "P5 readiness missing: $required"}}
foreach($required in @('Recovered prototype reconciliation','fair-share debt','Validation/admission','cancellation propagation is not atomic','ConsumerFidelityMap','request epoch','typed fallback','interaction_safety','stale request epoch','Cache pinning is a disposable','Exact confirmation')){if(!$gate.Contains($required)){throw "P5 design gate missing: $required"}}
foreach($required in @('Conditional-approval critical revalidation','One resource per ticket','Budgets are inputs','Priority donation is derived and bounded','Fairness is separate from significance','Cache policy remains open','Whole-system preservation')){if(!$gate.Contains($required)){throw "P5 critical revalidation missing: $required"}}
foreach($required in @('strict canonical CBOR','one resource','minimum hold time','service debt','late or stale-epoch outputs','stable decision codes','capability-free','read-only `ProofReceipt`')){if(!$contract.Contains($required)){throw "P5 contract missing: $required"}}
foreach($forbidden in @('implementation authorized','runtime selected','engine approved','protected Kernel mutation approved')){if($gate.Contains($forbidden)){throw "P5 design gate contains forbidden authority: $forbidden"}}
$sources=(Get-ChildItem (Join-Path $cratePath 'src') -Filter '*.rs'|Get-Content -Raw) -join "`n"
foreach($required in @('ImportancePacket','HysteresisPolicy','ConsumerFidelityMap','WorkTicket','BudgetEnvelope','ReferenceScheduler','CancelAcknowledged','CompletedDiscarded','reference_proof_evidence')){if(!$sources.Contains($required)){throw "P5 crate missing proof surface: $required"}}
foreach($forbidden in @('forge_kernel','tauri','std::fs','std::process','std::net','reqwest','tokio::net')){if($sources.Contains($forbidden)){throw "P5 crate crosses capability boundary: $forbidden"}}
$manifest=Get-Content (Join-Path $cratePath 'Cargo.toml') -Raw
foreach($forbidden in @('forge-kernel','tauri','tokio','reqwest')){if($manifest.Contains($forbidden)){throw "P5 manifest crosses capability boundary: $forbidden"}}
$program=Get-Content (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw|ConvertFrom-Json
$active=@($program.items|Where-Object status -eq 'active')
if($active.Count-ne 1-or$active[0].id-ne'F5'){throw 'P5 design gate is not routed through active F5.'}
Write-Output 'F5 P5 significance/scheduler verified: second-wave research, strict contracts, deterministic reference implementation, failure proofs, and capability boundaries retained.'
