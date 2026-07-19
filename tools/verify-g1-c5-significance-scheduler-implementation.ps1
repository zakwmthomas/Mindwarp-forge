param([switch]$EmitObservationReceipt)
$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
$crate=Join-Path $root 'crates\significance-scheduler'
$fixtureRoot=Join-Path $root 'tools\fixtures\c5-significance-scheduler-receipt'
$manifestPath=Join-Path $fixtureRoot 'Cargo.toml'
$boundedPathsPath=Join-Path $fixtureRoot 'bounded-paths.txt'
$observationReceiptPath=Join-Path $root 'docs\canonical-system\G1_C5_LOCAL_PLATFORM_OBSERVATIONS.json'
$temp=Join-Path ([IO.Path]::GetTempPath()) "forge-c5-receipt-v1-$PID"
$priorRoot=$env:FORGE_ROOT
$priorTarget=$env:CARGO_TARGET_DIR

function Get-C5SourceManifestSha([string]$Commit) {
  $paths=if([string]::IsNullOrWhiteSpace($Commit)){
    @(Get-Content -LiteralPath $boundedPathsPath)
  }else{
    @((git show "$Commit`:tools/fixtures/c5-significance-scheduler-receipt/bounded-paths.txt") -split "`r?`n")
  }
  $paths=@($paths|Where-Object{!([string]::IsNullOrWhiteSpace($_))})
  if(!$paths.Count-or$paths.Count-ne@($paths|Select-Object -Unique).Count-or$paths-notcontains'tools/fixtures/c5-significance-scheduler-receipt/bounded-paths.txt'){throw 'C5 bounded path list is empty, duplicate or not self-binding.'}
  if(@($paths|Where-Object{$_-match'\\'-or$_.StartsWith('/')-or@($_-split'/')|Where-Object{$_-in@('','.', '..')}}).Count){throw 'C5 bounded path list contains a noncanonical path.'}
  $rows=foreach($relative in $paths){
    $full=Join-Path $root $relative
    if(!(Test-Path -LiteralPath $full -PathType Leaf)){throw "C5 source manifest file missing: $relative"}
    $blob=if([string]::IsNullOrWhiteSpace($Commit)){(git hash-object -- $full).Trim()}else{(git rev-parse "$Commit`:$relative").Trim()}
    if($LASTEXITCODE-ne0-or$blob-notmatch'^[0-9a-f]{40,64}$'){throw "C5 source manifest blob unavailable: $relative"}
    "$relative`:$blob"
  }
  $digest=[Security.Cryptography.SHA256]::Create()
  try{([BitConverter]::ToString($digest.ComputeHash([Text.Encoding]::UTF8.GetBytes(($rows-join "`n")))).Replace('-','').ToLowerInvariant())}finally{$digest.Dispose()}
}
function Get-C5TreeManifestSha([string]$Commit) {
  $treeLines=@(git -c core.quotePath=true ls-tree -r --full-tree $Commit)
  if($LASTEXITCODE-ne0-or!$treeLines.Count){throw 'C5 tracked-tree manifest is unavailable.'}
  $digest=[Security.Cryptography.SHA256]::Create()
  try{([BitConverter]::ToString($digest.ComputeHash([Text.Encoding]::UTF8.GetBytes(($treeLines-join"`n")))).Replace('-','').ToLowerInvariant())}finally{$digest.Dispose()}
}
function Invoke-C5Process([string]$Arguments) {
  $info=New-Object Diagnostics.ProcessStartInfo
  $info.FileName='cargo';$info.Arguments=$Arguments;$info.UseShellExecute=$false
  $info.RedirectStandardOutput=$true;$info.RedirectStandardError=$true;$info.CreateNoWindow=$true
  $process=New-Object Diagnostics.Process;$process.StartInfo=$info
  if(!$process.Start()){throw 'Failed to launch C5 semantic receipt process.'}
  $stdoutTask=$process.StandardOutput.ReadToEndAsync();$stderrTask=$process.StandardError.ReadToEndAsync()
  $process.WaitForExit()
  [pscustomobject]@{pid=$process.Id;exit_code=$process.ExitCode;stdout=$stdoutTask.GetAwaiter().GetResult();stderr=$stderrTask.GetAwaiter().GetResult()}
}
function ConvertFrom-C5ReceiptHex([string]$Text) {
  $hex=$Text.Trim()
  if($hex-notmatch'^[0-9a-f]+$'-or$hex.Length%2-ne0-or$hex.Length/2-gt65536){throw 'C5 semantic receipt is not bounded lowercase hexadecimal.'}
  $bytes=New-Object byte[] ($hex.Length/2)
  for($i=0;$i-lt$bytes.Length;$i++){$bytes[$i]=[Convert]::ToByte($hex.Substring($i*2,2),16)}
  $bytes
}
function Get-C5Sha256([byte[]]$Bytes) {
  $digest=[Security.Cryptography.SHA256]::Create()
  try{([BitConverter]::ToString($digest.ComputeHash($Bytes)).Replace('-','').ToLowerInvariant())}finally{$digest.Dispose()}
}
function Get-C5TextSha256([string]$Text) {Get-C5Sha256 ([Text.Encoding]::UTF8.GetBytes($Text))}

