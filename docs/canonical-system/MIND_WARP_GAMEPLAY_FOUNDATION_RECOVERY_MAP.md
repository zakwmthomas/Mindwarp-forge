# Mind Warp Gameplay Foundation Recovery Map

**Status:** owner-authorized gameplay-design recovery record; proposal, not
promotion or implementation authority  
**Products in scope:** Main Mind Warp and the relationship between its gameplay
and Forge  
**Products kept separate:** Quantum Tunnel, Forge itself, and the historical
AI/control-room tooling  
**Implementation rule:** clean-room gameplay design only. No legacy source code,
schema, architecture, numeric balance, runtime claim, or completion claim may be
reused. Historical material is evidence of creative intent and past
experimentation only.

## Why this record exists

Forge now has a detailed causal-universe and production-system route, but Main
Mind Warp does not yet have an equivalent player-facing foundation. Historical
files contain many potentially valuable mechanics, yet they mix the large Main
Mind Warp vision with the deliberately smaller Quantum Tunnel product and with
unverified implementation claims. This record preserves the concepts while
preventing that confusion.

The immediate design problem is not “which old feature should be restored?” It
is:

> What does a player repeatedly do in Mind Warp, why does it matter, how does it
> change the player and universe, and which parts truly require Forge?

## Source and authority boundary

| Source | Permitted use | Forbidden inference |
|---|---|---|
| Current Forge canon | Causal-world, identity, history, significance, construction and representation constraints | That an engine system is already a game mechanic |
| Recovered survival pack | Architecture and prototype evidence | Production readiness or player value |
| Historical Quantum Tunnel Git commit | Names, intended behaviours, system inventory and failed experiment evidence | Safe or reusable code, sound architecture, working implementation or Main Mind Warp canon |
| Project-scoped Cursor transcripts | Direct owner intent, corrections, observations and unresolved ideas | Blanket transcript authority, automatic ingestion or proof that an assistant completed a request |
| Devin logs and resulting files | Evidence that work was attempted and where artifacts were placed | Recovered Devin reasoning or verified implementation; the local session database is empty |

## Recovery evidence census

This is the read-only 2026-07-18 audit snapshot that motivated the map. Counts
describe evidence volume, not quality or completion.

- The historical Quantum Tunnel Git `HEAD` contains 6,021 tracked files,
  including 1,820 C# files and 365 Markdown files.
- Its inspected working copy contained 2,195 tracked deletions, 66 modified
  paths and 97 untracked paths. Deleted working-copy files remain recoverable
  from the commit, but neither state is a trustworthy runtime baseline.
- Historical code references are broad: terms such as wormhole, breeding,
  housing, trade, civilization, multiplayer, achievements, photo mode,
  accessibility and onboarding occur across multiple source and design files.
  A reference count never proves a coherent or working system.
- The two project-scoped Cursor stores contain 812 JSONL transcript files,
  approximately 46 MB. Separating parent conversations from delegated
  subagents yielded 27 parent sessions and 419 user records.
- Direct owner records include the physical tube split, progress-scaled
  portals, reactive tunnel illumination, varied meteor intent, shuffled boss
  rewards, continuous-form modular ships, EchoCraft companions, starfield
  entry presentation and the request to audit achievements/cards/mechanics for
  synergy.
- The inspected Devin session database contained no retained sessions. Devin
  recovery is therefore artifact- and log-based only.
- Current Forge knowledge queries returned no useful routed records for
  wormholes, creature breeding, mystical magic, player housing or the gameplay
  loop. Civilization produced detected records, but the inspected examples
  lacked usable project/workstream/entity routing.
- At audit time the active Forge working tree already contained 60 tracked
  change lines and 368 untracked paths. This gameplay record must preserve that
  unrelated work rather than normalize or clean it.

## Structural discrepancies retained from the audit

1. The earlier recovered-source audit described survival-pack families but did
   not name the historical Unity/Git repository or its Cursor corpus.
2. The current master route reaches G1 closeout and a gated R1 runtime-adapter
   decision without an intervening player-experience or gameplay-canon
   milestone.
3. The unresolved-gap register described causal production and runtime gaps but
   not missing player verbs, loops, progression, economy, narrative, social or
   accessibility ownership.
