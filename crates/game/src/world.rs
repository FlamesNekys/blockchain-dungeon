use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

// тип ничем не владеет вне себя, поэтому побитовая копия - полноценная копия
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    #[must_use]
    pub fn neighbor(self, direction: Direction) -> Option<Position> {
        match direction {
            Direction::North => Some(Position {
                x: self.x,
                y: self.y.checked_sub(1)?,
            }),
            Direction::South => Some(Position {
                x: self.x,
                y: self.y.checked_add(1)?,
            }),
            Direction::West => Some(Position {
                x: self.x.checked_sub(1)?,
                y: self.y,
            }),
            Direction::East => Some(Position {
                x: self.x.checked_add(1)?,
                y: self.y,
            }),
        }
    }

    // для наших правил важно только что дистанция достаточно большая (больше 1 шага), поэтому используется saturating
    #[must_use]
    pub fn distance(self, other: Position) -> u32 {
        self.x
            .abs_diff(other.x)
            .saturating_add(self.y.abs_diff(other.y))
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
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
    #[must_use]
    pub fn new(width: u32, height: u32) -> World {
        let count = u64::from(width) * u64::from(height);

        World {
            width,
            height,
            tiles: (0..count).map(|_| Tile::Floor).collect(),
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

    #[must_use]
    pub fn from_rows(rows: &[&str]) -> Option<World> {
        let first_size = rows.first()?.chars().count();
        let total_rows = rows.len();
        if first_size == 0 {
            return None;
        }
        let mut tiles = Vec::new();
        for row in rows {
            if row.chars().count() != first_size {
                return None;
            }
            for sign in row.chars() {
                match sign {
                    '#' => tiles.push(Tile::Wall),
                    '.' => tiles.push(Tile::Floor),
                    _ => return None,
                }
            }
        }
        Some(World {
            height: u32::try_from(total_rows).ok()?,
            width: u32::try_from(first_size).ok()?,
            tiles,
        })
    }
}
