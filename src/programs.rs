use crate::program::{Card, Direction, Instruction, Program};

pub fn builtins() -> Vec<Program> {
    vec![
        just_stop(),
        light_right(),
        light_left(),
        go_right(),
        light_the_world(),
    ]
}

fn just_stop() -> Program {
    let card = Card::terminate();
    Program {
        name: "Just stop".to_string(),
        initial_card: 0,
        cards: vec![card],
    }
}

fn light_right() -> Program {
    let card = Card::light_right();
    Program {
        name: "Light to the right".to_string(),
        initial_card: 0,
        cards: vec![card],
    }
}

fn light_left() -> Program {
    let card = Card::light_left();
    Program {
        name: "Light to the left".to_string(),
        initial_card: 0,
        cards: vec![card],
    }
}

fn go_right() -> Program {
    let card = Card::go_right();
    Program {
        name: "Go right".to_string(),
        initial_card: 0,
        cards: vec![card],
    }
}

fn light_the_world() -> Program {
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