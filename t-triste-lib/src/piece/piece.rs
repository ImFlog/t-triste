use bevy::prelude::*;

use crate::cursor::Cursor;

use crate::SQUARE_WIDTH;

// Components
#[derive(Component)]
pub struct Position;

pub trait Piece {
    fn positions(&self) -> Vec<Vec3>;
    fn color(&self) -> Color;
    fn rotate(&mut self);
    fn snap(&mut self);
    fn move_it(&mut self, cursor: &Res<Cursor>);
    fn set_moving(&mut self, moving: bool);
    fn is_moving(&self) -> bool;

    fn is_even_odd(&self, current_pos: Vec2) -> bool {
        self.positions().iter().any(|piece_pos| {
            piece_pos.x - (SQUARE_WIDTH / 2) as f32 <= current_pos.x
                && current_pos.x <= piece_pos.x + (SQUARE_WIDTH / 2) as f32
                && piece_pos.y - (SQUARE_WIDTH / 2) as f32 <= current_pos.y
                && current_pos.y <= piece_pos.y + (SQUARE_WIDTH / 2) as f32
        })
    }
}
