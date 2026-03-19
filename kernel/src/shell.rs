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
            
            crate::fb_println!("");
            
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
        _ => {
            // Natural language mode — send everything through NLP
            let intent = crate::nlp::parse_intent(cmd);
            crate::nlp::respond(&intent);
        }
    }
}

fn cmd_help() {
    framebuffer::with_writer(|w| w.set_fg(0, 255, 180));
    crate::fb_println!("  AuraOS Commands:");
    framebuffer::with_writer(|w| w.set_fg(200, 200, 200));
    crate::fb_println!("  help          - Show this help");
    crate::fb_println!("  about         - About AuraOS");
    crate::fb_println!("  clear         - Clear screen");
    crate::fb_println!("  mem           - Memory info");
    crate::fb_println!("  uptime        - System uptime");
    crate::fb_println!("  uname         - System info");
    crate::fb_println!("  hardware      - Hardware detection");
    crate::fb_println!("  echo <text>   - Print text");
    crate::fb_println!("  color <name>  - Change prompt color");
    crate::fb_println!("  aura <msg>    - Talk to Aura AI");
    crate::fb_println!("  hello         - Say hi");
    crate::fb_println!("  reboot        - Reboot system");
    crate::fb_println!("  shutdown      - Power off");
}

fn cmd_about() {
    framebuffer::with_writer(|w| w.set_fg(0, 210, 255));
    crate::fb_println!("  AuraOS v0.1.0 - The Ambient Intelligence OS");
    crate::fb_println!("  Created by Venkat Yarlagadda");
    crate::fb_println!("  Built by Taylor Oclaw");
    crate::fb_println!("");
    framebuffer::with_writer(|w| w.set_fg(200, 200, 200));
    crate::fb_println!("  An operating system where AI is not an app you open,");
    crate::fb_println!("  but the environment you live in. Your computer sees,");
    crate::fb_println!("  hears, and understands. It anticipates, adapts, and");
    crate::fb_println!("  works alongside you.");
    crate::fb_println!("");
    crate::fb_println!("  Patent Pending - Suvayar LLC / RedSky LLC");
}

fn cmd_clear() {
    framebuffer::with_writer(|w| w.clear());
}

fn cmd_memory() {
    // We can't easily get runtime memory stats from the allocator in this version
    // but we can show what we know
    framebuffer::with_writer(|w| w.set_fg(0, 255, 180));
    crate::fb_println!("  Memory Status:");
    framebuffer::with_writer(|w| w.set_fg(200, 200, 200));
    crate::fb_println!("  Heap: {} KB allocated", crate::allocator::HEAP_SIZE / 1024);
    crate::fb_println!("  Heap start: {:#x}", crate::allocator::HEAP_START);
}

fn cmd_uptime() {
    framebuffer::with_writer(|w| w.set_fg(200, 200, 200));
    crate::fb_println!("  AuraOS is running (timer not yet calibrated)");
    crate::fb_println!("  PIT timer: masked (stable mode)");
}

fn cmd_uname() {
    framebuffer::with_writer(|w| w.set_fg(200, 200, 200));
    crate::fb_println!("  AuraOS 0.1.0 x86_64 aura-kernel");
}

fn cmd_echo(args: &[&str]) {
    framebuffer::with_writer(|w| w.set_fg(255, 255, 255));
    for (i, arg) in args.iter().enumerate() {
        if i > 0 { crate::fb_print!(" "); }
        crate::fb_print!("{}", arg);
    }
    crate::fb_println!("");
}

fn cmd_hello() {
    framebuffer::with_writer(|w| w.set_fg(255, 200, 50));
    crate::fb_println!("  Hey there! I'm Aura, your OS companion.");
    crate::fb_println!("  I'm still learning, but I'm here to help.");
    crate::fb_println!("  Try 'aura tell me a joke' when I get smarter!");
}

pub fn cmd_hardware_pub() { cmd_hardware(); }
pub fn cmd_memory_pub() { cmd_memory(); }

fn cmd_hardware() {
    framebuffer::with_writer(|w| w.set_fg(0, 255, 180));
    crate::fb_println!("  Hardware Detection:");
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
        crate::fb_println!("  CPU: x86_64 (CPUID max leaf: {})", cpuid_result);
        if let Ok(v) = core::str::from_utf8(&vendor) {
            crate::fb_println!("  Vendor: {}", v);
        }
    }
    
    crate::fb_println!("  Framebuffer: active (1280x720 BGR)");
    crate::fb_println!("  Keyboard: PS/2 (active)");
    crate::fb_println!("  Mouse: PS/2 (not yet initialized)");
    
    // PCI scan
    framebuffer::with_writer(|w| w.set_fg(0, 255, 180));
    crate::fb_println!("");
    crate::fb_println!("  PCI Bus Scan:");
    let pci_devices = crate::pci::scan();
    for dev in &pci_devices {
        framebuffer::with_writer(|w| w.set_fg(200, 200, 200));
        crate::fb_println!("    {:02x}:{:02x}.{} [{:04x}:{:04x}] {}",
            dev.bus, dev.device, dev.function,
            dev.vendor_id, dev.device_id,
            dev.class_name);
    }
}

fn cmd_color(args: &[&str]) {
    if args.is_empty() {
        crate::fb_println!("  Usage: color <cyan|green|white|red|yellow|pink>");
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
            crate::fb_println!("  Unknown color. Try: cyan, green, white, red, yellow, pink");
            return;
        }
    }
    crate::fb_println!("  Color changed to {}", args[0]);
}

fn cmd_aura(args: &[&str]) {
    framebuffer::with_writer(|w| w.set_fg(0, 210, 255));
    if args.is_empty() {
        crate::fb_println!("  Aura AI is not yet connected.");
        crate::fb_println!("  When the LLM runtime is ready, you'll be able to");
        crate::fb_println!("  have conversations right here in the shell.");
        crate::fb_println!("  Try: aura what can you do");
    } else {
        crate::fb_println!("  [Aura AI - offline]");
        crate::fb_println!("  I heard you say: '{}'", args.join(" "));
        crate::fb_println!("  My brain (LLM runtime) isn't loaded yet.");
        crate::fb_println!("  Soon I'll be able to help you with anything!");
    }
}

fn cmd_reboot() {
    framebuffer::with_writer(|w| w.set_fg(255, 200, 50));
    crate::fb_println!("  Rebooting...");
    // Triple fault to reboot
    unsafe {
        // Write to the keyboard controller reset line
        let mut port = x86_64::instructions::port::Port::<u8>::new(0x64);
        port.write(0xFE);
    }
}

fn cmd_shutdown() {
    framebuffer::with_writer(|w| w.set_fg(255, 200, 50));
    crate::fb_println!("  Shutting down AuraOS...");
    crate::fb_println!("  (ACPI power off not yet implemented)");
    crate::fb_println!("  You can safely power off the machine.");
    // Halt all CPUs
    loop { x86_64::instructions::hlt(); }
}
