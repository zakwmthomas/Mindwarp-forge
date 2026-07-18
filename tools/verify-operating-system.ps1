$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$policyPath = Join-Path $root 'governance\policy-registry.json'
$workPath = Join-Path $root 'context\active\WORKER_BATCH_STATE.json'
foreach ($path in @($policyPath, $workPath)) { if (!(Test-Path -LiteralPath $path)) { throw "Operating-system record missing: $path" } }
$policy = Get-Content -LiteralPath $policyPath -Raw | ConvertFrom-Json
if ($policy.schema_version -ne 1) { throw 'Unsupported policy registry schema.' }
$ids = @($policy.policies.id) + @($policy.proposals.id)
if (($ids | Group-Object | Where-Object Count -gt 1).Count -gt 0) { throw 'Policy registry has duplicate IDs.' }
if (@($policy.policies | Where-Object status -eq 'approved').Count -lt 4) { throw 'Required approved core policies are missing.' }
$work = Get-Content -LiteralPath $workPath -Raw | ConvertFrom-Json
if ($work.schema_version -ne 3 -or [string]::IsNullOrWhiteSpace($work.next_action)) { throw 'Canonical active checkpoint is invalid.' }
Write-Output "Operating system verified: $(@($policy.policies).Count) approved policies; active work package $($work.batch_id)."
