use std::collections::HashSet;

use Direction::{Left, Right};

use crate::level::Level;
use crate::program::{Direction, Program};

pub struct LevelExecution {
    level: Level,
    program: Program,
    case_executions: Vec<TestCaseExecution>,
}

impl LevelExecution {
    fn new(level: Level, program: Program) -> LevelExecution {
        LevelExecution {
            level,
            program,
            case_executions: Vec::new(),
        }
    }

    fn step(&mut self) {
        if self.is_terminated() {
            return;
        }
    }

    pub fn is_terminated(&self) -> bool {
        self.case_executions.len() == self.level.cases.len()
            && self
                .case_executions
                .last()
                .map(TestCaseExecution::is_terminated)
                .unwrap_or(true)
    }
}

pub struct TestCaseExecution {
    positions_on: HashSet<i64>,
    current_card_index: Option<usize>,
    current_position: i64,
    program: Program,
    steps: u64,
}

impl TestCaseExecution {
    pub fn new(level: &Level, program: Program) -> TestCaseExecution {
        let current_card = Some(program.initial_card);
        let positions_on = level.cases[0].initial_tape.clone();
        let current_position = 0;
        TestCaseExecution {
            current_card_index: current_card,
            positions_on,
            current_position,
            program,
            steps: 0,
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

mod test {
    use crate::execution::TestCaseExecution;
    use crate::level::sandbox;
    use crate::levels::night_time;
    use crate::program::Direction::Right;
    use crate::program::{Card, Instruction, Program};

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
        let mut engine = TestCaseExecution::new(&level, program);
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
        let mut engine = TestCaseExecution::new(&level, program);
        let terminated = engine.run(100);
        assert!(terminated);
    }
}
