# G1 GP3 encounter grammar fixed registry

Status: exact authored pre-source values frozen for independent review.

## Digest framing and GP0 authority

Every digest uses SHA-256 over unsigned big-endian framing:
`u32_be(domain UTF-8 byte length) || domain UTF-8 || u64_be(payload byte length) || payload`.
Payload is the owning value's canonical JSON. Domains are exactly:

- session: `mindwarp.gp3.fixed-session.v1`
- fact: `mindwarp.gp3.session-fact.v1`
- risk: `mindwarp.gp3.risk.v1`
- whole threat: `mindwarp.gp3.threat.v1`
- outcome mutation: `mindwarp.gp3.consequence.mutation.v1`
- opportunity cost: `mindwarp.gp3.consequence.opportunity-cost.v1`
- memory: `mindwarp.gp3.consequence.memory.v1`
- grant: `mindwarp.gp3.consequence.grant.v1`
- named decision: `mindwarp.gp3.consequence.named-decision.v1`
- selected threat element: `mindwarp.gp3.consequence.threat-contribution.v1`
- whole situation: `mindwarp.gp3.fixed-situation.v1`
- whole grammar: `mindwarp.gp3.fixed-grammar.v1`

Whole-situation digests hash the exact canonical situation with its own
`situation_digest` field set to the empty string. The whole-grammar digest
hashes the exact canonical grammar with `grammar_digest` empty and the five
pinned situation digests present. They are not reused from GP2 or supplied by
a caller.

| Fixed record | Canonical digest |
|---|---|
| `gp3.s1.colony-conduit` | `f2f804581e02364ce7632ca8307be1340935840a705f66a12fd47e497e19cc86` |
| `gp3.s2.storm-nest` | `c56fdd98459d0500ca0d8ad3c752ff98512f9770d4b79bab50c603286dda17c5` |
| `gp3.s3.memory-gate` | `08ead7accbe9b188888fd7a465dbe7e761c487162f1c7c6f2936771772167151` |
| `gp3.s4.signal-anchor` | `c258b1b83e86cc52f30502c8e8d29d7bbda161ce7abc1031d5612a65c84d5328` |
| `gp3.s5.afterlight` | `8c80d9b3a70c7ce77b82f78fe77e532b63b2ed6357f5f375c615b33111099766` |
| whole grammar | `e8865e011d8b7ada0787303d49e4c769ff19164dc7a51f52d396e80b2c408b44` |

| Session | Canonical digest |
|---|---|
| `gp0.s1.colony-conduit` | `e7726be13efcf68e875e538103252aa46b3fd6c9e4ef86af95fc4622c160c274` |
| `gp0.s2.storm-nest` | `84c95d330549ba4d48ee2d47320558a29ec47dc5634cb450c383f9b5bd8bb0be` |
| `gp0.s3.memory-gate` | `3084d7d21fdb248cc3e83082a77164528d550ac2a15a6e51174cf4505e01dc68` |
| `gp0.s4.signal-anchor` | `b6428f7febdfab0560b975f02b47bb3dcbd7e940a4a88fd65028aa6c685a4033` |
| `gp0.s5.afterlight` | `a301a3d5cbfb73951cb4e6e0b453a1971ebc05fc351ab09563d7950d9820ab6b` |

## Exact evidence and risk references

Evidence class is `authored_gameplay_non_c3_b` for every row.

| ID | Kind | Canonical digest |
|---|---|---|
| `s1.flow-loss` | observation | `ede3df39e09b8ee3ff892db9a1269e844b00da30903a99a6da252dc27c1f76dc` |
| `s1.colony-distress` | inference | `7ad7843ee4f9d5d88e37960153671789729fe7356b026d75909cf1759965ca64` |
| `s2.exposure` | observation | `8eed424adeea133a75f47d3402ef377adf3d5a965b1d970ffc877d69edfecf7a` |
| `s2.crystal-hazard` | inference | `2b8db4f119ee94502950898da2c8646663a4be7060da2d59ac552588cf5fde7f` |
| `s3.ledger` | observation | `82f71d9a5325c12a314a264638d2cd3d300779f67b04e09f3fbca22c56d93145` |
| `s3.testimony` | inference | `57aa4036590f172dda0cc30d497d7bf84bbc6788edfb97743609731df692c430` |
| `s4.timing` | observation | `aff6b9b452e4e88990b2396d3d0c13dff2b26ce066aa813dc9ae90774d25b3a2` |
| `s4.wire-scavengers` | observation | `f5525b1330623eee7ea750d76830587e7a227009a13af486215e0302c5bf49db` |
| `s4.event` | inference | `6669f287271133e354e14bf5a1405e334ec5e5fbf6863a1d64eed6968b5b872f` |
| `s5.history` | observation | `2f809a32dbf1153c39339a2f4b077348b6d033cac84eb879f9694d92d56609b2` |
| `s5.relocation` | inference | `f6d21d44b631eba4975058e080faa8bcdc7fe030f813f616e3e8ef33f56ee99c` |

