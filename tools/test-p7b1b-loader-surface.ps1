$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$toolPath = Join-Path $root 'tools\prove-p7b1b-loader-surface.ps1'
$receiptPath = Join-Path $root 'evidence\p7b1b\loader-surface-proof.json'
foreach ($path in @($toolPath, $receiptPath)) { if (!(Test-Path -LiteralPath $path -PathType Leaf)) { throw "Loader-surface artifact missing: $path" } }
$source = Get-Content -LiteralPath $toolPath -Raw
foreach ($required in @('b1319077ce29984c50ea84d52f775bb7a0b0e868744c9a42e86d10d6167bcb66','25109e5e9c4de262b4dc4020f2b75b6ba6515c3c6570fad575e131d7bf820856','ReparsePoint rejected','image is not PE32+','section raw ranges overlap','Receipt overwrite rejected','canary_executed = $false','runtime_cause_proved = $false')) { if (!$source.Contains($required)) { throw "Loader-surface proof is missing shield: $required" } }
foreach ($forbidden in @('Start-Process','Invoke-Expression','CreateAppContainerProfile','ResumeThread','MiniDumpWriteDump','WER LocalDumps','dumpbin.exe','cargo.exe','Command::new')) { if ($source.Contains($forbidden)) { throw "Loader-surface proof crosses the offline boundary: $forbidden" } }
& powershell -NoProfile -ExecutionPolicy Bypass -File $toolPath -SelfTest
if ($LASTEXITCODE -ne 0) { throw 'Loader-surface hostile fixtures failed.' }
$receipt = Get-Content -LiteralPath $receiptPath -Raw | ConvertFrom-Json
if ($receipt.schema -ne 1 -or $receipt.status -ne 'completed_claim_limited' -or $receipt.candidates.Count -ne 2) { throw 'Loader-surface receipt schema/status/candidate count is invalid.' }
foreach ($field in @('canary_executed','profile_created','registry_modified','acl_modified','capability_added','runtime_cause_proved','denial_proved')) { if ($receipt.$field -ne $false) { throw "Loader-surface receipt overclaims: $field" } }
foreach ($candidate in $receipt.candidates) { if ($candidate.image.pe_magic -ne '0x020b' -or $candidate.image.machine -ne '0x8664' -or !$candidate.image.sections -or !$candidate.image.imports) { throw "Loader-surface candidate is incomplete: $($candidate.name)" } }
$escapedPath = Join-Path $root 'evidence\p7b1b-escape.json'; if (Test-Path -LiteralPath $escapedPath) { throw 'Negative-fixture output already exists.' }
$prior = $ErrorActionPreference; $ErrorActionPreference = 'Continue'; $output = & powershell -NoProfile -ExecutionPolicy Bypass -File $toolPath -OutputPath $escapedPath 2>&1; $exitCode = $LASTEXITCODE; $ErrorActionPreference = $prior
if ($exitCode -eq 0 -or ($output -join "`n") -notmatch 'exact evidence boundary' -or (Test-Path -LiteralPath $escapedPath)) { throw 'Loader-surface proof did not reject prefix-confusion output.' }
Write-Output 'P7b-1b loader-surface proof verified: exact inputs, direct PE32+ parsing, hostile-fixture rejection, fixed output, and claim limits hold.'
