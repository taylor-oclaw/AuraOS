# AuraOS Architecture — v7 (VyMatik-Native)

## Philosophy
AuraOS doesn't have a filesystem. It has a **substrate** — a VyMatik data layer where every piece of data is a Particle with N dimensions, gravitational bonds, entropy decay, and membrane-based sharing. The OS IS VyMatik's native platform.

## Ring Architecture

### Ring 0 — `foundation/` (boots, trusts nothing)
boot/, arch/, hal/, process/, ipc/, power/

### Ring 1 — `io/` (raw hardware)
drivers/ (block, net, gpu, input, audio, display, sensor, biometric, iot), media/ (codecs)

### Ring 2 — `substrate/` (VyMatik data layer)
The heart of AuraOS. Replaces traditional filesystem.
- void/ — instance management
- manifold/ — per-user data worlds
- realm/ — emergent data domains (not folders)
- singularity/ — data clustering engine
- particle/ — atomic data units with N dimensions
- gravity/ — relationship bonds between particles
- entropy/ — data decay and deep storage
- density/ — access-pattern intelligent caching
- resonance/ — multi-dimensional encoding
- light_speed/ — access boundaries and locality
- dark_matter/ — hidden influence data
- nullspace/ — unauthorized = data doesn't exist
- quantum/ — observation creates audit trail
- membrane/ — data sharing contracts
- defs/ — on-disk storage format
- rift/ — legacy compatibility (ext4, NTFS, FAT)
- net/ — network as data flow
- index/ — semantic search and knowledge graph

### Ring 3 — `trust/` (security as physics)
crypto/, auth/, policy/, sandbox/, particle_trust/, membrane_auth/, ai_defense/, dark_matter_mgr/

### Ring 4 — `cortex/` (intelligence)
engine/, perception/, cognition/, memory/ (gravity-bonded), learning/ (density patterns), safety/

### Ring 5 — `agent/` (action)
orchestrator/, skills/ (ASF, MCP, A2A), automation/, compose/, mesh/, marketplace/

### Ring 6 — `experience/` (presentation)
desktop/, shell/, rendering/, voice_ui/, widgets/, notifications/, accessibility/, i18n/, apps/

### Ring 7 — `world/` (external context)
enterprise/, family/, home/, vehicle/, social/, commerce/

### `aether/` — Communication protocol
protocol/ (binary), transport/ (stdio, HTTP, WS), bridge/ (traditional API compat)

### `services/` — Cross-cutting
observe/, config/, sync/, runtime/, update/, migration/

## VyMatik Concepts in AuraOS
| VyMatik | AuraOS |
|---------|--------|
| Void | OS instance (one per device) |
| Manifold | User space |
| Realm | Emergent data domain (replaces folders) |
| Singularity | Natural data clusters |
| Particle | Every piece of data |
| Dimensions | Unlimited metadata per particle |
| Gravity | Data relationships |
| Entropy | Data decay over time |
| Density | Intelligent caching |
| Light Speed | Access locality |
| Dark Matter | Hidden policy data |
| NullSpace | Unauthorized = nonexistent |
| Quantum | Observation = audit trail |
| Membrane | Sharing contracts |
| Aether | Universal protocol |
| Rift | Legacy compatibility |
