extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub type AgentId = u64;

pub enum ContentType {
    Document,
    Terminal,
    Media,
    Notification,
    Dashboard,
    Chat,
}

pub struct Surface {
    pub id: u64,
    pub title: String,
    pub content_type: ContentType,
    pub relevance: f32,
    pub source_agent: AgentId,
    pub visible: bool,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

pub struct AttentionManager {
    pub surfaces: Vec<Surface>,
    pub current_task: Option<String>,
    pub next_id: u64,
}

impl AttentionManager {
    pub fn new() -> Self {
        Self { surfaces: Vec::new(), current_task: None, next_id: 1 }
    }

    pub fn create_surface(&mut self, title: &str, ct: ContentType, agent: AgentId, rel: f32) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.surfaces.push(Surface {
            id, title: String::from(title), content_type: ct,
            relevance: rel, source_agent: agent, visible: rel > 0.5,
            x: 0, y: 0, w: 800, h: 600,
        });
        id
    }

    pub fn set_task(&mut self, task: &str) {
        self.current_task = Some(String::from(task));
        let task_lower = task.to_lowercase();
        for s in &mut self.surfaces {
            if s.title.to_lowercase().contains(&task_lower) {
                s.relevance += 0.3;
                s.visible = true;
            } else {
                s.relevance -= 0.1;
                if s.relevance < 0.3 { s.visible = false; }
            }
        }
    }

    pub fn focus(&mut self, id: u64) {
        if let Some(s) = self.surfaces.iter_mut().find(|s| s.id == id) {
            s.visible = true;
            s.relevance = 1.0;
        }
    }

    pub fn visible_surfaces(&self) -> Vec<&Surface> {
        self.surfaces.iter().filter(|s| s.visible).collect()
    }

    pub fn auto_layout(&mut self, sw: u32, sh: u32) {
        let ids: Vec<u64> = self.surfaces.iter().filter(|s| s.visible).map(|s| s.id).collect();
        let n = ids.len() as u32;
        if n == 0 { return; }
        let cols = if n <= 1 { 1 } else if n <= 4 { 2 } else { 3 };
        let rows = (n + cols - 1) / cols;
        let cw = sw / cols;
        let ch = sh / rows;
        for (i, id) in ids.iter().enumerate() {
            if let Some(s) = self.surfaces.iter_mut().find(|s| s.id == *id) {
                s.x = (i as u32 % cols) * cw;
                s.y = (i as u32 / cols) * ch;
                s.w = cw;
                s.h = ch;
            }
        }
    }
}
