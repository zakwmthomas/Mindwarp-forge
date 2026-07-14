# P7b-1b Offline PE Loader-Surface Proof Result

## Result

The owner-approved, capability-free proof completed over the two exact retained
candidate images. It parsed their bytes directly and did not launch a canary,
create an LPAC profile, change registry or ACL state, add a capability, weaken
containment, or interact with the protected Kernel.

The receipt is `evidence/p7b1b/loader-surface-proof.json`, SHA-256
`9e6d266b518438760e11d6c6158bab6c930dde1fbc6e97ad5aa467365d73968e`.
The parser source SHA-256 bound into that receipt is
`0fdfc4f08b42c19926a70c95037b4df776be75541d438f87f40cc8008a64b4a6`.

## Bound observations

| Field | Dynamic image | Static image | Diagnostic meaning |
|---|---|---|---|
| Candidate SHA-256 | `b1319077ce29984c50ea84d52f775bb7a0b0e868744c9a42e86d10d6167bcb66` | `25109e5e9c4de262b4dc4020f2b75b6ba6515c3c6570fad575e131d7bf820856` | Exact retained inputs accepted |
| Direct DLLs / imported functions | 11 / 115 | 5 / 123 | Static CRT removes the six already known direct DLLs but imports more symbols from its retained DLL set |
| Delay imports | 0 | 0 | No delay-load discriminator |
| TLS callbacks | 1 at `0x0000000140020460` | 1 at `0x0000000140020460` | No count or address discriminator; callback behavior was not executed or proved |
| Entry point RVA | `0x00030f80` | `0x00030f80` | No entry-point discriminator |
| Load config | 320 bytes, `f0fdc2b85769d1ec16235e3bf0e3f3ba5d922c3791f57f9a0e147c410766edae` | 320 bytes, `1615945703c9a9c7a02f98959cb10ba10f757f4c1cf49b990b4bff5530afa724` | Identity differs, as expected for different images; no field-level runtime cause is inferred |
| Sections | `.text`, `.rdata`, `.data`, `.pdata`, `.reloc` | same plus `.fptable` | Structural difference bound; it does not identify the startup failure |
| Resources / manifests | no resource directory / 0 manifests | no resource directory / 0 manifests | No embedded-resource discriminator |

The parser also binds every section record and direct import DLL/function pair
in canonical JSON. Duplicate DLL descriptors are merged by DLL name with a
sorted unique function set; the original byte image remains bound by hash.

## Adversarial and claim checks

The proof rejects truncation, PE32, invalid `e_lfanew`, overlapping section raw
ranges, and an out-of-range import RVA. It also rejects reparse-point inputs,
non-exact candidate hashes, ambiguous RVA mappings, unknown delay-import pointer
layouts, prefix-confused output paths, and conflicting receipt overwrite.

The admitted receipt states `canary_executed=false`, `profile_created=false`,
`registry_modified=false`, `acl_modified=false`, `capability_added=false`,
`runtime_cause_proved=false`, and `denial_proved=false`.

## Conclusion and stop rule

H3's previously unknown PE fields are now bound. Neither image has a delay
import, both have the same single TLS callback address and entry point, and
neither embeds resources. The load-config hashes and static `.fptable` section
differ, but those image identities do not distinguish a plausible cause of the
shared `0xC0000142` result.

Therefore the static-optimization stop rule fires: do not create another build
variant or infer a runtime DLL, registry key, object, or denied access. P7b-1b
remains blocked without a denial proof. Any future dynamic module observation
requires a separate, exact owner decision; leaving the package blocked is also
valid.
