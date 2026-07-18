[CmdletBinding()]
param(
    [string]$ReviewNote = 'Reviewed all handoff sections against the active substage.'
)
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$statePath = Join-Path $root 'context\active\WORKER_BATCH_STATE.json'
& (Join-Path $PSScriptRoot 'forge-writer-lease.ps1') -Mode assert -SessionId $env:CODEX_THREAD_ID -AllowLiveDatabaseMutation
if (!$?) { throw 'Canonical worker handoff receipt refresh requires the active checkpoint-bound Forge writer lease.' }
. (Join-Path $PSScriptRoot 'worker-handoff-integrity.ps1')
$state = Get-Content -LiteralPath $statePath -Raw | ConvertFrom-Json
$receipts = [ordered]@{}
foreach ($section in Get-WorkerHandoffSectionNames) {
    $receipts[$section] = [ordered]@{
        stage_id = [string]$state.substage_id
        content_sha256 = Get-WorkerHandoffSectionHash -State $state -Section $section
        review_disposition = 'revised'
        review_note = $ReviewNote
    }
}
$state.handoff_section_receipts = [pscustomobject]$receipts
$state | ConvertTo-Json -Depth 30 | Set-Content -LiteralPath $statePath -Encoding utf8
Write-Output "Worker handoff receipts refreshed for $($state.substage_id)."
