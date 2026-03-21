extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_std]
mod profile_notification_work_to_personal {
    extern crate alloc;
    use alloc::string::String;
    use alloc::vec::Vec;

    pub struct ProfileNotificationWorkToPersonal {
        user_id: u32,
        notifications: Vec<String>,
    }

    impl ProfileNotificationWorkToPersonal {
        pub fn new(user_id: u32) -> Self {
            ProfileNotificationWorkToPersonal {
                user_id,
                notifications: Vec::new(),
            }
        }

        pub fn add_notification(&mut self, notification: String) {
            self.notifications.push(notification);
        }

        pub fn get_notifications(&self) -> &Vec<String> {
            &self.notifications
        }

        pub fn clear_notifications(&mut self) {
            self.notifications.clear();
        }

        pub fn has_notifications(&self) -> bool {
            !self.notifications.is_empty()
        }

        pub fn remove_notification(&mut self, index: usize) -> Option<String> {
            if index < self.notifications.len() {
                Some(self.notifications.remove(index))
            } else {
                None
            }
        }
    }
}
