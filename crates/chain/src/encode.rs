use crate::Hash;
use game::{
    Action, Direction, GameState, Inventory, Player, PlayerId, PlayerState, Position, Tile, World,
};

pub trait Encode {
    fn encode(&self, out: &mut Vec<u8>);

    fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();
        self.encode(&mut res);
        res
    }

    fn digest(&self) -> Hash {
        Hash::of(&self.to_bytes())
    }
}

/// Записывает длину строки или списка как `u32`.
///
/// # Panics
///
/// Останавливается, если длина не помещается в u32, что в рамках
/// этой игры невозможно
pub(crate) fn encode_len(len: usize, out: &mut Vec<u8>) {
    u32::try_from(len)
        .expect("Длина всегда помещается в u32")
        .encode(out);
}

impl Encode for u32 {
    fn encode(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.to_le_bytes());
    }
}
impl Encode for u64 {
    fn encode(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.to_le_bytes());
    }
}
impl Encode for i32 {
    fn encode(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.to_le_bytes());
    }
}
impl Encode for u8 {
    fn encode(&self, out: &mut Vec<u8>) {
        out.push(*self);
    }
}

impl<const N: usize> Encode for [u8; N] {
    fn encode(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(self);
    }
}

impl Encode for str {
    fn encode(&self, out: &mut Vec<u8>) {
        encode_len(self.len(), out);
        out.extend_from_slice(self.as_bytes());
    }
}

impl Encode for String {
    fn encode(&self, out: &mut Vec<u8>) {
        self.as_str().encode(out);
    }
}

impl<T: Encode> Encode for Vec<T> {
    fn encode(&self, out: &mut Vec<u8>) {
        self.as_slice().encode(out);
    }
}

impl<T: Encode> Encode for [T] {
    fn encode(&self, out: &mut Vec<u8>) {
        encode_len(self.len(), out);
        for item in self {
            item.encode(out);
        }
    }
}

impl<T: Encode + ?Sized> Encode for &T {
    fn encode(&self, out: &mut Vec<u8>) {
        (**self).encode(out);
    }
}

impl<T: Encode> Encode for Option<T> {
    fn encode(&self, out: &mut Vec<u8>) {
        match self {
            None => [0].encode(out),
            Some(value) => {
                [1].encode(out);
                value.encode(out);
            }
        }
    }
}

impl Encode for PlayerId {
    fn encode(&self, out: &mut Vec<u8>) {
        self.0.encode(out);
    }
}

impl Encode for Position {
    fn encode(&self, out: &mut Vec<u8>) {
        self.x.encode(out);
        self.y.encode(out);
    }
}

impl Encode for Direction {
    fn encode(&self, out: &mut Vec<u8>) {
        match self {
            Direction::North => 0_u8.encode(out),
            Direction::South => 1_u8.encode(out),
            Direction::East => 2_u8.encode(out),
            Direction::West => 3_u8.encode(out),
        }
    }
}

impl Encode for Tile {
    fn encode(&self, out: &mut Vec<u8>) {
        match self {
            Tile::Wall => 0_u8.encode(out),
            Tile::Floor => 1_u8.encode(out),
            Tile::Chest(item) => {
                2_u8.encode(out);
                item.encode(out);
            }
        }
    }
}

impl Encode for World {
    fn encode(&self, out: &mut Vec<u8>) {
        self.width().encode(out);
        self.height().encode(out);
        self.tiles().encode(out);
    }
}

impl Encode for Inventory {
    fn encode(&self, out: &mut Vec<u8>) {
        self.items().encode(out);
        self.equipped().encode(out);
    }
}

impl Encode for PlayerState {
    fn encode(&self, out: &mut Vec<u8>) {
        match self {
            PlayerState::Alive { health } => {
                0_u8.encode(out);
                health.encode(out);
            }
            PlayerState::Dead { died_at_turn } => {
                1_u8.encode(out);
                died_at_turn.encode(out);
            }
        }
    }
}

impl Encode for Player {
    fn encode(&self, out: &mut Vec<u8>) {
        self.name.encode(out);
        self.position.encode(out);
        self.state.encode(out);
        self.inventory.encode(out);
    }
}

impl Encode for Action {
    fn encode(&self, out: &mut Vec<u8>) {
        match self {
            Action::Move(direction) => {
                0_u8.encode(out);
                direction.encode(out);
            }
            Action::Attack { target } => {
                1_u8.encode(out);
                target.encode(out);
            }
            Action::PickUp => 2_u8.encode(out),
            Action::UseItem { item } => {
                3_u8.encode(out);
                item.encode(out);
            }
            Action::Give { to, item } => {
                4_u8.encode(out);
                to.encode(out);
                item.encode(out);
            }
            Action::Wait => 5_u8.encode(out),
        }
    }
}

impl Encode for GameState {
    fn encode(&self, out: &mut Vec<u8>) {
        self.world().encode(out);
        encode_len(self.players().len(), out);
        for (id, player) in self.players() {
            id.encode(out);
            player.encode(out);
        }
        self.turn().encode(out);
    }
}

impl Encode for Hash {
    fn encode(&self, out: &mut Vec<u8>) {
        self.as_bytes().encode(out);
    }
}
