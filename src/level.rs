use std::collections::HashSet;

use crate::level::Target::{Position, TapeExact};

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

impl Target {
    pub fn position(position: i64) -> Target {
        Position { position }
    }

    pub fn tape(tape: &[i64]) -> Target {
        TapeExact { tape: tape.iter().copied().collect() }
    }
}