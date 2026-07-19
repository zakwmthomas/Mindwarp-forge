param([switch]$ReceiptOnly)
$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
$temp=Join-Path ([IO.Path]::GetTempPath()) ('forge-c4-receipt-'+[guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path (Join-Path $temp 'src') -Force|Out-Null
$priorRoot=$env:FORGE_ROOT
function Get-C4SourceManifestSha {
  $paths=@('Cargo.lock','crates/hierarchy-history/Cargo.toml','crates/hierarchy-history/src/hierarchy.rs','crates/hierarchy-history/src/history.rs','crates/hierarchy-history/src/lib.rs','crates/hierarchy-history/src/proof.rs','crates/entity-lifecycle-history-binding/Cargo.toml','crates/entity-lifecycle-history-binding/src/lib.rs','tools/fixtures/c4-hierarchy-history-receipt/main.rs')
  $rows=foreach($relative in $paths){$full=Join-Path $root $relative;if(!(Test-Path -LiteralPath $full -PathType Leaf)){throw "C4 source manifest file missing: $relative"};$hash=(Get-FileHash -LiteralPath $full -Algorithm SHA256).Hash.ToLowerInvariant();"$relative`:$hash"}
  $digest=[Security.Cryptography.SHA256]::Create();try{([BitConverter]::ToString($digest.ComputeHash([Text.Encoding]::UTF8.GetBytes(($rows-join "`n")))).Replace('-','').ToLowerInvariant())}finally{$digest.Dispose()}
}
function Invoke-C4Process([string]$Arguments){$info=New-Object Diagnostics.ProcessStartInfo;$info.FileName='cargo';$info.Arguments=$Arguments;$info.UseShellExecute=$false;$info.RedirectStandardOutput=$true;$info.RedirectStandardError=$true;$info.CreateNoWindow=$true;$process=New-Object Diagnostics.Process;$process.StartInfo=$info;if(!$process.Start()){throw 'Failed to launch C4 receipt process.'}$stdout=$process.StandardOutput.ReadToEnd();$stderr=$process.StandardError.ReadToEnd();$process.WaitForExit();[pscustomobject]@{pid=$process.Id;exit_code=$process.ExitCode;stdout=$stdout;stderr=$stderr}}
try {
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

  $i686Result=Invoke-C4Process "run --quiet --offline --target i686-pc-windows-msvc --manifest-path `"$(Join-Path $temp 'Cargo.toml')`""
  if($i686Result.exit_code-ne0-or$i686Result.stdout.Trim()-ne$one){throw 'Same-host i686 semantic execution drifted.'}
  cargo check --quiet --offline --target aarch64-linux-android --manifest-path (Join-Path $temp 'Cargo.toml')
  if($LASTEXITCODE-ne0){throw 'Android compile-only evidence failed.'}
  if((Get-C4SourceManifestSha)-ne$sourceManifestSha){throw 'portability.source-mismatch'}

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
  Write-Output "G1 C4 local implementation verified: 58 core, 8 receipt and 8 portability hostile owners; semantic $semanticSha; source manifest $sourceManifestSha. Independent second-platform execution remains unavailable."
}
finally {
  $env:FORGE_ROOT=$priorRoot
  if(Test-Path -LiteralPath $temp){Remove-Item -LiteralPath $temp -Recurse -Force}
}
