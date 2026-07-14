# P7b-1b Dynamic Observation Trial 5 Result

## Result

The standing-delegated routine diagnostic was consumed exactly once as run ID
`85b051eec7ca37971e3ca4d3fceb2678`. It failed before receiving a debug event:
`WaitForDebugEvent` timed out with Windows error 121 while the primary thread
was still explicitly suspended. It therefore identified no module, exception,
runtime cause, or denial behavior.

- Receipt: `evidence/p7b1b/trial-5-debug-events.json`
- Receipt SHA-256:
  `8bd2baa58086c85237ef392fc9281d5d9c673018f26c786f26d71bebd94055c1`
- Exact static candidate SHA-256:
  `25109e5e9c4de262b4dc4020f2b75b6ba6515c3c6570fad575e131d7bf820856`
- Executed runner SHA-256:
  `09cdb3f3b1a20449e4f74863d4b96aaad050d378605d48cb2ae0eeb1d425068a`
- Receipt status: `failed`; `cleanup_ok=false`; event count `0`.
- Exact receipt error includes `The semaphore timeout period has expired`
  (Windows error 121), plus `process did not settle during cleanup: 258`,
  and one access-denied removal of the exact owned staging directory.

The receipt truthfully retains `debug_semantics_changed=true`,
`denial_proved=false`, `runtime_cause_proved=false`, zero capability addition,
and no registry modification. It does not convert this run into a containment
trial or pass.

## Independent cleanup

After the runner exited, independent checks found no containment process,
profile mapping, package folder, loopback exemption, or other exact temp path.
One exact owned staging directory remained. Its DACL had been restored to the
owner's original inherited access; the directory and staged canary were then
removed explicitly after verifying the resolved target was the exact run-ID
path beneath the system temp directory.

The independent cleanup receipt is
`evidence/p7b1b/trial-5-independent-cleanup.json`. It records zero remaining
owned processes, exact temp paths, package folders, profile mappings, and
loopback exemptions, plus unchanged executed runner and candidate hashes. This
post-run evidence establishes eventual cleanup, but it does not rewrite the
original receipt's `cleanup_ok=false`.

## Root cause and prospective repair

The implementation waited for `CREATE_PROCESS_DEBUG_EVENT` before calling
`ResumeThread`. On this host, no event was delivered in that ordering. This is
an observer-ordering failure, not evidence about the canary's DLL startup.

The prospective source now preserves all suspended-host and immutable-hash
checks first, calls `ResumeThread`, then enters the debug-event loop. Microsoft
documents that the create-process debug event occurs before the initial thread
runs in user mode, so the observer can still receive the event before canary
user code. The prospective cleanup path also calls
`DebugActiveProcessStop` before job termination if observation fails with an
active debug relationship. Eight runner tests and formatting pass after this
repair. Prospective source SHA-256 is
`fc94fe9616acabc875b26098d6089598259bbd8c5e1946175a8fa81921ace354`.

This repair was not executed. **Do not retry Trial 5.** The one-run stop rule
also forbids fabricating an event trace or claiming the repaired ordering works
at runtime.

## Authority boundary and next state

No external debugger program, dump, process-memory read, register/stack/symbol
inspection, added capability, weakened LPAC/job/mitigation, renderer, engine,
publishing, spending, promotion, or protected-Kernel mutation occurred.

P7b-1b remains unproved and the failing DLL remains unknown. The standing
routine-test delegation remains valid for future genuinely routine packages,
but it does not erase this package's exact one-run stop rule or authorize a
silent retry under a new name.