| Risk ID | Canonical digest |
|---|---|
| `conduit-failure` | `34f71479ff84038f058d28329c55e1963defc7c0de46b91932a7961b99e77ddc` |
| `storm-arrival` | `6635fd2eb31d6fb4f860e15f04de29a0a5f808154855c89a84f7f763cf6d0d1e` |
| `channel-harm` | `797f0dae90cb076cd58441c4463564c5b458c36fc81ba43cf1cd4fbd852926ac` |
| `anchor-collapse` | `a99aaaec8c2a979afdd86f7b69cb9369997d7215239eba5533fb23df3357157e` |
| `buffer-violation` | `2f60e3a09f1d4efe8ad69dd7b396131c73930e73f3d1cc4ee6d315b83f36f040` |

## Exact ordered domain facets

Each row is `situation | facet_id | kind | supporting evidence | exact proposition`.

| Situation | Facet ID | Kind | Evidence | Exact proposition |
|---|---|---|---|---|
| S1 | `s1.facet.conduit-pressure` | environment | `s1.flow-loss` | Flow loss is localized at the failing colony conduit. |
| S1 | `s1.facet.resident-colony` | creature | `s1.colony-distress` | Pump vibration threatens a resident colony without choosing the intervention. |
| S1 | `s1.facet.urgent-water` | society | `s1.flow-loss` | Clinic and fire crews require an explainable water decision. |
| S1 | `s1.facet.bypass-system` | construction | `s1.flow-loss`, `s1.colony-distress` | Full repair, a spare bypass, and timed rationing impose different material obligations. |
| S2 | `s2.facet.storm-ridge` | environment | `s2.exposure`, `s2.crystal-hazard` | The exposed route and authored storm window make delay consequential. |
| S2 | `s2.facet.brood-predator` | creature | `s2.exposure` | Brood exposure and predator approach are readable without making diversion terminal. |
| S2 | `s2.facet.caretaker-obligation` | society | `s2.exposure` | The caretaker relationship distinguishes relocation from extraction. |
| S2 | `s2.facet.nest-shelter` | construction | `s2.exposure`, `s2.crystal-hazard` | Shelter placement must answer exposure and the named ridge hazard. |
| S3 | `s3.facet.west-channel` | environment | `s3.ledger` | Full opening risks the west channel while essential passage remains necessary. |
| S3 | `s3.facet.contradictory-claims` | society | `s3.ledger`, `s3.testimony` | East and west claims remain separately legible and cannot be merged by assertion. |
| S3 | `s3.facet.memory-gate` | construction | `s3.ledger`, `s3.testimony` | The gate supports timed, forced-partial, alternate-route, or unchanged states. |
| S4 | `s4.facet.anchor-load` | environment | `s4.timing` | Anchor load and crossing conditions expose collapse risk. |
| S4 | `s4.facet.wire-scavengers` | creature | `s4.wire-scavengers` | Wire scavengers obstruct work but their diversion cannot repair or rescue. |
| S4 | `s4.facet.iven-caravan` | society | `s4.timing`, `s4.wire-scavengers` | Iven's rescue and caravan service create distinct obligations. |
| S4 | `s4.facet.signal-window` | anomaly | `s4.timing`, `s4.event` | The authored three-action signal window competes with four-action permanent repair. |
| S4 | `s4.facet.signal-anchor` | construction | `s4.timing`, `s4.event` | Temporary brace, permanent repair, and detour preserve different consequences. |
| S5 | `s5.facet.habitat-buffer` | environment | `s5.history`, `s5.relocation` | Passage pressure is bounded by the colony habitat buffer. |
| S5 | `s5.facet.colony-scavengers` | creature | `s5.relocation` | Colony state and food scavengers remain distinct living pressures. |
| S5 | `s5.facet.traveller-obligation` | society | `s5.history` | Travellers and Mara must remember cleanup and habitat obligations. |
| S5 | `s5.facet.nightway` | construction | `s5.history`, `s5.relocation` | The passage must be chartered with obligations or dismantled. |

