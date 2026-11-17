use std::collections::BTreeMap;

use crate::{GameError, POTION, Player, PlayerId, Position, SWORD, Tile, World};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    pub(crate) world: World,
    // Нельзя использовать HashMap, так как порядок обхода будет зависеть от случайного начального значения,
    // а нам важна детерминированность, чтобы у разных участников при одинаковых данных всегда получался одинаковый результат
    pub(crate) players: BTreeMap<PlayerId, Player>,
    pub(crate) turn_count: u64,
}

impl GameState {
    #[must_use]
    pub fn new(world: World) -> GameState {
        GameState {
            world,
            players: BTreeMap::new(),
            turn_count: 0,
        }
    }

    #[must_use]
    pub fn genesis() -> GameState {
        let mut world = World::new(7, 5);
        for y in 0..5 {
            for x in 0..7 {
                if y == 0 || y == 4 || x == 0 || x == 6 || ((x == 4 || x == 2) && y == 2) {
                    world.set_tile(Position { x, y }, Tile::Wall);
                }
            }
        }
        world.set_tile(Position { x: 3, y: 1 }, Tile::Chest(SWORD.to_owned()));
        world.set_tile(Position { x: 3, y: 3 }, Tile::Chest(POTION.to_owned()));
        let mut init = GameState::new(world);
        init.players
            .insert(PlayerId(1), Player::new("Anna", Position { x: 1, y: 1 }));
        init.players
            .insert(PlayerId(2), Player::new("Boris", Position { x: 5, y: 3 }));
        init
    }

    #[must_use]
    pub fn world(&self) -> &World {
        &self.world
    }

    #[must_use]
    pub fn players(&self) -> &BTreeMap<PlayerId, Player> {
        &self.players
    }

    #[must_use]
    pub fn player(&self, id: PlayerId) -> Option<&Player> {
        self.players.get(&id)
    }

    #[must_use]
    pub fn turn(&self) -> u64 {
        self.turn_count
    }

    #[must_use]
    pub fn living_player_at(&self, position: Position) -> Option<PlayerId> {
        for (id, player) in &self.players {
            if player.position == position && player.is_alive() {
                return Some(*id);
            }
        }
        None
    }

    pub(crate) fn check_can_stand(&self, position: Position) -> Result<(), GameError> {
        match self.world().tile(position) {
            None => return Err(GameError::OutOfBounds),
            Some(Tile::Wall) => return Err(GameError::Wall),
            Some(_) => {}
        }
        if let Some(id) = self.living_player_at(position) {
            return Err(GameError::Occupied { by: id });
        }
        Ok(())
    }

    /// Добавляет живого игрока с полным здоровьем.
    ///
    /// # Errors
    ///
    /// `PlayerExists`, если номер занят; `OutOfBounds`, `Wall` или `Occupied`, если в клетку нельзя встать.
    pub fn add_player(
        &mut self,
        id: PlayerId,
        name: &str,
        position: Position,
    ) -> Result<(), GameError> {
        if self.players().contains_key(&id) {
            return Err(GameError::PlayerExists(id));
        }
        self.check_can_stand(position)?;
        self.players.insert(id, Player::new(name, position));
        Ok(())
    }
}