try {
  if(!(Test-Path -LiteralPath $manifestPath -PathType Leaf)-or!(Test-Path -LiteralPath $boundedPathsPath -PathType Leaf)){throw 'C5 semantic receipt fixture is missing.'}
  if($EmitObservationReceipt-and(git status --porcelain)){throw 'C5 platform observation emission requires a clean committed source tree.'}
  $sourceCommit=(git rev-parse HEAD).Trim()
  if($LASTEXITCODE-ne0-or$sourceCommit-notmatch'^[0-9a-f]{40}$'){throw 'C5 source commit is unavailable.'}
  $treeManifestSha=Get-C5TreeManifestSha $sourceCommit
  $sourceManifestSha=Get-C5SourceManifestSha
  if($EmitObservationReceipt-and(Get-C5SourceManifestSha $sourceCommit)-ne$sourceManifestSha){throw 'C5 platform observation emission requires every bounded source at HEAD.'}

  $readiness=Get-Content (Join-Path $root 'docs\canonical-system\G1_C5_CLOSURE_READINESS.md') -Raw
  $source=(Get-ChildItem (Join-Path $crate 'src') -Filter '*.rs'|Get-Content -Raw)-join "`n"
  $testPaths=@('eight_domain_scheduler_closure.rs','c5_contract_hostiles.rs','c5_scheduler_hostiles.rs','c5_residency_trace_authority_hostiles.rs','c5_pressure_simulation.rs')|ForEach-Object{Join-Path $crate ("tests\"+$_)}
  $test=($testPaths|ForEach-Object{Get-Content $_ -Raw})-join "`n"
  $crateManifest=Get-Content (Join-Path $crate 'Cargo.toml') -Raw
  foreach($required in @('ConsumerDomainV1','DomainFidelityMapSetV1','ImportanceDecisionBindingV1','CompletionReceiptV1','AdmissionReceiptV1','PressureTraceV2','ResidencyIntentV1','StarvationDiagnosed')){if(!$source.Contains($required)){throw "C5 implementation surface missing: $required"}}
  foreach($required in @('fallback_must_preserve_domain_and_work_class','completion_is_accepted_only_from_running_and_never_from_inactive_or_terminal','strict_trace_has_domain_budget_and_stable_code_identity','residency_intents_are_streaming_only_bounded_and_strict')){if(!$test.Contains($required)){throw "C5 integration assertion missing: $required"}}
  $documentIds=@([regex]::Matches($readiness,'(?m)^- `([a-z]+\.[a-z0-9-]+)`\r?$')|ForEach-Object{$_.Groups[1].Value})
  $sourceIds=@([regex]::Matches($source,'"([a-z]+\.[a-z0-9-]+)"')|ForEach-Object{$_.Groups[1].Value}|Where-Object{$_ -in $documentIds})
  if($documentIds.Count-ne92-or(@($documentIds|Sort-Object -Unique).Count-ne92)){throw 'Frozen C5 hostile registry is not exactly 92 unique IDs.'}
  if((Compare-Object ($documentIds|Sort-Object) ($sourceIds|Sort-Object))){throw 'Rust C5 hostile registry differs from the frozen readiness registry.'}
  foreach($id in $documentIds){$fn=$id.Replace('.','_').Replace('-','_');if(!$test.Contains($id)-and!$test.Contains("fn $fn")){throw "C5 hostile has no executable test mapping: $id"}}
  foreach($forbidden in @('forge_kernel','tauri','std::fs','std::process','std::net','reqwest','tokio::net')){if($source.Contains($forbidden)){throw "C5 source crosses capability boundary: $forbidden"}}
  foreach($forbidden in @('forge-kernel','tauri','tokio','reqwest')){if($crateManifest.Contains($forbidden)){throw "C5 manifest crosses capability boundary: $forbidden"}}

  & cargo test -p significance-scheduler --locked
  if($LASTEXITCODE-ne0){throw 'C5 significance-scheduler tests failed.'}
  & cargo clippy -p significance-scheduler --all-targets --locked -- -D warnings
  if($LASTEXITCODE-ne0){throw 'C5 significance-scheduler strict Clippy failed.'}

  New-Item -ItemType Directory -Path $temp -Force|Out-Null
  $env:FORGE_ROOT=$root;$env:CARGO_TARGET_DIR=Join-Path $temp 'target'
  & cargo run --quiet --locked --offline --manifest-path $manifestPath -- --self-test
  if($LASTEXITCODE-ne0){throw 'C5 semantic receipt self-test failed.'}
  $nativeArgs="run --quiet --locked --offline --manifest-path `"$manifestPath`""
  $first=Invoke-C5Process $nativeArgs;$second=Invoke-C5Process $nativeArgs
  if($first.exit_code-ne0-or$second.exit_code-ne0-or$first.pid-eq$second.pid){throw 'portability.single-process'}
  $firstBytes=ConvertFrom-C5ReceiptHex $first.stdout;$secondBytes=ConvertFrom-C5ReceiptHex $second.stdout
  if($firstBytes.Length-ne$secondBytes.Length-or[Convert]::ToBase64String($firstBytes)-ne[Convert]::ToBase64String($secondBytes)){throw 'portability.stdout-mismatch'}
  $semanticSha=Get-C5Sha256 $firstBytes
  $nativeStdout=Get-C5TextSha256 $first.stdout;$nativeStderr=Get-C5TextSha256 $first.stderr
  if((Get-C5TextSha256 $second.stdout)-ne$nativeStdout-or(Get-C5TextSha256 $second.stderr)-ne$nativeStderr){throw 'C5 fresh native process output drifted.'}

  $i686Args="run --quiet --locked --offline --target i686-pc-windows-msvc --manifest-path `"$manifestPath`""
  $i686Result=Invoke-C5Process $i686Args
  if($i686Result.exit_code-ne0){throw 'Same-host i686 semantic execution failed.'}
  $i686Bytes=ConvertFrom-C5ReceiptHex $i686Result.stdout
  if($i686Bytes.Length-ne$firstBytes.Length-or[Convert]::ToBase64String($i686Bytes)-ne[Convert]::ToBase64String($firstBytes)){throw 'Same-host i686 semantic execution drifted.'}
  $androidArgs="check --quiet --locked --offline --target aarch64-linux-android --manifest-path `"$manifestPath`""
  $androidResult=Invoke-C5Process $androidArgs
  if($androidResult.exit_code-ne0){throw 'Android ARM64 compile-only evidence failed.'}
  if((Get-C5SourceManifestSha)-ne$sourceManifestSha){throw 'portability.source-mismatch'}

  $i686Stdout=Get-C5TextSha256 $i686Result.stdout;$i686Stderr=Get-C5TextSha256 $i686Result.stderr
  $androidStdout=Get-C5TextSha256 $androidResult.stdout;$androidStderr=Get-C5TextSha256 $androidResult.stderr
  $nativeExe=Join-Path $temp 'target\debug\c5-significance-scheduler-receipt.exe'
  $i686Exe=Join-Path $temp 'target\i686-pc-windows-msvc\debug\c5-significance-scheduler-receipt.exe'
  if(!(Test-Path -LiteralPath $nativeExe -PathType Leaf)-or!(Test-Path -LiteralPath $i686Exe -PathType Leaf)){throw 'C5 observed executable is missing.'}
  $nativeExeHash=(Get-FileHash $nativeExe -Algorithm SHA256).Hash.ToLowerInvariant();$i686ExeHash=(Get-FileHash $i686Exe -Algorithm SHA256).Hash.ToLowerInvariant()
  $rustcText=((rustc -vV)-join "`n");$cargoText=(cargo -V)

  & (Join-Path $root 'tools\test-g1-c5-portability-classifier.ps1')
  if(!$?){throw 'C5 portability classifier failed.'}

  if($EmitObservationReceipt){
    if(git status --porcelain){throw 'C5 platform observation source tree changed during verification.'}
    if((git rev-parse HEAD).Trim()-ne$sourceCommit){throw 'C5 platform observation source commit changed during verification.'}
    if((Get-C5TreeManifestSha $sourceCommit)-ne$treeManifestSha-or(Get-C5SourceManifestSha $sourceCommit)-ne$sourceManifestSha-or(Get-C5SourceManifestSha)-ne$sourceManifestSha){throw 'C5 platform observation source provenance changed during verification.'}
    $receipt=[ordered]@{schema_version=1;receipt_id='G1-C5-LOCAL-PLATFORM-OBSERVATIONS';semantic_receipt_sha256=$semanticSha;source_commit=$sourceCommit;tracked_tree_manifest_sha256=$treeManifestSha;bounded_source_manifest_sha256=$sourceManifestSha;rustc=$rustcText;cargo=$cargoText;observations=@(
      [ordered]@{classification='native_same_host_execution';target='x86_64-pc-windows-msvc';os='windows';architecture='x86_64';pointer_width=64;endian='little';runner='cargo-native';command=$nativeArgs;exit_code=$first.exit_code;process_ids=@($first.pid,$second.pid);stdout_sha256=@($nativeStdout,$nativeStdout);stderr_sha256=@($nativeStderr,$nativeStderr);executable=[ordered]@{present=$true;sha256=$nativeExeHash}},
      [ordered]@{classification='same_host_second_architecture';target='i686-pc-windows-msvc';os='windows';architecture='i686';pointer_width=32;endian='little';runner='cargo-i686';command=$i686Args;exit_code=$i686Result.exit_code;process_ids=@($i686Result.pid);stdout_sha256=@($i686Stdout);stderr_sha256=@($i686Stderr);executable=[ordered]@{present=$true;sha256=$i686ExeHash}},
      [ordered]@{classification='compile_only';target='aarch64-linux-android';os='android';architecture='aarch64';pointer_width=64;endian='little';runner='cargo-check';command=$androidArgs;exit_code=$androidResult.exit_code;process_ids=@($androidResult.pid);stdout_sha256=@($androidStdout);stderr_sha256=@($androidStderr);executable=[ordered]@{present=$false;reason='cargo check produces metadata only; no execution occurred'}}
    );independent_second_platform_execution=$false;promotion_authority=$false;c6_authority=$false}
    $receipt|ConvertTo-Json -Depth 12|Set-Content -LiteralPath $observationReceiptPath -Encoding utf8
  }

  if(Test-Path -LiteralPath $observationReceiptPath -PathType Leaf){
    $retainedReceipt=Get-Content -Raw $observationReceiptPath|ConvertFrom-Json
    if((Get-C5SourceManifestSha $retainedReceipt.source_commit)-ne$sourceManifestSha){throw 'C5 retained receipt source commit differs from the verified bounded source manifest.'}
    & (Join-Path $root 'tools\verify-g1-c5-platform-observation-receipt.ps1') -ReceiptPath $observationReceiptPath -SemanticSha256 $semanticSha -BoundedSourceManifestSha256 $sourceManifestSha -Rustc $rustcText -Cargo $cargoText -NativeCommand $nativeArgs -I686Command $i686Args -AndroidCommand $androidArgs -NativeStdout $nativeStdout -NativeStderr $nativeStderr -I686Stdout $i686Stdout -I686Stderr $i686Stderr -AndroidStdout $androidStdout -AndroidStderr $androidStderr -NativeExecutable $nativeExeHash -I686Executable $i686ExeHash
    if(!$?){throw 'C5 retained platform observation verification failed.'}
  }
  Write-Output "G1 C5 local implementation and portability gate verified: typed eight-domain surface, 90 Rust tests, strict Clippy, executable mapping for all 92 hostile IDs, capability-negative boundary, semantic $semanticSha across two fresh native x64 processes and same-host i686 execution, Android ARM64 compile-only, bounded source manifest $sourceManifestSha. Independent second-platform execution, promotion authority, C6 authority and closure remain unavailable."
}
finally {
  $env:FORGE_ROOT=$priorRoot;$env:CARGO_TARGET_DIR=$priorTarget
  if(Test-Path -LiteralPath $temp -PathType Container){Remove-Item -LiteralPath $temp -Recurse -Force}
}
