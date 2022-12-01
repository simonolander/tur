#[derive(Clone)]
pub struct Program {
    pub name: String,
    pub initial_card: usize,
    pub cards: Vec<Card>,
}

#[derive(Clone)]
pub struct Card {
    pub name: String,
    pub tape_on: Instruction,
    pub tape_off: Instruction,
}

impl Card {
    pub fn terminate() -> Card {
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

    pub fn light_right() -> Card {
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

    pub fn light_left() -> Card {
        Card {
            name: "Light left".to_string(),
            tape_on: Instruction {
                write_symbol: true,
                move_direction: Direction::Left,
                next_card: Some(0),
            },
            tape_off: Instruction {
                write_symbol: true,
                move_direction: Direction::Left,
                next_card: Some(0),
            },
        }
    }

    pub fn go_right() -> Card {
        Card {
            name: "Go right".to_string(),
            tape_on: Instruction {
                write_symbol: true,
                move_direction: Direction::Right,
                next_card: Some(0),
            },
            tape_off: Instruction {
                write_symbol: false,
                move_direction: Direction::Right,
                next_card: Some(0),
            },
        }
    }

    pub fn go_left() -> Card {
        Card {
            name: "Go left".to_string(),
            tape_on: Instruction {
                write_symbol: true,
                move_direction: Direction::Left,
                next_card: Some(0),
            },
            tape_off: Instruction {
                write_symbol: false,
                move_direction: Direction::Left,
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
