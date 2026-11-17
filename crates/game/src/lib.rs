//! Правила игры «Подземелье блокчейна».

mod action;
mod inventory;
mod player;
mod render;
mod rules;
mod state;
mod world;

pub use action::{Action, Event, GameError};
pub use inventory::Inventory;
pub use player::{Player, PlayerId, PlayerState};
pub use render::render;
pub use rules::{BASE_DAMAGE, POTION, POTION_HEAL, START_HEALTH, SWORD, SWORD_DAMAGE, apply};
pub use state::GameState;
pub use world::{Direction, Position, Tile, World};
