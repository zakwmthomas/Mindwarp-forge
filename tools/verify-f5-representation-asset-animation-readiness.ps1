$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
$attributesPath=Join-Path $root '.gitattributes'
if(!(Test-Path -LiteralPath $attributesPath)-or!(Get-Content -LiteralPath $attributesPath -Raw).Contains('evidence/** -text')){throw 'Immutable evidence bytes are not protected from Git text conversion.'}
$readinessPath=Join-Path $root 'docs\canonical-system\REPRESENTATION_ASSET_ANIMATION_READINESS.md'
$gatePath=Join-Path $root 'docs\canonical-system\REPRESENTATION_ASSET_ANIMATION_DESIGN_GATE.md'
$p7bGatePath=Join-Path $root 'docs\canonical-system\P7B_CONTROLLED_PERCEPTION_DESIGN_GATE.md'
$p7b1GatePath=Join-Path $root 'docs\canonical-system\P7B1_CONTAINMENT_DESIGN_GATE.md'
$p7b1bGatePath=Join-Path $root 'docs\canonical-system\P7B1B_DENIAL_CANARY_DESIGN_GATE.md'
$p7b1bResultPath=Join-Path $root 'docs\canonical-system\P7B1B_DENIAL_CANARY_RESULT.md'
$p7b1bAnalysisPath=Join-Path $root 'docs\canonical-system\P7B1B_DENIAL_CANARY_FAILURE_ANALYSIS.md'
$p7b1bTrial2Path=Join-Path $root 'docs\canonical-system\P7B1B_DENIAL_CANARY_TRIAL2_RESULT.md'
$p7b1bTrial3Path=Join-Path $root 'docs\canonical-system\P7B1B_DENIAL_CANARY_TRIAL3_RESULT.md'
$p7b1bStartupPath=Join-Path $root 'docs\canonical-system\P7B1B_STARTUP_COMPATIBILITY_DESIGN.md'
$p7b1bTrial4Path=Join-Path $root 'docs\canonical-system\P7B1B_DENIAL_CANARY_TRIAL4_RESULT.md'
$p7b1bLoaderDiagnosisPath=Join-Path $root 'docs\canonical-system\P7B1B_LOADER_DIAGNOSIS_DESIGN_GATE.md'
$p7b1bLoaderResultPath=Join-Path $root 'docs\canonical-system\P7B1B_LOADER_SURFACE_PROOF_RESULT.md'
$p7b1bLoaderToolPath=Join-Path $root 'tools\prove-p7b1b-loader-surface.ps1'
$p7b1bLoaderTestPath=Join-Path $root 'tools\test-p7b1b-loader-surface.ps1'
$p7b1bLoaderReceiptPath=Join-Path $root 'evidence\p7b1b\loader-surface-proof.json'
$p7b1bDynamicObservationPath=Join-Path $root 'docs\canonical-system\P7B1B_ROUTINE_TEST_DELEGATION_AND_DYNAMIC_OBSERVATION.md'
$p7b1bTrial5ResultPath=Join-Path $root 'docs\canonical-system\P7B1B_DYNAMIC_OBSERVATION_TRIAL5_RESULT.md'
$p7b1bTrial5ReceiptPath=Join-Path $root 'evidence\p7b1b\trial-5-debug-events.json'
$p7b1bTrial5CleanupPath=Join-Path $root 'evidence\p7b1b\trial-5-independent-cleanup.json'
$p7b1bPostTrial5RoutePath=Join-Path $root 'docs\canonical-system\P7B1B_POST_TRIAL5_ROUTE_DECISION.md'
$p7b1bRepairedResultPath=Join-Path $root 'docs\canonical-system\P7B1B_REPAIRED_OBSERVER_VALIDATION_RESULT.md'
$p7b1bRepairedReceiptPath=Join-Path $root 'evidence\p7b1b\repaired-observer-validation.json'
$p7b1bRepairedCleanupPath=Join-Path $root 'evidence\p7b1b\repaired-observer-independent-cleanup.json'
$p7b1bAlternativeRoutePath=Join-Path $root 'docs\canonical-system\P7B1B_ALTERNATIVE_CONTAINMENT_ROUTE.md'
$p7b1bNoUpgradePath=Join-Path $root 'docs\canonical-system\P7B1B_NO_OS_UPGRADE_REBASELINE.md'
$p7bBuiltinPath=Join-Path $root 'docs\canonical-system\P7B_BUILTIN_REFERENCE_VIEWPORT_DECISION.md'
$p7bStimulusResultPath=Join-Path $root 'docs\canonical-system\P7B_BUILTIN_VIEWPORT_CONTROLLED_STIMULUS_RESULT.md'
$p7b1bStartupToolPath=Join-Path $root 'tools\prove-p7b1b-startup-compatibility.ps1'
$p7b1bStartupReceiptPath=Join-Path $root 'evidence\p7b1b\startup-compatibility-proof.json'
$p7b1bTrial4ReceiptPath=Join-Path $root 'evidence\p7b1b\trial-4-static-crt.json'
$p7bContractPath=Join-Path $root 'contracts\perception-protocol-contract.md'
$p7b1ContractPath=Join-Path $root 'contracts\containment-profile-contract.md'
$p7bBuiltinContractPath=Join-Path $root 'contracts\reference-viewport-contract.md'
$p7bStimulusContractPath=Join-Path $root 'contracts\viewport-stimulus-contract.md'
$contractPath=Join-Path $root 'contracts\representation-contract.md'
$registryPath=Join-Path $root 'docs\canonical-system\system-registry.json'
$cratePath=Join-Path $root 'crates\representation-contract'
$p7bCratePath=Join-Path $root 'crates\perception-protocol'
$p7b1CratePath=Join-Path $root 'crates\containment-profile'
$p7bBuiltinCratePath=Join-Path $root 'crates\reference-viewport'
$p7bStimulusCratePath=Join-Path $root 'crates\viewport-stimulus'
$p7b1bCanaryPath=Join-Path $root 'crates\containment-denial-canary'
$p7b1bRunnerPath=Join-Path $root 'crates\containment-canary-runner'
foreach($path in @($readinessPath,$gatePath,$p7bGatePath,$p7b1GatePath,$p7b1bGatePath,$p7b1bResultPath,$p7b1bAnalysisPath,$p7b1bTrial2Path,$p7b1bTrial3Path,$p7b1bStartupPath,$p7b1bTrial4Path,$p7b1bLoaderDiagnosisPath,$p7b1bLoaderResultPath,$p7b1bLoaderToolPath,$p7b1bLoaderTestPath,$p7b1bLoaderReceiptPath,$p7b1bDynamicObservationPath,$p7b1bTrial5ResultPath,$p7b1bTrial5ReceiptPath,$p7b1bTrial5CleanupPath,$p7b1bPostTrial5RoutePath,$p7b1bRepairedResultPath,$p7b1bRepairedReceiptPath,$p7b1bRepairedCleanupPath,$p7b1bAlternativeRoutePath,$p7b1bNoUpgradePath,$p7bBuiltinPath,$p7bStimulusResultPath,$p7b1bStartupToolPath,$p7b1bStartupReceiptPath,$p7b1bTrial4ReceiptPath,$contractPath,$p7bContractPath,$p7b1ContractPath,$p7bBuiltinContractPath,$p7bStimulusContractPath,$registryPath,$cratePath,$p7bCratePath,$p7b1CratePath,$p7bBuiltinCratePath,$p7bStimulusCratePath,$p7b1bCanaryPath,$p7b1bRunnerPath)){if(!(Test-Path $path)){throw "P7 artifact missing: $path"}}
$readiness=Get-Content $readinessPath -Raw
$gate=Get-Content $gatePath -Raw
$p7bGate=Get-Content $p7bGatePath -Raw
$p7b1Gate=Get-Content $p7b1GatePath -Raw
$p7b1bGate=Get-Content $p7b1bGatePath -Raw
$p7b1bResult=Get-Content $p7b1bResultPath -Raw
$p7b1bAnalysis=Get-Content $p7b1bAnalysisPath -Raw
$p7b1bTrial2=Get-Content $p7b1bTrial2Path -Raw
$p7b1bTrial3=Get-Content $p7b1bTrial3Path -Raw
$p7b1bStartup=Get-Content $p7b1bStartupPath -Raw
$p7b1bTrial4=Get-Content $p7b1bTrial4Path -Raw
$p7b1bLoaderDiagnosis=Get-Content $p7b1bLoaderDiagnosisPath -Raw
$p7b1bLoaderResult=Get-Content $p7b1bLoaderResultPath -Raw
$p7b1bDynamicObservation=Get-Content $p7b1bDynamicObservationPath -Raw
$p7b1bTrial5Result=Get-Content $p7b1bTrial5ResultPath -Raw
$p7b1bPostTrial5Route=Get-Content $p7b1bPostTrial5RoutePath -Raw
$p7b1bRepairedResult=Get-Content $p7b1bRepairedResultPath -Raw
$p7b1bAlternativeRoute=Get-Content $p7b1bAlternativeRoutePath -Raw
$p7b1bNoUpgrade=Get-Content $p7b1bNoUpgradePath -Raw
$p7bBuiltin=Get-Content $p7bBuiltinPath -Raw
$p7bContract=Get-Content $p7bContractPath -Raw
$p7b1Contract=Get-Content $p7b1ContractPath -Raw
$contract=Get-Content $contractPath -Raw
foreach($required in @('RepresentationDecision','ArtifactManifest','MaterialRegionSet','ArticulationPlan','TemporalFidelityPlan','VisualReviewReceipt','Readiness gaps deliberately left open')){if(!$readiness.Contains($required)){throw "P7 readiness missing: $required"}}
foreach($required in @('P7a contract harness','P7b perception atlas','Recovered evidence audit','Primary-practice reconciliation','single_feasible_representation','no universal weighted score','hostile-reference rejection','Whole-system alignment','indeterminate_budget','Exact confirmation (satisfied)','Verified P7a reference result')){if(!$gate.Contains($required)){throw "P7 design gate missing: $required"}}
foreach($required in @('P4 hierarchy/history','P5 significance/scheduler','P6 semantics/construction','Reference Studio','Runtime adapter','protected-Kernel')){if(!$gate.Contains($required)){throw "P7 neighbour boundary missing: $required"}}
foreach($required in @('does **not** authorize geometry','filesystem/network/process','review or P7b','runtime or engine integration','promotion')){if(!$gate.Contains($required)){throw "P7 authority boundary missing: $required"}}
foreach($required in @('P7b-0 capability-free protocol contract','ReviewProtocol','EnvironmentProfile','StimulusManifest','ObservationSet','AnalysisReceipt','Future P7b-1 containment boundary','Future P7b-2 controlled review protocol','Adversarial failure matrix','Whole-system reconciliation','Exact next confirmation')){if(!$p7bGate.Contains($required)){throw "P7b design gate missing: $required"}}
foreach($required in @('no universal weighted quality score','single-owner','not population preference','network/clipboard/devices/vGPU disabled','Metric improves but human judgment worsens','does **not** authorize rendering','protected-Kernel mutation')){if(!$p7bGate.Contains($required)){throw "P7b safety boundary missing: $required"}}
foreach($required in @('Exact next confirmation (satisfied)','Verified P7b-0 result','Eighteen independent/adversarial tests','Critical review after the first green run','P7b-1 runner','creates and views no image')){if(!$p7bGate.Contains($required)){throw "P7b verified result missing: $required"}}
foreach($required in @('P7b-1a containment-profile reference','P7b-1b denial canary','P7b-1c renderer compatibility','Local feasibility snapshot','AppContainer / LPAC','Output quarantine and admission','Hostile fixture matrix','Whole-system alignment','Exact next confirmation')){if(!$p7b1Gate.Contains($required)){throw "P7b-1 design gate missing: $required"}}
foreach($required in @('not a security boundary by itself','full-trust MSIX','do not upgrade or enable','launch no process','fresh write-only quarantine','protected-Kernel mutation')){if(!$p7b1Gate.Contains($required)){throw "P7b-1 safety boundary missing: $required"}}
foreach($required in @('Exact next confirmation (satisfied)','Verified P7b-1a result','Nineteen independent/adversarial tests','Critical review after the first 18-test pass','read-only','Eleven-module boundaries','policy consistency only')){if(!$p7b1Gate.Contains($required)){throw "P7b-1 verified result missing: $required"}}
foreach($required in @('zero-capability **Less Privileged','stable Win32 AppContainer APIs','Experimental Create Process in Sandbox','Per-run state machine','Host verification before resume','Bounded denial probes','Independent observation and cleanup','Failure matrix','Whole-system alignment','Exact owner confirmation')){if(!$p7b1bGate.Contains($required)){throw "P7b-1b design gate missing: $required"}}
foreach($required in @('never derive','bInheritHandles=FALSE','PROCESS_CREATION_ALL_APPLICATION_PACKAGES_OPT_OUT','active-process limit 1','no automatic retry','Repository write probe','does **not** authorize elevation','protected-Kernel mutation')){if(!$p7b1bGate.Contains($required)){throw "P7b-1b safety boundary missing: $required"}}
foreach($required in @('failed safely before canary resume','196f4519aac9bec367cbfee0cdc70c22','ERROR_INVALID_PARAMETER','cleanup_ok=true','no arbitrary-code containment claim','zero AppContainer loopback exemptions','regular-AppContainer fallback','new explicit owner authorization')){if(!$p7b1bResult.Contains($required)){throw "P7b-1b result missing: $required"}}
foreach($required in @('cannot identify which','10.0.26200.8655','Documentation correction','Synthetic in-memory `AccessCheck` discriminator','exactly `0x2`','class46-and-access-check','access-check-after-class46-error87','No runner command','fresh owner authorization')){if(!$p7b1bAnalysis.Contains($required)){throw "P7b-1b analysis missing: $required"}}
foreach($required in @('c6a42d8af4bf79d860f95a7f8e101d22','image-load mitigations missing','cleanup_ok=true','before `ResumeThread`','no denial pass','86653b5573ea54eedc332e385cc9f63a6681c567c47922c27dbf2b7d66d925cf','`0x1 | 0x2 | 0x4 = 0x7`','incorrectly required mask `0xb`','did not record the observed flag word','no automatic retry')){if(!$p7b1bTrial2.Contains($required)){throw "P7b-1b Trial 2 result missing: $required"}}
foreach($required in @('e6b479fd2ba785d6315dadd5c3ba1154','0xC0000142','STATUS_DLL_INIT_FAILED','verify_suspended','cleanup_ok=true','dac8c40284507b0af09de0e7a8434221ed621fe1890e39567f38c49a084e464b','zero AppContainer loopback-exemption SID entries','unchanged canary hash','does **not** justify adding `registryRead`','exact failing module is therefore unresolved','no same-package retry')){if(!$p7b1bTrial3.Contains($required)){throw "P7b-1b Trial 3 result missing: $required"}}
foreach($required in @('prototype_tested_not_executed','cargo build --locked --offline','100,352 bytes larger','11','5','1123373704a528e86c81e3d32e16c1842d95ecd84002565e9b0fd1cb0b0e3585','canary_executed=false','lpac_compatibility_proved=false','does **not** justify','fresh one-run package')){if(!$p7b1bStartup.Contains($required)){throw "P7b-1b startup design missing: $required"}}
foreach($required in @('7fe469dbd720775e3075eba05fdc441f','0xC0000142','3221225794','access-check-after-class46-error87','cleanup_ok=true','b69d8c2467fe19d10ebf67a709d4c574ac23634d07aadf899cc883c6c43ec670','25109e5e9c4de262b4dc4020f2b75b6ba6515c3c6570fad575e131d7bf820856','61167968fcaa251309d2140a7b07079dca24a782b2e33820e64e456772bfb350','zero `containment-*` processes','narrow hypothesis','does **not** justify `registryRead`','no retry is permitted')){if(!$p7b1bTrial4.Contains($required)){throw "P7b-1b Trial 4 result missing: $required"}}
foreach($required in @('offline PE loader-surface proof','Exact retained observations','H1','H2','H3','H4','H5','Information-gain comparison','P10 baseline and stop rule','Adversarial requirements for the selected proof','canary_executed=false','runtime_cause_proved=false','Stop/refocus condition','Exact owner confirmation','does not authorize','WER LocalDumps','MiniDumpWriteDump')){if(!$p7b1bLoaderDiagnosis.Contains($required)){throw "P7b-1b loader diagnosis design missing: $required"}}
foreach($required in @('9e6d266b518438760e11d6c6158bab6c930dde1fbc6e97ad5aa467365d73968e','0fdfc4f08b42c19926a70c95037b4df776be75541d438f87f40cc8008a64b4a6','11 / 115','5 / 123','0x0000000140020460','f0fdc2b85769d1ec16235e3bf0e3f3ba5d922c3791f57f9a0e147c410766edae','1615945703c9a9c7a02f98959cb10ba10f757f4c1cf49b990b4bff5530afa724','static-optimization stop rule fires','runtime_cause_proved=false','denial_proved=false','separate, exact owner decision')){if(!$p7b1bLoaderResult.Contains($required)){throw "P7b-1b loader result missing: $required"}}
foreach($required in @('Standing owner delegation','routine, bounded tests','DEBUG_ONLY_THIS_PROCESS','WaitForDebugEvent','ContinueDebugEvent','DBG_EXCEPTION_NOT_HANDLED','exact retained static candidate once','diagnostic-only','denial_proved=false','runtime_cause_proved=false','may not read process memory','run exactly once','different program','protected-Kernel mutation')){if(!$p7b1bDynamicObservation.Contains($required)){throw "P7b-1b dynamic-observation design missing: $required"}}
foreach($required in @('85b051eec7ca37971e3ca4d3fceb2678','8bd2baa58086c85237ef392fc9281d5d9c673018f26c786f26d71bebd94055c1','09cdb3f3b1a20449e4f74863d4b96aaad050d378605d48cb2ae0eeb1d425068a','semaphore timeout period has expired','cleanup_ok=false','event count `0`','independent cleanup','DebugActiveProcessStop','fc94fe9616acabc875b26098d6089598259bbd8c5e1946175a8fa81921ace354','repair was not executed','Do not retry Trial 5','P7b-1b remains unproved')){if(!$p7b1bTrial5Result.Contains($required)){throw "P7b-1b Trial 5 result missing: $required"}}
foreach($required in @('Option A — one repaired observer validation','Option B — leave P7b-1b blocked','demonstrated cleanup regression','exact retained static candidate','eight runner tests','no external debugger program','different program or project approach','Advancing P7b-1c','P10 stop rule','Run the repaired observer once','Leave P7b-1b blocked','standing routine-test delegation')){if(!$p7b1bPostTrial5Route.Contains($required)){throw "P7b-1b post-Trial-5 route missing: $required"}}
foreach($required in @('c09a15ae38d7bbfac9a7f86cd14b5c40','36af2a8ec422cbcf0cced35d61b560f4fce518773f50534b9a472337e274c3b8','7cbf42965342d085721c94db3eff946a6ef73106a9af49034fecc1639879bc94','event count `7`','ntdll.dll','kernel32.dll','KernelBase.dll','cleanup_ok=true','runtime_cause_proved=false','denial_proved=false','P10 stop rule has fired','diagnostic family is terminal','P7b-1b remains blocked')){if(!$p7b1bRepairedResult.Contains($required)){throw "P7b-1b repaired observer result missing: $required"}}
foreach($required in @('Windows `EditionID=Core`','Windows Sandbox disposable-VM protocol','separate kernel','Networking and clipboard are enabled by default','Hyper-V-isolated Windows containers do not currently support GPU acceleration','experimental, has no public header','Alt-0 - capability-free Windows Sandbox protocol','Alt-1 - host eligibility owner gate','Alt-2 - disposable-VM denial canary','Alt-3 - renderer compatibility','vGPU disabled','one immutable read-only input mapping','writable output quarantine mapping','P10 boundary','Build the capability-free Windows Sandbox protocol','Leave executable perception blocked','does not authorize an edition upgrade')){if(!$p7b1bAlternativeRoute.Contains($required)){throw "P7b-1b alternative route missing: $required"}}
foreach($forbidden in @('Silently skips the P7b-1b evidence gate','fall back to a weaker process boundary')){if(!$p7b1bAlternativeRoute.Contains($forbidden)){throw "P7b-1b alternative route lost rejection: $forbidden"}}
foreach($required in @('owner explicitly rejected upgrading','F5 is the **engine-neutral proof** milestone','created a circular','move **execution**, not evidence quality, to R1','runtime_containment_pending','G1 may promote only the capability-free neutral contracts','R1 must prove before execution','WSL 2 is available on Windows Home','Oracle VirtualBox or another full VM','Sandboxie Plus','Windows Sandbox/Hyper-V on this host','Closed unless the owner independently changes the constraint','Move executable containment to R1 and keep the hard blocker','Keep F5 blocked','does not authorize F5/G1 to','claim runtime or visual proof')){if(!$p7b1bNoUpgrade.Contains($required)){throw "P7b-1b no-upgrade rebaseline missing: $required"}}
foreach($required in @("use Forge's built-in viewer and continue",'Corrected threat model','arbitrary-code boundary problem','unknown scripts, plugins, packages','seven adversarial tests','three synchronized','does not prove an asset factory','External program becomes an implicit prerequisite','do not build a general renderer','P7B_BUILTIN_VIEWPORT_CONTROLLED_STIMULUS_RESULT.md')){if(!$p7bBuiltin.Contains($required)){throw "P7b built-in viewport result missing: $required"}}
foreach($required in @('deterministic, data-only','front, side, and top integer','no filesystem, network, process','future Unity installation','genuinely untrusted executable content','neutral-t-pose-articulation-fixture-v3','squared length `14400`','articulation control','without changing segment','prevents a pose judgment')){if(!(Get-Content $p7bBuiltinContractPath -Raw).Contains($required)){throw "P7b built-in viewport contract missing: $required"}}
if(!(Get-Content (Join-Path $p7bBuiltinCratePath 'src\lib.rs') -Raw).Contains('forge-wireframe-orthographic-v1')){throw 'P7b built-in viewport implementation profile missing.'}
$p7bStimulusResult=Get-Content $p7bStimulusResultPath -Raw
foreach($required in @('Broken connection','Silhouette collapse','Articulation drift','protocol pairs:','observed_claim_count=0','`forge-desktop`: 25 tests pass','no second owner observation was fabricated','blank-by-default direct','Poisoned-fixture repair result','artifact-reference-viewport-003','short reference limbs fail before projection','preserving both','implementation readiness only')){if(!$p7bStimulusResult.Contains($required)){throw "P7b viewport stimulus result missing: $required"}}
$p7bStimulusContract=Get-Content $p7bStimulusContractPath -Raw
foreach($required in @('exact SHA-256 scene fingerprint','not_observed','confidence zero','observed_claim_count=0','no filesystem, network, process')){if(!$p7bStimulusContract.Contains($required)){throw "P7b viewport stimulus contract missing: $required"}}
foreach($required in @('Direct owner-observation entry','no pair, outcome, or confidence selected','current base scene fingerprint','authority_effect=none','does not approve, promote')){if(!$p7bStimulusContract.Contains($required)){throw "P7b owner-observation contract missing: $required"}}
foreach($required in @('artifact-reference-viewport-003','preserves both forearm segment lengths','silently rebound')){if(!$p7bStimulusContract.Contains($required)){throw "P7b v3 stimulus contract missing: $required"}}
$p7bStimulusSources=(Get-ChildItem (Join-Path $p7bStimulusCratePath 'src') -Filter '*.rs'|Get-Content -Raw) -join "`n"
foreach($required in @('BrokenConnection','SilhouetteCollapse','ArticulationDrift','awaiting_owner_observation','observed_claim_count: 0')){if(!$p7bStimulusSources.Contains($required)){throw "P7b viewport stimulus implementation missing: $required"}}
foreach($required in @('OwnerObservationInput','OwnerObservationReceipt','stale base scene fingerprint','not_observed is a placeholder','authority_effect: "none"')){if(!$p7bStimulusSources.Contains($required)){throw "P7b owner-observation implementation missing: $required"}}
$p7bBuiltinSources=(Get-ChildItem (Join-Path $p7bBuiltinCratePath 'src') -Filter '*.rs'|Get-Content -Raw) -join "`n"
foreach($required in @('neutral-t-pose-articulation-fixture-v3','artifact-reference-viewport-003','validate_reference_fixture_semantics','reference limb segment is not the declared v3 length','reference limb length drifts across pose frames','articulation_control_changes_pose_without_changing_segment_length')){if(!$p7bBuiltinSources.Contains($required)){throw "P7b v3 fixture implementation missing: $required"}}
foreach($required in @('Direct owner review result','broken_connection` | `satisfied` | 90','silhouette_collapse` | `satisfied` | 90','articulation_drift` | `indeterminate` | 40','possible poisoned-fixture limitation','simple-stick','does not prove that the fixture is suitable')){if(!$p7bStimulusResult.Contains($required)){throw "P7b direct owner-review result missing: $required"}}
foreach($forbidden in @('forge_kernel','tauri::','std::fs','std::process','std::net','reqwest','tokio::net')){if($p7bStimulusSources.Contains($forbidden)){throw "P7b viewport stimulus crosses capability boundary: $forbidden"}}
foreach($required in @('owner subsequently rejected any Windows operating-system edition upgrade','supersedes this record')){if(!$p7b1bAlternativeRoute.Contains($required)){throw "P7b-1b superseded alternative route missing: $required"}}
$trial5Bytes=[IO.File]::ReadAllBytes($p7b1bTrial5ReceiptPath);$trial5Hasher=[Security.Cryptography.SHA256]::Create();try{$trial5Hash=(-join @($trial5Hasher.ComputeHash($trial5Bytes)|ForEach-Object{$_.ToString('x2')}))}finally{$trial5Hasher.Dispose()}
if($trial5Hash-ne'8bd2baa58086c85237ef392fc9281d5d9c673018f26c786f26d71bebd94055c1'){throw 'P7b-1b Trial 5 receipt hash drifted.'}
$trial5=[Text.Encoding]::UTF8.GetString($trial5Bytes)|ConvertFrom-Json
if($trial5.trial-ne'P7b-1b-diagnostic'-or$trial5.run_id-ne'85b051eec7ca37971e3ca4d3fceb2678'-or$trial5.status-ne'failed'-or$trial5.cleanup_ok-ne$false-or$trial5.diagnostic.event_count-ne 0-or$trial5.diagnostic.denial_proved-ne$false-or$trial5.diagnostic.runtime_cause_proved-ne$false){throw 'P7b-1b Trial 5 receipt lost its exact failed no-proof claim.'}
$trial5Cleanup=Get-Content $p7b1bTrial5CleanupPath -Raw|ConvertFrom-Json
if($trial5Cleanup.source_receipt_sha256-ne$trial5Hash-or$trial5Cleanup.source_receipt_cleanup_ok-ne$false-or$trial5Cleanup.manual_exact_stage_cleanup_performed-ne$true-or$trial5Cleanup.independent_cleanup_complete-ne$true-or$trial5Cleanup.containment_process_count-ne 0-or$trial5Cleanup.exact_temp_path_count-ne 0-or$trial5Cleanup.package_folder_count-ne 0-or$trial5Cleanup.profile_mapping_count-ne 0-or$trial5Cleanup.loopback_exemption_count-ne 0-or$trial5Cleanup.denial_proved-ne$false-or$trial5Cleanup.runtime_cause_proved-ne$false){throw 'P7b-1b Trial 5 independent cleanup receipt is invalid.'}
$repairedBytes=[IO.File]::ReadAllBytes($p7b1bRepairedReceiptPath);$repairedHasher=[Security.Cryptography.SHA256]::Create();try{$repairedHash=(-join @($repairedHasher.ComputeHash($repairedBytes)|ForEach-Object{$_.ToString('x2')}))}finally{$repairedHasher.Dispose()}
if($repairedHash-ne'36af2a8ec422cbcf0cced35d61b560f4fce518773f50534b9a472337e274c3b8'){throw 'P7b-1b repaired observer receipt hash drifted.'}
$repaired=[Text.Encoding]::UTF8.GetString($repairedBytes)|ConvertFrom-Json
if($repaired.trial-ne'P7b-1b-diagnostic'-or$repaired.run_id-ne'c09a15ae38d7bbfac9a7f86cd14b5c40'-or$repaired.status-ne'diagnostic_completed'-or$repaired.cleanup_ok-ne$true-or@($repaired.cleanup_errors).Count-ne 0-or$repaired.runner_sha256-ne'7cbf42965342d085721c94db3eff946a6ef73106a9af49034fecc1639879bc94'-or$repaired.diagnostic.candidate_sha256-ne'25109e5e9c4de262b4dc4020f2b75b6ba6515c3c6570fad575e131d7bf820856'-or$repaired.diagnostic.lpac_verification-ne'access-check-after-class46-error87'-or$repaired.diagnostic.event_count-ne 7-or$repaired.diagnostic.denial_proved-ne$false-or$repaired.diagnostic.runtime_cause_proved-ne$false){throw 'P7b-1b repaired observer receipt lost its exact clean no-proof result.'}
$repairedKinds=@($repaired.diagnostic.events|ForEach-Object kind)
if(($repairedKinds-join ',')-ne'create_process,load_dll,load_dll,load_dll,unload_dll,unload_dll,exit_process'-or$repaired.diagnostic.events[1].path-notlike'*\ntdll.dll'-or$repaired.diagnostic.events[2].path-notlike'*\kernel32.dll'-or$repaired.diagnostic.events[3].path-notlike'*\KernelBase.dll'-or$repaired.diagnostic.events[6].exit_code-ne'0xC0000142'){throw 'P7b-1b repaired observer event order drifted.'}
$repairedCleanup=Get-Content $p7b1bRepairedCleanupPath -Raw|ConvertFrom-Json
if($repairedCleanup.source_receipt_sha256-ne$repairedHash-or$repairedCleanup.source_receipt_cleanup_ok-ne$true-or$repairedCleanup.manual_cleanup_performed-ne$false-or$repairedCleanup.independent_cleanup_complete-ne$true-or$repairedCleanup.containment_process_count-ne 0-or$repairedCleanup.exact_temp_path_count-ne 0-or$repairedCleanup.all_owned_temp_path_count-ne 0-or$repairedCleanup.package_folder_count-ne 0-or$repairedCleanup.profile_mapping_count-ne 0-or$repairedCleanup.loopback_exemption_count-ne 0-or$repairedCleanup.denial_proved-ne$false-or$repairedCleanup.runtime_cause_proved-ne$false){throw 'P7b-1b repaired observer independent cleanup receipt is invalid.'}
$loaderReceiptBytes=[IO.File]::ReadAllBytes($p7b1bLoaderReceiptPath);$loaderHasher=[Security.Cryptography.SHA256]::Create();try{$loaderReceiptHash=(-join @($loaderHasher.ComputeHash($loaderReceiptBytes)|ForEach-Object{$_.ToString('x2')}))}finally{$loaderHasher.Dispose()}
if($loaderReceiptHash-ne'9e6d266b518438760e11d6c6158bab6c930dde1fbc6e97ad5aa467365d73968e'){throw 'P7b-1b loader receipt hash drifted.'}
$loaderReceipt=[Text.Encoding]::UTF8.GetString($loaderReceiptBytes)|ConvertFrom-Json
if($loaderReceipt.schema-ne 1-or$loaderReceipt.status-ne'completed_claim_limited'-or$loaderReceipt.parser_sha256-ne'0fdfc4f08b42c19926a70c95037b4df776be75541d438f87f40cc8008a64b4a6'-or@($loaderReceipt.candidates).Count-ne 2){throw 'P7b-1b loader receipt identity is invalid.'}
foreach($field in @('canary_executed','profile_created','registry_modified','acl_modified','capability_added','runtime_cause_proved','denial_proved')){if($loaderReceipt.$field-ne$false){throw "P7b-1b loader receipt overclaims: $field"}}
$loaderDynamic=$loaderReceipt.candidates|Where-Object name -eq dynamic;$loaderStatic=$loaderReceipt.candidates|Where-Object name -eq static
if(@($loaderDynamic.image.imports).Count-ne 11-or@($loaderStatic.image.imports).Count-ne 5-or@($loaderDynamic.image.delay_imports).Count-ne 0-or@($loaderStatic.image.delay_imports).Count-ne 0-or$loaderDynamic.image.tls.callback_count-ne 1-or$loaderStatic.image.tls.callback_count-ne 1){throw 'P7b-1b loader receipt lost the exact import/TLS observation.'}
$startupReceipt=Get-Content $p7b1bStartupReceiptPath -Raw|ConvertFrom-Json
if($startupReceipt.status-ne'prototype_tested_not_executed'-or$startupReceipt.canary_executed-ne$false-or$startupReceipt.lpac_profile_created-ne$false-or$startupReceipt.capability_added-ne$false-or$startupReceipt.lpac_compatibility_proved-ne$false){throw 'P7b-1b startup receipt crosses its capability-free claim boundary.'}
if(!$startupReceipt.normal_workspace.unchanged-or$startupReceipt.static.imports-contains'vcruntime140.dll'-or@($startupReceipt.static.imports|Where-Object{$_-like'api-ms-win-crt-*'}).Count-ne 0){throw 'P7b-1b startup receipt lost workspace or import-reduction evidence.'}
$trial4ReceiptBytes=[System.IO.File]::ReadAllBytes($p7b1bTrial4ReceiptPath)
$trial4Hasher=[Security.Cryptography.SHA256]::Create()
try{$trial4ReceiptHash=(-join @($trial4Hasher.ComputeHash($trial4ReceiptBytes)|ForEach-Object{$_.ToString('x2')}))}finally{$trial4Hasher.Dispose()}
if($trial4ReceiptHash-ne'b69d8c2467fe19d10ebf67a709d4c574ac23634d07aadf899cc883c6c43ec670'){throw 'P7b-1b Trial 4 receipt hash drifted.'}
$trial4Receipt=[Text.Encoding]::UTF8.GetString($trial4ReceiptBytes)|ConvertFrom-Json
if($trial4Receipt.schema-ne 1-or$trial4Receipt.trial-ne'P7b-1b'-or$trial4Receipt.run_id-ne'7fe469dbd720775e3075eba05fdc441f'-or$trial4Receipt.moniker-ne'MindwarpForge.P7b1b.7fe469dbd720775e3075eba05fdc441f'-or$trial4Receipt.status-ne'failed'-or$trial4Receipt.contract_version-ne 1-or$trial4Receipt.cleanup_ok-ne$true-or$trial4Receipt.error-ne'post-resume canary exit 0xC0000142 (3221225794) after access-check-after-class46-error87'-or@($trial4Receipt.cleanup_errors).Count-ne 0){throw 'P7b-1b Trial 4 receipt does not retain the exact clean-failure result.'}
foreach($required in @('capability-free P7b-0','ReviewProtocol','EnvironmentProfile','StimulusManifest','ObservationSet','AnalysisReceipt','indeterminate_budget','read-only ProofReceipt','not Mind Warp art direction')){if(!$p7bContract.Contains($required)){throw "P7b contract missing: $required"}}
foreach($required in @('capability-free P7b-1a','ToolIdentity','BoundaryProfile','InputPolicy','OutputPolicy','ResourceBudget','RecoveryPlan','policy_ready_not_executed','job objects cannot','fresh per-run quarantine','Nineteen independent/adversarial tests','read-only')){if(!$p7b1Contract.Contains($required)){throw "P7b-1 contract missing: $required"}}
$registry=Get-Content $registryPath -Raw|ConvertFrom-Json
$selector=$registry.systems|Where-Object id -eq 'representation-selector'
if(!$selector-or$selector.status-ne'prototype_tested'){throw 'Representation selector is not recorded at its verified P7a status.'}
foreach($id in @('asset-factory','procedural-animation')){if(($registry.systems|Where-Object id -eq $id).status-ne'specified'){throw "$id crossed the P7a proof boundary."}}
foreach($required in @('strict synthetic evidence','Category labels and universal weighted scores','paths, URIs, traversal','indeterminate_budget','read-only `ProofReceipt`','separately gated')){if(!$contract.Contains($required)){throw "P7a contract missing: $required"}}
$sources=(Get-ChildItem (Join-Path $cratePath 'src') -Filter '*.rs'|Get-Content -Raw) -join "`n"
foreach($required in @('RepresentationContractPackage','RepresentationPortfolio','ArtifactManifest','MaterialRegionPlan','ArticulationPlan','TemporalFidelityPlan','ReviewCase','IndeterminateBudget','reference_proof_evidence')){if(!$sources.Contains($required)){throw "P7a crate missing proof surface: $required"}}
foreach($forbidden in @('forge_kernel','tauri','std::fs','std::process','std::net','reqwest','tokio::net')){if($sources.Contains($forbidden)){throw "P7a crate crosses capability boundary: $forbidden"}}
$manifest=Get-Content (Join-Path $cratePath 'Cargo.toml') -Raw
foreach($forbidden in @('forge-kernel','tauri','tokio','reqwest')){if($manifest.Contains($forbidden)){throw "P7a manifest crosses capability boundary: $forbidden"}}
$p7bSources=(Get-ChildItem (Join-Path $p7bCratePath 'src') -Filter '*.rs'|Get-Content -Raw) -join "`n"
foreach($required in @('PerceptionProtocolPackage','ReviewProtocol','EnvironmentProfile','StimulusManifest','ObservationSet','AnalysisReceipt','IndeterminateBudget','reference_proof_evidence')){if(!$p7bSources.Contains($required)){throw "P7b crate missing proof surface: $required"}}
foreach($forbidden in @('forge_kernel','tauri','std::fs','std::process','std::net','reqwest','tokio::net')){if($p7bSources.Contains($forbidden)){throw "P7b crate crosses capability boundary: $forbidden"}}
$p7bManifest=Get-Content (Join-Path $p7bCratePath 'Cargo.toml') -Raw
foreach($forbidden in @('forge-kernel','tauri','tokio','reqwest')){if($p7bManifest.Contains($forbidden)){throw "P7b manifest crosses capability boundary: $forbidden"}}
$p7b1Sources=(Get-ChildItem (Join-Path $p7b1CratePath 'src') -Filter '*.rs'|Get-Content -Raw) -join "`n"
foreach($required in @('ContainmentProfilePackage','ToolIdentity','BoundaryProfile','InputPolicy','OutputPolicy','ResourceBudget','RecoveryPlan','ContainmentReadinessReceipt','IndeterminateBudget','reference_proof_evidence')){if(!$p7b1Sources.Contains($required)){throw "P7b-1 crate missing proof surface: $required"}}
foreach($forbidden in @('forge_kernel','tauri','std::fs','std::process','std::net','reqwest','tokio::net')){if($p7b1Sources.Contains($forbidden)){throw "P7b-1 crate crosses capability boundary: $forbidden"}}
$p7b1Manifest=Get-Content (Join-Path $p7b1CratePath 'Cargo.toml') -Raw
foreach($forbidden in @('forge-kernel','tauri','tokio','reqwest')){if($p7b1Manifest.Contains($forbidden)){throw "P7b-1 manifest crosses capability boundary: $forbidden"}}
$p7b1bCanarySources=(Get-ChildItem (Join-Path $p7b1bCanaryPath 'src') -Filter '*.rs'|Get-Content -Raw) -join "`n"
foreach($required in @('TokenIsAppContainer','TokenCapabilities','fs::read(&sentinel)','--child','TcpStream::connect_timeout','report exceeded fixed bound')){if(!$p7b1bCanarySources.Contains($required)){throw "P7b-1b canary missing fixed probe: $required"}}
foreach($forbidden in @('forge_kernel','tauri','reqwest','tokio','serde','std::env::var("PATH")')){if($p7b1bCanarySources.Contains($forbidden)){throw "P7b-1b canary crosses fixed boundary: $forbidden"}}
$p7b1bCanaryManifest=Get-Content (Join-Path $p7b1bCanaryPath 'Cargo.toml') -Raw
foreach($required in @('windows-sys','test = false')){if(!$p7b1bCanaryManifest.Contains($required)){throw "P7b-1b canary manifest missing: $required"}}
foreach($forbidden in @('forge-kernel','tauri','tokio','reqwest','serde')){if($p7b1bCanaryManifest.Contains($forbidden)){throw "P7b-1b canary manifest crosses boundary: $forbidden"}}
$p7b1bBuild=Get-Content (Join-Path $p7b1bCanaryPath 'build.rs') -Raw
if(!$p7b1bBuild.Contains('/APPCONTAINER')){throw 'P7b-1b canary is not linker-marked AppContainer-only.'}
$p7b1bRunnerSources=(Get-ChildItem (Join-Path $p7b1bRunnerPath 'src') -Filter '*.rs'|Get-Content -Raw) -join "`n"
foreach($required in @('TokenIsLessPrivilegedAppContainer','TOKEN_QUERY | TOKEN_DUPLICATE','PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES','PROC_THREAD_ATTRIBUTE_ALL_APPLICATION_PACKAGES_POLICY','PROC_THREAD_ATTRIBUTE_CHILD_PROCESS_POLICY','PROC_THREAD_ATTRIBUTE_JOB_LIST','PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY','verify_job_trace','staging DACL restoration was not byte/control identical','bounded_inventory','parse_report','DeleteAppContainerProfile','--owner-authorized-single-run','validate_lpac_observation','validate_image_load_flags','REQUIRED_IMAGE_LOAD_FLAGS: u32 = 0x7','format_post_resume_exit','post-resume canary exit 0x','AccessCheck','S-1-15-2-2','class46-and-access-check','access-check-after-class46-error87','--standing-delegated-routine-test','--owner-authorized-repaired-validation','DEBUG_ONLY_THIS_PROCESS','WaitForDebugEvent','ContinueDebugEvent','DBG_EXCEPTION_NOT_HANDLED','debug_semantics_changed','denial_proved','runtime_cause_proved','trial-5-debug-events.json','repaired-observer-validation.json')){if(!$p7b1bRunnerSources.Contains($required)){throw "P7b-1b runner missing shield: $required"}}
foreach($forbidden in @('forge_kernel','tauri','reqwest','tokio','serde_json','Command::new','powershell','CheckNetIsolation')){if($p7b1bRunnerSources.Contains($forbidden)){throw "P7b-1b runner crosses fixed boundary: $forbidden"}}
if($p7b1bRunnerSources.IndexOf('ResumeThread(r.thread.as_ref().unwrap().0)')-gt$p7b1bRunnerSources.IndexOf('drive_debug_events(trace)?')){throw 'Prospective Trial 5 repair still waits for debug events before resume.'}
$p7b1bRunnerManifest=Get-Content (Join-Path $p7b1bRunnerPath 'Cargo.toml') -Raw
foreach($required in @('containment-profile','windows-sys','0.61.2')){if(!$p7b1bRunnerManifest.Contains($required)){throw "P7b-1b runner manifest missing: $required"}}
foreach($forbidden in @('forge-kernel','tauri','tokio','reqwest','serde')){if($p7b1bRunnerManifest.Contains($forbidden)){throw "P7b-1b runner manifest crosses boundary: $forbidden"}}
$program=Get-Content (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw|ConvertFrom-Json
$active=@($program.items|Where-Object status -eq 'active')
$f5=@($program.items|Where-Object id -eq 'F5')[0]
if($active.Count-ne 1-or($active[0].id-ne'F5'-and!($f5.status-eq'complete'-and$active[0].milestone-in@('G1','R1')))){throw 'P7 design gate is not retained through the F5 or later route.'}
Write-Output 'F5 P7 verified: strict P7a/P7b contracts, retained containment evidence, and the owner-approved no-install built-in reference viewport remain bounded, deterministic, read-only, and authority-negative.'
