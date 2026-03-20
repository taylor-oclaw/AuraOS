//! AuraOS Shell — command interpreter
//! 
//! The first interface to the OS. Type commands, get results.
//! Eventually this becomes a background service behind the GUI,
//! but it's always accessible.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;
use crate::framebuffer;

static SHELL_BUFFER: spin::Mutex<ShellState> = spin::Mutex::new(ShellState::new());

pub struct ShellState {
    input: [u8; 256],
    cursor: usize,
    history_count: u64,
}

impl ShellState {
    pub const fn new() -> Self {
        ShellState {
            input: [0u8; 256],
            cursor: 0,
            history_count: 0,
        }
    }
}

/// Called from keyboard interrupt handler when a character is typed
pub fn handle_key(c: char) {
    let mut state = SHELL_BUFFER.lock();
    
    match c {
        '\n' | '\r' => {
            // Execute command
            let cmd_len = state.cursor;
            let mut cmd_buf = [0u8; 256];
            cmd_buf[..cmd_len].copy_from_slice(&state.input[..cmd_len]);
            state.cursor = 0;
            state.history_count += 1;
            drop(state); // Release lock before executing
            
            
            // Convert to str
            if let Ok(cmd_str) = core::str::from_utf8(&cmd_buf[..cmd_len]) {
                let cmd = cmd_str.trim();
                if !cmd.is_empty() {
                    execute_command(cmd);
                }
            }
            
            // New prompt
            framebuffer::with_writer(|w| w.set_fg(0, 210, 255));
            crate::fb_print!("  you> ");
            framebuffer::with_writer(|w| w.set_fg(255, 255, 255));
        }
        '\u{8}' => {
            // Backspace
            if state.cursor > 0 {
                state.cursor -= 1;
                framebuffer::with_writer(|w| w.backspace());
            }
        }
        c if c.is_ascii() && c as u32 >= 0x20 => {
            let cur = state.cursor;
            if cur < 255 {
                state.input[cur] = c as u8;
                state.cursor = cur + 1;
                crate::fb_print!("{}", c);
            }
        }
        _ => {}
    }
}

fn execute_command(cmd: &str) {
    // First try exact commands (for power users / dev mode)
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.is_empty() { return; }
    
    match parts[0] {
        // Dev mode commands (prefix with /)
        "/help" => cmd_help(),
        "/clear" | "/cls" => cmd_clear(),
        "/mem" => cmd_memory(),
        "/hw" => cmd_hardware(),
        "/uname" => cmd_uname(),
        "/reboot" => cmd_reboot(),
        "/shutdown" => cmd_shutdown(),
        "/color" => cmd_color(&parts[1..]),
        "/echo" => cmd_echo(&parts[1..]),
        "/tz" => cmd_timezone(&parts[1..]),
        _ => {
            // Natural language mode — send everything through NLP
            let intent = crate::nlp::parse_intent(cmd);
            crate::nlp::respond(&intent);
        }
    }
}

fn cmd_help() {
    framebuffer::with_writer(|w| w.set_fg(0, 255, 180));
    framebuffer::with_writer(|w| w.set_fg(200, 200, 200));
}

fn cmd_about() {
    framebuffer::with_writer(|w| w.set_fg(0, 210, 255));
    framebuffer::with_writer(|w| w.set_fg(200, 200, 200));
}

fn cmd_clear() {
    framebuffer::with_writer(|w| w.clear());
}

fn cmd_memory() {
    // We can't easily get runtime memory stats from the allocator in this version
    // but we can show what we know
    framebuffer::with_writer(|w| w.set_fg(0, 255, 180));
    framebuffer::with_writer(|w| w.set_fg(200, 200, 200));
}

fn cmd_uptime() {
    framebuffer::with_writer(|w| w.set_fg(200, 200, 200));
}

fn cmd_uname() {
    framebuffer::with_writer(|w| w.set_fg(200, 200, 200));
}

fn cmd_echo(args: &[&str]) {
    framebuffer::with_writer(|w| w.set_fg(255, 255, 255));
    for (i, arg) in args.iter().enumerate() {
        if i > 0 { crate::fb_print!(" "); }
        crate::fb_print!("{}", arg);
    }
}

fn cmd_hello() {
    framebuffer::with_writer(|w| w.set_fg(255, 200, 50));
}

pub fn cmd_hardware_pub() { cmd_hardware(); }
pub fn cmd_memory_pub() { cmd_memory(); }

fn cmd_hardware() {
    framebuffer::with_writer(|w| w.set_fg(0, 255, 180));
    framebuffer::with_writer(|w| w.set_fg(200, 200, 200));
    
    // Read CPUID
    #[cfg(target_arch = "x86_64")]
    {
        // Basic CPU info via CPUID - must save/restore rbx (LLVM uses it)
        let cpuid_result: u32;
        let mut vendor = [0u8; 12];
        unsafe {
            let (ebx_val, ecx_val, edx_val): (u32, u32, u32);
            core::arch::asm!(
                "push rbx",
                "cpuid",
                "mov {ebx_out:e}, ebx",
                "pop rbx",
                inout("eax") 0u32 => cpuid_result,
                ebx_out = out(reg) ebx_val,
                out("ecx") ecx_val,
                out("edx") edx_val,
            );
            vendor[0..4].copy_from_slice(&ebx_val.to_le_bytes());
            vendor[4..8].copy_from_slice(&edx_val.to_le_bytes());
            vendor[8..12].copy_from_slice(&ecx_val.to_le_bytes());
        }
        if let Ok(v) = core::str::from_utf8(&vendor) {
        }
    }
    
    
    // PCI scan
    framebuffer::with_writer(|w| w.set_fg(0, 255, 180));
    let pci_devices = crate::pci::scan();
    for dev in &pci_devices {
        framebuffer::with_writer(|w| w.set_fg(200, 200, 200));
            dev.bus, dev.device, dev.function,
            dev.vendor_id, dev.device_id,
            dev.class_name);
    }
}

fn cmd_color(args: &[&str]) {
    if args.is_empty() {
        return;
    }
    match args[0] {
        "cyan" => framebuffer::with_writer(|w| w.set_fg(0, 210, 255)),
        "green" => framebuffer::with_writer(|w| w.set_fg(0, 255, 128)),
        "white" => framebuffer::with_writer(|w| w.set_fg(255, 255, 255)),
        "red" => framebuffer::with_writer(|w| w.set_fg(255, 80, 80)),
        "yellow" => framebuffer::with_writer(|w| w.set_fg(255, 220, 50)),
        "pink" => framebuffer::with_writer(|w| w.set_fg(255, 100, 200)),
        _ => {
            return;
        }
    }
}

fn cmd_aura(args: &[&str]) {
    framebuffer::with_writer(|w| w.set_fg(0, 210, 255));
    if args.is_empty() {
    } else {
    }
}

fn cmd_timezone(args: &[&str]) {
    if args.is_empty() {
            crate::rtc::timezone_name(),
            if crate::rtc::timezone_offset() >= 0 { "+" } else { "" },
            crate::rtc::timezone_offset());
        return;
    }
    match args[0] {
    }
}

fn cmd_reboot() {
    framebuffer::with_writer(|w| w.set_fg(255, 200, 50));
    // Triple fault to reboot
    unsafe {
        // Write to the keyboard controller reset line
        let mut port = x86_64::instructions::port::Port::<u8>::new(0x64);
        port.write(0xFE);
    }
}

fn cmd_shutdown() {
    framebuffer::with_writer(|w| w.set_fg(255, 200, 50));
    // Halt all CPUs
    loop { x86_64::instructions::hlt(); }
}
