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
        name: "just_stop".to_string(),
        initial_card: 0,
        cards: vec![card],
    }
}

fn light_right() -> Program {
    let card = Card::light_right();
    Program {
        name: "light_right".to_string(),
        initial_card: 0,
        cards: vec![card],
    }
}

fn light_left() -> Program {
    let card = Card::light_left();
    Program {
        name: "light_left".to_string(),
        initial_card: 0,
        cards: vec![card],
    }
}

fn go_right() -> Program {
    let card = Card::go_right();
    Program {
        name: "go_right".to_string(),
        initial_card: 0,
        cards: vec![card],
    }
}

fn light_the_world() -> Program {
    let left = Card {
        name: "LEFT".to_string(),
        tape_on: Instruction {
            write_symbol: None,
            move_direction: Some(Direction::Left),
            next_card: Some(0),
        },
        tape_off: Instruction {
            write_symbol: Some(true),
            move_direction: Some(Direction::Right),
            next_card: Some(1),
        },
    };
    let right = Card {
        name: "RIGHT".to_string(),
        tape_on: Instruction {
            write_symbol: None,
            move_direction: Some(Direction::Right),
            next_card: Some(1),
        },
        tape_off: Instruction {
            write_symbol: Some(true),
            move_direction: Some(Direction::Left),
            next_card: Some(0),
        },
    };
    Program {
        name: "light_the_world".to_string(),
        initial_card: 0,
        cards: vec![left, right],
    }
}