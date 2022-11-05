use std::rc::{Rc, Weak};

#[derive(Clone)]
pub struct Program {
    pub name: String,
    pub initial_card: usize,
    pub cards: Vec<Card>,
}

impl Program {
    pub fn just_stop() -> Program {
        let card = Card::terminate();
        Program {
            name: "Just stop".to_string(),
            initial_card: 0,
            cards: vec![card],
        }
    }

    pub fn light_to_the_right() -> Program {
        let card = Card::light_right();
        Program {
            name: "Light to the right".to_string(),
            initial_card: 0,
            cards: vec![card],
        }
    }

    pub fn go_right() -> Program {
        let card = Card::go_right();
        Program {
            name: "Go right".to_string(),
            initial_card: 0,
            cards: vec![card],
        }
    }

    pub fn light_the_world() -> Program {
        let left = Card {
            name: "LEFT".to_string(),
            tape_on: Instruction {
                write_symbol: true,
                move_direction: Direction::Left,
                next_card: Some(0),
            },
            tape_off: Instruction {
                write_symbol: true,
                move_direction: Direction::Right,
                next_card: Some(1),
            },
        };
        let right = Card {
            name: "RIGHT".to_string(),
            tape_on: Instruction {
                write_symbol: true,
                move_direction: Direction::Right,
                next_card: Some(1),
            },
            tape_off: Instruction {
                write_symbol: true,
                move_direction: Direction::Left,
                next_card: Some(0),
            },
        };
        Program {
            name: "Light the world".to_string(),
            initial_card: 0,
            cards: vec![left, right],
        }
    }
}

#[derive(Clone)]
pub struct Card {
    pub name: String,
    pub tape_on: Instruction,
    pub tape_off: Instruction,
}

impl Card {
    fn terminate() -> Card {
        Card {
            name: "Terminate".to_string(),
            tape_on: Instruction {
                write_symbol: true,
                move_direction: Direction::Left,
                next_card: None,
            },
            tape_off: Instruction {
                write_symbol: false,
                move_direction: Direction::Left,
                next_card: None,
            },
        }
    }

    fn light_right() -> Card {
        Card {
            name: "Light right".to_string(),
            tape_on: Instruction {
                write_symbol: true,
                move_direction: Direction::Left,
                next_card: Some(0),
            },
            tape_off: Instruction {
                write_symbol: true,
                move_direction: Direction::Right,
                next_card: Some(0),
            },
        }
    }

    fn go_right() -> Card {
        Card {
            name: "Go right".to_string(),
            tape_on: Instruction {
                write_symbol: true,
                move_direction: Direction::Left,
                next_card: Some(0),
            },
            tape_off: Instruction {
                write_symbol: false,
                move_direction: Direction::Right,
                next_card: Some(0),
            },
        }
    }
}

#[derive(Clone)]
pub struct Instruction {
    pub write_symbol: bool,
    pub move_direction: Direction,
    pub next_card: Option<usize>,
}

#[derive(Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}
