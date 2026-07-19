param([switch]$ReceiptOnly,[string]$ObservationReceiptPath)
$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
$temp=Join-Path ([IO.Path]::GetTempPath()) 'forge-c4-receipt-v1'
New-Item -ItemType Directory -Path (Join-Path $temp 'src') -Force|Out-Null
$priorRoot=$env:FORGE_ROOT
function Get-C4SourceManifestSha([string]$Commit) {
  $paths=@('Cargo.lock','crates/hierarchy-history/Cargo.toml','crates/hierarchy-history/src/hierarchy.rs','crates/hierarchy-history/src/history.rs','crates/hierarchy-history/src/lib.rs','crates/hierarchy-history/src/proof.rs','crates/entity-lifecycle-history-binding/Cargo.toml','crates/entity-lifecycle-history-binding/src/lib.rs','tools/fixtures/c4-hierarchy-history-receipt/main.rs','tools/verify-g1-c4-closure-readiness.ps1','tools/verify-g1-c4-hierarchy-history-implementation.ps1','tools/verify-g1-c4-platform-observation-receipt.ps1','tools/test-g1-c4-portability-classifier.ps1','tools/verify-g1-c4-record-consistency.ps1')
  $rows=foreach($relative in $paths){$full=Join-Path $root $relative;if(!(Test-Path -LiteralPath $full -PathType Leaf)){throw "C4 source manifest file missing: $relative"};$blob=if([string]::IsNullOrWhiteSpace($Commit)){(git hash-object -- $full).Trim()}else{(git rev-parse "$Commit`:$relative").Trim()};if($LASTEXITCODE-ne0-or$blob-notmatch'^[0-9a-f]{40,64}$'){throw "C4 source manifest blob unavailable: $relative"};"$relative`:$blob"}
  $digest=[Security.Cryptography.SHA256]::Create();try{([BitConverter]::ToString($digest.ComputeHash([Text.Encoding]::UTF8.GetBytes(($rows-join "`n")))).Replace('-','').ToLowerInvariant())}finally{$digest.Dispose()}
}
function Get-C4TreeManifestSha([string]$Commit){$treeText=((git ls-tree -r --full-tree $Commit|Out-String).TrimEnd());$treeHasher=[Security.Cryptography.SHA256]::Create();try{([BitConverter]::ToString($treeHasher.ComputeHash([Text.Encoding]::UTF8.GetBytes($treeText))).Replace('-','').ToLowerInvariant())}finally{$treeHasher.Dispose()}}
function Invoke-C4Process([string]$Arguments){$info=New-Object Diagnostics.ProcessStartInfo;$info.FileName='cargo';$info.Arguments=$Arguments;$info.UseShellExecute=$false;$info.RedirectStandardOutput=$true;$info.RedirectStandardError=$true;$info.CreateNoWindow=$true;$process=New-Object Diagnostics.Process;$process.StartInfo=$info;if(!$process.Start()){throw 'Failed to launch C4 receipt process.'}$stdoutTask=$process.StandardOutput.ReadToEndAsync();$stderrTask=$process.StandardError.ReadToEndAsync();$process.WaitForExit();$stdout=$stdoutTask.GetAwaiter().GetResult();$stderr=$stderrTask.GetAwaiter().GetResult();[pscustomobject]@{pid=$process.Id;exit_code=$process.ExitCode;stdout=$stdout;stderr=$stderr}}
try {
  if($ObservationReceiptPath-and(git status --porcelain)){throw 'Platform observation receipt requires a clean source tree.'}
  $sourceCommit=(git rev-parse HEAD).Trim();$treeManifestSha=Get-C4TreeManifestSha $sourceCommit
  $sourceManifestSha=Get-C4SourceManifestSha
  $path=$root.Replace('\','/')
  $manifest=@"
[package]
name="c4-hierarchy-history-receipt"
version="0.1.0"
edition="2024"
[dependencies]
addressable-world-binding={path="$path/crates/addressable-world-binding"}
derived-world-rules={path="$path/crates/derived-world-rules"}
entity-lifecycle={path="$path/crates/entity-lifecycle"}
entity-lifecycle-history-binding={path="$path/crates/entity-lifecycle-history-binding"}
field-basis={path="$path/crates/field-basis"}
hierarchy-history={path="$path/crates/hierarchy-history"}
mindwarp-gameplay-foundation={path="$path/crates/mindwarp-gameplay-foundation"}
minicbor={version="0.26",features=["std"]}
sha2="0.10"
"@
  Set-Content -LiteralPath (Join-Path $temp 'Cargo.toml') -Value $manifest -Encoding utf8
  Copy-Item -LiteralPath (Join-Path $root 'tools\fixtures\c4-hierarchy-history-receipt\main.rs') -Destination (Join-Path $temp 'src\main.rs')
  $env:FORGE_ROOT=$root
  cargo run --quiet --offline --manifest-path (Join-Path $temp 'Cargo.toml') -- --self-test
  if($LASTEXITCODE-ne0){throw 'C4 semantic receipt self-test failed.'}
  $args="run --quiet --offline --manifest-path `"$(Join-Path $temp 'Cargo.toml')`"";$first=Invoke-C4Process $args;$second=Invoke-C4Process $args
  if($first.exit_code-ne0-or$second.exit_code-ne0){throw 'C4 semantic receipt process failed.'};$one=$first.stdout.Trim();$two=$second.stdout.Trim();if($first.pid-eq$second.pid-or$one-ne$two-or$one-notmatch'^[0-9a-f]+$'){throw 'portability.stdout-mismatch or portability.single-process'}
  $raw=New-Object byte[] ($one.Length/2);for($i=0;$i-lt$raw.Length;$i++){$raw[$i]=[Convert]::ToByte($one.Substring($i*2,2),16)}
  $sha=[Security.Cryptography.SHA256]::Create();try{$semanticSha=([BitConverter]::ToString($sha.ComputeHash($raw)).Replace('-','').ToLowerInvariant());$stdoutHashes=@($first.stdout,$second.stdout)|ForEach-Object{([BitConverter]::ToString($sha.ComputeHash([Text.Encoding]::UTF8.GetBytes($_))).Replace('-','').ToLowerInvariant())};$stderrHashes=@($first.stderr,$second.stderr)|ForEach-Object{([BitConverter]::ToString($sha.ComputeHash([Text.Encoding]::UTF8.GetBytes($_))).Replace('-','').ToLowerInvariant())}}finally{$sha.Dispose()}
  if($ReceiptOnly){Write-Output "C4 semantic receipt verified across two fresh native processes: $semanticSha";return}

  $i686Args="run --quiet --offline --target i686-pc-windows-msvc --manifest-path `"$(Join-Path $temp 'Cargo.toml')`"";$i686Result=Invoke-C4Process $i686Args
  if($i686Result.exit_code-ne0-or$i686Result.stdout.Trim()-ne$one){throw 'Same-host i686 semantic execution drifted.'}
  $androidArgs="check --quiet --offline --target aarch64-linux-android --manifest-path `"$(Join-Path $temp 'Cargo.toml')`"";$androidResult=Invoke-C4Process $androidArgs
  if($androidResult.exit_code-ne0){throw 'Android compile-only evidence failed.'}
  if((Get-C4SourceManifestSha)-ne$sourceManifestSha){throw 'portability.source-mismatch'}
  $hash=[Security.Cryptography.SHA256]::Create();try{$i686Stdout=([BitConverter]::ToString($hash.ComputeHash([Text.Encoding]::UTF8.GetBytes($i686Result.stdout))).Replace('-','').ToLowerInvariant());$i686Stderr=([BitConverter]::ToString($hash.ComputeHash([Text.Encoding]::UTF8.GetBytes($i686Result.stderr))).Replace('-','').ToLowerInvariant());$androidStdout=([BitConverter]::ToString($hash.ComputeHash([Text.Encoding]::UTF8.GetBytes($androidResult.stdout))).Replace('-','').ToLowerInvariant());$androidStderr=([BitConverter]::ToString($hash.ComputeHash([Text.Encoding]::UTF8.GetBytes($androidResult.stderr))).Replace('-','').ToLowerInvariant())}finally{$hash.Dispose()}
  $nativeExe=Join-Path $temp 'target\debug\c4-hierarchy-history-receipt.exe';$i686Exe=Join-Path $temp 'target\i686-pc-windows-msvc\debug\c4-hierarchy-history-receipt.exe';$nativeExeHash=(Get-FileHash $nativeExe -Algorithm SHA256).Hash.ToLowerInvariant();$i686ExeHash=(Get-FileHash $i686Exe -Algorithm SHA256).Hash.ToLowerInvariant();$rustcText=((rustc -vV)-join "`n");$cargoText=(cargo -V)

  & (Join-Path $root 'tools\verify-g1-c4-closure-readiness.ps1')
  if(!$?){throw 'C4 readiness regression failed.'}
  cargo fmt --all -- --check;if($LASTEXITCODE-ne0){throw 'C4 formatting failed.'}
  cargo clippy -p hierarchy-history -p entity-lifecycle-history-binding --all-targets --offline -- -D warnings
  if($LASTEXITCODE-ne0){throw 'C4 focused clippy failed.'}
  cargo test -p hierarchy-history -p addressable-world-binding -p entity-lifecycle -p entity-lifecycle-history-binding --offline
  if($LASTEXITCODE-ne0){throw 'C4 focused regressions failed.'}
  cargo test -p mindwarp-vertical-persistence --offline
  if($LASTEXITCODE-ne0){throw 'C4V regression failed.'}

  $readiness=Get-Content -Raw (Join-Path $root 'tools\verify-g1-c4-closure-readiness.ps1')
  $line=($readiness -split "`r?`n"|Where-Object{$_ -like '*identity.dynamic-zero-parent*'})
  $ids=[regex]::Matches($line,"'([^']+)'")|ForEach-Object{$_.Groups[1].Value}
  if($ids.Count-ne74){throw 'Frozen hostile registry was not found.'}
  $coreText=(Get-Content -Raw (Join-Path $root 'crates\hierarchy-history\tests\c4_closure.rs'))+(Get-Content -Raw (Join-Path $root 'crates\entity-lifecycle-history-binding\tests\c4_cohort_binding.rs'))
  foreach($id in $ids[0..57]){if(!$coreText.Contains($id)){throw "Core hostile lacks executable ownership: $id"}}
  $fixture=Get-Content -Raw (Join-Path $root 'tools\fixtures\c4-hierarchy-history-receipt\main.rs')
  foreach($id in $ids[58..65]){if(!$fixture.Contains($id)){throw "Receipt hostile lacks executable ownership: $id"}}
  $portabilityText=(Get-Content -Raw $MyInvocation.MyCommand.Path)+(Get-Content -Raw (Join-Path $root 'tools\test-g1-c4-portability-classifier.ps1'))
  foreach($id in $ids[66..73]){if(!$portabilityText.Contains($id)){throw "Portability hostile lacks verifier ownership: $id"}}

  & (Join-Path $root 'tools\test-g1-c4-portability-classifier.ps1');if(!$?){throw 'C4 portability classifier failed.'}
  if($ObservationReceiptPath){
    if(git status --porcelain){throw 'Platform observation source tree changed during verification.'}
    if((git rev-parse HEAD).Trim()-ne$sourceCommit){throw 'Platform observation source commit changed during verification.'}
    if((Get-C4TreeManifestSha $sourceCommit)-ne$treeManifestSha){throw 'Platform observation tracked-tree manifest changed during verification.'}
    if((Get-C4SourceManifestSha)-ne$sourceManifestSha){throw 'Platform observation bounded source manifest changed during verification.'}
    $receipt=[ordered]@{schema_version=1;receipt_id='G1-C4-LOCAL-PLATFORM-OBSERVATIONS';semantic_receipt_sha256=$semanticSha;source_commit=$sourceCommit;tracked_tree_manifest_sha256=$treeManifestSha;bounded_source_manifest_sha256=$sourceManifestSha;rustc=$rustcText;cargo=$cargoText;observations=@(
      [ordered]@{classification='native_same_host_execution';target='x86_64-pc-windows-msvc';os='windows';architecture='x86_64';pointer_width=64;endian='little';runner='cargo-native';command=$args;exit_code=$first.exit_code;process_ids=@($first.pid,$second.pid);stdout_sha256=$stdoutHashes;stderr_sha256=$stderrHashes;executable=[ordered]@{present=$true;sha256=$nativeExeHash}},
      [ordered]@{classification='same_host_second_architecture';target='i686-pc-windows-msvc';os='windows';architecture='i686';pointer_width=32;endian='little';runner='cargo-i686';command=$i686Args;exit_code=$i686Result.exit_code;process_ids=@($i686Result.pid);stdout_sha256=@($i686Stdout);stderr_sha256=@($i686Stderr);executable=[ordered]@{present=$true;sha256=$i686ExeHash}},
      [ordered]@{classification='compile_only';target='aarch64-linux-android';os='android';architecture='aarch64';pointer_width=64;endian='little';runner='cargo-check';command=$androidArgs;exit_code=$androidResult.exit_code;process_ids=@($androidResult.pid);stdout_sha256=@($androidStdout);stderr_sha256=@($androidStderr);executable=[ordered]@{present=$false;reason='cargo check produces metadata only; no execution occurred'}}
    );independent_second_platform_execution=$false;promotion_authority=$false}
    $receipt|ConvertTo-Json -Depth 12|Set-Content -LiteralPath $ObservationReceiptPath -Encoding utf8
  }
  $retainedPath=if($ObservationReceiptPath){$ObservationReceiptPath}else{Join-Path $root 'docs\canonical-system\G1_C4_LOCAL_PLATFORM_OBSERVATIONS.json'}
  $retainedReceipt=Get-Content -Raw $retainedPath|ConvertFrom-Json
  if((Get-C4SourceManifestSha $retainedReceipt.source_commit)-ne$sourceManifestSha){throw 'C4 receipt source commit does not contain the verified bounded source manifest.'}
  & (Join-Path $root 'tools\verify-g1-c4-platform-observation-receipt.ps1') -ReceiptPath $retainedPath -SemanticSha256 $semanticSha -BoundedSourceManifestSha256 $sourceManifestSha -Rustc $rustcText -Cargo $cargoText -NativeCommand $args -I686Command $i686Args -AndroidCommand $androidArgs -NativeStdout $stdoutHashes[0] -NativeStderr $stderrHashes[0] -I686Stdout $i686Stdout -I686Stderr $i686Stderr -AndroidStdout $androidStdout -AndroidStderr $androidStderr -NativeExecutable $nativeExeHash -I686Executable $i686ExeHash
  if(!$?){throw 'C4 retained platform observation verification failed.'}
  if(!$ObservationReceiptPath){& (Join-Path $root 'tools\verify-g1-c4-record-consistency.ps1');if(!$?){throw 'C4 retained record consistency failed.'}}
  Write-Output "G1 C4 local implementation verified: 58 core, 8 receipt and 8 portability hostile owners; semantic $semanticSha; source manifest $sourceManifestSha. Independent second-platform execution remains unavailable."
}
finally {
  $env:FORGE_ROOT=$priorRoot
}
