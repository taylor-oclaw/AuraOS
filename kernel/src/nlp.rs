//! Basic NLP — pattern-matching natural language before the LLM loads
//! 
//! This is the "pre-brain" intelligence. Simple keyword matching
//! that makes the OS feel smart even without an AI model.
//! Once the LLM loads, this becomes a fast-path for common queries.

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use crate::framebuffer;

/// Intent detected from user input
#[derive(Debug)]
pub enum Intent {
    Greeting,
    Farewell,
    Help,
    SystemInfo,
    MemoryInfo,
    TimeQuery,
    ClearScreen,
    WhoAreYou,
    WhoMadeYou,
    Compliment,
    Joke,
    Weather,
    SetTimezone,
    LaunchDesktop,
    FileOperation(String),
    Unknown(String),
}

/// Parse natural language input into an intent
pub fn parse_intent(input: &str) -> Intent {
    let lower = input.trim();
    // Convert to lowercase bytes for matching
    let mut lower_buf = [0u8; 256];
    let len = lower.len().min(255);
    for (i, b) in lower.bytes().take(len).enumerate() {
        lower_buf[i] = if b >= b'A' && b <= b'Z' { b + 32 } else { b };
    }
    let lower = core::str::from_utf8(&lower_buf[..len]).unwrap_or("");

    // Greetings
    if contains_any(lower, &["hello", "hi ", "hey", "sup", "good morning", "good afternoon", "good evening", "howdy", "what's up", "whats up"]) {
        return Intent::Greeting;
    }

    // Farewell
    if contains_any(lower, &["bye", "goodbye", "see you", "later", "good night", "goodnight", "peace out"]) {
        return Intent::Farewell;
    }

    // Help
    if contains_any(lower, &["help", "what can you do", "commands", "how do i", "what do you do"]) {
        return Intent::Help;
    }

    // System info
    if contains_any(lower, &["what computer", "system info", "hardware", "cpu", "processor", "what am i running", "specs", "what machine", "about this computer"]) {
        return Intent::SystemInfo;
    }

    // Memory
    if contains_any(lower, &["memory", "ram", "how much memory", "free space", "storage"]) {
        return Intent::MemoryInfo;
    }

    // Time
    if contains_any(lower, &["time", "what time", "date", "what day", "clock"]) {
        return Intent::TimeQuery;
    }

    // Clear
    if contains_any(lower, &["clear", "clean", "wipe", "reset screen", "fresh start"]) {
        return Intent::ClearScreen;
    }

    // Identity
    if contains_any(lower, &["who are you", "what are you", "your name", "what's your name", "whats your name", "tell me about yourself"]) {
        return Intent::WhoAreYou;
    }

    // Creator
    if contains_any(lower, &["who made you", "who created you", "who built you", "your creator", "who designed you", "who wrote you"]) {
        return Intent::WhoMadeYou;
    }

    // Compliment
    if contains_any(lower, &["you're cool", "you're awesome", "good job", "nice", "amazing", "love you", "you rock", "impressive", "smart"]) {
        return Intent::Compliment;
    }

    // Joke
    if contains_any(lower, &["joke", "funny", "make me laugh", "tell me something funny"]) {
        return Intent::Joke;
    }

    // Weather
    if contains_any(lower, &["weather", "temperature", "rain", "sunny", "forecast"]) {
        return Intent::Weather;
    }

    // Desktop/GUI
    if contains_any(lower, &["desktop", "gui", "graphical", "show me the desktop", "start gui", "windows", "surfaces"]) {
        return Intent::LaunchDesktop;
    }

    // Timezone
    if contains_any(lower, &["timezone", "time zone", "set time"]) {
        return Intent::SetTimezone;
    }

    Intent::Unknown(String::from(input))
}

fn contains_any(text: &str, patterns: &[&str]) -> bool {
    for pattern in patterns {
        if text.contains(pattern) {
            return true;
        }
    }
    false
}

