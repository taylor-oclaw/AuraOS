extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut cache = PredictAppPreloadCache::new();
    cache.add_app("app1");
    cache.add_app("app2");
    cache.remove_app("app1");
    if cache.contains_app("app2") {
        println!("App2 is in the cache");
    }
    let apps = cache.get_all_apps();
    for app in apps.iter() {
        println!("{}", app);
    }
}

pub struct PredictAppPreloadCache {
    apps: Vec<String>,
}

impl PredictAppPreloadCache {
    pub fn new() -> Self {
        PredictAppPreloadCache { apps: Vec::new() }
    }

    pub fn add_app(&mut self, app_name: &str) {
        if !self.contains_app(app_name) {
            self.apps.push(String::from(app_name));
        }
    }

    pub fn remove_app(&mut self, app_name: &str) {
        self.apps.retain(|app| app != app_name);
    }

    pub fn contains_app(&self, app_name: &str) -> bool {
        self.apps.iter().any(|app| app == app_name)
    }

    pub fn get_all_apps(&self) -> Vec<String> {
        self.apps.clone()
    }

    pub fn clear_cache(&mut self) {
        self.apps.clear();
    }
}
