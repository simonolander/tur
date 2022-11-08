use crate::level::Level;
use crate::level_dto::LevelDto;

fn sandbox() -> Level {
    serde_yaml::from_str::<LevelDto>(include_str!("../res/level/sandbox.yaml"))
        .unwrap()
        .into()
}

fn night_time() -> Level {
    serde_yaml::from_str::<LevelDto>(include_str!("../res/level/night_time.yaml"))
        .unwrap()
        .into()
}

fn moth() -> Level {
    serde_yaml::from_str::<LevelDto>(include_str!("../res/level/the_moth.yaml"))
        .unwrap()
        .into()
}
