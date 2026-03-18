#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod vga;
mod serial;
mod interrupts;
mod gdt;
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
    // Initialize GDT and IDT
    gdt::init();
    interrupts::init_idt();

    // Clear screen and print banner
    vga::clear_screen();
    vga::print_banner();

    serial::serial_println!("AuraOS kernel initialized");
    serial::serial_println!("Boot info: {:?}", boot_info.memory_regions.len());

    // Initialize memory
    if let Some(phys_offset) = boot_info.physical_memory_offset.into_option() {
        serial::serial_println!("Physical memory offset: {:#x}", phys_offset);
    }

    vga::println!("");
    vga::println!("  Welcome to AuraOS v0.1.0");
    vga::println!("  The future of human-computer interaction.");
    vga::println!("");
    vga::println!("  [kernel] GDT initialized");
    vga::println!("  [kernel] IDT initialized");
    vga::println!("  [kernel] Memory regions: {}", boot_info.memory_regions.len());
    vga::println!("");
    vga::println!("  Type 'help' for available commands.");
    vga::println!("");

    // Halt loop
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial::serial_println!("KERNEL PANIC: {}", info);
    vga::set_color(vga::Color::Red, vga::Color::Black);
    vga::println!("\n  !! KERNEL PANIC !!");
    vga::println!("  {}", info);
    loop {
        x86_64::instructions::hlt();
    }
}
pub mod net;
