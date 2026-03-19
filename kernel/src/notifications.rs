extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;

#[derive(Clone, Copy, PartialEq)]
pub enum NotifLevel { Info, Warning, Error, Success }

pub struct Notification {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub level: NotifLevel,
    pub timestamp: u64,
    pub read: bool,
    pub source: String,
}

pub struct NotificationManager {
    notifications: Vec<Notification>,
    next_id: u64,
    max_count: usize,
}

impl NotificationManager {
    pub fn new() -> Self {
        Self { notifications: Vec::new(), next_id: 1, max_count: 100 }
    }

    pub fn notify(&mut self, title: &str, body: &str, level: NotifLevel, source: &str, timestamp: u64) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        if self.notifications.len() >= self.max_count {
            self.notifications.remove(0);
        }
        self.notifications.push(Notification {
            id,
            title: String::from(title),
            body: String::from(body),
            level,
            timestamp,
            read: false,
            source: String::from(source),
        });
        id
    }

    pub fn dismiss(&mut self, id: u64) {
        if let Some(n) = self.notifications.iter_mut().find(|n| n.id == id) {
            n.read = true;
        }
    }

    pub fn dismiss_all(&mut self) {
        for n in self.notifications.iter_mut() {
            n.read = true;
        }
    }

    pub fn unread_count(&self) -> usize {
        self.notifications.iter().filter(|n| !n.read).count()
    }

    pub fn recent(&self, count: usize) -> &[Notification] {
        let start = self.notifications.len().saturating_sub(count);
        &self.notifications[start..]
    }
}
