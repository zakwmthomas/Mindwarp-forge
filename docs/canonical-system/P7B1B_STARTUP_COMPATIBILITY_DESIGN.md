# P7b-1b Capability-Free Startup Compatibility Design

## Decision

Trial 3 remains a failed containment trial. Its post-resume
`0xC0000142` (`STATUS_DLL_INIT_FAILED`) proves neither the failing module nor
the missing resource. This package therefore changes no capability, policy,
runner, profile, ACL, or production binary. It tests one smaller startup
candidate without executing it: link the denial canary with Rust's documented
`+crt-static` target feature in an isolated target directory and compare that
image with an explicitly dynamic-CRT build.

The result is `prototype_tested_not_executed`. It is build and import evidence,
not proof that the static candidate starts under LPAC and not a P7b-1b denial
pass.

## Primary-practice reconciliation

- Rust's `rustc` code-generation documentation defines `+crt-static` and
  `-crt-static` as the target-feature controls for static C-runtime linkage.
- The Rust Reference uses `target_feature = "crt-static"` to distinguish the
  two linkage modes.
- Microsoft's `/MD, /MT, /LD` documentation defines `/MT` as the static,
  multithreaded CRT choice. Static linkage can remove loader dependencies, but
  neither Rust nor Microsoft documentation claims LPAC compatibility.
- Trial 3's status is deliberately not converted into a guessed DLL, registry,
  or capability diagnosis. In particular, this evidence does **not** justify
  adding `registryRead`.

Sources:

- https://doc.rust-lang.org/stable/rustc/codegen-options/
- https://doc.rust-lang.org/stable/reference/linkage.html
- https://learn.microsoft.com/en-us/cpp/build/reference/md-mt-ld-use-run-time-library

## Reproducible comparison

`tools/prove-p7b1b-startup-compatibility.ps1` builds both variants with
`cargo build --locked --offline` for `x86_64-pc-windows-msvc`. The target
directories are isolated under `target/p7b1b-startup-proof`; the normal canary
and runner are hashed before and after. The proof tool parses PE security flags
directly, uses the locally installed `dumpbin.exe` only to list imports, binds
the tool and inspector hashes into its receipt, rejects reparse-point or
out-of-bound outputs, and contains no canary/runner launch path.

| Evidence | Dynamic CRT | Static CRT |
|---|---:|---:|
| SHA-256 | `b1319077ce29984c50ea84d52f775bb7a0b0e868744c9a42e86d10d6167bcb66` | `25109e5e9c4de262b4dc4020f2b75b6ba6515c3c6570fad575e131d7bf820856` |
| Bytes | 287,744 | 388,096 |
| Unique direct imports | 11 | 5 |
| `vcruntime140.dll` | present | absent |
| `api-ms-win-crt-*` | six present | absent |
| PE32+ / AppContainer / high-entropy ASLR / dynamic base / NX | retained | retained |

The static image is 100,352 bytes larger but removes all seven observed direct
dynamic-CRT imports. Its remaining direct imports are a strict subset of the
dynamic image: `advapi32.dll`, `api-ms-win-core-synch-l1-2-0.dll`,
`kernel32.dll`, `ntdll.dll`, and `ws2_32.dll`. The normal workspace canary and
runner hashes remain unchanged.

The bounded receipt is
`evidence/p7b1b/startup-compatibility-proof.json`, SHA-256
`1123373704a528e86c81e3d32e16c1842d95ecd84002565e9b0fd1cb0b0e3585`.
Two consecutive proof runs produced the same receipt.

## Adversarial review and failure boundaries

- A smaller direct import list does not prove successful initialization;
  transitive system loading and LPAC policy still exist.
- Static CRT reduces loader surface at a binary-size cost. The evidence does
  not establish performance, servicing, renderer, or product-quality benefit.
- The proof fails if AppContainer, ASLR, dynamic-base, or NX markings drift; if
  static linkage retains a dynamic CRT import; if it adds an import; if normal
  binaries change; or if output leaves the exact evidence boundary.
- The receipt states `canary_executed=false`, `lpac_profile_created=false`,
  `capability_added=false`, and `lpac_compatibility_proved=false`.
- Existing broker diagnostics retain the exact hexadecimal/decimal exit status
  and successful suspended-host LPAC verification mode. Deeper loader tracing
  would require a separately assessed tool/authority path and is not silently
  installed or enabled here.

## Whole-system alignment

This is diagnostic build evidence inside active F5. It does not change the
P7a representation contract, P7b controlled-perception protocol, P7b-1a
containment profile, P7b-1c renderer boundary, runtime adapter, engine,
publishing path, spending authority, or protected Kernel. It preserves the
project principle that evidence narrows uncertainty without weakening a gate
or promoting a prototype.

## Exact next boundary

A future static-CRT LPAC attempt, if selected by the master program, must be a
fresh one-run package with new preflight, exact binary/hash binding, existing
zero-capability policy, suspended-host verification, fixed denial probes,
independent cleanup, and one terminal receipt. This design package itself
authorizes and performs no execution and cannot be treated as that trial.
