# AuraOS Directory Reorganization Plan

## Current: All files flat in kernel/src/ (1,189 .rs files)
## Target: Organized directory structure

```
kernel/src/
├── main.rs, os_init.rs (root - boot entry)
├── boot/        (6) gdt, interrupts, memory, allocator, boot_sequence, panic_handler
├── core/        (100+) scheduler, process, syscall, ipc, signals, memory management
├── drivers/     (89) hardware drivers (gpu, usb, storage, network, audio, input, sensors)
├── fs/          (47) DEFS filesystem, compat layers, mount, archive
├── net/         (48) TCP/IP, DNS, HTTP, WebSocket, VPN, firewall
├── security/    (54) crypto, trust system, auth, certificates, AI security
├── ai/          (290) agent system, inference, providers, safety, memory
├── ui/          (76) compositor, window manager, widgets, rendering, accessibility
├── apps/        (91) built-in apps (shell, browser, editor, settings, etc.)
├── mesh/        (115) compute mesh, enterprise, personal, compliance
├── skills/      (97) ASF, MCP, A2A, skill runtime
├── enterprise/  (78) MDM, MAM, SSO, compliance, company management
├── family/      (41) family hub, parental controls, shared storage
├── marketplace/ (38) app store client, payments, ratings
├── i18n/        (3+) language, speech, voice (more queued)
├── compat/      (9) Linux ABI, Android, Wine compatibility
└── notifications/ (2+) notification system, mini apps (more queued)
```

## Blocked By:
- main.rs directly references modules like `desktop::init()`, `gdt::init()`
- Need to update all crate:: references to use new paths
- Macro exports (fb_println!, serial_println!) need #[macro_use] on parent module
- Recommend: use `pub use` re-exports in directory mod.rs files

## When to Do This:
- After core boot chain is stabilized
- Before starting real GUI implementation
- Use a proper Rust refactoring tool or IDE support
