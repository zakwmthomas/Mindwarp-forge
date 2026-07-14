param([switch]$Check)
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$programPath = Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json'
$outputPath = Join-Path $root 'docs\project-atlas\MASTER_PLAN.md'
$program = Get-Content -LiteralPath $programPath -Raw | ConvertFrom-Json

$lines = [Collections.Generic.List[string]]::new()
$lines.Add('# Forge Master Plan')
$lines.Add('')
$lines.Add('> GENERATED from `docs/canonical-system/MASTER_PROGRAM.json`. Do not edit this view directly.')
$lines.Add('')
$executing = @($program.items | Where-Object state -eq 'executing')
$lines.Add("Canonical lifecycle: schema v$($program.schema_version); executing item: ``$($executing[0].id)``.")
$lines.Add('Planning doctrine: `docs/canonical-system/MASTER_PLAN_V2.md`.')
$lines.Add('')
foreach ($milestone in @($program.items.milestone | Select-Object -Unique)) {
  $lines.Add("## $milestone")
  $lines.Add('')
  foreach ($item in @($program.items | Where-Object milestone -eq $milestone)) {
    $dependencies = if (@($item.depends_on).Count) { @($item.depends_on) -join ', ' } else { 'none' }
    $lines.Add("- **$($item.id)** - $($item.state); gate: $($item.gate); depends on: $dependencies.")
    $lines.Add("  Next: $($item.next_action)")
  }
  $lines.Add('')
}
$expected = ($lines -join "`n").TrimEnd() + "`n"

if ($Check) {
  if (!(Test-Path -LiteralPath $outputPath -PathType Leaf)) { throw 'Generated master-plan view is missing.' }
  $actual = (Get-Content -LiteralPath $outputPath -Raw).Replace("`r`n", "`n")
  if ($actual -ne $expected) { throw 'Generated master-plan view is stale.' }
  Write-Output 'Generated master-plan view is current.'
  exit
}

[IO.File]::WriteAllText($outputPath, $expected, [Text.UTF8Encoding]::new($false))
Write-Output "Refreshed $outputPath"
