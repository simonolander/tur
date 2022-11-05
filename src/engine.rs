use std::collections::HashSet;

use Direction::{Left, Right};

use crate::level::Level;
use crate::program::{Card, Direction, Program};

pub struct Engine {
    positions_on: HashSet<i64>,
    current_card_index: Option<usize>,
    current_position: i64,
    program: Program,
    steps: u64,
}

impl Engine {
    pub fn new(level: &Level, program: Program) -> Engine {
        let current_card = Some(program.initial_card.clone());
        let positions_on = level.initial_state.clone();
        let current_position = 0;
        Engine {
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

    pub fn is_terminated(&self) -> bool {
        self.current_card_index.is_none()
    }

    pub fn get_current_position(&self) -> i64 {
        return self.current_position;
    }

    pub fn get_tape_at(&self, position: i64) -> bool {
        self.positions_on.contains(&position)
    }
}

mod test {
    use crate::engine::Engine;
    use crate::level::sandbox;
    use crate::program::Direction::Right;
    use crate::program::{Card, Instruction, Program};
    use std::rc::Rc;

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
        let mut engine = Engine::new(&level, program);
        assert!(!engine.is_terminated());
        engine.step();
        assert!(engine.is_terminated());
    }
}
