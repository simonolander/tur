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

impl From<LevelDto> for Level {
    fn from(dto: LevelDto) -> Self {
        let cases = if dto.cases.is_empty() {
            vec![TestCase::default()]
        } else {
            dto.cases
                .iter()
                .map(|tc: &TestCaseDto| TestCase {
                    initial_tape: tc.initial_tape.iter().copied().collect(),
                    target: tc.target.clone().or(dto.target.clone()).map(|target| Target::from(&target)),
                })
                .collect()
        };
        Level {
            name: dto.name,
            description: dto.description,
            cases,
        }
    }
}

impl From<Level> for LevelDto {
    fn from(level: Level) -> Self {
        let to_test_case_dto = |test_case: TestCase| {
            TestCaseDto {
                initial_tape: test_case.initial_tape.iter().copied().collect(),
                target: test_case.target.map(TargetDto::from),
            }
        };

        LevelDto {
            name: level.name,
            description: level.description,
            cases: level.cases.into_iter().map(to_test_case_dto).collect(),
            target: None,
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

impl From<Target> for TargetDto {
    fn from(target: Target) -> Self {
        match target {
            Target::TapeExact { tape } => TapeExact { tape: tape.iter().copied().collect() },
            Target::Position { position } => Position { position }
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
