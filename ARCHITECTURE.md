# AuraOS — Architecture Document
## Two Operating Systems. One Vision.

**Created:** 2026-03-18
**Author:** Taylor Oclaw + Venkat Yarlagadda
**Status:** Architecture Phase

---

## The Vision

Two Rust-based operating systems built from the ground up:

1. **AuraOS** — For Humans. Beautiful, intuitive, AI-augmented.
2. **NexusOS** — For Agents. Minimal, efficient, purpose-built for autonomous AI.

Both share a common microkernel (`Aura Core`) but diverge in their userspace, interface, and philosophy.

---

## Part 1: AuraOS (Human OS)

### Design Philosophy: "The computer disappears"

Every major computing interface shift has removed a layer of friction:
- **CLI** (1970s) — Type commands. Expert-only.
- **GUI** (1984, Mac) — Point and click. Windows, icons, menus.
- **Touch** (2007, iPhone) — Direct manipulation. No mouse needed.
- **Voice** (2014, Alexa) — Speak naturally. No screen needed.
- **Spatial** (2024, Vision Pro) — The world IS the interface.
- **Ambient** (2026, AuraOS) — The OS understands you. No interaction needed unless you want it.

### AuraOS Interface: "Ambient Intelligence"

AuraOS doesn't have a traditional desktop. Instead:

#### The Surface
- A **fluid, living canvas** that adapts to context
- At a desk? It becomes a workspace with your active projects
- Cooking? It shows the recipe and timers
- Walking? It minimizes to voice + haptic
- No "apps" — just **intents and surfaces** that materialize when needed

#### Interaction Modes (all simultaneous)
1. **Gaze** — Look at something to focus it. Dwell to select.
2. **Voice** — Natural language, always listening (with consent)
3. **Gesture** — Hand/body tracking for spatial interaction
4. **Touch** — Traditional fallback, glass surfaces
5. **Neural** (future) — BCI integration ready
6. **Thought** — AI predicts what you need before you ask

#### The Companion
- Every AuraOS instance has an **AI Companion** — a persistent, personal intelligence
- It's not an assistant you invoke. It's always there, like a second brain.
- It learns your patterns, preferences, routines
- It can speak, show visuals, control your environment
- Think: Jarvis, but real, and it runs locally

### Technical Architecture

```
┌─────────────────────────────────────────────┐
│                 USERSPACE                     │
│                                              │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐    │
│  │ Surface  │ │ Companion│ │ Intent   │    │
│  │ Renderer │ │ Engine   │ │ Router   │    │
│  └────┬─────┘ └────┬─────┘ └────┬─────┘    │
│       │            │            │            │
│  ┌────┴────────────┴────────────┴─────┐     │
│  │         Aura Shell (Wayland+)      │     │
│  │    Compositing · Layout · Input    │     │
│  └────────────────┬───────────────────┘     │
│                   │                          │
│  ┌────────────────┴───────────────────┐     │
│  │         Service Layer              │     │
│  │  Storage · Network · Sensors ·     │     │
│  │  Crypto · IPC · AI Runtime         │     │
│  └────────────────┬───────────────────┘     │
├───────────────────┼─────────────────────────┤
│    KERNEL SPACE   │                          │
│  ┌────────────────┴───────────────────┐     │
│  │         Aura Core (Microkernel)    │     │
│  │  Memory · Scheduler · IPC ·        │     │
│  │  Capabilities · HAL                │     │
│  └────────────────────────────────────┘     │
└─────────────────────────────────────────────┘
```

### Key Components

#### 1. Aura Core (Microkernel) — Shared with NexusOS
- ~10,000 lines of Rust (tiny, auditable)
- Capability-based security (no root/sudo)
- Async-first IPC (message passing, not syscalls)
- Memory isolation per process
- Formal verification targets

#### 2. Aura Shell (Display Server + Compositor)
- Wayland-based but extended for ambient computing
- GPU-accelerated compositing (Vulkan/wgpu)
- Multi-modal input fusion (gaze + voice + gesture + touch)
- Adaptive layout engine (context-aware surfaces)
- Accessibility-first (every interaction mode is a first-class citizen)

#### 3. Surface Renderer
- No "windows" — **Surfaces** that flow and adapt
- Built on wgpu (WebGPU) for cross-platform GPU rendering
- Declarative UI framework (like SwiftUI but for OS-level)
- Theme engine: glassmorphism, depth, motion, organic animations
- Dark/light adapt to ambient light

#### 4. Companion Engine (Local AI)
- Runs locally on-device (privacy-first)
- GGUF model format support (llama.cpp compatible)
- Whisper for speech-to-text
- TTS for voice output
- Vision model for understanding what's on screen
- Memory system (persistent context across sessions)
- Tool use (can control the OS, open things, manage files)

#### 5. Intent Router
- Replaces the traditional "app launcher"
- User expresses intent ("I need to write a letter to Mom")
- Router finds the best surface/tool combination
- Composes a workflow, not just launches an app
- Learns from usage patterns

### File System: AuraFS
- Copy-on-write, ZFS-inspired (like Redox)
- Content-addressable storage
- Automatic versioning (every file has history)
- Semantic tagging (AI auto-tags files by content)
- Encryption at rest by default
- No traditional folder hierarchy — semantic search + collections

