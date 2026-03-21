//! AuraOS Initialization — wires ALL subsystems together
//!
//! This is the master glue code that turns 152 modules into a running OS.

extern crate alloc;
use alloc::string::String;

// Core boot infrastructure
use crate::boot_sequence::BootSequence;
use crate::device_manager::DeviceManager;
use crate::display_server::DisplayServer;
use crate::event_loop::EventLoop;
use crate::system_config::SystemConfig;

// Agent-native systems
use crate::kernel_orchestrator::KernelOrchestrator;
use crate::knowledge_graph::KnowledgeGraph;
use crate::notification_center::NotificationCenter;
use crate::nlp_bridge::NlpBridge;
use crate::attention_mgr::AttentionManager;
use crate::agent_hierarchy::AgentHierarchy;

// OS subsystems (Waves 25-28)
use crate::multi_monitor::MultiMonitor;
use crate::gpu_driver::GpuDriver;
use crate::io_scheduler::IoScheduler;
use crate::wifi_stack::WifiManager;
use crate::bluetooth_mgr::BluetoothManager;
use crate::theme_engine::ThemeEngine;
use crate::animation_system::AnimationEngine;
use crate::status_bar::StatusBar;
use crate::lock_screen::LockScreen;
use crate::login_screen::LoginScreen;
use crate::shutdown_mgr::ShutdownManager;
use crate::power_profiles::PowerManager;
use crate::i18n_system::I18nManager;
use crate::datetime_mgr::DateTimeManager;
use crate::notif_sounds::SoundManager;
use crate::file_permissions::PermissionEngine;
use crate::search_indexer::SearchIndexer;
use crate::sandbox_manager::SandboxManager;
use crate::backup_manager::BackupManager;
use crate::user_profiles::ProfileManager;
use crate::clipboard_plus::ClipboardManager;
use crate::screen_capture::ScreenCapture;
use crate::print_manager::PrintManager;
use crate::drag_drop::DragDropManager;
use crate::accessibility::AccessibilityManager;
use crate::wasm_runtime::WasmRuntime;
use crate::self_evolution::SelfEvolution;
use crate::compat_bridge::CompatBridge;
use crate::voice_engine::VoiceEngine;
use crate::app_marketplace::AppMarketplace;

/// Core OS state — ALL subsystems
pub struct AuraOs {
    // Boot & hardware
    pub boot: BootSequence,
    pub devices: DeviceManager,
    pub gpu: GpuDriver,
    pub io: IoScheduler,

    // Display & UI
    pub display: DisplayServer,
    pub monitors: MultiMonitor,
    pub theme: ThemeEngine,
    pub animations: AnimationEngine,
    pub status_bar: StatusBar,
    pub attention: AttentionManager,

    // Input
    pub events: EventLoop,
    pub voice: VoiceEngine,
    pub drag_drop: DragDropManager,
    pub clipboard: ClipboardManager,

    // Agent systems
    pub agents: AgentHierarchy,
    pub orchestrator: KernelOrchestrator,
    pub knowledge: KnowledgeGraph,
    pub wasm: WasmRuntime,
    pub sandbox: SandboxManager,
    pub marketplace: AppMarketplace,
    pub evolution: SelfEvolution,

    // Networking
    pub wifi: WifiManager,
    pub bluetooth: BluetoothManager,
    pub compat: CompatBridge,

    // System services
    pub config: SystemConfig,
    pub notifications: NotificationCenter,
    pub sounds: SoundManager,
    pub nlp: NlpBridge,
    pub permissions: PermissionEngine,
    pub search: SearchIndexer,
    pub profiles: ProfileManager,
    pub datetime: DateTimeManager,
    pub i18n: I18nManager,
    pub power: PowerManager,
    pub backup: BackupManager,
    pub a11y: AccessibilityManager,
    pub capture: ScreenCapture,
    pub print: PrintManager,

    // Security & auth
    pub lock: LockScreen,
    pub login: LoginScreen,
    pub shutdown: ShutdownManager,

    // State
    pub running: bool,
}

impl AuraOs {
    pub fn new() -> Self {
        Self {
            boot: BootSequence::new(),
            devices: DeviceManager::new(),
            gpu: GpuDriver::new(),
            io: IoScheduler::new(),
            display: DisplayServer::new(1920, 1080),
            monitors: MultiMonitor::new(),
            theme: ThemeEngine::new(),
            animations: AnimationEngine::new(),
            status_bar: StatusBar::new(),
            attention: AttentionManager::new(),
            events: EventLoop::new(),
            voice: VoiceEngine::new(),
            drag_drop: DragDropManager::new(),
            clipboard: ClipboardManager::new(),
            agents: AgentHierarchy::new(10000),
            orchestrator: KernelOrchestrator::new(),
            knowledge: KnowledgeGraph::new(),
            wasm: WasmRuntime::new(),
            sandbox: SandboxManager::new(),
            marketplace: AppMarketplace::new(),
            evolution: SelfEvolution::new("device-001"),
            wifi: WifiManager::new(),
            bluetooth: BluetoothManager::new(),
            compat: CompatBridge::new(),
            config: SystemConfig::new(),
            notifications: NotificationCenter::new(),
            sounds: SoundManager::new(),
            nlp: NlpBridge::new(),
            permissions: PermissionEngine::new(),
            search: SearchIndexer::new(),
            profiles: ProfileManager::new(),
            datetime: DateTimeManager::new(),
            i18n: I18nManager::new(),
            power: PowerManager::new(),
            backup: BackupManager::new(),
            a11y: AccessibilityManager::new(),
            capture: ScreenCapture::new(),
            print: PrintManager::new(),
            lock: LockScreen::new(),
            login: LoginScreen::new(),
            shutdown: ShutdownManager::new(),
            running: false,
        }
    }

    /// Full boot sequence — 8 phases
    pub fn boot(&mut self) {
        // Phase 1: Hardware
        self.devices.probe_all();
        self.gpu.init();
        self.boot.advance(50);

        // Phase 2: Memory (done by bootloader)
        self.boot.advance(10);

        // Phase 3: Filesystem (DEFS)
        self.boot.advance(100);

        // Phase 4: Agents
        self.boot.advance(30);

        // Phase 5: Orchestrator
        self.orchestrator.register_capability(1, "system", 0);
        self.boot.advance(20);

        // Phase 6: Display
        self.display.boot_screen();
        self.boot.advance(50);

        // Phase 7: Event loop + services
        self.events.start();
        // WiFi ready
        self.bluetooth.enable();
        self.voice.start_listening();
        self.boot.advance(20);

        // Phase 8: Ready
        self.login.add_account("admin", "Administrator");
        self.boot.advance(10);
        self.running = true;
    }

    /// Main loop tick
    pub fn tick(&mut self) {
        if !self.running || self.shutdown.is_shutting_down() { return; }
        self.datetime.tick();
        self.animations.tick(16); // ~60fps
        while let Some(event) = self.events.poll() {
            match event {
                crate::event_loop::OsEvent::Shutdown => {
                    self.shutdown.initiate(crate::shutdown_mgr::ShutdownAction::Shutdown);
                }
                _ => {}
            }
        }
    }
}
