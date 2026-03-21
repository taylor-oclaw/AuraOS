# AuraOS Architecture — v9 (Final)

## Three Products, One Language
- **AuraOS** = the operating system. Manages hardware, runs apps, AI-native experience.
- **DEFS** = the file format/filesystem. Stores data as Particles with Dimensions in Manifolds. Used by AuraOS natively.
- **VyMatik** = the database server. Separate product. Thin install on AuraOS+DEFS, full install on other OSes.

## Ring Architecture

### Ring 0 — `foundation/`
boot/, arch/ (x86_64, aarch64, riscv), hal/, process/, ipc/, power/

### Ring 1 — `io/`
drivers/ (block, net, gpu, input, audio, display, sensor, biometric, iot), media/

### Ring 2 — `defs/`
The OS filesystem layer. Reads/writes the DEFS format on disk.
- particle_io/, dimension_io/, gravity_io/, manifold_io/
- entropy_io/, density_io/, quantum_io/, membrane_io/
- journal/, snapshot/, dedup/, encrypt/
- vfs/ (POSIX compat), compat/ (ext4, NTFS, FAT, APFS read)
- backup/ (export, replicate, archive)

### Ring 3 — `trust/`
crypto/, auth/, policy/, ai_defense/

### Ring 4 — `cortex/`
engine/, perception/, cognition/, memory/, learning/, safety/

### Ring 5 — `agent/`
orchestrator/, skills/ (ASF, MCP, A2A), automation/, compose/, mesh/

### Ring 6 — `experience/`
desktop/, shell/, rendering/, voice_ui/, widgets/, notifications/, accessibility/, i18n/, apps/, marketplace/

### Ring 7 — `world/`
enterprise/, family/, home/, vehicle/, social/, commerce/

### `aether/` — Communication protocol
protocol/, transport/, bridge/

### `services/` — Cross-cutting
observe/, config/, sync/, runtime/, update/, migration/

## VyMatik Installation Modes

### On AuraOS + DEFS (native)
OS already reads/writes DEFS particles. VyMatik adds: server API, query/intent engine, multi-tenant, replication.

### On Linux/Windows + ext4/NTFS (standalone)
VyMatik installs everything: its own particle storage, gravity engine, density cache, quantum logging — stored in files on the host filesystem. Like SQL Server .mdf files.

## Design Principles
- DEFS is a filesystem (like ext4/NTFS), not a database
- AuraOS reads DEFS natively
- VyMatik reads DEFS natively when available
- No queries — intent-based data access via cortex
- No folders — Realms emerge from data relationships
- No traditional backups — quantum log + entropy layers + continuous replication
- Each ring depends only on rings below it
