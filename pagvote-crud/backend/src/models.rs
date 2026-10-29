use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Contestant {
    pub id: u64,
    pub name: String,
    pub contestant_number: u32,
    pub bio: String,
}

/// Input for both create and update. Kept as one shape on purpose:
/// the demo has no partial-update (PATCH) case, so PUT replaces all fields.
#[derive(Debug, Deserialize)]
pub struct ContestantInput {
    pub name: String,
    pub contestant_number: u32,
    pub bio: String,
}

impl ContestantInput {
    /// Returns the first validation problem, or None when the input is usable.
    /// Name must not be blank; contestant numbers start at 1.
    /// Bio may be empty — a contestant is not required to have one yet.
    pub fn validate(&self) -> Option<String> {
        if self.name.trim().is_empty() {
            return Some("name must not be empty".to_string());
        }
        if self.contestant_number == 0 {
            return Some("contestant_number must be at least 1".to_string());
        }
        None
    }

    pub fn cleaned(&self) -> (String, String) {
        (self.name.trim().to_string(), self.bio.trim().to_string())
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl ErrorResponse {
    pub fn new(message: &str) -> Self {
        Self {
            error: message.to_string(),
        }
    }
}
