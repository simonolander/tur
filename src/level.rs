use std::collections::HashSet;

#[derive(Clone)]
pub struct Level {
    pub name: String,
    pub description: String,
    pub cases: Vec<TestCase>,
}

#[derive(Clone, Default)]
pub struct TestCase {
    pub initial_tape: HashSet<i64>,
    pub target: Option<Target>,
}

#[derive(Clone, Eq, PartialEq)]
pub enum Target {
    TapeExact { tape: HashSet<i64> },
    Position { position: i64 },
}