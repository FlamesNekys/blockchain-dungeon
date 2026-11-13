//! Правила игры «Подземелье блокчейна».

// тип ничем не владеет вне себя, поэтому побитовая копия - полноценная копия
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

// копируется только явно из-за владения String в куче у Chest, побитовая копия дала бы двух владельцев
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Floor,
    Chest(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct World {
    width: u32,
    height: u32,
    tiles: Vec<Tile>,
}

impl World {
    /// # Panics
    ///
    /// Если карта не помещается в адресное пространство.
    #[must_use]
    pub fn new(width: u32, height: u32) -> World {
        let count = usize::try_from(u64::from(width) * u64::from(height))
            .expect("карта не помещается в адресное пространство");

        World {
            width,
            height,
            tiles: vec![Tile::Floor; count],
        }
    }

    fn index_of(&self, position: Position) -> Option<usize> {
        let x = u32::try_from(position.x).ok()?;
        let y = u32::try_from(position.y).ok()?;
        if x >= self.width || y >= self.height {
            return None;
        }
        let index = u64::from(y) * u64::from(self.width) + u64::from(x);
        usize::try_from(index).ok()
    }

    #[must_use]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[must_use]
    pub fn height(&self) -> u32 {
        self.height
    }

    #[must_use]
    pub fn tile(&self, position: Position) -> Option<&Tile> {
        let index = self.index_of(position)?;
        self.tiles.get(index)
    }

    pub fn set_tile(&mut self, position: Position, tile: Tile) -> bool {
        let Some(slot) = self.index_of(position).and_then(|i| self.tiles.get_mut(i)) else {
            return false;
        };

        *slot = tile;
        true
    }

    pub fn take_item(&mut self, position: Position) -> Option<String> {
        let index = self.index_of(position)?;
        let slot = self.tiles.get_mut(index)?;

        let Tile::Chest(item) = slot else {
            return None;
        };
        let item = std::mem::take(item);
        *slot = Tile::Floor;
        Some(item)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Inventory {
    items: Vec<String>,
    equipped: Option<String>,
}

impl Inventory {
    pub fn add(&mut self, item: String) {
        self.items.push(item);
    }

    #[must_use]
    pub fn items(&self) -> &[String] {
        self.items.as_slice()
    }

    #[must_use]
    pub fn equipped(&self) -> Option<&str> {
        self.equipped.as_deref()
    }

    pub fn remove(&mut self, name: &str) -> Option<String> {
        let index = self.items.iter().position(|item| *item == name)?;
        Some(self.items.remove(index))
    }

    pub fn equip(&mut self, name: &str) -> bool {
        if let Some(item) = self.remove(name) {
            if let Some(prev) = self.equipped.replace(item) {
                self.items.push(prev);
            }
            true
        } else {
            false
        }
    }

    #[must_use]
    pub fn longest_item_name(&self) -> Option<&str> {
        let mut longest: Option<&str> = None;
        for item in &self.items {
            match longest {
                Some(name) if item.len() > name.len() => longest = Some(item),
                Some(_) => (),
                None => longest = Some(item),
            }
        }
        longest
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub position: Position,
    pub health: u32,
    pub inventory: Inventory,
}

impl Player {
    #[must_use]
    pub fn new(name: &str, position: Position) -> Player {
        Player {
            name: name.to_owned(),
            position,
            health: 100,
            inventory: Inventory::default(),
        }
    }
}

fn check_tile(
    world: &World,
    position: Position,
    direction: Direction,
) -> Option<(&Tile, Position)> {
    let new_pos = match direction {
        Direction::North => Position {
            x: position.x,
            y: position.y.checked_sub(1)?,
        },
        Direction::South => Position {
            x: position.x,
            y: position.y.checked_add(1)?,
        },
        Direction::East => Position {
            x: position.x.checked_add(1)?,
            y: position.y,
        },
        Direction::West => Position {
            x: position.x.checked_sub(1)?,
            y: position.y,
        },
    };
    if let Some(tile) = world.tile(new_pos)
        && tile != &Tile::Wall
    {
        return Some((tile, new_pos));
    }
    None
}

pub fn step(world: &World, player: &mut Player, direction: Direction) -> bool {
    if let Some((_, pos)) = check_tile(world, player.position, direction) {
        player.position = pos;
        true
    } else {
        false
    }
}

pub fn pick_up(world: &mut World, player: &mut Player) -> bool {
    match world.take_item(player.position) {
        Some(item) => {
            player.inventory.add(item);
            true
        }
        None => false,
    }
}

pub fn give_item(players: &mut [Player], from: usize, to: usize, item: &str) -> bool {
    if from == to || from >= players.len() || to >= players.len() {
        return false;
    }
    let Some(moved) = players[from].inventory.remove(item) else {
        return false;
    };
    players[to].inventory.add(moved);
    true
}