Only S4 has an anomaly facet.

## Exact authored approaches

Every non-retreat row requires the listed evidence and prepared tool; every
retreat has no prepared tool. Step syntax is `kind(subjects): proposition`.

| Approach -> outcome | Tool | Ordered steps | Risk disposition | Exact explanation; exact limitation |
|---|---|---|---|---|
| `s1.approach.direct` -> `s1.direct` | `full-flow-kit` | repair(`conduit`,`pump`): Restore full pump flow; coordinate(`clinic-water`,`fire-water`): Route urgent service | resolved | Localized loss supports direct repair and full service while GP0 records displacement; It does not preserve the colony habitat. |
| `s1.approach.bypass` -> `s1.bypass` | `colony-safe-kit` | construct(`greenhouse-spare`,`conduit`): Install the spare bypass; care(`colony`): Hold vibration below the authored distress condition | mitigated | Flow and colony evidence support restricted stable bypass; It consumes the greenhouse spare and delays orchard recovery. |
| `s1.approach.ration` -> `s1.ration` | `timed-controller` | coordinate(`water`,`colony`): Synchronize delivery windows; repair(`conduit`): Contain rather than erase failure | mitigated | Timed windows preserve the colony while containing the failing conduit; Supply remains constrained and repair is unfinished. |
| `s1.approach.retreat` -> `s1.retreat` | none | withdraw(`player`,`conduit`): Leave emergency ration active | unchanged | Withdrawal preserves a stable stop through GP0 emergency rationing; The conduit remains unrepaired and orchard stress advances. |
| `s2.approach.relocate` -> `s2.relocate` | `sheltered-nest-kit` | care(`brood`,`nest`): Move the brood from exposure; construct(`nest`,`shelter`): Stabilize the sheltered nest | resolved | Exposure and ridge evidence support relocation without requiring predator diversion; The old nest is abandoned. |
| `s2.approach.harvest` -> `s2.harvest` | `insulated-specimen-kit` | extract(`crystal-specimen`,`ridge`): Take the named authored specimen; care(`brood`): Avoid direct harm while accepting displacement | transferred | Ridge evidence supports extraction with explicit brood displacement; Caretaker cooperation is withdrawn and no scientific authority is created. |
| `s2.approach.retreat` -> `s2.retreat` | none | coordinate(`nest-caretaker`,`brood`): Dispatch emergency stabilization; withdraw(`player`,`nest`): Leave before the storm | transferred | Exposure supports a caretaker dispatch before withdrawal; Direct player assistance is foregone. |
| `s3.approach.charter` -> `s3.charter` | `joint-ledger-kit` | negotiate(`east-keeper`,`west-keeper`): Preserve contradictory claims; coordinate(`gate`,`passage-charter`): Assign timed windows and joint monitoring | mitigated | Ledger and testimony support timed passage without false agreement; Unrestricted passage is foregone. |
| `s3.approach.force` -> `s3.force` | `urgent-crossing-kit` | coerce(`gate`,`essential-traveller`): Complete one urgent crossing; withdraw(`gate`,`player`): Reseal without ownership judgment | mitigated | Evidence permits only a forced partial crossing, not resolution; Ownership remains unresolved and cooperation is damaged. |
| `s3.approach.alternate` -> `s3.alternate` | `essential-path-kit` | traverse(`alternate-path`,`essential-travellers`): Mark essential-only travel; coordinate(`gate`,`gate-watch`): Keep the disputed gate sealed | resolved | Contradictory claims and channel risk support an alternate route; Nonessential travel remains deferred. |
| `s3.approach.retreat` -> `s3.retreat` | none | withdraw(`player`,`gate`): Leave both claims and gate unchanged | unchanged | Withdrawal avoids inventing agreement or opening the gate; Essential travel waits and channel inspection is delayed. |
| `s4.approach.temporary` -> `s4.temporary-rescue` | `temporary-brace-kit` | construct(`anchor`,`brace`): Fit a two-action temporary brace; traverse(`iven`,`crossing`): Return Iven and record the signal | accepted | Timing evidence supports rescue and signal capture before brace expiry; Permanent repair is not completed and the caravan is delayed. |
| `s4.approach.permanent` -> `s4.permanent` | `permanent-anchor-kit` | repair(`anchor`,`crossing`): Complete permanent repair; coordinate(`caravan`,`crossing`): Resume caravan service | resolved | Anchor evidence supports permanent repair as a distinct priority; The signal expires and its coordinate is missed. |
| `s4.approach.long` -> `s4.long-route` | `north-route-kit` | traverse(`north-detour`,`iven`): Rescue Iven by the named detour; coordinate(`caravan`,`north-detour`): Record the longer route | resolved | Evidence supports rescue without loading the failed anchor; The signal expires and caravan delay extends. |
| `s4.approach.retreat` -> `s4.retreat` | none | coordinate(`caravan-leader`,`rescue-watch`): Schedule a later watch; withdraw(`player`,`anchor`): Leave the failed anchor | unchanged | Withdrawal retains a visible signal expiry and later rescue decision; Iven's rescue and anchor repair are delayed. |
| `s5.approach.nightway` -> `s5.nightway` | `nightway-charter-kit` | construct(`nightway-boundary`,`passage`): Mark the registered route; coordinate(`travellers`,`cleanup`): Assign cleanup and habitat obligations | mitigated | Exact latest S1 history and relocation evidence support a bounded charter; Unbounded travel is foregone and obligations remain enforceable. |
| `s5.approach.dismantle` -> `s5.dismantle` | `passage-dismantling-kit` | dismantle(`passage`,`nightway-boundary`): Remove the passage; coordinate(`keeper-mara`,`travellers`): Record closure and alternate service | resolved | Exact latest S1 history supports dismantling when obligations fail; The traveller route closes. |
| `s5.approach.retreat` -> `s5.retreat` | none | withdraw(`player`,`passage`): Leave passage unresolved under temporary watch | unchanged | Exact latest S1 history permits a stable withdrawal without erasing Afterlight; Scavengers remain active and buffer protection is delayed. |

