use bevy::prelude::*;
use bevy::sprite::Sprite;

use crate::piece::SQUARE_WIDTH;

use super::piece_builder::PieceBuilder;

/// Plugin that creates and renders the game board
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Board::new(300, 250))
            .add_systems(Startup, draw_board);
    }
}

/// Marker component for entities that are part of the board
#[derive(Component)]
struct BoardPosition;

/// Represents the game board where pieces can be placed.
/// Currently a fixed 3x5 grid:
/// ```text
/// * * * * *
/// * * * * *
/// * * * * *
/// ```
#[derive(Resource)]
pub struct Board {
    /// Positions of all squares that make up the board
    pub positions: Vec<Vec3>,
    /// Minimum X coordinate of the board (left edge)
    pub min_x: f32,
    /// Minimum Y coordinate of the board (bottom edge)
    pub min_y: f32,
    /// Maximum X coordinate of the board (right edge)
    pub max_x: f32,
    /// Maximum Y coordinate of the board (top edge)
    pub max_y: f32,
    // TODO: Track which positions are filled - vec[bool[]] ?
}

impl Board {
    fn new(start_x: i32, start_y: i32) -> Self {
        let nb_rows = 3;
        let nb_cols = 5;
        let mut positions = vec![];
        for i in 0..nb_rows {
            positions.append(&mut PieceBuilder::new_horizontal_rectangle(
                start_x,
                start_y + (i * SQUARE_WIDTH),
                nb_cols,
                0.,
            ));
        }
        Board {
            positions,
            min_x: start_x as f32,
            min_y: start_y as f32,
            max_x: (start_x + (nb_cols * SQUARE_WIDTH)) as f32,
            max_y: (start_y + (nb_rows * SQUARE_WIDTH)) as f32,
        }
    }
}

// Systems
fn draw_board(
    board: Res<Board>,
    mut commands: Commands,
) {
    let color = Color::srgb(0.60, 0.40, 0.);
    board
        .positions
        .iter()
        .for_each(|position| {
            commands
                .spawn((
                    Sprite {
                        color,
                        custom_size: Some(Vec2::new(
                            (SQUARE_WIDTH - 1) as f32,
                            (SQUARE_WIDTH - 1) as f32,
                        )),
                        ..default()
                    },
                    Transform::from_translation(*position),
                    BoardPosition,
                ));
        });
}
