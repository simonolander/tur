use std::collections::{HashMap, HashSet};
use std::hash::Hash;

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

impl TryFrom<ProgramDto> for Program {
    type Error = serde_yaml::Error;

    fn try_from(value: ProgramDto) -> Result<Self, Self::Error> {
        let name = if value.name.is_empty() {
            return Err(Error::custom("Name cannot be empty"));
        } else {
            value.name
        };

        let duplicate_card_names = retain_duplicates(value.cards.iter().map(|card| card.name.clone()).collect());
        if !duplicate_card_names.is_empty() {
            let message = duplicate_card_names.into_iter().collect::<Vec<String>>().join(", ");
            return Err(Error::custom(format!("Duplicate card names: {}", message)));
        }

        let card_name_map: HashMap<String, usize> = value.cards.iter()
            .enumerate().map(|(index, card)| (card.name.clone(), index))
            .collect();

        let &initial_card = card_name_map.get(&value.initial_card).ok_or::<Self::Error>(Error::custom(format!("Initial card not found: {}", value.initial_card)))?;

        let cards = value.cards.iter().map(|card| card.try_into(&card_name_map)).collect::<Result<_, _>>()?;

        Ok(Program {
            name,
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
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct InstructionDto {
    pub write_symbol: bool,
    pub move_direction: DirectionDto,
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
            move_direction: self.move_direction.into(),
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

fn retain_duplicates(items: Vec<String>) -> HashSet<String> {
    let mut set = HashSet::new();
    items.into_iter().filter(|item| !set.insert(item.clone())).collect()
}

#[cfg(test)]
mod tests {}
