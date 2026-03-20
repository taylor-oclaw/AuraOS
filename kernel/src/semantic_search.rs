extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SearchDocument {
    pub id: u64,
    pub content: String,
    pub source: String,
    pub embedding: Vec<f32>,
    pub score: f32,
    pub agent_id: u64
}

pub struct SearchIndex {
    pub documents: Vec<SearchDocument>,
    pub next_id: u64
}

impl SearchIndex {
    pub fn new() -> Self {
        Self { 
            documents: Vec::new(), 
            next_id: 1 
        }
    }

    pub fn index(&mut self, content: &str, source: &str, agent_id: u64) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let embedding = Self::simple_embed(content);
        self.documents.push(SearchDocument { 
            id, 
            content: String::from(content), 
            source: String::from(source), 
            embedding, 
            score: 0.0, 
            agent_id 
        });
        id
    }

    fn simple_embed(text: &str) -> Vec<f32> {
        let mut emb = Vec::new();
        let bytes = text.as_bytes();
        for i in 0..32 {
            let val = if i < bytes.len() { 
                bytes[i] as f32 / 255.0 
            } else { 
                0.0 
            };
            emb.push(val);
        }
        emb
    }

    fn sqrt_approx(x: f32) -> f32 { if x <= 0.0 { return 0.0; } let mut guess = x / 2.0; for _ in 0..10 { guess = (guess + x / guess) / 2.0; } guess }
    fn cosine_sim(a: &[f32], b: &[f32]) -> f32 {
        let mut dot = 0.0f32;
        let mut na = 0.0f32;
        let mut nb = 0.0f32;
        for i in 0..a.len().min(b.len()) {
            dot += a[i] * b[i];
            na += a[i] * a[i];
            nb += b[i] * b[i];
        }
        if na == 0.0 || nb == 0.0 { 
            0.0 
        } else { 
            dot / (Self::sqrt_approx(na) * Self::sqrt_approx(nb)) 
        }
    }

    pub fn search(&self, query: &str, top_k: usize) -> Vec<(u64, f32)> {
        let qemb = Self::simple_embed(query);
        let mut scores: Vec<(u64, f32)> = self.documents.iter()
            .map(|d| (d.id, Self::cosine_sim(&qemb, &d.embedding)))
            .collect();
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(core::cmp::Ordering::Equal));
        scores.truncate(top_k);
        scores
    }

    pub fn get(&self, id: u64) -> Option<&SearchDocument> {
        self.documents.iter().find(|d| d.id == id)
    }

    pub fn count(&self) -> usize {
        self.documents.len()
    }
}
