//! AuraOS Intent Router
//! 
//! Replaces the traditional app launcher.
//! Users express what they WANT TO DO, not which app to open.
//! 
//! "I need to write a letter to Mom" → opens writing surface + contact lookup
//! "Show me my photos from last week" → opens gallery with date filter
//! "Play some jazz" → opens music surface with jazz playlist

/// An intent represents what the user wants to accomplish
#[derive(Debug, Clone)]
pub struct Intent {
    pub category: IntentCategory,
    pub action: String,
    pub entities: Vec<Entity>,
    pub confidence: f32,
    pub source: IntentSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntentCategory {
    Communication,  // Email, messaging, calls
    Creation,       // Writing, drawing, coding
    Consumption,    // Reading, watching, listening
    Organization,   // Calendar, tasks, files
    Navigation,     // Maps, directions
    Commerce,       // Shopping, payments
    Information,    // Search, lookup, weather
    System,         // Settings, updates, security
    Social,         // Social media, sharing
    Productivity,   // Spreadsheets, documents
    Entertainment,  // Games, media
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub kind: EntityKind,
    pub value: String,
    pub confidence: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityKind {
    Person,
    Place,
    Time,
    Date,
    File,
    App,
    Url,
    Number,
    Genre,
    Topic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntentSource {
    Voice,
    Keyboard,
    Gesture,
    Gaze,
    Predicted,   // AI predicted this intent from context
}

/// Route an intent to the appropriate surface(s)
pub fn route_intent(intent: &Intent) -> Vec<SurfaceAction> {
    // TODO: Match intent to available surfaces
    // For now, return a basic mapping
    match intent.category {
        IntentCategory::Communication => vec![
            SurfaceAction::Open("messaging".into()),
        ],
        IntentCategory::Creation => vec![
            SurfaceAction::Open("editor".into()),
        ],
        IntentCategory::Information => vec![
            SurfaceAction::Open("search".into()),
        ],
        IntentCategory::System => vec![
            SurfaceAction::Open("settings".into()),
        ],
        _ => vec![
            SurfaceAction::ShowSearch,
        ],
    }
}

#[derive(Debug, Clone)]
pub enum SurfaceAction {
    Open(String),
    Focus(String),
    Close(String),
    ShowSearch,
    ShowCommandPalette,
}
