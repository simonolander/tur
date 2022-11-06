use std::collections::HashSet;

#[derive(Clone)]
pub struct Level {
    pub name: String,
    pub description: String,
    pub cases: Vec<TestCase>,
}

#[derive(Clone)]
pub struct TestCase {
    pub initial_tape: HashSet<i64>,
    pub target: Option<Target>,
}

#[derive(Clone)]
pub enum Target {
    TapeExact { tape: HashSet<i64> },
}

pub fn sandbox() -> Level {
    Level {
        name: "Sandbox".to_string(),
        description: "".to_string(),
        cases: vec![TestCase {
            initial_tape: HashSet::new(),
            target: None,
        }],
    }
}
