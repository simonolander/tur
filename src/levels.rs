use std::collections::HashSet;

use crate::level::{Level, Target, TestCase};
use crate::level::Target::Position;
use crate::level_dto::LevelDto;

fn sandbox() -> Level {
    serde_yaml::from_str::<LevelDto>(include_str!("../res/level/sandbox.yaml"))
        .unwrap()
        .into()
}

fn move_eight_right() -> Level {
    Level {
        name: "move8".to_string(),
        description: "Move eight steps to the right, and terminate the program".to_string(),
        cases: vec![
            TestCase {
                initial_tape: vec![8].into_iter().collect(),
                target: Some(Target::position(8)),
            },
            TestCase {
                initial_tape: Default::default(),
                target: Some(Target::position(8)),
            },
            TestCase {
                initial_tape: vec![2, 3, 5, 7, 11, 13].into_iter().collect(),
                target: Some(Target::position(8)),
            },
        ],
    }
}

fn night_time() -> Level {
    serde_yaml::from_str::<LevelDto>(include_str!("../res/level/night_time.yaml"))
        .unwrap()
        .into()
}

fn moth() -> Level {
    serde_yaml::from_str::<LevelDto>(include_str!("../res/level/moth.yaml"))
        .unwrap()
        .into()
}

pub fn builtins() -> Vec<Level> {
    vec![
        sandbox(),
        move_eight_right(),
        night_time(),
        moth(),
    ]
}