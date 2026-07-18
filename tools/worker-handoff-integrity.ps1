$script:WorkerHandoffSections = @(
  'objective',
  'next_action',
  'stage_context',
  'visual_quality_gate',
  'simulation_ladder',
  'unresolved_risks',
  'evidence_requirements',
  'verification_plan',
  'resume_after',
  'evidence',
  'verification_receipts',
  'transition'
)

function Get-WorkerHandoffSectionNames {
  return @($script:WorkerHandoffSections)
}

function Get-WorkerHandoffSectionValue {
  param(
    [Parameter(Mandatory = $true)]$State,
    [Parameter(Mandatory = $true)][string]$Section
  )
  if ($script:WorkerHandoffSections -notcontains $Section) {
    throw "Unknown worker handoff section: $Section"
  }
  if ($State -is [System.Collections.IDictionary]) {
    if (!$State.Contains($Section)) { throw "Worker handoff section is missing: $Section" }
    return $State[$Section]
  }
  if ($State.PSObject.Properties.Name -notcontains $Section) {
    throw "Worker handoff section is missing: $Section"
  }
  return $State.$Section
}

function Get-WorkerHandoffSectionHash {
  param(
    [Parameter(Mandatory = $true)]$State,
    [Parameter(Mandatory = $true)][string]$Section
  )
  if ($script:WorkerHandoffSections -notcontains $Section) {
    throw "Unknown worker handoff section: $Section"
  }
  if ($State -is [System.Collections.IDictionary]) {
    if (!$State.Contains($Section)) { throw "Worker handoff section is missing: $Section" }
    $canonical = ConvertTo-Json -InputObject $State[$Section] -Depth 100 -Compress
  } else {
    if ($State.PSObject.Properties.Name -notcontains $Section) {
      throw "Worker handoff section is missing: $Section"
    }
    $canonical = ConvertTo-Json -InputObject $State.$Section -Depth 100 -Compress
  }
  if ($null -eq $canonical) { throw "Worker handoff section could not be serialized: $Section" }
  $bytes = [System.Text.Encoding]::UTF8.GetBytes($canonical)
  $sha = [System.Security.Cryptography.SHA256]::Create()
  try {
    return ([System.BitConverter]::ToString($sha.ComputeHash($bytes))).Replace('-', '').ToLowerInvariant()
  } finally {
    $sha.Dispose()
  }
}
