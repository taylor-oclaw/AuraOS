#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

mod serial;
mod framebuffer;
mod gdt;
mod interrupts;
mod memory;
mod allocator;
mod keyboard;
mod shell;
mod nlp;
mod aura_engine;
mod crypto;
mod rtc;
mod math;
mod gguf;
mod tokenizer;
mod transformer;
mod pci;
mod font;
mod random;
mod ata;
mod icmp;
mod http;
mod mouse;
mod acpi;
mod memfs;
mod surface;
mod e1000;
mod usb;
mod mesh;
mod scheduler;
mod identity;
mod desktop;
mod text_editor;
mod widget;
mod serial_console;
mod behavior;
mod updater;
mod gui_text;
mod input_router;
mod apps;
mod audio;
mod tcpip;
mod clipboard;
mod notifications;
mod wm;
mod logger;
mod permissions;
mod theme;
mod power_mgmt;
mod ring_buffer;
mod arp;
mod vfs;
mod graphics;
mod proc_sched;
mod ipc;
mod events;
mod sysinfo;
mod env;
mod pipes;
mod timers;
mod device_mgr;
mod cmd_parser;
mod signals;
mod storage;

mod uptime;
mod kbd_layout;
mod string_utils;
mod sync_primitives;
mod dns;
mod panic_handler;
mod compositor;
mod fat32;
mod syscall;
mod virtio_net;
mod elf_loader;
mod dhcp;
mod cursor;
mod dma;
mod mem_pool;
mod color;
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
