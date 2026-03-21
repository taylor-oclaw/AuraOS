extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct VirtualDesktop {
    pub id: u64,
    pub name: String,
    pub surfaces: Vec<u64>,
    pub wallpaper: Option<String>,
    pub active: bool,
}

pub struct DesktopManager {
    pub desktops: Vec<VirtualDesktop>,
    pub active_desktop: usize,
    pub next_id: u64,
}

impl DesktopManager {
    pub fn new() -> Self {
        let mut dm = Self {
            desktops: Vec::new(),
            active_desktop: 0,
            next_id: 1,
        };
        dm.create("Desktop 1");
        dm
    }

    pub fn create(&mut self, name: &str) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let is_first = self.desktops.is_empty();
        self.desktops.push(VirtualDesktop {
            id,
            name: String::from(name),
            surfaces: Vec::new(),
            wallpaper: None,
            active: is_first,
        });
        id
    }

    pub fn switch_to(&mut self, idx: usize) {
        if idx < self.desktops.len() {
            for d in &mut self.desktops {
                d.active = false;
            }
            self.desktops[idx].active = true;
            self.active_desktop = idx;
        }
    }

    pub fn next(&mut self) {
        let next = (self.active_desktop + 1) % self.desktops.len();
        self.switch_to(next);
    }

    pub fn prev(&mut self) {
        let prev = if self.active_desktop == 0 {
            self.desktops.len() - 1
        } else {
            self.active_desktop - 1
        };
        self.switch_to(prev);
    }

    pub fn add_surface(&mut self, surface_id: u64) {
        if let Some(d) = self.desktops.get_mut(self.active_desktop) {
            d.surfaces.push(surface_id);
        }
    }

    pub fn move_surface(&mut self, surface_id: u64, to_desktop: usize) {
        for d in &mut self.desktops {
            d.surfaces.retain(|s| *s != surface_id);
        }
        if let Some(d) = self.desktops.get_mut(to_desktop) {
            d.surfaces.push(surface_id);
        }
    }

    pub fn count(&self) -> usize {
        self.desktops.len()
    }
}
