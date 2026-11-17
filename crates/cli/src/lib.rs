//! Разбор команд, которые игрок вводит в терминале.

use std::{error::Error, fmt::Display, num::ParseIntError};

use game::{Action, Direction, PlayerId};

use crate::CommandError::{
    Empty, ExtraArgument, MissingArgument, MissingVerb, UnknownDirection, UnknownVerb,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    pub who: PlayerId,
    pub action: Action,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandError {
    Empty,
    BadNumber(ParseIntError),
    MissingVerb,
    UnknownVerb(String),
    UnknownDirection(String),
    MissingArgument,
    ExtraArgument(String),
}

impl Error for CommandError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::BadNumber(err) => Some(err),
            Empty | MissingVerb | UnknownVerb(..) | UnknownDirection(..) | MissingArgument
            | ExtraArgument(..) => None,
        }
    }
}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadNumber(err) => write!(f, "неправильный номер игрока: {err}"),
            Self::Empty => write!(f, "пустая команда"),
            Self::UnknownVerb(verb) => write!(f, "неизвестное действие: {verb}"),
            Self::MissingArgument => write!(f, "аргумент не задан"),
            Self::ExtraArgument(arg) => write!(f, "лишний аргумент: {arg}"),
            Self::MissingVerb => write!(f, "команда не указана"),
            Self::UnknownDirection(dir) => write!(f, "неизвестное направление: {dir}"),
        }
    }
}

impl From<ParseIntError> for CommandError {
    fn from(err: ParseIntError) -> Self {
        Self::BadNumber(err)
    }
}

pub fn parse_command(line: &str) -> Result<Command, CommandError> {
    let mut parts = line.split_whitespace();
    let who = PlayerId(parts.next().ok_or(CommandError::Empty)?.parse()?);
    let verb = parts.next().ok_or(CommandError::MissingVerb)?;
    let action = match verb {
        "move" => Action::Move(parse_direction(
            parts.next().ok_or(CommandError::MissingArgument)?,
        )?),
        "attack" => Action::Attack {
            target: PlayerId(parts.next().ok_or(CommandError::MissingArgument)?.parse()?),
        },
        "pick" => Action::PickUp,
        "use" => Action::UseItem {
            item: parts
                .next()
                .ok_or(CommandError::MissingArgument)?
                .to_owned(),
        },
        "give" => {
            let target = PlayerId(parts.next().ok_or(CommandError::MissingArgument)?.parse()?);
            let item = parts
                .next()
                .ok_or(CommandError::MissingArgument)?
                .to_owned();
            Action::Give { to: target, item }
        }
        "wait" => Action::Wait,
        other => return Err(CommandError::UnknownVerb(other.to_owned())),
    };
    if let Some(ext) = parts.next() {
        return Err(CommandError::ExtraArgument(ext.to_owned()));
    }
    Ok(Command { who, action })
}

fn parse_direction(word: &str) -> Result<Direction, CommandError> {
    match word {
        "north" => Ok(Direction::North),
        "south" => Ok(Direction::South),
        "east" => Ok(Direction::East),
        "west" => Ok(Direction::West),
        other => Err(CommandError::UnknownDirection(other.to_owned())),
    }
}
