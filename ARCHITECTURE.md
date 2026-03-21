# AuraOS Architecture — v8 (Final)

## The Core Insight
A database stores data. A filesystem stores data. An OS manages data. They're all doing the same thing. AuraOS unifies them.

ALL VyMatik concepts are NATIVE to the OS kernel. Your computer doesn't run VyMatik — it IS a VyMatik instance. When VyMatik the database product is installed, it only adds the server/API/query layer because everything else already exists in the OS.

## Three Products, One Foundation
- **AuraOS** = OS with VyMatik concepts built into the kernel
- **DEFS** = on-disk format that stores Particles natively
- **VyMatik Server** = network database server (thin on AuraOS, full on other OSes)

## Ring Architecture

### Ring 0 — `foundation/`
Boot, arch (x86_64, aarch64, riscv), HAL, process, IPC, power

### Ring 1 — `io/`
Drivers (block, net, gpu, input, audio, display, sensor, biometric, iot), media codecs

### Ring 2 — `vymatik/` (data layer — native to the OS)
All VyMatik concepts live here as OS primitives:
- `particle/` — atomic data unit (replaces inodes)
- `dimension/` — N-dimensional properties per particle
- `gravity/` — relationship bonds between particles
- `resonance/` — multi-dimensional encoding
- `density/` — access-pattern intelligent caching
- `entropy/` — data decay and deep storage
- `light_speed/` — access locality boundaries
- `dark_matter/` — hidden influence data
- `nullspace/` — unauthorized = nonexistent
- `quantum/` — observation = audit trail
- `membrane/` — sharing contracts
- `manifold/` — per-user data worlds
- `realm/` — emergent data domains
- `singularity/` — natural data clustering
- `defs/` — on-disk storage format
- `vfs/` — virtual filesystem (POSIX compat)
- `compat/` — ext4, NTFS, FAT read/write
- `net_data/` — network as data flow
- `index/` — semantic search, knowledge graph

When VyMatik Server installs on AuraOS: only adds query engine + network API.
When VyMatik Server installs on Linux: installs ALL of the above from scratch.

### Ring 3 — `trust/`
crypto/, auth/, policy/, sandbox/, particle_trust/, membrane_auth/, ai_defense/, dark_matter_mgr/

### Ring 4 — `cortex/`
engine/ (inference, models, accelerators), perception/ (STT, OCR, vision), cognition/ (NLP, intent, reasoning), memory/ (gravity-bonded), learning/ (density patterns), safety/

### Ring 5 — `agent/`
orchestrator/, skills/ (ASF, MCP, A2A), automation/, compose/, mesh/, marketplace/

### Ring 6 — `experience/`
desktop/, shell/, rendering/, voice_ui/, widgets/, notifications/, accessibility/, i18n/, apps/

### Ring 7 — `world/`
enterprise/, family/, home/, vehicle/, social/, commerce/

### `aether/` — Communication protocol
protocol/ (binary), transport/, bridge/

### `services/` — Cross-cutting
observe/, config/, sync/, runtime/, update/, migration/
