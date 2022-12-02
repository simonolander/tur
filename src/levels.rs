use crate::level::Level;
use crate::level_dto::LevelDto;

pub(crate) fn sandbox() -> Level {
    serde_yaml::from_str::<LevelDto>(include_str!("../res/level/sandbox.yaml"))
        .unwrap()
        .into()
}

pub fn night_time() -> Level {
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
        night_time(),
        moth(),
    ]
}