4. Abstract Forge capability can resemble gameplay coverage while still
   supplying no player decision, feedback, reward, failure or emotional arc.
5. Historical design documents and transcripts mix direct owner intent,
   assistant proposals, queued worker instructions, implementation claims and
   playtest contradictions. Each recovered concept needs provenance and
   adjudication rather than bulk promotion.
6. Main Mind Warp and Quantum Tunnel were allowed to share language and
   experiments without a durable product boundary, which makes currencies,
   progression, scope and platform assumptions easy to misapply.

## Product separation

### Forge

Forge is the engine-neutral causal production and continuity substrate. It is
not the player experience.

### Main Mind Warp

Main Mind Warp is the large game destination: a persistent, explorable universe
where physical conditions, living systems, cultures, construction, history and
player choices can combine into experiences that were not individually
authored.

### Quantum Tunnel

Quantum Tunnel was described by the owner as a smaller, faster game and proof
vehicle intended partly to generate income for the larger project. Its tunnel
run, cards, bosses and currencies are useful experiments. They are not the
default shape, economy or scope of Main Mind Warp.

## Candidate player promise

> Enter a causally coherent but surprising universe; travel into the unknown;
> understand, survive, befriend, alter and build within it; return with new
> knowledge and capability; and find that places, beings and societies remember
> what happened.

This is a candidate synthesis, not approved marketing language. It emphasizes
the distinctive value Forge could provide: discoveries and consequences are
grounded in one persistent world state rather than generated as disconnected
content.

## Candidate base-game loop

```text
Orient at a personal anchor or living hub
    -> choose a destination, question, need or opportunity
    -> prepare a body, vessel, companions, knowledge and tools
    -> travel through ordinary or strange space
    -> observe, navigate, communicate, survive, fight, harvest or help
    -> make a consequential choice or discovery
    -> continue deeper, establish a foothold, or return
    -> convert outcomes into knowledge, relationships, capability and construction
    -> encounter the world's remembered response
```

The loop deliberately supports more than combat. A viable base game must decide
which verbs are always available, which are optional play styles, and how
nonviolent discovery, social play, construction and creature relationships can
remain mechanically complete rather than decorative.

## Time-scale map

| Scale | Candidate experience | Required design proof |
|---|---|---|
| Seconds | Move, look, sense, evade, target, interact and receive legible feedback | A small satisfying verb set without procedural novelty |
| Minutes | Navigate a local hazard, encounter, conversation, creature, ruin or resource opportunity | Multiple meaningful approaches and readable consequences |
| Session | Prepare, depart, discover, decide, gain or lose something, and reach a stable stopping point | A complete loop with tension, recovery and no compulsory grind |
| Multi-session | Develop knowledge, skills, equipment, creatures, home, reputation and routes | Progress creates new decisions rather than only larger numbers |
| Long arc | Change worlds, relationships, cultures and access to the universe | History remains understandable and personally attributable |

## Gameplay-domain map

