extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum AppType {
    Native,
    Wasm,
    Crucible,
}

pub enum AppStatus {
    Available,
    Installed,
    Updating,
    Disabled,
}

pub struct AppManifest {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub app_type: AppType,
    pub version: String,
    pub author: String,
    pub size_bytes: u64,
    pub capabilities_required: Vec<String>,
    pub trust_score: f32,
    pub downloads: u64,
    pub status: AppStatus,
}

pub struct AppReview {
    pub app_id: u64,
    pub reviewer: String,
    pub rating: u8,
    pub comment: String,
}

pub struct AppMarketplace {
    pub apps: Vec<AppManifest>,
    pub reviews: Vec<AppReview>,
    pub installed: Vec<u64>,
    pub next_id: u64,
}

impl AppMarketplace {
    pub fn new() -> Self {
        Self {
            apps: Vec::new(),
            reviews: Vec::new(),
            installed: Vec::new(),
            next_id: 1,
        }
    }

    pub fn publish(&mut self, name: &str, desc: &str, app_type: AppType, author: &str) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.apps.push(AppManifest {
            id,
            name: String::from(name),
            description: String::from(desc),
            app_type,
            version: String::from("1.0.0"),
            author: String::from(author),
            size_bytes: 0,
            capabilities_required: Vec::new(),
            trust_score: 0.5,
            downloads: 0,
            status: AppStatus::Available,
        };
        id
    }

    pub fn install(&mut self, app_id: u64) -> bool {
        if let Some(app) = self.apps.iter_mut().find(|a| a.id == app_id) {
            app.status = AppStatus::Installed;
            app.downloads += 1;
            self.installed.push(app_id);
            true
        } else {
            false
        }
    }

    pub fn search(&self, query: &str) -> Vec<&AppManifest> {
        let q = query.to_lowercase();
        self.apps.iter().filter(|a| a.name.to_lowercase().contains(&q) || a.description.to_lowercase().contains(&q)).collect()
    }

    pub fn top_rated(&self) -> Vec<&AppManifest> {
        let mut sorted: Vec<&AppManifest> = self.apps.iter().collect();
        sorted.sort_by(|a, b| b.trust_score.partial_cmp(&a.trust_score).unwrap_or(core::cmp::Ordering::Equal));
        sorted
    }
)}
