use serde::{Deserialize, Serialize};

use TargetDto::TapeExact;

use crate::level::{Level, Target, TestCase};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct LevelDto {
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    cases: Vec<TestCaseDto>,
    #[serde(default)]
    target: Option<TargetDto>,
}

impl Into<Level> for LevelDto {
    fn into(self) -> Level {
        Level {
            name: self.name,
            description: self.description,
            cases: self
                .cases
                .iter()
                .map(|tc| TestCase {
                    initial_tape: tc.initial_tape.iter().copied().collect(),
                    target: self.target.as_ref().map(|t| t.into()),
                })
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct TestCaseDto {
    #[serde(default)]
    initial_tape: Vec<i64>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(tag = "type")]
enum TargetDto {
    TapeExact { tape: Vec<i64> },
}

impl From<&TargetDto> for Target {
    fn from(dto: &TargetDto) -> Self {
        match dto {
            TapeExact { tape } => Target::TapeExact {
                tape: tape.iter().copied().collect(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::level_dto::{LevelDto, TestCaseDto};
    use crate::level_dto::TargetDto::TapeExact;

    #[test]
    fn deserialize_sandbox() {
        let expected = LevelDto {
            name: "Sandbox".to_string(),
            description: "".to_string(),
            cases: Vec::new(),
            target: None,
        };
        let string = include_str!("../res/level/sandbox.yaml");
        let actual: LevelDto = serde_yaml::from_str(string).unwrap();
        assert_eq!(expected, actual)
    }

    #[test]
    fn deserialize_night_time() {
        let expected = LevelDto {
            name: "Night time".to_string(),
            description: "At some point >= 0, there's a light on. Turn it off.".to_string(),
            cases: vec![
                TestCaseDto {
                    initial_tape: vec![3],
                },
                TestCaseDto {
                    initial_tape: vec![8],
                },
                TestCaseDto {
                    initial_tape: vec![0],
                },
            ],
            target: Some(TapeExact { tape: Vec::new() }),
        };
        let string = include_str!("../res/level/night_time.yaml");
        let actual: LevelDto = serde_yaml::from_str(string).unwrap();
        assert_eq!(expected, actual)
    }
}