| Domain | Recovered concept candidates | Forge bridge | Missing design owner or decision |
|---|---|---|---|
| Player identity | Personal anchor, homeworld sanctuary, bodies/looks, equipment and vessels | Universe identity and history | Who the player is, what persists through death or embodiment changes, and what can be lost |
| Hub and home | Living hub, Master Room/AB Room, housing, local services and visible activity | Hierarchy, construction, history and representation | One hub, many hubs, personal base, mobile vessel, or a layered combination |
| Exploration | Galactic map, system space, surfaces, ruins, rare conditions and tourism | Causal worlds, physical paths and significance | How unknowns are revealed and how discovery stays meaningful |
| Travel | Open cruise, wormhole lanes, portals, route mastery and sanctuary return | Identity-preserving paths and replay | Travel time, danger, preparation, interception, fast-travel limits and route ownership |
| Encounters | Hazards, creatures, people, signals, anomalies, enemies and bosses | Significance, scheduling, organism ecology and history | Encounter grammar, escalation, retreat and noncombat resolutions |
| Combat | Weapons, shields, damage, repair, creature battles and environmental danger | Materials, construction, organism state and animation | Combat fantasy, lethality, targeting, tactical depth and its importance relative to other play |
| Knowledge | Data, scans, lore, maps, research and understanding strange phenomena | Evidence, semantics, causal state and history | Whether knowledge is currency, capability, authored information, player inference, or several typed resources |
| Progression | Skills, cards, affixes, achievements, creature bonds and access standing | Significance and typed capabilities | Horizontal versus vertical growth, respec, caps, catch-up and anti-grind rules |
| Construction | Crafting, salvage, modular ships, settlements, recipes and aesthetic expression | Semantic construction and asset representation | Player grammar, validation, resource costs, repair and meaningful failure |
| Creatures | Discovery, bonding, breeding, growth, families, riding and aerial mounts | Organism ecology, inheritance and history | Consent/taming model, care loop, death, reproduction, utility and personhood boundary |
| Magic/strangeness | Applied field science, schools, spell fusion, innate creature affinities and lore alignment | Fields, semantics, organisms and representation | What makes magic distinct from technology, how it is learned, its costs and why cultures interpret it differently |
| Society | Civilizations, factions, wealth districts, immigration, tension, religion, trade and technology | History, significance, environment and semantics | Social agents, cultural change, diplomacy, law, conflict and narrative legibility |
| Economy | Harvesting, refining, crafting, trade hubs, regional prices, property and services | Material flows, history and scheduler | Sources/sinks, scarcity, exploitation limits, ownership and single-player versus shared-market assumptions |
| Consequence | World evolution, settlements, scars, seasons, festivals, eclipses and remembered choices | World history and representation | What may change, how quickly, what is reversible and how cause is communicated to the player |
| Social play | Co-op activities, PvP normalization, sharing discoveries, portable state and UGC | Deterministic identity and replay | Whether multiplayer is essential, optional or later; authority, moderation, trust and synchronization |
| Presentation | Reactive environments, procedural visual/audio motifs, camera, maps and photo mode | Representation and animation | Readability, sensory comfort, art direction, accessibility and platform-scaled presentation |

## Quantum Tunnel concepts worth retaining as candidates

These are retained because they express useful gameplay principles, not because
their old implementation should survive.

| Concept | Transferable principle | Main Mind Warp disposition |
|---|---|---|
| A tube physically branches into visible routes | A choice should exist in the world, not only in UI | Retain as a travel/choice presentation principle; do not require literal tubes |
| Progress-scaled portals skip mastered early distance | Respect mastery without granting free rewards | Retain as a route-mastery question |
| Tunnel surface illuminates near the player | Environment should react legibly without obscuring vision | Retain as a general responsive-environment principle |
| Mixed meteor pursuit behaviours | Concealed intent and motion diversity can create readable uncertainty | Retain for encounter grammar; discard old percentages |
| Boss reward card choices shuffle | Repeated rewards need genuine choice and variation | Retain as a reward-design principle, not proof that cards are the final progression form |
| Block-built ships merge into angular or organic continuous forms | Simple construction inputs can yield expressive coherent outputs | Retain as a high-value construction/representation candidate |
| EchoCraft companions have bounded independent development | Companion systems can create tactical and emotional investment | Retain as a companion/vessel question; name and economy remain product-specific |
| Starfield-to-wormhole entrance | Travel transitions should sell scale and origin | Retain as presentation intent only |
| Achievements, cards and mechanics are audited for synergy | Systems should reinforce one another instead of accumulating independently | Adopt as a gameplay-design evaluation rule |

## Concepts retained from the broader historical game vision

- a live hub that is part of the world rather than a detached menu;
- wormhole and open-space travel with distinct costs and meanings;
- hierarchical maps from local place to planetary, system and galactic scales;
- player housing, personal space and a sanctuary or anchor;
- knowledge/data, skills, creature bonds and access standing as different forms
  of progress;
- salvage, crafting, trade and regionally responsive economies;
- genetically grounded creatures, family lines, mounts and coevolution;
- civilizations, technology, culture, belief, migration and environmental
  adaptation;
- magic as understandable strangeness grounded in world conditions, not an
  arbitrary particle-effect list;
- seasons, eclipses, festivals, settlement scars and other persistent visible
  history;
- cooperative activities and portable/shareable discoveries as later
  candidates;
