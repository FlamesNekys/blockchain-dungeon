use crate::{GameState, Position, Tile};

#[must_use]
pub fn render(state: &GameState) -> String {
    let mut picture = String::new();
    for y in 0..i32::try_from(state.world().height()).unwrap_or(i32::MAX) {
        for x in 0..i32::try_from(state.world().width()).unwrap_or(i32::MAX) {
            let pos = Position { x, y };
            if let Some(id) = state.living_player_at(pos) {
                if let Some(ch) = char::from_digit(id.0 % 10, 10) {
                    picture.push(ch);
                }
            } else if let Some(tile) = state.world().tile(pos) {
                match tile {
                    Tile::Floor => picture.push('.'),
                    Tile::Wall => picture.push('#'),
                    Tile::Chest(_) => picture.push('$'),
                }
            }
        }
        picture.push('\n');
    }
    picture
}
