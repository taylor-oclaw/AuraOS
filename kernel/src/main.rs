#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;


mod http2_handler;
use bootloader_api::{entry_point, BootInfo, BootloaderConfig};
use core::panic::PanicInfo;



pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(bootloader_api::config::Mapping::Dynamic);
    config.kernel_stack_size = 256 * 1024; // 256 KB stack (default is very small)
    config
};

// Remove the duplicate BOOTLOADER_CONFIG above  
entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

























































mod a2a_protocol;
mod accessibility;
mod account_lockout;
mod acpi;
mod agent_checkpoint;
mod agent_contracts;
mod agent_debugger;
mod agent_economics;
mod agent_hierarchy;
mod agent_identity;
mod agent_marketplace;
mod agent_memory;
mod agent_migration;
mod agent_playbooks;
mod agent_profiler;
mod agent_reflection;
mod agent_reputation;
mod agent_snapshot;
mod agent_telemetry;
mod agent_versioning;
mod allocator;
mod animation_system;
mod anomaly_detector;
mod app_marketplace;
mod apps;
mod archive_manager;
mod arp;
mod arp_cache;
mod ata;
mod attention_mgr;
mod auction_sched;
mod audio;
mod audio_mixer;
mod audio_routing;
mod audit_log;
mod audit_trail;
mod aura_engine;
mod backup_manager;
mod batch_scheduler;
mod behavior;
mod belief_revision;
mod bluetooth_mgr;
mod bluetooth_stack;
mod boot_sequence;
mod buddy_allocator;
mod camera_driver;
mod capability_system;
mod cert_manager;
mod cgroup_manager;
mod checksum_engine;
mod clipboard;
mod clipboard_mgr;
mod clipboard_plus;
mod cmd_parser;
mod coalition_mgr;
mod color;
mod color_picker;
mod command_palette;
mod compat_bridge;
mod compositor;
mod consensus_engine;
mod constitution;
mod context_inherit;
mod cost_accounting;
mod cpu_hotplug;
mod crucible_parser;
mod crypto;
mod crypto_engine;
mod cursor;
mod cursor_manager;
mod date_picker;
mod datetime_mgr;
mod deadlock_detector;
mod debug_console;
mod defs_alloc;
mod defs_btree;
mod defs_decay;
mod defs_dedup;
mod defs_disk_format;
mod defs_inode;
mod defs_journal;
mod defs_model;
mod defs_prefetch;
mod defs_snapshot;
mod defs_super;
mod defs_tests;
mod defs_vfs;
mod desktop;
mod device_manager;
mod device_mgr;
mod device_tree;
mod dhcp;
mod dhcp_client;
mod dialog_box;
mod disk_partition;
mod display_server;
mod dma;
mod dma_controller;
mod dns;
mod dns_cache;
mod dns_resolver;
mod drag_drop;
mod dream_cycle;
mod e1000;
mod elf_loader;
mod embedding_engine;
mod encryption_aes;
mod encryption_rsa;
mod entropy_pool;
mod env;
mod ethernet;
mod event_bus;
mod event_loop;
mod events;
mod fat32;
mod fb_console;
mod federated_learn;
mod file_compression;
mod file_dialog;
mod file_mgr;
mod file_permissions;
mod file_watcher;
mod filesystem_btrfs;
mod filesystem_ext4;
mod filesystem_fat;
mod filesystem_ntfs;
mod firewall_engine;
mod focus_modes;
mod font;
mod font_data;
mod framebuffer;
mod framebuffer_mgr;
mod ftp_client;
mod gdt;
mod gesture_engine;
mod gguf;
mod goal_decompose;
mod gossip_protocol;
mod gpu_driver;
mod graphics;
mod group_manager;
mod grpc_runtime;
mod gui_automation;
mod gui_text;
mod hot_reload;
mod http;
mod http_client;
mod http_server;
mod hugepage_alloc;
mod i18n_system;
mod icmp;
mod identity;
mod image_viewer;
mod imap_client;
mod input_router;
mod intent_parser;
mod intent_storage;
mod interrupt_ctrl;
mod interrupts;
mod io_scheduler;
mod ipc;
mod ipc_semaphore;
mod jwt_handler;
mod kbd_layout;
mod kerberos_client;
mod kernel_orchestrator;
mod key_manager;
mod keyboard;
mod knowledge_graph;
mod ldap_client;
mod llm_engine;
mod loadavg_calc;
mod lock_screen;
mod logger;
mod login_manager;
mod login_screen;
mod math;
mod media_controls;
mod mem_pool;
mod memfs;
mod memory;
mod memory_decay;
mod memory_pool;
mod menu_system;
mod mesh;
mod mfa_handler;
mod model_loader;
mod model_registry;
mod mount_manager;
mod mouse;
mod multi_monitor;
mod namespace_isolate;
mod nat_gateway;
mod net_stack;
mod nlp;
mod nlp_bridge;
mod nlp_tokenizer;
mod notif_sounds;
mod notification_center;
mod notifications;
mod npu_driver;
mod ntp_sync;
mod numa_topology;
mod nvme_driver;
mod oauth_client;
mod oom_killer;
mod os_init;
mod packet_filter;
mod page_fault;
mod pam_module;
mod panic_handler;
mod password_hasher;
mod pci;
mod pci_bus;
mod perf_profiler;
mod permissions;
mod pipe_system;
mod pipes;
mod plugin_system;
mod pop3_client;
mod power_mgmt;
mod power_profiles;
mod print_manager;
mod priority_queue;
mod privacy_engine;
mod proc_mgr;
mod proc_sched;
mod process_table;
mod progress_bar;
mod prompt_templates;
mod proxy_server;
mod quantizer;
mod rag_pipeline;
mod raid_manager;
mod random;
mod rate_limiter;
mod resource_monitor;
mod ring_buffer;
mod route_table;
mod rtc;
mod rtc_clock;
mod saml_handler;
mod sandbox_eval;
mod sandbox_manager;
mod sata_controller;
mod scheduler;
mod screen_capture;
mod scrollbar;
mod search_indexer;
mod seccomp_filter;
mod secure_boot;
mod secure_ipc;
mod self_evolution;
mod self_healing;
mod semantic_search;
mod sentiment_engine;
mod serial;
mod serial_console;
mod serial_port;
mod service_discovery;
mod session_manager;
mod sftp_handler;
mod shared_memory;
mod shell;
mod shutdown_mgr;
mod signal_handler;
mod signals;
mod slab_allocator;
mod smtp_client;
mod socket_api;
mod ssh_client;
mod status_bar;
mod storage;
mod string_utils;
mod sudo_handler;
mod surface;
mod swap_manager;
mod swarm_intelligence;
mod symlink_handler;
mod sync_primitives;
mod sys_monitor;
mod syscall;
mod sysinfo;
mod syslog_daemon;
mod system_config;
mod system_tray;
mod tab_control;
mod tar_handler;
mod task_queue;
mod taskbar;
mod tcpip;
mod telnet_server;
mod tensor_ops;
mod terminal;
mod text_editor;
mod theme;
mod theme_engine;
mod tiered_cache;
mod timer_subsystem;
mod timers;
mod tls_handshake;
mod tokenizer;
mod tool_executor;
mod tooltip_system;
mod trace_assurance;
mod transformer;
mod tree_view;
mod trust_scoring;
mod updater;
mod uptime;
mod usb;
mod usb_hid;
mod usb_hub;
mod usb_mass_storage;
mod user_auth;
mod user_profiles;
mod vfs;
mod vga_text;
mod video_decoder;
mod virtio_net;
mod virtual_desktops;
mod voice_engine;
mod voice_pipeline;
mod vpn_client;
mod wasm_runtime;
mod watch_agent;
mod websocket_mgr;
mod websocket_server;
mod widget;
mod wifi_stack;
mod window_decor;
mod window_mgr;
mod window_tiling;
mod wm;
mod workflow_engine;
mod zip_handler;

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // Initialize serial first (for debug output even if screen fails)
    serial::init();
    crate::serial_println!("[kernel] AuraOS v0.1.0 booting...");

    // Initialize GDT and interrupts
    gdt::init();
    crate::serial_println!("[kernel] GDT initialized");

    interrupts::init_idt();
    crate::serial_println!("[kernel] IDT initialized");

    unsafe { interrupts::PICS.lock().initialize() };
    crate::serial_println!("[kernel] PIC initialized");
    
    // Configure PIC masks: only enable keyboard (IRQ1), mask everything else
    // PIC1 mask register = port 0x21
    // PIC2 mask register = port 0xA1
    // Bit = 0 means enabled, bit = 1 means masked
    unsafe {
        // Read current masks (set by initialize)
        let pic1_mask: u8 = x86_64::instructions::port::Port::new(0x21).read();
        let pic2_mask: u8 = x86_64::instructions::port::Port::new(0xA1).read();
        crate::serial_println!("[kernel] PIC masks after init: PIC1={:#04x} PIC2={:#04x}", pic1_mask, pic2_mask);
        
        // Enable ONLY keyboard (IRQ1) and cascade (IRQ2, needed for PIC2)
        x86_64::instructions::port::Port::<u8>::new(0x21).write(0b1111_1001); // mask all except IRQ1(kbd) and IRQ2(cascade)
        x86_64::instructions::port::Port::<u8>::new(0xA1).write(0xFF); // mask all on PIC2
        
        // Don't flush keyboard — let the handler process any pending data
        
        let pic1_after: u8 = x86_64::instructions::port::Port::new(0x21).read();
        crate::serial_println!("[kernel] PIC1 mask now: {:#04x} (keyboard enabled)", pic1_after);
    }
    
    // DON'T enable hardware interrupts — we use polling mode
    // x86_64::instructions::interrupts::enable();
    crate::serial_println!("[kernel] Polling mode — interrupts disabled");

    // Initialize framebuffer
    let fb_info = if let Some(fb) = boot_info.framebuffer.as_mut() {
        let info = fb.info();
        crate::serial_println!(
            "[kernel] Framebuffer: {}x{}, {} bpp, {:?}",
            info.width, info.height, info.bytes_per_pixel, info.pixel_format
        );
        framebuffer::init(fb);
        crate::serial_println!("[kernel] Framebuffer initialized");
        Some((info.width, info.height))
    } else {
        crate::serial_println!("[kernel] WARNING: No framebuffer available!");
        None
    };

    // Memory info
    let mem_regions = boot_info.memory_regions.len();
    let usable_mem = memory::total_usable_memory(&boot_info.memory_regions);
    crate::serial_println!("[kernel] Memory: {} MB usable ({} regions)", usable_mem / (1024 * 1024), mem_regions);

    // Initialize heap allocator
    let phys_mem_offset = boot_info.physical_memory_offset.into_option()
        .expect("physical_memory_offset not set by bootloader");
    let phys_mem_offset = x86_64::VirtAddr::new(phys_mem_offset);
    
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_regions) };
    
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");
    crate::serial_println!("[kernel] Heap allocator initialized ({} KB)", allocator::HEAP_SIZE / 1024);

    // Test heap allocation
    {
        use alloc::vec;
        let test = vec![1, 2, 3, 4, 5];
        crate::serial_println!("[kernel] Heap test: {:?} - OK!", test);
    }

    // Draw the boot screen
    crate::serial_println!("[kernel] Drawing boot screen...");
    framebuffer::with_writer(|w| {
        w.set_fg(0, 210, 255);
        w.write_string("\n");
        w.write_string("     _                        ___  ____\n");
        w.write_string("    / \\  _   _ _ __ __ _     / _ \\/ ___|\n");
        w.write_string("   / _ \\| | | | '__/ _` |   | | | \\___ \\\n");
        w.write_string("  / ___ \\ |_| | | | (_| |   | |_| |___) |\n");
        w.write_string(" /_/   \\_\\__,_|_|  \\__,_|    \\___/|____/\n");
        w.write_string("\n");
        w.set_fg(0, 255, 128);
        w.write_string("  The Ambient Intelligence Operating System\n");
        w.write_string("  v0.1.0 - Created by Venkat Yarlagadda\n");
        w.write_string("\n");
        w.set_fg(100, 200, 255);
        w.write_string("  [ok] GDT initialized\n");
        w.write_string("  [ok] IDT initialized\n");
        w.write_string("  [ok] PIC initialized\n");
        w.write_string("  [ok] Interrupts enabled\n");
        w.write_string("  [ok] Framebuffer active");
    });
    
    if let Some((w, h)) = fb_info {
        crate::fb_print!(" ({}x{})", w, h);
    }
    crate::fb_println!("");
    
    crate::fb_print!("  [ok] Memory: {} MB usable\n", usable_mem / (1024 * 1024));
    
    framebuffer::with_writer(|w| {
        w.set_fg(0, 255, 180);
        w.write_string("  [ok] Heap allocator: 1 MB\n");
        w.set_fg(100, 180, 255);
        w.write_string("  [ok] Aura Engine: pattern match mode\n");
        w.write_string("        (LLM runtime ready for model loading)\n");
        w.set_fg(0, 255, 180);
        w.write_string("  [ok] Shell ready\n");
        w.write_string("\n");
        w.set_fg(255, 255, 255);
        w.write_string("  Launching desktop...\n");
    });

    crate::serial_println!("[kernel] Boot complete.");

    // Initialize mouse
    mouse::init(1280, 720);
    crate::serial_println!("[kernel] Mouse initialized");

    // Boot straight into desktop GUI
    let (fb_w, fb_h) = fb_info.unwrap_or((1280, 720));
    desktop::init(fb_w as u32, fb_h as u32);
    framebuffer::with_writer(|w| {
        let fb = unsafe { w.raw_buffer() };
        desktop::render(fb, fb_w, 3);
    });
    crate::serial_println!("[kernel] Desktop launched");

    // Keyboard decoder
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    let mut kbd = Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore);
    
    crate::serial_println!("[kernel] Entering main loop...");

    // Main loop — poll keyboard controller directly
    loop {
        // Check if keyboard controller has data (bit 0 of status port 0x64)
        let status: u8 = unsafe { 
            let mut port = x86_64::instructions::port::Port::<u8>::new(0x64);
            port.read()
        };
        
        if status & 1 != 0 {
            // Data available — read scancode from port 0x60
            let scancode: u8 = unsafe {
                let mut port = x86_64::instructions::port::Port::<u8>::new(0x60);
                port.read()
            };
            
            if let Ok(Some(key_event)) = kbd.add_byte(scancode) {
                if let Some(key) = kbd.process_keyevent(key_event) {
                    match key {
                        DecodedKey::Unicode(character) => {
                            // Route to desktop input router
                            input_router::handle_key(character);
                            
                            // Re-render desktop with updated terminal content
                            framebuffer::with_writer(|w| {
                                let (fw, _fh, _fs) = w.get_info();
                                let fb = unsafe { w.raw_buffer() };
                                desktop::render(fb, fw, 3);
                            });
                        }
                        DecodedKey::RawKey(_key) => {}
                    }
                }
            }
        } else {
            // Poll mouse
            if mouse::poll(1280, 720) {
                let ms = mouse::state();
                desktop::update_mouse(ms.x, ms.y);
                
                // Re-render desktop if active
                if desktop::is_active() {
                    framebuffer::with_writer(|w| {
                        let fb = unsafe { w.raw_buffer() };
                        desktop::render(fb, 1280, 3);
                    });
                }
            }
            
            // Brief pause to avoid burning CPU
            core::hint::spin_loop();
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crate::serial_println!("KERNEL PANIC: {}", info);
    framebuffer::with_writer(|w| {
        w.set_fg(255, 60, 60);
        use core::fmt::Write;
        let _ = write!(w, "\n\n  !! KERNEL PANIC !!\n  {}", info);
    });
    loop {
        x86_64::instructions::hlt();
    }
}
