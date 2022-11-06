use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct LevelDto {
    name: String,
}

#[cfg(test)]
mod tests {
    use crate::level_dto::LevelDto;

    #[test]
    fn deserialize_sandbox() {
        let expected = LevelDto {
            name: "Sandbox".to_string(),
        };
        let string = include_str!("../res/level/sandbox.yaml");
        let actual: LevelDto = serde_yaml::from_str(string).unwrap();
        assert_eq!(expected, actual)
    }
}
