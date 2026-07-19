function Test-G1C5FullGateReconciliationRoute {
    [CmdletBinding()]
    param([Parameter(Mandatory = $true)]$Checkpoint)

    $expectedAuthority = 'Owner-authorized bounded C5 significance/scheduler implementation and capability-free closure proof only. Exact dependency C4. Frozen candidate G1_C5_CLOSURE_READINESS.md. No C3B, C6, C7, broad G1 closure, runtime controllers, runtime executors, cache mutation, storage mutation, product weights, AI generation, rendering implementation, filesystem, network, process, Companion, Greenfield, visual assets or Kernel mutation.'
    foreach ($field in @('batch_id', 'master_program_item', 'state', 'substage_id', 'authority_lane')) {
        if ($Checkpoint.$field -isnot [string]) { return $false }
    }
    return $Checkpoint.batch_id -eq 'G1-C5-SIGNIFICANCE-SCHEDULER-CLOSURE-V1' -and
        $Checkpoint.master_program_item -eq 'C5' -and
        $Checkpoint.state -eq 'executing' -and
        $Checkpoint.substage_id -eq 'c5-full-gate-route-reconciliation' -and
        $Checkpoint.authority_lane -eq $expectedAuthority
}

function Test-G1C5RecordedClosureRoute {
    [CmdletBinding()]
    param([Parameter(Mandatory = $true)]$Checkpoint)

    $expectedAuthority = 'Owner-authorized recorded C5 significance/scheduler capability-free closure evidence only. Exact dependency C4. C5 remains the sole master-program cursor pending a separate C6 transition. No C3B, C6 activation, C7, broad G1 closure, runtime controllers, runtime executors, cache mutation, storage mutation, product weights, AI generation, rendering implementation, filesystem, network, process, Companion, Greenfield, visual assets, promotion authority or Kernel mutation.'
    foreach ($field in @('batch_id', 'master_program_item', 'state', 'substage_id', 'authority_lane')) {
        if ($Checkpoint.$field -isnot [string]) { return $false }
    }
    return $Checkpoint.batch_id -eq 'G1-C5-SIGNIFICANCE-SCHEDULER-CLOSURE-V1' -and
        $Checkpoint.master_program_item -eq 'C5' -and
        $Checkpoint.state -eq 'recorded' -and
        $Checkpoint.substage_id -eq 'c5-registered-closure-recorded' -and
        $Checkpoint.authority_lane -eq $expectedAuthority
}
