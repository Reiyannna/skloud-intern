use crate::models::{Contestant, ContestantStatus};
use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::atomic::{AtomicU64, Ordering};

/// In-memory store, deliberately simple for the onboarding demo.
/// Data lives only while the server runs — restart clears it and
/// the three seed contestants below are loaded again.
/// A future step would swap this struct for a database layer
/// without changing the handlers' public behaviour.
pub struct AppState {
    contestants: RwLock<HashMap<u64, Contestant>>,
    next_id: AtomicU64,
}

impl AppState {
    pub fn new() -> Self {
        let state = Self {
            contestants: RwLock::new(HashMap::new()),
            // First real id is 1; ids 1-3 are taken by the seeds below.
            next_id: AtomicU64::new(4),
        };
        state.seed();
        state
    }

    fn seed(&self) {
        let seeds = [
            (
                1,
                "Maya Reyes",
                1,
                "Demo contestant — edit me.",
                ContestantStatus::InReview,
            ),
            (
                2,
                "Liam Torres",
                2,
                "Demo contestant — edit me.",
                ContestantStatus::Approved,
            ),
            (
                3,
                "Sofia Cruz",
                3,
                "Demo contestant — edit me.",
                ContestantStatus::InReview,
            ),
        ];
        let mut map = self.contestants.write().unwrap();
        for (id, name, number, bio, status) in seeds {
            map.insert(
                id,
                Contestant {
                    id,
                    name: name.to_string(),
                    contestant_number: number,
                    bio: bio.to_string(),
                    status,
                },
            );
        }
    }

    pub fn list(&self) -> Vec<Contestant> {
        let map = self.contestants.read().unwrap();
        let mut all: Vec<Contestant> = map.values().cloned().collect();
        // HashMap order is not stable, so sort by id for a stable list.
        all.sort_by_key(|c| c.id);
        all
    }

    pub fn get(&self, id: u64) -> Option<Contestant> {
        self.contestants.read().unwrap().get(&id).cloned()
    }

    /// True when another contestant already uses this number.
    /// `exclude_id` lets an update keep its own number without
    /// tripping the check on itself.
    pub fn number_in_use(&self, number: u32, exclude_id: Option<u64>) -> bool {
        self.contestants
            .read()
            .unwrap()
            .values()
            .any(|c| c.contestant_number == number && Some(c.id) != exclude_id)
    }

    pub fn create(
        &self,
        name: String,
        contestant_number: u32,
        bio: String,
        status: ContestantStatus,
    ) -> Contestant {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let contestant = Contestant {
            id,
            name,
            contestant_number,
            bio,
            status,
        };
        self.contestants
            .write()
            .unwrap()
            .insert(id, contestant.clone());
        contestant
    }

    pub fn update(
        &self,
        id: u64,
        name: String,
        contestant_number: u32,
        bio: String,
        status: ContestantStatus,
    ) -> Option<Contestant> {
        let mut map = self.contestants.write().unwrap();
        let contestant = map.get_mut(&id)?;
        contestant.name = name;
        contestant.contestant_number = contestant_number;
        contestant.bio = bio;
        contestant.status = status;
        Some(contestant.clone())
    }

    pub fn delete(&self, id: u64) -> bool {
        self.contestants.write().unwrap().remove(&id).is_some()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_in_use_ignores_self_on_update() {
        let state = AppState::new();
        // Seed 1 uses number 1, seed 2 uses number 2.
        assert!(state.number_in_use(1, None));
        assert!(state.number_in_use(2, None));
        assert!(!state.number_in_use(9, None));
        // Updating seed 1 to its own number must not count as a duplicate.
        assert!(!state.number_in_use(1, Some(1)));
        // Updating seed 1 to seed 2's number must count as a duplicate.
        assert!(state.number_in_use(2, Some(1)));
    }
}