Step IDs are exactly `{approach_id}.step.1`, then `.step.2`, in the displayed
order. Every approach admits the session's complete evidence list in the order
shown above. Each observation becomes an `observed_fact` prerequisite and each
inference becomes an `available_inference` prerequisite with the exact fact
digest. Every non-retreat approach then requires its exact `prepared_tool`.
Retreat instead has the sole `authored_state` prerequisite
`stable-withdrawal-available`. S1 non-retreat rows additionally require
`urgent-water-demand`; S2 `storm-before-two-major-actions`; S3
`essential-passage-required`; S4 temporary rescue
`three-action-signal-window`, permanent `four-action-permanent-repair`, and
long route `north-detour-available`; S5 non-retreat rows
`passage-obligations-active`. These are exact `authored_state` reference IDs
with no expected digest because they are fixed GP3 authored state, not GP0
evidence. Every S5 row additionally requires `exact_predecessor`
with latest S1 outcome admitted exactly from `s1.direct`, `s1.bypass`, or
`s1.ration`; missing, stale, reordered, foreign, or `s1.retreat` is rejected.
Threat diversion is never a prerequisite.

## Whole threat and selected contribution references

| Situation | Threat digest | Selected element `threat_contribution:0` digest |
|---|---|---|
| S2 `predator` | `21f3264314ee7d5ee7577eaaef7001e852d59bd1faa9b860e9d9a8f6bdbd5a86` | `6b10ab449e22bf7d23c88f757c9eec0ae63f698e2a6bf7da099c6bb8cfedfb58` |
| S4 `wire-scavengers` | `9d9e3507f19953aef3c7a2013fac50c370d15e94013318c45d5b31fad33aa248` | `854ce18c056c1aa961b6db1ceaed4329d8f6abeb4ba1b5f90bd310890505386f` |
| S5 `food-scavengers` | `00a45293ef76e54e105f2c4472554f930053661c3a5e1976e063c38b73345451` | `8e9f5253e558b891e71e9d34b31a8d3896be397b831b1db017a92a480c0ac7b3` |

Selected threat elements are world-contribution-only, optional, nonterminal,
referenced exactly once when selected, and never outcome consequences.

## Complete GP0 outcome consequence references

Each compact entry is `kind ordinal digest`. Every row is the complete exact
set for that outcome; no approach may omit, duplicate, or add an entry.

