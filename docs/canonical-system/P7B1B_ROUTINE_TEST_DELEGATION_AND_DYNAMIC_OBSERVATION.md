# P7b-1b Routine-Test Delegation and Dynamic Observation

## Standing owner delegation

On 2026-07-13 the owner clarified that routine, bounded tests may run without
another per-test confirmation. A fresh owner stop is required only when the
work changes project direction, selects or installs a different program,
weakens a boundary, introduces a materially different uncertain approach, or
crosses an existing authority boundary.

This is standing delegation for tests, not authority for spending, publishing,
credentials, promotion, engine selection, containment weakening,
protected-Kernel mutation, or unbounded retries. A test must still be deterministic,
bounded, cleanup-verified, recorded, and stopped when its expected information
gain is exhausted.

## Selected routine diagnostic

Extend the existing `containment-canary-runner` broker with one diagnostic-only
mode using native `DEBUG_ONLY_THIS_PROCESS`, `WaitForDebugEvent`, and
`ContinueDebugEvent`. It may run the exact retained static candidate once and
record only:

- ordered create-process, DLL-load, exception, and exit-process events;
- DLL final paths obtained from event-provided file handles;
- image base addresses, exception codes/addresses/first-chance flags, and the
  final process status;
- the existing suspended LPAC verification mode, exact candidate/runner hashes,
  cleanup result, and claim limits.

The broker must close every event-provided file handle, pass non-bootstrap
exceptions back as `DBG_EXCEPTION_NOT_HANDLED`, retain the three-second wall
bound, and preserve the existing unique profile, zero capabilities, job,
mitigation, immutable-input, ACL restoration, profile deletion, inventory, and
no-retry shields.

## Authority and claim boundary

The mode is diagnostic-only. It cannot produce a containment pass even if the
canary unexpectedly reaches its report. It must state that debug semantics
changed the launch, and set `denial_proved=false` and
`runtime_cause_proved=false` unless an exact event directly establishes a
cause. It may not read process memory, registers, stacks, symbols, debug
strings, dumps, registry state, unrelated files, or network state; attach to an
existing process; install or launch a debugger program; add capabilities; or
weaken LPAC, job, mitigation, child, ACL, cleanup, or admission policy.

## P10 baseline and stop rule

- **Baseline:** identical Trial 3/4 terminal `0xC0000142`, five static direct
  DLLs, no module order, exception event, or failing module.
- **Expected gain:** one ordered native loader/exception trace without an
  external tool or dump.
- **Cost:** one small broker mode, deterministic helper tests, one three-second
  run, cleanup verification, and receipt integration.
- **Uncertainty:** debug semantics may alter timing or initialization, and an
  exit event may still name no failing DLL.
- **Regression guard:** the original `run-once` mode and its six tests remain
  unchanged; diagnostic output is a fresh separate receipt and never a pass.
- **Stop/refocus:** run exactly once. If no event directly identifies a cause,
  do not retry or add broader debugging. Record the remaining ambiguity and
  return to the canonical selector.

## Primary API basis

Accessed 2026-07-13: Microsoft documents that `CREATE_SUSPENDED` retains the
primary thread until `ResumeThread`; `DEBUG_ONLY_THIS_PROCESS` attaches the
creating thread as debugger; `WaitForDebugEvent` must be called by that same
thread; DLL-load events provide a file handle that the debugger must close;
and `ContinueDebugEvent` controls whether exception processing continues.
`GetFinalPathNameByHandleW` supplies the final path for the event handle. These
contracts justify the bounded observer but do not guarantee it will identify
the cause.