- accessibility, onboarding, input remapping, subtitles, colour-safe
  presentation and photo mode as product requirements rather than polish.

## Principal unmade gameplay decisions

1. **Primary fantasy:** explorer, survivor, scientist, builder, trader,
   companion-keeper, protector, conqueror, citizen, or a supported mixture.
2. **Default verbs:** the smallest moment-to-moment action set that remains fun
   before procedural variety is added.
3. **Home structure:** fixed homeworld, social hub, personal vessel, constructible
   settlement, or layered progression through several.
4. **Failure:** death, injury, rescue, lost cargo, damaged relationships,
   historical consequences, rollback limits and recovery.
5. **Combat weight:** universal requirement, one supported path, or an avoidable
   activity with equally deep alternatives.
6. **Progression shape:** knowledge and access, capability composition, numerical
   power, authored milestones, relationships, or a controlled combination.
7. **Content legibility:** how players understand why a world, organism, culture
   or event exists without exposing Forge internals.
8. **Authored versus emergent narrative:** which emotional arcs require authored
   framing and which can arise from persistent state.
9. **Economy scope:** private simulation, asynchronous sharing, or a common
   market, each of which implies radically different abuse and balance problems.
10. **Multiplayer necessity:** whether the base game must be complete alone
    before any networked feature is considered.

## Clean-room gameplay development route

### GP0 — Product and player-fantasy boundary

- separate Main Mind Warp from Quantum Tunnel in every concept record;
- select the primary fantasy and supporting fantasies;
- define target session length, player count assumptions and emotional tone;
- define the non-negotiable reason Forge is necessary for the game.

**Exit evidence:** one-page player promise, explicit non-goals and five example
session stories that all express the same game.

### GP1 — Base-loop paper model

- define the smallest verb set;
- define preparation, departure, encounter, consequence and return;
- model failure, recovery and stable stopping points;
- test the loop with fixed authored examples before adding procedural breadth.

**Exit evidence:** the loop produces meaningful choices without relying on
graphics, vast content, grinding or multiplayer.

### GP2 — Progression, knowledge and economy model

- separate knowledge, access, relationship, construction and power growth;
- identify every source, sink, exploit and reset boundary;
- prove that progress unlocks new decisions rather than only accelerating old
  ones.

**Exit evidence:** several simulated player paths remain viable and no single
resource collapses every form of progression.

### GP3 — Encounter and content grammar

- define how environments, creatures, societies, anomalies and constructed
  systems become gameplay situations;
- bind each candidate situation to causal Forge evidence;
- preserve authored pacing and readable intent.

**Exit evidence:** the same grammar yields materially different but explainable
encounters, including noncombat solutions.

### GP4 — One vertical experience contract

Candidate only:

> living hub -> prepare -> unusual route -> local discovery or conflict ->
> consequential choice -> return -> visible remembered response

This is one consumer first. It does not authorize the entire gameplay map.

**Exit evidence:** a runtime-independent interaction/state specification,
adversarial examples and a reasoned runtime-adapter requirement list.

### GP5 — Runtime trial

Only after the normal owner gate and R1 decision: implement the minimum vertical
experience in a selected runtime without importing historical code.

## Evaluation rules for every gameplay concept

Every proposed mechanic must answer:

1. What player fantasy and decision does it serve?
2. Is it enjoyable in a fixed small example before procedural generation?
3. What other systems does it reinforce, duplicate or undermine?
4. What causal Forge information is necessary, and what remains ordinary game
   design?
5. How does the player read cause, risk, state and consequence?
6. What is the cheapest falsifier?
7. What is the accessibility and sensory-comfort impact?
8. What exploit, grind, dominant strategy or content treadmill might it create?
9. What does it cost to author, simulate, present and test?
10. What evidence would cause it to be rejected rather than expanded?

## Current disposition

- Preserve all concepts in this map as **candidates** until adjudicated.
- Reuse **no historical code**.
- Do not treat the historical presence of C# files or design documents as
  implementation evidence.
- Do not select currencies, balance numbers, game engine, networking model or
  monetization from historical materials.
- Do not interrupt, broaden or promote the active C3 package.
- Begin further gameplay development at GP0, then GP1. Nothing broader is
  locked in.
