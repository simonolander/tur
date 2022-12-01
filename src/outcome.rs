use std::collections::HashSet;
use std::fmt::{Display, Formatter, write};

use crate::execution::{LevelExecution, TestCaseExecution};
use crate::level::{Level, TestCase};
use crate::program::Program;

pub struct Outcome {
    pub level: Level,
    pub program: Program,
    pub results: Vec<TestCaseResult>,
}

impl From<LevelExecution> for Outcome {
    fn from(le: LevelExecution) -> Self {
        Outcome {
            level: le.level,
            program: le.program,
            results: le.executions.into_iter().map(TestCaseResult::from).collect()
        }
    }
}

pub struct TestCaseResult {
    steps: u64,
    position: i64,
    tape: HashSet<i64>,
}

impl From<TestCaseExecution> for TestCaseResult {
    fn from(tce: TestCaseExecution) -> Self {
        TestCaseResult {
            steps: tce.steps,
            position: tce.current_position,
            tape: Default::default()
        }
    }
}