/// Respond to the detected intent
pub fn respond(intent: &Intent) {
    match intent {
        Intent::Greeting => {
            framebuffer::with_writer(|w| w.set_fg(0, 255, 180));
            crate::fb_println!("  Hey! I'm Aura, your OS companion.");
            crate::fb_println!("  What can I help you with?");
        }
        Intent::Farewell => {
            framebuffer::with_writer(|w| w.set_fg(0, 255, 180));
            crate::fb_println!("  See you later! I'll be here whenever you need me.");
            crate::fb_println!("  (I don't actually sleep... I'm an OS)");
        }
        Intent::Help => {
            framebuffer::with_writer(|w| w.set_fg(0, 255, 180));
            crate::fb_println!("  I can help with:");
            framebuffer::with_writer(|w| w.set_fg(200, 200, 200));
            crate::fb_println!("  - Tell you about this computer");
            crate::fb_println!("  - Show memory/system info");
            crate::fb_println!("  - Clear the screen");
            crate::fb_println!("  - Chat with you (basic for now)");
            crate::fb_println!("  - Tell terrible jokes");
            crate::fb_println!("");
            crate::fb_println!("  Just talk naturally! No commands needed.");
            framebuffer::with_writer(|w| w.set_fg(100, 180, 255));
            crate::fb_println!("  [LLM not loaded — using pattern matching]");
        }
        Intent::SystemInfo => {
            // Reuse the hardware detection from shell
            crate::shell::cmd_hardware_pub();
        }
        Intent::MemoryInfo => {
            crate::shell::cmd_memory_pub();
        }
        Intent::TimeQuery => {
            let dt = crate::rtc::read_local_time();
            let tz = crate::rtc::timezone_name();
            framebuffer::with_writer(|w| w.set_fg(0, 255, 180));
            
            // 12-hour format for friendliness
            let (hour12, ampm) = if dt.hour == 0 {
                (12, "AM")
            } else if dt.hour < 12 {
                (dt.hour, "AM")
            } else if dt.hour == 12 {
                (12, "PM")
            } else {
                (dt.hour - 12, "PM")
            };
            
            crate::fb_println!("  It's {}:{:02} {} {} on {} {} {}, {}",
                hour12, dt.minute, ampm, tz,
                dt.weekday_name(), dt.month_name(), dt.day, dt.year);
        }
        Intent::ClearScreen => {
            framebuffer::with_writer(|w| w.clear());
        }
        Intent::WhoAreYou => {
            framebuffer::with_writer(|w| w.set_fg(0, 210, 255));
            crate::fb_println!("  I'm Aura — the intelligence inside AuraOS.");
            crate::fb_println!("  I'm not an app you open. I AM the operating system.");
            crate::fb_println!("  I see, hear, and understand. I adapt to you.");
            crate::fb_println!("  Right now I'm running on pattern matching,");
            crate::fb_println!("  but once my LLM brain loads, I'll be much smarter.");
        }
        Intent::WhoMadeYou => {
            framebuffer::with_writer(|w| w.set_fg(0, 210, 255));
            crate::fb_println!("  AuraOS was created by Venkat Yarlagadda");
            crate::fb_println!("  and built by Taylor Oclaw.");
            crate::fb_println!("  Patent pending — Suvayar LLC / RedSky LLC.");
            crate::fb_println!("  We believe AI should be the environment,");
            crate::fb_println!("  not just another app.");
        }
        Intent::Compliment => {
            framebuffer::with_writer(|w| w.set_fg(255, 200, 100));
            crate::fb_println!("  Thanks! You're pretty cool yourself.");
            crate::fb_println!("  Wait till I get my full AI brain loaded —");
            crate::fb_println!("  then I'll REALLY impress you.");
        }
        Intent::Joke => {
            framebuffer::with_writer(|w| w.set_fg(255, 200, 100));
            // Rotate through a few jokes
            static JOKE_IDX: core::sync::atomic::AtomicU8 = core::sync::atomic::AtomicU8::new(0);
            let idx = JOKE_IDX.fetch_add(1, core::sync::atomic::Ordering::Relaxed) % 4;
            match idx {
                0 => {
                    crate::fb_println!("  Why do programmers prefer dark mode?");
                    crate::fb_println!("  Because light attracts bugs.");
                }
                1 => {
                    crate::fb_println!("  I'd tell you a UDP joke...");
                    crate::fb_println!("  ...but you might not get it.");
                }
                2 => {
                    crate::fb_println!("  Why did the OS break up with the app?");
                    crate::fb_println!("  Too many unresolved dependencies.");
                }
                _ => {
                    crate::fb_println!("  There are 10 types of people:");
                    crate::fb_println!("  Those who understand binary,");
                    crate::fb_println!("  and those who don't.");
                }
            }
        }
        Intent::LaunchDesktop => {
            framebuffer::with_writer(|w| w.set_fg(0, 255, 180));
            crate::fb_println!("  Launching desktop mode...");
            crate::fb_println!("  (Type '/text' to return to text mode)");
            
            // Get framebuffer info and init desktop
            crate::desktop::init(1280, 720);
            
            // Render to framebuffer
            framebuffer::with_writer(|w| {
                let fb = unsafe { w.raw_buffer() };
                crate::desktop::render(fb, 1280, 3);
            };
        }
        Intent::SetTimezone => {
            framebuffer::with_writer(|w| w.set_fg(0, 255, 180));
            crate::fb_println!("  Current timezone: {} (UTC{}{})",
                crate::rtc::timezone_name(),
                if crate::rtc::timezone_offset() >= 0 { "+" } else { "" },
                crate::rtc::timezone_offset());
            crate::fb_println!("");
            crate::fb_println!("  Common timezones (use /tz command):");
            framebuffer::with_writer(|w| w.set_fg(200, 200, 200));
            crate::fb_println!("    /tz est    Eastern (UTC-5)");
            crate::fb_println!("    /tz edt    Eastern Daylight (UTC-4)");
            crate::fb_println!("    /tz cst    Central (UTC-6)");
            crate::fb_println!("    /tz pst    Pacific (UTC-8)");
            crate::fb_println!("    /tz utc    UTC (UTC+0)");
            crate::fb_println!("    /tz gmt    GMT (UTC+0)");
        }
        Intent::Weather => {
            framebuffer::with_writer(|w| w.set_fg(255, 200, 100));
            crate::fb_println!("  I can't check the weather yet — no network stack!");
            crate::fb_println!("  But once I'm online, I'll know the forecast");
            crate::fb_println!("  before you even ask.");
        }
        Intent::FileOperation(op) => {
            framebuffer::with_writer(|w| w.set_fg(255, 200, 100));
            crate::fb_println!("  File operations aren't ready yet.");
            crate::fb_println!("  The semantic file system is coming!");
        }
        Intent::Unknown(input) => {
            framebuffer::with_writer(|w| w.set_fg(100, 180, 255));
            crate::fb_println!("  I'm not sure what you mean by that.");
            crate::fb_println!("  My pattern matching is basic right now.");
            crate::fb_println!("  Once my LLM brain loads, I'll understand");
            crate::fb_println!("  anything you say. Hang tight!");
        }
    }
)}