| Outcome | Complete references |
|---|---|
| `s1.direct` | `mutation 0 83d5af42d7a5a27848e5b419d1b257701e8bf9da7363a0c96ac90dac7cb233d9`; `mutation 1 63301a7360f74bcce0c7a38c286d7fec2c915f4a281aa9a9b7d9731bd303f7ce`; `mutation 2 ea35e7e9d1e1dbbc246ba158ebae854640c8d780597ca2a5d1d3d7a081e6bd1e`; `mutation 3 a54d1f41ef392d556a0e5bc58a1360126a8c0164e9d6c455724ca45f1252e786`; `opportunity_cost 0 f63799865dc7afe14f76d6d3f4eee975e19b4466daf0a341481f71543bf3743b`; `memory 0 7d6f3a5e69467a2c91dc76d4671844fa0998adbe4cefd0d371c0cb299532f286`; `grant 0 543f3312b4762df78c21cb9e2cefde66cfa1d3b2210d2136100619756f5af6a5`; `named_decision 0 e095f8e56d98f20d9d88c1ab0391a852cbe67f13860a346ff3ecea8bedd20f17` |
| `s1.bypass` | `mutation 0 7cfb247d0eab742d48bbb55453a7556621482a092abe13ada354bd0bacc37b27`; `mutation 1 91af33bb09a880e190596e3299e87e45535346c6bf81e7052a6708d9362dd678`; `mutation 2 46075a67fbd77467c89f3c93c5404d31f874e484e7744e5e7837659d09b11ab0`; `mutation 3 708175417a308a1571c4c7696b4bef5563224125096e3be3cb76ebc71d64f303`; `opportunity_cost 0 6eec0b711049b02835a0e13edd2a2b5dd83d97fe109cc05e6c0b80ae8cfae9b2`; `memory 0 af5a896876279bd9bdadb6abe543bb6020a07cc4dcce359c34bf1207a58b5f5e`; `grant 0 b18cead2284758cedc46b29e13e88633612a8dc468d2b483824dbf2da1b2e150`; `named_decision 0 c3778e7a1ed7e1985781417f39597f8c3488ab4f30afd446c35ff6017098bc1f` |
| `s1.ration` | `mutation 0 72c3f0cb0a93e7d70ac57f8aa569246081b1c53dc9402bbb21a2ead3b2e28443`; `mutation 1 5b399f8aaa0f4842e33f5ad240af66f1feee48713ec22b1200654d612ee80b44`; `mutation 2 e4a0486d17807b88e706706e415e46c410f19da9da921dcbdaa8c91cbd5a2631`; `opportunity_cost 0 ba2ff612841159993583cb138924c144659dd0ab78667686c7922577deb45c85`; `memory 0 69c7a991cbb97fac439c572d0bc4daabfabd5ea5d275a5fdee027a8e8573f431`; `grant 0 01c8c91b6ed04f108940a491597034a7d4bb0de83293497334a3d3815f9afc14`; `named_decision 0 04bbd3185f6280755053794dad3b0c4bcb57383c459480d23ac0dea861897804` |
| `s1.retreat` | `mutation 0 e78bec9713d0b5f914f714ae66745b62e3d3446924350ad6287c008f279ec9e8`; `mutation 1 7a6cf24096b0596e7e5a1dfd58626d8bc183826ce41edb9f0bcb8ca4ccb6224c`; `opportunity_cost 0 a4fa4d68b17311bb3878033babd6911faf54cc1d205cbe0e8594d42592f13879`; `memory 0 2f9a5067990d8bc938c44b9a6fde4f2469ba157f87e8bd4e07fdb9f427dd0bfc`; `grant 0 a840dd362903b8d271c0d81fc0842dece420ea9d37a9ea3f83f7b07c09d1bffd`; `named_decision 0 cce6bda98c3ba32b03a598d27422f3f3a9beaf6093dc9045b2da66c6a81d1d23` |
| `s2.relocate` | `mutation 0 59624d7b4534a772442a02894a9c7e38952574921af8458e9e7cb56c32cb3458`; `mutation 1 3ba21d9b11062b387e9316cfc21527de11f2fe0f3e966d61def8f20c64c6ede7`; `mutation 2 a1d3581410403937f3190ec3b84919c59e2c1415bee08a0e9b068225f04abdfd`; `opportunity_cost 0 74f47a6ac14bb75497ade5e78a56b6276d4d4cd7ea4435878aac35ad0dfb39d8`; `memory 0 b6ac7cc49824f6cd62b382a707f4676f68777707c92073f261fd57bac017a9fb`; `grant 0 443335581ff3a5ac526a9e0b4ad9bdd3cd7bbb2dc5f50eac61aea0a108e73e60`; `named_decision 0 728938576072d91e463e99739769df920dd26425bf01f29e4d2790bcab55413a` |
| `s2.harvest` | `mutation 0 ceb68a8d64a20a9b5309508f29c9e9a2d4776b35faa76b2af90bd1cdd4ef20f1`; `mutation 1 e81cf8679a640c99a63aa5224f464d09cc00e7435e05be42b48686873fa341e8`; `opportunity_cost 0 8c8723b648212a69112e173c7dfeb822d78714e4bd6847d7e1792771c5121e70`; `memory 0 b6f7dd395f3fb09a48eeb9f32f3af336b19ea814aa15004c03db6a8db8c45b3f`; `grant 0 0865701d14e39a74702f99263d631019764e61c4aeeda1d40bf26dbb6c891821`; `named_decision 0 5afec38866c71a5e9084d3cd92d7968d1a45664cef5646f93777a9caaf7d7fdf` |
| `s2.retreat` | `mutation 0 712422f94cb4ab6e2ad71683637a3582f331ebfe76ad4985ec4041d71f992690`; `mutation 1 bc60bcee0f43e8e42e50e035ed1214a288104adccac431e1dca055055a1c8fca`; `opportunity_cost 0 9b91758d4a9085fd7314c53824e0d932b083b785f9c3bdfb52e88305ad0f0d24`; `memory 0 9a7d04dab0b304c67f717cc28025d76e563de53ba834b8c56dcc3a29aafd797e`; `grant 0 e575fb90a034c74bfdd022fbbf02f9074fade121f7c25da948bdcae5830078a2`; `named_decision 0 4820b4b0343f1ada2e47f62ba451f674567eaf1cf77cbf7f5d087116bee205aa` |
| `s3.charter` | `mutation 0 1bcf8ddb69bfae9798ec255803111a995b736ebd5cd2cbef5906bc7e4ac302f9`; `mutation 1 4d43ca7ad6d38b9609ebbf80ebe3ef2e5d6c8f8131ceb402e69bb7441eb9f967`; `mutation 2 1977cb33c91fcccbed2ae4009801758d04907478a499a1ac8564965d084491a0`; `opportunity_cost 0 9ba3abedc184e9d6657e5871f3a84d9dc97391e7acdae6472d0da37870626733`; `memory 0 ae561d7894f4a866a82f91d4607fe6607b835891aa5b0323cc2129c33d8447e9`; `memory 1 c4f327b522d8b078f2cf03c90c1fbd2fe2a1194012e00f8123ec3235bc882802`; `grant 0 4af5a74b22a89a0db5cc29bf61d748167c76365717ab301d40ee33b698f0e11d`; `grant 1 14a723cddde62326d4470413a87f9f649c4eca8aaf1c7e4cad03bb44709821ea`; `named_decision 0 b13cab732d46e31d1edb3b5a1593ccb9e6ee6c9fcb7de01e0a31340c5e57178f` |
| `s3.force` | `mutation 0 52a83c8a7bd6e9be0a4ae73b52e34e780feeddba49a12ba9dce883a7ebcf30e9`; `mutation 1 843ec0b1e0d67ebf138cea99c1b328215221e242a83bc3d8c1ef85ef44434d29`; `mutation 2 a4baffbdd70b2f6c05a9b162797229cc773615b4b69b2a217404ee30e002858e`; `opportunity_cost 0 31eb758a632aca096a6525468d323383f362fcc3a35d3f41b00bb018b00ecf87`; `memory 0 f6a467e99cdacb811ef439e90d78319072a1337e6df75161c9cfed35b2e9699c`; `memory 1 254040b204abf2f193bb38d2debc4d633f6652fa647a7bc1a49d3aa0704e7f9c`; `grant 0 4b4992d60342ecf4ef73a4d798433bfa5b7e5b32ee92da08adb3007aebf45c05`; `named_decision 0 dc37d55a41956318297791249c579a8dd48cc51feeeca52089a9e479c1e97d9e` |
| `s3.alternate` | `mutation 0 6a076abb7901d555c4d051ff49300ed86cfe42b4539c0e25eff1ab677eeb3f3b`; `mutation 1 b92185e7c5c97f098d0f052056e8ee3d5d154112a4e0393da6251ff052547c45`; `opportunity_cost 0 0b134d8a7ebba4d9111ecc5b6a3f7f025b5a05b77c3e644df5142f86e73987ea`; `memory 0 ce5ebd9c91e537e58619c3b4d061e180fc42fc53b7e806718e7c73f12a185de3`; `memory 1 8024a14761483aafc6efed4667371fcb8261b74e611c3a710e9f1ab5c54ff042`; `grant 0 b9a62aa3b6453f1f35e1f5ff79f1273edf521a16c90c56a7dff3a9c0b2ff4339`; `named_decision 0 d9afcb7d04bf8194864d5d56b56401148cc360e570a4fedb4ebb52e04b350ae0` |
| `s3.retreat` | `mutation 0 b92185e7c5c97f098d0f052056e8ee3d5d154112a4e0393da6251ff052547c45`; `mutation 1 f482576a486e9566311a696a98c687de55b0725905f6b211b2cf98022b3bbbbe`; `opportunity_cost 0 add4f64941c9634b64fd4bf341ebd8ef3c3565fffe69224de206435a47ac8781`; `memory 0 72ab2aca0a19f2310b7019339e5853b4998ed6241e605c1f7b734204da33e425`; `memory 1 0feb65442620e72ad4a68337df7a3bc58ed9a5b0b1bf7a5a63b6d5a5d6079661`; `grant 0 3dd8bdfc3734d60bbfd03203b803456007962aa9a4d6275a09841643350b3a1e`; `named_decision 0 a5b77ff34c03f02b1d4fc942cdf9bc5d0d83803f581029eca861bb5c5810923c` |
| `s4.temporary-rescue` | `mutation 0 7c13efa0258ffd1206d56d3ec4ae2cd7ee3dea3e144348163682b54bd66878d4`; `mutation 1 f399dc99d082ddb7ef789f1995f3432413bdff1a6536b5abd03feb0eca2e936e`; `mutation 2 349d0b9d6fca52646270631ba2465809da0b968dbc388a05f06f474edec630b0`; `mutation 3 6991c5e9b8d9e477ac6343b99fb4ea000bbae43a14140682c52e4a87aa1cc07e`; `mutation 4 82678bdbbb9476ec0637b2cf55fab24e97c819b1ea6549a98c8b421698036ea5`; `mutation 5 c899a4bf5a268ceea715cf996de481a59caf18aabf7fece1fe1ff0f8534eef7d`; `opportunity_cost 0 61b30d2161578b347c4f219d04abde909d1ba9e5a4187f430921e248a2a21623`; `memory 0 a6cdbe846ab81f331761e17d00e36ca9165bbe9f2823b51ea7b5d9d1055a72bb`; `grant 0 30136f42422cc110394ba8a279112f97053dcba4ea57ac3a297fb84e9340624b`; `named_decision 0 438d880b53e6fc13c6ab7ee01b58a94565ac114935dfae04e65fa9c0a6fd1942` |
| `s4.permanent` | `mutation 0 c26ededb19b501a9dfec887cd2a74e69b82369cb764424d247ad844dfde9c430`; `mutation 1 40159aac2f2a71bc54195dcc0d1854def980683fa959c53cf4846eed5226ad4b`; `mutation 2 e7b61a32f72030eb8241f720a2933f376bb3361ea11365aab415fa858af10351`; `opportunity_cost 0 02794a6c4220b6d442609fc24fdf1f1dfe65aa0d9b06699bacebf55c25c8d042`; `memory 0 e0c6240bd9c347121a35592414036a79f583acafe09b60505e888f2af17b534e`; `grant 0 dd1302773d1f1c5f1c5033fb749990529d1c93c0f715c8324a4f07e66a0b3627`; `named_decision 0 97d12948086570d6fa6ce430d0b0642a6d0be23e813ca0c9995fabdf98181641` |
| `s4.long-route` | `mutation 0 c4cf2292393ba9a6f81e454cf78be13398081196b6bd17f078b800369ca9f61f`; `mutation 1 7f9b5effc544565b16c897e15807034acfb7be71110fec8c9f191e29433c3269`; `mutation 2 e7b61a32f72030eb8241f720a2933f376bb3361ea11365aab415fa858af10351`; `opportunity_cost 0 e2322698c4f81022822ebba7ccf0ca1394bf806dfac989757a4d70f70aa9ad55`; `memory 0 dc62dfa6507671673cd66c51f901891d409a8496f53b0d63c7b8938bab470946`; `grant 0 883cb16914955bf9cfee84a0df5856f0552abe6f5e33c4e9606d88d2d803204f`; `named_decision 0 758555f8bb78dd186de08d35575be267c82e6d079ecd88d0476801a9f0f097ea` |
| `s4.retreat` | `mutation 0 32f30a8b809e71648fa84e984d194328017869822a61e465df9b49889e6706e9`; `mutation 1 cd6b51ef101cd358cad545ce2383b089b511d5d9c653bf134060f4dca4242b1a`; `opportunity_cost 0 dbfda5a821d4de1efcf65ce82ec1cca0699b3f5cef1c1cfe0f169d61d989515c`; `memory 0 f9334511ebcac2694f90212a06aba9428846730a6827cf2fda9976aabd1c812e`; `grant 0 45676092ea1e7b8fbe62d3e90410ed734675b1edc3b552b2bf3aae4eb82eaa74`; `named_decision 0 2b6b0d8736754a7e59ec05ae3030411a7fd6b7f215a390c53348a3409675a7a8` |
| `s5.nightway` | `mutation 0 74fbd61aca5155bb6b6fd8e7938b800bafb2876ee59cc1ef7d3d24076e7add10`; `mutation 1 591ce9a485b46d88978fef3eac6f8f9a9212881d182c73c58dcb954fbb14d9c3`; `mutation 2 ecda288b782399223a6c9268f7b24e37f9729cf217b4ddc90a57cf66d5a6b14b`; `mutation 3 21719d95fffc9bc9c59134b138b91ef8be520fabcbd6ac36eecd5cc0c725820f`; `opportunity_cost 0 ea4ac85c7a9a79ce3bd2e118e6e8bf2f8117bc59d84768cfa9a4c6b37f511a38`; `memory 0 46d14429cf245cc8e05feada637d5b6af518be0e84f26c12cf4ec92aad437bcb`; `grant 0 cc0cf1f9850b13dc5b77247b06622bfa1f6755442a485101cea27beb3fa78d40`; `grant 1 26426f7b3b533edce026e9f1ee69a033f217d10b1ae89292e2473fbc07b0b0a5`; `named_decision 0 5c125543f0e0542085e584b8bb3de2ab90b99c101d10a8055a3f694b4bc526b9` |
| `s5.dismantle` | `mutation 0 da074570ab16a7df5c6d367ad7dbab9f539acddf98016823a57b11e01502ce98`; `mutation 1 f6b0878bb60e278d50a6f594075bc852e8e387fac35d7b7f812e3e57719c1b25`; `opportunity_cost 0 169cde4b50d34fd42ad5f3c2aedbcf7d2599f91ee3c685d1e295eb3b265d1884`; `memory 0 c55b88b9d96a0d6e2d2f47d25990bff690e6e4896a245bdf063d3c9bdbc1437e`; `memory 1 8a12b50eecb9a9f73a69659fcd1028bdf5af50c292a5434eab01ae39639125c6`; `grant 0 8ef15d9e1b42b09a2486c46d2ad549bc032f72ff021a43f1def4f0de6f3b8244`; `named_decision 0 060e7178cad50c02c19c7fcde4e5ad39d3ef517aae23929497b6749d3ad081d5` |
| `s5.retreat` | `mutation 0 82f600a0bf2e0bce75a1a733a1f416deb9e5696de0aee0e00ab4cf70bc3a40c3`; `mutation 1 df6faa0bad1a2f30bb067322d2a8972711f28f90c39b0c7573b92d664cf824d0`; `opportunity_cost 0 b8cc8390538cc0a19b895980fc12d1fcc2e6188440a3784e0274fddb1f3a01c3`; `memory 0 165102a6c282682b6be219afcccbe54ba971c5e8cff18566acaacf824512d882`; `grant 0 0326a69d1167bc07412bd6a144ff06312b51c6a2cd9b2fdda5effff1d765640a`; `named_decision 0 989c852e8a755d65ec999095773d82eeb0a9865da826ac6803e9b2d9e3341832` |

The registry contains no anomaly facet outside S4, no combat approach, no
threat prerequisite, no generated candidate, and no GP2 mapping.
