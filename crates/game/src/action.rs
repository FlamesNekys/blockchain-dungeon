use thiserror::Error;

use crate::{Direction, Position, player::PlayerId};

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum GameError {
    #[error("игрока {0} не существует")]
    UnknownPlayer(PlayerId),
    #[error("клетка занята игроком {by}")]
    Occupied { by: PlayerId },
    #[error("на пути стена")]
    Wall,
    #[error("игрок {0} погиб")]
    PlayerDead(PlayerId),
    #[error("игрок {0} уже существует")]
    PlayerExists(PlayerId),
    #[error("клетка за пределами карты")]
    OutOfBounds,
    #[error("здесь нечего подбирать")]
    NothingToPickUp,
    #[error("в инвентаре нет предмета «{0}»")]
    NoSuchItem(String),
    #[error("нельзя выбрать целью самого себя")]
    SelfTarget,
    #[error("цель слишком далеко")]
    TooFar,
    #[error("счётчик ходов переполнен")]
    TurnOverflow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Move(Direction),
    Attack { target: PlayerId },
    PickUp,
    UseItem { item: String },
    Give { to: PlayerId, item: String },
    Wait,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Moved {
        player: PlayerId,
        to: Position,
    },
    PickedUp {
        player: PlayerId,
        item: String,
    },
    Healed {
        player: PlayerId,
        health: u32,
    },
    Equipped {
        player: PlayerId,
        item: String,
    },
    Gave {
        from: PlayerId,
        to: PlayerId,
        item: String,
    },
    Damaged {
        player: PlayerId,
        by: PlayerId,
        amount: u32,
        health: u32,
    },
    Died {
        player: PlayerId,
    },
    Waited {
        player: PlayerId,
    },
}
