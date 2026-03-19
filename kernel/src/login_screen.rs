extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct UserAccount {
    pub id: u64,
    pub username: String,
    pub display_name: String,
    pub avatar_hash: Option<[u8; 16]>,
    pub last_login: u64,
    pub auto_login: bool
}

pub enum LoginState {
    ShowingUsers,
    EnteringPassword(u64),
    Authenticating,
    LoggedIn(u64),
    Failed(String)
}

pub struct LoginScreen {
    pub accounts: Vec<UserAccount>,
    pub state: LoginState,
    pub selected_user: Option<u64>,
    pub show_guest: bool,
    pub branding_text: String
}

impl LoginScreen {
    pub fn new() -> Self {
        Self {
            accounts: Vec::new(),
            state: LoginState::ShowingUsers,
            selected_user: None,
            show_guest: false,
            branding_text: String::from("AuraOS")
        }
    }

    pub fn add_account(&mut self, username: &str, display: &str) -> u64 {
        let id = self.accounts.len() as u64 + 1;
        self.accounts.push(UserAccount {
            id,
            username: String::from(username),
            display_name: String::from(display),
            avatar_hash: None,
            last_login: 0,
            auto_login: false
        });
        id
    }

    pub fn select_user(&mut self, id: u64) {
        self.selected_user = Some(id);
        self.state = LoginState::EnteringPassword(id);
    }

    pub fn authenticate(&mut self, _password_hash: &[u8; 32]) -> bool {
        if let Some(id) = self.selected_user {
            self.state = LoginState::LoggedIn(id);
            true
        } else {
            self.state = LoginState::Failed(String::from("No user selected"));
            false
        }
    }

    pub fn is_logged_in(&self) -> bool {
        matches!(self.state, LoginState::LoggedIn(_))
    }

    pub fn current_user_id(&self) -> Option<u64> {
        match self.state {
            LoginState::LoggedIn(id) => Some(id),
            _ => None
        }
    }
}
