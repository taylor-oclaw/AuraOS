extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub enum NotifPriority {
    Low,
    Medium,
    High,
    Critical,
}

pub enum NotifAction {
    Approve,
    Deny,
    Dismiss,
    Custom(String),
}

pub struct Notification {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub priority: NotifPriority,
    pub source_agent: u64,
    pub requires_response: bool,
    pub actions: Vec<String>,
    pub response: Option<NotifAction>,
    pub created_at: u64,
    pub read: bool,
}

pub struct ShareNotification {
    pub notification_id: u64,
    pub data_description: String,
    pub requester: String,
    pub recipient: String,
    pub data_zone: u8,
}

pub struct NotificationCenter {
    pub notifications: Vec<Notification>,
    pub share_requests: Vec<ShareNotification>,
    pub next_id: u64,
    pub quiet_hours: Option<(u64, u64)>,
}

impl NotificationCenter {
    pub fn new() -> Self {
        Self {
            notifications: Vec::new(),
            share_requests: Vec::new(),
            next_id: 1,
            quiet_hours: Some((23, 8)),
        }
    }

    pub fn notify(&mut self, title: &str, body: &str, priority: NotifPriority, agent: u64) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.notifications.push(Notification {
            id,
            title: String::from(title),
            body: String::from(body),
            priority,
            source_agent: agent,
            requires_response: false,
            actions: Vec::new(),
            response: None,
            created_at: 0,
            read: false,
        };
        id
    }

    pub fn share_request(&mut self, desc: &str, requester: &str, recipient: &str, zone: u8) -> u64 {
        let id = self.notify(
            &(String::from("Share Request: ") + desc),
            &(String::from(requester) + " wants to share with " + recipient),
            NotifPriority::High,
            0,
        );
        self.share_requests.push(ShareNotification {
            notification_id: id,
            data_description: String::from(desc),
            requester: String::from(requester),
            recipient: String::from(recipient),
            data_zone: zone,
        };
        if let Some(n) = self.notifications.iter_mut().find(|n| n.id == id) {
            n.requires_response = true;
            n.actions = vec![
                String::from("Approve"),
                String::from("Deny"),
                String::from("Approve Redacted"),
            ];
        }
        id
    }

    pub fn respond(&mut self, notif_id: u64, action: NotifAction) {
        if let Some(n) = self.notifications.iter_mut().find(|n| n.id == notif_id) {
            n.response = Some(action);
            n.read = true;
        }
    }

    pub fn unread_count(&self) -> usize {
        self.notifications.iter().filter(|n| !n.read).count()
    }

    pub fn critical_unread(&self) -> Vec<&Notification> {
        self.notifications
            .iter()
            .filter(|n| !n.read && matches!(n.priority, NotifPriority::Critical))
            .collect()
    }
))}
