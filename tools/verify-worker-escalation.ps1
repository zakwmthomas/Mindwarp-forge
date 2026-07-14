param([Parameter(Mandatory=$true)][string]$Path)
$ErrorActionPreference = 'Stop'
$records = @(Get-Content -LiteralPath $Path | Where-Object { $_ } | ForEach-Object { $_ | ConvertFrom-Json })
$audits = @($records | Where-Object type -eq 'optimization_audit')
$escalations = @($records | Where-Object type -eq 'owner_escalation')
if ($audits.Count -lt 3) { throw 'Insufficient optimization-audit history.' }
$lastThree = @($audits | Select-Object -Last 3)
$allFailed = @($lastThree | Where-Object result -eq 'failed').Count -eq 3
if ($allFailed -and $escalations.Count -ne 1) { throw 'Three failed optimization audits require exactly one owner escalation.' }
if (!$allFailed -and $escalations.Count -ne 0) { throw 'Successful correction must not create an owner escalation.' }
Write-Output "Worker escalation verified: failed_sequence=$allFailed; escalations=$($escalations.Count)."
