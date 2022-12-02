use serde::{Deserialize, Serialize};

use TargetDto::{Position, TapeExact};

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
        let cases = if self.cases.is_empty() {
            vec![TestCase::default()]
        } else {
            self.cases
                .iter()
                .map(|tc: &TestCaseDto| TestCase {
                    initial_tape: tc.initial_tape.iter().copied().collect(),
                    target: tc.target.clone().or(self.target.clone()).map(|target| Target::from(&target)),
                })
                .collect()
        };
        Level {
            name: self.name,
            description: self.description,
            cases,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct TestCaseDto {
    #[serde(default)]
    initial_tape: Vec<i64>,
    #[serde(default)]
    target: Option<TargetDto>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(tag = "type")]
enum TargetDto {
    TapeExact { tape: Vec<i64> },
    Position { position: i64 },
}

impl From<&TargetDto> for Target {
    fn from(dto: &TargetDto) -> Self {
        match dto {
            TapeExact { tape } => Target::TapeExact {
                tape: tape.iter().copied().collect(),
            },
            Position { position } => Target::Position {
                position: *position,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::level_dto::{LevelDto, TestCaseDto};
    use crate::level_dto::TargetDto::{Position, TapeExact};

    #[test]
    fn deserialize_sandbox() {
        let expected = LevelDto {
            name: "sandbox".to_string(),
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
            name: "night_time".to_string(),
            description: "At some point >= 0, there's a light on. Turn it off.".to_string(),
            cases: vec![
                TestCaseDto {
                    initial_tape: vec![3],
                    target: None,
                },
                TestCaseDto {
                    initial_tape: vec![8],
                    target: None,
                },
                TestCaseDto {
                    initial_tape: vec![0],
                    target: None,
                },
            ],
            target: Some(TapeExact { tape: Vec::new() }),
        };
        let string = include_str!("../res/level/night_time.yaml");
        let actual: LevelDto = serde_yaml::from_str(string).unwrap();
        assert_eq!(expected, actual)
    }

    #[test]
    fn deserialize_the_moth() {
        let expected = LevelDto {
            name: "moth".to_string(),
            description: "At some position there is a light on. Halt the program on that position."
                .to_string(),
            cases: vec![
                TestCaseDto {
                    initial_tape: vec![18],
                    target: Some(Position { position: 18 }),
                },
                TestCaseDto {
                    initial_tape: vec![-13],
                    target: Some(Position { position: -13 }),
                },
                TestCaseDto {
                    initial_tape: vec![0],
                    target: Some(Position { position: 0 }),
                },
            ],
            target: None,
        };
        let string = include_str!("../res/level/moth.yaml");
        let actual: LevelDto = serde_yaml::from_str(string).unwrap();
        assert_eq!(expected, actual)
    }
}
