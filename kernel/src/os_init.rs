//! AuraOS Initialization — wires all subsystems together
//!
//! This is the glue code that turns 126 individual modules into a running OS.

extern crate alloc;
use alloc::string::String;

use crate::boot_sequence::BootSequence;
use crate::device_manager::DeviceManager;
use crate::display_server::DisplayServer;
use crate::event_loop::EventLoop;
use crate::system_config::SystemConfig;
use crate::kernel_orchestrator::KernelOrchestrator;
use crate::knowledge_graph::KnowledgeGraph;
use crate::notification_center::NotificationCenter;
use crate::nlp_bridge::NlpBridge;
use crate::attention_mgr::AttentionManager;

/// Core OS state — all subsystems in one place
pub struct AuraOs {
    pub boot: BootSequence,
    pub devices: DeviceManager,
    pub display: DisplayServer,
    pub events: EventLoop,
    pub config: SystemConfig,
    pub agent_count: u64,
    pub orchestrator: KernelOrchestrator,
    pub trust_level: u8,
    pub knowledge: KnowledgeGraph,
    pub notifications: NotificationCenter,
    pub tool_count: u64,
    pub nlp: NlpBridge,
    pub attention: AttentionManager,
    pub running: bool,
}

impl AuraOs {
    pub fn new() -> Self {
        Self {
            boot: BootSequence::new(),
            devices: DeviceManager::new(),
            display: DisplayServer::new(1920, 1080),
            events: EventLoop::new(),
            config: SystemConfig::new(),
            agent_count: 0,
            orchestrator: KernelOrchestrator::new(),
            trust_level: 2,
            knowledge: KnowledgeGraph::new(),
            notifications: NotificationCenter::new(),
            tool_count: 0,
            nlp: NlpBridge::new(),
            attention: AttentionManager::new(),
            running: false,
        }
    }

    /// Phase 1: Hardware probe
    pub fn init_hardware(&mut self) {
        self.devices.probe_all();
        self.boot.advance(50);
    }

    /// Phase 2: Memory (already done by bootloader, just mark)
    pub fn init_memory(&mut self) {
        self.boot.advance(10);
    }

    /// Phase 3: Mount DEFS
    pub fn init_filesystem(&mut self) {
        // DEFS mount will go here
        self.boot.advance(100);
    }

    /// Phase 4: Agent registry
    pub fn init_agents(&mut self) {
        self.agent_count = 1; // OS core agent
        self.boot.advance(30);
    }

    /// Phase 5: Start orchestrator
    pub fn init_orchestrator(&mut self) {
        self.orchestrator.register_capability(1, "system", 0);
        self.boot.advance(20);
    }

    /// Phase 6: Display server
    pub fn init_display(&mut self) {
        self.display.boot_screen();
        self.boot.advance(50);
    }

    /// Phase 7: Event loop
    pub fn init_event_loop(&mut self) {
        self.events.start();
        self.events.register_handler("keyboard", 1);
        self.events.register_handler("mouse", 2);
        self.events.register_handler("agent_messages", 3);
        self.events.register_handler("voice", 4);
        self.boot.advance(20);
    }

    /// Phase 8: Ready
    pub fn finalize(&mut self) {
        self.boot.advance(10);
        self.running = true;
        self.notifications.notify(
            "AuraOS Ready",
            "All subsystems initialized",
            crate::notification_center::NotifPriority::Low,
            1,
        );
    }

    /// Full boot sequence
    pub fn boot(&mut self) {
        self.init_hardware();
        self.init_memory();
        self.init_filesystem();
        self.init_agents();
        self.init_orchestrator();
        self.init_display();
        self.init_event_loop();
        self.finalize();
    }

    /// Main loop tick
    pub fn tick(&mut self) {
        if !self.running { return; }

        // Process all pending events
        while let Some(event) = self.events.poll() {
            match event {
                crate::event_loop::OsEvent::KeyPress(key) => {
                    // Forward to NLP bridge or active surface
                }
                crate::event_loop::OsEvent::Shutdown => {
                    self.running = false;
                }
                _ => {}
            }
        }
    }
}
