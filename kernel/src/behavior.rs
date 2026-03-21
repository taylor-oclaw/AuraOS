//! Behavior Engine — Self-Adapting OS
//! Tracks user patterns and adapts the OS automatically.
//! The OS literally learns how you work and optimizes itself.

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

/// An event the user performed
#[derive(Debug, Clone)]
pub enum UserEvent {
    AppOpened(String),
    CommandRun(String),
    FileAccessed(String),
    SettingChanged(String, String),
    TimeActive(u8), // Hour of day (0-23)
    SurfaceCreated(String),
    SurfaceClosed(String),
}

/// A suggestion from the behavior engine
#[derive(Debug, Clone)]
pub enum Suggestion {
    PrelaunchApp(String),
    SuggestLayout(String),
    AdjustSetting(String, String),
    PrecacheFile(String),
    DimScreen,
    WakeScreen,
}

/// Tracks usage of a named item
#[derive(Debug, Clone)]
struct UsageCounter {
    name: String,
    count: u32,
    last_hour: u8,
}

/// The behavior engine
pub struct BehaviorEngine {
    app_usage: Vec<UsageCounter>,
    command_usage: Vec<UsageCounter>,
    file_usage: Vec<UsageCounter>,
    hourly_activity: [u32; 24],
    total_events: u64,
    learning: bool,
}

impl BehaviorEngine {
    pub fn new() -> Self {
        BehaviorEngine {
            app_usage: Vec::new(),
            command_usage: Vec::new(),
            file_usage: Vec::new(),
            hourly_activity: [0; 24],
            total_events: 0,
            learning: true,
        }
    }

    /// Learn from a user event
    pub fn learn(&mut self, event: UserEvent) {
        if !self.learning { return; }
        self.total_events += 1;

        match event {
            UserEvent::AppOpened(ref name) => {
                self.increment_usage(&mut self.app_usage.clone(), name);
            }
            UserEvent::CommandRun(ref cmd) => {
                self.increment_usage(&mut self.command_usage.clone(), cmd);
            }
            UserEvent::FileAccessed(ref path) => {
                self.increment_usage(&mut self.file_usage.clone(), path);
            }
            UserEvent::TimeActive(hour) => {
                if (hour as usize) < 24 {
                    self.hourly_activity[hour as usize] += 1;
                }
            }
            _ => {}
        }
    }

    fn increment_usage(&mut self, _list: &mut Vec<UsageCounter>, name: &str) {
        // Find or create counter
        let found = self.app_usage.iter_mut().find(|u| u.name == name);
        if let Some(counter) = found {
            counter.count += 1;
        } else {
            self.app_usage.push(UsageCounter {
                name: String::from(name),
                count: 1,
                last_hour: 0,
            });
        }
    }

    /// Get suggestions based on learned patterns
    pub fn suggest(&self) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();

        // Suggest most-used apps for prelaunching
        if let Some(top_app) = self.app_usage.iter().max_by_key(|u| u.count) {
            if top_app.count > 5 {
                suggestions.push(Suggestion::PrelaunchApp(top_app.name.clone()));
            }
        }

        // Suggest dim screen during inactive hours
        let current_hour = crate::rtc::read_local_time().hour as usize;
        if current_hour < 24 && self.hourly_activity[current_hour] == 0 && self.total_events > 100 {
            suggestions.push(Suggestion::DimScreen);
        }

        suggestions
    }

    /// Most used app
    pub fn top_app(&self) -> Option<&str> {
        self.app_usage.iter()
            .max_by_key(|u| u.count)
            .map(|u| u.name.as_str())
    }

    /// Is the user typically active at this hour?
    pub fn is_active_hour(&self, hour: u8) -> bool {
        if (hour as usize) < 24 {
            self.hourly_activity[hour as usize] > 0
        } else {
            false
        }
    }

    pub fn event_count(&self) -> u64 {
        self.total_events
    }
}
