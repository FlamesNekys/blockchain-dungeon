use std::fmt;

use crate::world::Position;
use crate::{Inventory, START_HEALTH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlayerId(pub u32);

impl fmt::Display for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub position: Position,
    pub state: PlayerState,
    pub inventory: Inventory,
}

impl Player {
    #[must_use]
    pub fn new(name: &str, position: Position) -> Self {
        Self {
            name: name.to_owned(),
            position,
            state: PlayerState::Alive {
                health: START_HEALTH,
            },
            inventory: Inventory::default(),
        }
    }

    #[must_use]
    pub fn is_alive(&self) -> bool {
        matches!(self.state, PlayerState::Alive { .. })
    }

    #[must_use]
    pub fn health(&self) -> Option<u32> {
        match self.state {
            PlayerState::Alive { health } => Some(health),
            PlayerState::Dead { .. } => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerState {
    Alive { health: u32 },
    Dead { died_at_turn: u64 },
}
