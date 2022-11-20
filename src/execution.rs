use std::collections::HashSet;

use Direction::{Left, Right};

use crate::level::Level;
use crate::program::{Direction, Program};

pub struct LevelExecution {
    pub level: Level,
    pub program: Program,
    pub executions: Vec<TestCaseExecution>,
}

impl LevelExecution {
    pub fn new(level: Level, program: Program) -> LevelExecution {
        let executions = level.cases.iter().map(|tc| TestCaseExecution::new(tc.initial_tape.clone(), program.clone())).collect();
        LevelExecution {
            level,
            program,
            executions,
        }
    }

    pub fn step(&mut self) {
        if self.is_terminated() {
            return;
        }
        if let Some(ex) = self.current_execution() {
            ex.step()
        }
    }

    pub fn current_execution(&mut self) -> Option<&mut TestCaseExecution> {
        self.executions.iter_mut().find(|e| !e.is_terminated())
    }

    pub fn is_terminated(&self) -> bool {
        self.executions
            .last()
            .map(TestCaseExecution::is_terminated)
            .unwrap_or(true)
    }
}

pub struct TestCaseExecution {
    positions_on: HashSet<i64>,
    current_card_index: Option<usize>,
    current_position: i64,
    steps: u64,
    program: Program,
}

impl TestCaseExecution {
    pub fn new(positions_on: HashSet<i64>, program: Program) -> TestCaseExecution {
        TestCaseExecution {
            positions_on,
            current_card_index: Some(0),
            current_position: 0,
            steps: 0,
            program,
        }
    }

    pub fn step(&mut self) {
        if let Some(index) = self.current_card_index {
            let card = self.program.cards.get(index).unwrap();
            let current_position = &self.current_position;
            let on = self.positions_on.contains(current_position);
            let instruction = if on { &card.tape_on } else { &card.tape_off };
            if instruction.write_symbol {
                self.positions_on.insert(*current_position);
            } else {
                self.positions_on.remove(current_position);
            }
            match instruction.move_direction {
                Left => self.current_position -= 1,
                Right => self.current_position += 1,
            }
            self.current_card_index = instruction.next_card;
            self.steps += 1;
        }
    }

    pub fn run(&mut self, max_steps: u64) -> bool {
        for _n in 0..max_steps {
            self.step();
            if self.is_terminated() {
                return true;
            }
        }
        false
    }

    pub fn is_terminated(&self) -> bool {
        self.current_card_index.is_none()
    }

    pub fn get_current_position(&self) -> i64 {
        self.current_position
    }

    pub fn get_tape_at(&self, position: i64) -> bool {
        self.positions_on.contains(&position)
    }
}

#[cfg(test)]
mod tests {
    use crate::execution::TestCaseExecution;
    use crate::levels::{night_time, sandbox};
    use crate::program::{Card, Instruction, Program};
    use crate::program::Direction::Right;

    #[test]
    fn test_just_stop() {
        let level = sandbox();
        let terminate = Instruction {
            write_symbol: false,
            move_direction: Right,
            next_card: None,
        };
        let card = Card {
            name: "Terminate".into(),
            tape_on: terminate.clone(),
            tape_off: terminate,
        };
        let program = Program {
            name: String::from("It stops"),
            initial_card: 0,
            cards: vec![card],
        };
        let mut engine = TestCaseExecution::new(level.cases[0].initial_tape.clone(), program);
        assert!(!engine.is_terminated());
        engine.step();
        assert!(engine.is_terminated());
    }

    #[test]
    fn night_time_solve() {
        let level = night_time();
        let program = Program {
            name: "".to_string(),
            initial_card: 0,
            cards: vec![Card {
                name: "".to_string(),
                tape_on: Instruction {
                    write_symbol: false,
                    move_direction: Right,
                    next_card: None,
                },
                tape_off: Instruction {
                    write_symbol: false,
                    move_direction: Right,
                    next_card: Some(0),
                },
            }],
        };
        let mut engine = TestCaseExecution::new(level.cases[0].initial_tape.clone(), program);
        let terminated = engine.run(100);
        assert!(terminated);
    }
}
