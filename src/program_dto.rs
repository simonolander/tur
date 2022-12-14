use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use serde::de::Error;

use crate::program::{Card, Direction, Instruction, Program};
use crate::program::Direction::{Left, Right};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ProgramDto {
    name: String,
    #[serde(default)]
    description: String,
    initial_card: String,
    #[serde(default)]
    cards: Vec<CardDto>,
}

impl From<Program> for ProgramDto {
    fn from(program: Program) -> Self {
        let card_names: Vec<String> = program.cards.iter().map(|card| card.name.clone()).collect();
        let initial_card = card_names[program.initial_card].clone();
        let cards: Vec<CardDto> = program.cards.into_iter().map(CardDto::from_card(&card_names)).collect();
        ProgramDto {
            name: program.name,
            description: program.description,
            initial_card,
            cards,
        }
    }
}

impl TryFrom<ProgramDto> for Program {
    type Error = serde_yaml::Error;

    fn try_from(dto: ProgramDto) -> Result<Self, Self::Error> {
        let name = if dto.name.is_empty() {
            return Err(Error::custom("Name cannot be empty"));
        } else {
            dto.name
        };

        let description = dto.description;

        let duplicate_card_names = retain_duplicates(dto.cards.iter().map(|card| card.name.clone()).collect());
        if !duplicate_card_names.is_empty() {
            let message = duplicate_card_names.into_iter().collect::<Vec<String>>().join(", ");
            return Err(Error::custom(format!("Duplicate card names: {}", message)));
        }

        let card_name_map: HashMap<String, usize> = dto.cards.iter()
            .enumerate().map(|(index, card)| (card.name.clone(), index))
            .collect();

        let &initial_card = card_name_map.get(&dto.initial_card).ok_or::<Self::Error>(Error::custom(format!("Initial card not found: {}", dto.initial_card)))?;

        let cards = dto.cards.iter().map(|card| card.try_into(&card_name_map)).collect::<Result<_, _>>()?;

        Ok(Program {
            name,
            description,
            initial_card,
            cards,
        })
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct CardDto {
    name: String,
    instruction_on: InstructionDto,
    instruction_off: InstructionDto,
}

impl CardDto {
    fn try_into(&self, card_name_map: &HashMap<String, usize>) -> Result<Card, serde_yaml::Error> {
        let card = Card {
            name: self.name.clone(),
            tape_on: self.instruction_on.try_into_with_map(card_name_map)?,
            tape_off: self.instruction_off.try_into_with_map(card_name_map)?,
        };
        Ok(card)
    }

    fn from_card<'a>(card_names: &'a[String]) -> impl Fn(Card) -> CardDto + 'a {
        |card| {
            CardDto {
                name: card.name,
                instruction_on: InstructionDto {
                    write_symbol: card.tape_on.write_symbol,
                    move_direction: card.tape_on.move_direction.map(DirectionDto::from),
                    next_card: card.tape_on.next_card.map(|index| card_names[index].clone()),
                },
                instruction_off: InstructionDto {
                    write_symbol: card.tape_off.write_symbol,
                    move_direction: card.tape_off.move_direction.map(DirectionDto::from),
                    next_card: card.tape_off.next_card.map(|index| card_names[index].clone()),
                },
            }
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct InstructionDto {
    pub write_symbol: Option<bool>,
    pub move_direction: Option<DirectionDto>,
    pub next_card: Option<String>,
}

impl InstructionDto {
    pub fn try_into_with_map(&self, card_name_map: &HashMap<String, usize>) -> Result<Instruction, serde_yaml::Error> {
        let next_card = if let Some(name) = &self.next_card {
            Some(*card_name_map.get(name).ok_or::<serde_yaml::Error>(Error::custom(format!("Card does not exist: {}", &name)))?)
        } else {
            None
        };
        let instruction = Instruction {
            write_symbol: self.write_symbol,
            move_direction: self.move_direction.map(|dir| dir.into()),
            next_card,
        };
        Ok(instruction)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Copy, Clone)]
pub enum DirectionDto {
    Left,
    Right,
}

impl From<DirectionDto> for Direction {
    fn from(direction: DirectionDto) -> Self {
        match direction {
            DirectionDto::Left => Left,
            DirectionDto::Right => Right,
        }
    }
}

impl From<Direction> for DirectionDto {
    fn from(direction: Direction) -> Self {
        match direction {
            Left => DirectionDto::Left,
            Right => DirectionDto::Right,
        }
    }
}

fn retain_duplicates(items: Vec<String>) -> HashSet<String> {
    let mut set = HashSet::new();
    items.into_iter().filter(|item| !set.insert(item.clone())).collect()
}

#[cfg(test)]
mod tests {}
