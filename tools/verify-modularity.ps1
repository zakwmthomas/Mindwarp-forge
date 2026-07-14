param(
  [string]$Root,
  [string]$BoundaryPath
)
$ErrorActionPreference = 'Stop'
$root = if ($Root) { (Resolve-Path $Root).Path } else { Split-Path -Parent $PSScriptRoot }
$boundaryPath = if ($BoundaryPath) { (Resolve-Path $BoundaryPath).Path } else { Join-Path $root 'governance\module-boundaries.json' }
$policy = Get-Content $boundaryPath -Raw | ConvertFrom-Json
if ($policy.schema_version -ne 1) { throw 'Unsupported module-boundary schema.' }
$errors = [System.Collections.Generic.List[string]]::new()
$modules = @{}; foreach ($module in @($policy.modules)) {
  if ($modules.ContainsKey($module.id)) { $errors.Add("Duplicate module id: $($module.id)") }
  $modules[$module.id] = $module
}

foreach ($module in @($policy.modules)) {
  foreach ($dependency in @($module.dependencies)) {
    if (!$modules.ContainsKey($dependency)) { $errors.Add("$($module.id): unknown dependency $dependency") }
  }
  $moduleRoot = Join-Path $root $module.root
  if (!(Test-Path $moduleRoot)) { $errors.Add("$($module.id): missing module root $($module.root)"); continue }
  $extensions = @($module.source_extensions)
  foreach ($file in Get-ChildItem $moduleRoot -Recurse -File | Where-Object {
    $extensions -contains $_.Extension -and $_.FullName -notmatch '[\\/](?:target|node_modules|dist|gen)[\\/]'
  }) {
    foreach ($rule in @($module.forbidden_imports)) {
      $matches = Select-String -LiteralPath $file.FullName -Pattern $rule.pattern -CaseSensitive
      foreach ($match in @($matches)) {
        $relative = $file.FullName.Substring($root.Length).TrimStart('\','/').Replace('\','/')
        $errors.Add("$($module.id): forbidden import at ${relative}:$($match.LineNumber): $($rule.reason)")
      }
    }
  }
}

$states = @{}; $path = [System.Collections.Generic.List[string]]::new()
function Visit-Module([string]$id) {
  if ($states[$id] -eq 'done') { return }
  if ($states[$id] -eq 'visiting') {
    $start = $path.IndexOf($id)
    $cycle = @($path.GetRange($start, $path.Count - $start)) + $id
    $errors.Add("Dependency cycle: $($cycle -join ' -> ')")
    return
  }
  $states[$id] = 'visiting'; $path.Add($id)
  foreach ($dependency in @($modules[$id].dependencies)) { if ($modules.ContainsKey($dependency)) { Visit-Module $dependency } }
  $path.RemoveAt($path.Count - 1); $states[$id] = 'done'
}
foreach ($id in @($modules.Keys | Sort-Object)) { Visit-Module $id }

if ($policy.verify_cargo_dependencies -ne $false) {
  $cargo = Join-Path $env:USERPROFILE '.cargo\bin\cargo.exe'
  if (!(Test-Path $cargo)) { $errors.Add('Cargo is unavailable for workspace dependency verification.') }
  else {
    $metadataText = & $cargo metadata --format-version 1 --no-deps --manifest-path (Join-Path $root 'Cargo.toml') 2>&1
    if ($LASTEXITCODE -ne 0) { $errors.Add("Cargo metadata failed: $($metadataText -join ' ')") }
    else {
      $metadata = $metadataText | ConvertFrom-Json
      $workspaceNames = @{}; foreach ($package in @($metadata.packages)) { $workspaceNames[$package.name] = $package }
      foreach ($module in @($policy.modules | Where-Object { Test-Path (Join-Path (Join-Path $root $_.root) 'Cargo.toml') })) {
        $manifest = (Resolve-Path (Join-Path (Join-Path $root $module.root) 'Cargo.toml')).Path
        $package = @($metadata.packages | Where-Object { [IO.Path]::GetFullPath($_.manifest_path) -eq $manifest }) | Select-Object -First 1
        if (!$package) { $errors.Add("$($module.id): Cargo package not found in workspace metadata"); continue }
        $actual = @($package.dependencies | Where-Object { $_.path -and $workspaceNames.ContainsKey($_.name) } | ForEach-Object name | Sort-Object -Unique)
        $declared = @($module.dependencies | Sort-Object -Unique)
        if (($actual -join ',') -ne ($declared -join ',')) {
          $errors.Add("$($module.id): declared dependencies [$($declared -join ', ')] differ from Cargo workspace dependencies [$($actual -join ', ')]")
        }
      }
    }
  }
}

if ($errors.Count -gt 0) {
  $errors | ForEach-Object { Write-Error $_ -ErrorAction Continue }
  throw "Modularity verification failed with $($errors.Count) issue(s)."
}
Write-Output "Modularity verified: $($modules.Count) modules; no forbidden imports or dependency cycles."