### Networking
- Mesh-first (peer-to-peer between AuraOS devices)
- End-to-end encryption for all communications
- Built-in VPN/overlay network
- Agent-to-agent protocol (compatible with NexusOS)

---

## Part 2: NexusOS (Agent OS) — Built Second

### Design Philosophy: "Maximum agency, minimum overhead"

- No display server. No GUI. No compositor.
- Pure headless runtime for AI agents
- Boot to agent in <1 second
- ~2MB kernel + 50MB userspace
- Can run thousands of instances on a single server

### NexusOS Components (brief)
- Aura Core microkernel (shared)
- Agent Runtime (sandboxed execution)
- Tool Registry (file, network, code execution)
- Message Bus (agent-to-agent communication)
- Capability system (agents can only do what they're permitted)

---

## Build Plan (AuraOS First)

### Phase 1: Bootable Kernel (Week 1-2)
- Freestanding Rust binary
- Boot via UEFI
- Basic memory management
- Serial console output
- Run in QEMU

### Phase 2: Microkernel (Week 3-4)
- Process management
- IPC (message passing)
- Capability system
- Basic scheduler
- Memory protection

### Phase 3: Userspace Foundation (Week 5-6)
- Init system
- VFS (Virtual File System)
- Device drivers (keyboard, mouse, display, storage)
- AuraFS (basic version)
- Network stack (TCP/IP)

### Phase 4: Display & Shell (Week 7-8)
- Framebuffer driver
- wgpu initialization
- Basic compositor
- Surface rendering
- Input handling (keyboard + mouse)

### Phase 5: Aura Shell (Week 9-10)
- Declarative UI framework
- Theme engine
- Layout engine
- Application framework
- Settings surfaces

### Phase 6: Companion Engine (Week 11-12)
- Local LLM runtime
- Whisper integration
- Intent router
- Memory/context system
- Tool use (file operations, app control)

### Phase 7: Polish & Demo (Week 13-14)
- Animations and transitions
- Sound system
- Multi-display support
- Demo applications
- ISO image generation

---

## Technology Stack

| Component | Technology |
|-----------|-----------|
| Language | Rust (100%, no C) |
| Kernel | Custom microkernel |
| Boot | UEFI + Limine bootloader |
| Display | wgpu (Vulkan/Metal/DX12) |
| GUI Framework | Custom (inspired by Iced/egui + SwiftUI) |
| File System | AuraFS (ZFS-inspired, CoW) |
| AI Runtime | llama.cpp (via Rust bindings) / candle |
| TTS/STT | Whisper.cpp + local TTS |
| IPC | Async message passing (tokio-inspired) |
| Testing | QEMU + real hardware |

---

## Name Candidates

| Human OS | Agent OS |
|----------|---------|
| **AuraOS** | **NexusOS** |
| LuminaOS | SentinelOS |
| NovaOS | CoreOS |
| EchoOS | PulseOS |
| VeilOS | MatrixOS |

**Current pick: AuraOS + NexusOS** (Aura = energy field/presence, Nexus = connection point)

---

## Repository Structure

```
AuraOS/
├── kernel/           # Aura Core microkernel
│   ├── src/
│   │   ├── arch/     # x86_64, aarch64, riscv64
│   │   ├── mm/       # Memory management
│   │   ├── sched/    # Scheduler
│   │   ├── ipc/      # Inter-process communication
│   │   ├── cap/      # Capability system
│   │   └── main.rs   # Kernel entry
│   └── Cargo.toml
├── boot/             # UEFI bootloader
├── drivers/          # Userspace drivers
│   ├── display/
│   ├── input/
│   ├── storage/
│   └── network/
├── services/         # System services
│   ├── vfs/          # Virtual file system
│   ├── aurafs/       # AuraFS implementation
│   ├── network/      # Network stack
│   └── init/         # Init system
├── shell/            # Aura Shell (compositor + UI)
│   ├── compositor/
│   ├── surfaces/
│   ├── themes/
│   └── input/
├── companion/        # AI Companion Engine
│   ├── llm/
│   ├── whisper/
│   ├── tts/
│   ├── intents/
│   └── memory/
├── apps/             # Built-in surfaces
│   ├── terminal/
│   ├── files/
│   ├── settings/
│   └── browser/
├── tools/            # Build tools
│   ├── mkiso/
│   └── qemu-run/
└── docs/
    ├── ARCHITECTURE.md
    ├── DESIGN.md
    └── CONTRIBUTING.md
```

---

## Patent Opportunities (for Suvayar)
1. **Intent-based OS navigation** — replacing app launchers with intent routing
2. **Ambient surface rendering** — context-aware UI that adapts to environment
3. **AI Companion OS integration** — local AI deeply integrated at OS level
4. **Capability-based agent sandboxing** — fine-grained permissions for autonomous agents
5. **Semantic file system** — AI-powered file organization without folders

---

## Inspiration Sources
- **Redox OS** — Rust microkernel, everything in userspace
- **Plan 9** — Everything is a file, network transparency
- **seL4** — Formally verified microkernel
- **Apple Vision Pro** — Spatial computing interface
- **Humane AI Pin** — Ambient computing, no screen
- **Rabbit R1** — Intent-based interaction
- **OpenClaw** — Agent orchestration patterns
- **BeOS/Haiku** — Beautiful, responsive, media-centric OS
- **NeXTSTEP** — Object-oriented OS design
