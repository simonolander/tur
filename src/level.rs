use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct Level {
    pub name: String,
    pub initial_state: HashSet<i64>,
    pub target_state: HashMap<i64, bool>,
}

pub fn sandbox() -> Level {
    Level {
        name: "Sandbox".to_string(),
        initial_state: HashSet::new(),
        target_state: HashMap::new(),
    }
}
