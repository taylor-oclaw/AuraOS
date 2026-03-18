#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod serial;
mod framebuffer;
mod gdt;
mod interrupts;
mod memory;

use bootloader_api::{entry_point, BootInfo, BootloaderConfig};
use core::panic::PanicInfo;

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(bootloader_api::config::Mapping::Dynamic);
    config
};

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
    
    // Mask the timer (IRQ0) to prevent double faults — we don't need it yet
    unsafe {
        let mut pic1_mask: u8 = x86_64::instructions::port::Port::new(0x21).read();
        pic1_mask |= 0x01; // Mask IRQ0 (timer)
        x86_64::instructions::port::Port::new(0x21).write(pic1_mask);
    }
    
    x86_64::instructions::interrupts::enable();
    crate::serial_println!("[kernel] PIC initialized, timer masked, interrupts enabled");

    // Initialize framebuffer
    if let Some(fb) = boot_info.framebuffer.as_mut() {
        let info = fb.info();
        crate::serial_println!(
            "[kernel] Framebuffer: {}x{}, {} bpp, {:?}",
            info.width, info.height, info.bytes_per_pixel, info.pixel_format
        );
        crate::serial_println!("[kernel] Framebuffer buffer len: {} bytes", info.byte_len);
        crate::serial_println!("[kernel] Initializing framebuffer writer...");
        framebuffer::init(fb);
        crate::serial_println!("[kernel] Framebuffer initialized successfully!");
    } else {
        crate::serial_println!("[kernel] WARNING: No framebuffer available!");
    }

    // Memory info
    let mem_regions = boot_info.memory_regions.len();
    let usable_mem = memory::total_usable_memory(&boot_info.memory_regions);
    crate::serial_println!("[kernel] Memory: {} MB usable ({} regions)", usable_mem / (1024 * 1024), mem_regions);

    // Draw the boot screen using direct framebuffer writes (less stack pressure)
    crate::serial_println!("[kernel] Drawing boot screen...");
    framebuffer::with_writer(|w| {
        w.write_string("\n");
        w.write_string("     _                        ___  ____\n");
        w.write_string("    / \\  _   _ _ __ __ _     / _ \\/ ___|\n");
        w.write_string("   / _ \\| | | | '__/ _` |   | | | \\___ \\\n");
        w.write_string("  / ___ \\ |_| | | | (_| |   | |_| |___) |\n");
        w.write_string(" /_/   \\_\\__,_|_|  \\__,_|    \\___/|____/\n");
        w.write_string("\n");
        w.set_fg(0, 255, 128);
        w.write_string("  The Ambient Intelligence Operating System\n");
        w.write_string("  v0.1.0 - Built by Venkat Yarlagadda\n");
        w.write_string("\n");
        w.set_fg(100, 200, 255);
        w.write_string("  [ok] GDT initialized\n");
        w.write_string("  [ok] IDT initialized\n");
        w.write_string("  [ok] PIC initialized\n");
        w.write_string("  [ok] Interrupts enabled\n");
        w.write_string("  [ok] Framebuffer active\n");
        w.write_string("  [ok] Memory: ");
    });
    crate::fb_print!("{}", usable_mem / (1024 * 1024));
    framebuffer::with_writer(|w| {
        w.write_string(" MB usable\n\n");
        w.set_fg(255, 255, 255);
        w.write_string("  Keyboard active - type something!\n\n");
        w.set_fg(0, 210, 255);
        w.write_string("  aura> ");
    });

    crate::serial_println!("[kernel] Boot complete. Waiting for input...");

    // Main loop — halt until interrupt
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crate::serial_println!("KERNEL PANIC: {}", info);
    // Try to print to framebuffer too
    framebuffer::with_writer(|w| {
        w.set_fg(255, 60, 60);
        use core::fmt::Write;
        let _ = write!(w, "\n\n  !! KERNEL PANIC !!\n  {}", info);
    });
    loop {
        x86_64::instructions::hlt();
    }
}
