use serde::{Deserialize, Serialize};

/// Only two statuses exist in this demo. Serde renames them to the exact
/// strings the frontend shows, so an unknown status is rejected by
/// deserialization before any handler runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContestantStatus {
    #[serde(rename = "In-Review")]
    InReview,
    #[serde(rename = "Approved")]
    Approved,
}

impl Default for ContestantStatus {
    fn default() -> Self {
        Self::InReview
    }
}

/// Naming decision, chosen 2026-10-29 after review:
/// Rust fields are snake_case (Rust convention), the JSON wire format
/// and all TypeScript code are camelCase (JS/TS convention).
/// `rename_all = "camelCase"` is the single place that maps between them,
/// so `contestant_number` in Rust becomes `contestantNumber` in JSON.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Contestant {
    pub id: u64,
    pub name: String,
    pub contestant_number: u32,
    pub bio: String,
    pub status: ContestantStatus,
}

/// Input for both create and update. Kept as one shape on purpose:
/// the demo has no partial-update (PATCH) case, so PUT replaces all fields.
/// `status` is optional on input only so older clients that omit it still
/// work — omitted means In-Review. The frontend always sends it.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContestantInput {
    pub name: String,
    pub contestant_number: u32,
    pub bio: String,
    pub status: Option<ContestantStatus>,
}

impl ContestantInput {
    /// Returns the first validation problem, or None when the input is usable.
    /// Name must not be blank and at most 100 characters; contestant numbers
    /// start at 1; bio at most 500 characters (empty is allowed — a
    /// contestant is not required to have one yet).
    pub fn validate(&self) -> Option<String> {
        let name = self.name.trim();
        if name.is_empty() {
            return Some("name must not be empty".to_string());
        }
        if name.chars().count() > 100 {
            return Some("name must be at most 100 characters".to_string());
        }
        if self.contestant_number == 0 {
            return Some("contestant_number must be at least 1".to_string());
        }
        if self.bio.chars().count() > 500 {
            return Some("bio must be at most 500 characters".to_string());
        }
        None
    }

    pub fn cleaned(&self) -> (String, String) {
        (self.name.trim().to_string(), self.bio.trim().to_string())
    }

    pub fn status_or_default(&self) -> ContestantStatus {
        self.status.unwrap_or_default()
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

#[cfg(test)]
mod tests {
    use super::*;

    fn input(name: &str, number: u32, bio: &str) -> ContestantInput {
        ContestantInput {
            name: name.to_string(),
            contestant_number: number,
            bio: bio.to_string(),
            status: None,
        }
    }

    #[test]
    fn valid_input_passes() {
        assert_eq!(input("Maya Reyes", 1, "Hi").validate(), None);
    }

    #[test]
    fn blank_name_fails() {
        assert_eq!(
            input("   ", 1, "").validate(),
            Some("name must not be empty".to_string())
        );
    }

    #[test]
    fn long_name_fails() {
        let long = "a".repeat(101);
        assert_eq!(
            input(&long, 1, "").validate(),
            Some("name must be at most 100 characters".to_string())
        );
    }

    #[test]
    fn zero_number_fails() {
        assert_eq!(
            input("Maya", 0, "").validate(),
            Some("contestant_number must be at least 1".to_string())
        );
    }

    #[test]
    fn long_bio_fails() {
        let long = "b".repeat(501);
        assert_eq!(
            input("Maya", 1, &long).validate(),
            Some("bio must be at most 500 characters".to_string())
        );
    }

    #[test]
    fn json_uses_camel_case() {
        let contestant = Contestant {
            id: 1,
            name: "Maya Reyes".to_string(),
            contestant_number: 1,
            bio: "".to_string(),
            status: ContestantStatus::InReview,
        };
        let json = serde_json::to_string(&contestant).unwrap();
        assert!(json.contains("contestantNumber"), "{json}");
        assert!(!json.contains("contestant_number"), "{json}");
    }
}
