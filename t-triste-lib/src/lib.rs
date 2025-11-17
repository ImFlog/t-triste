mod cursor;
mod piece;

use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};
use piece::{
    board::{self, Board},
    GameState, SQUARE_WIDTH,
};

// Plugin
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(1., 0.90, 1.)))
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "T-Triste".to_string(),
                    resolution: (800, 600).into(),
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    ..default()
                }),
                ..default()
            }))
            .add_systems(Startup, setup_camera)
            .add_plugins(cursor::CursorPlugin)
            .add_plugins(board::BoardPlugin)
            .add_plugins(piece::PiecePlugin)
            .add_systems(Update, incrust_in_board);
    }
}

// System
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn incrust_in_board(
    mut game_state: NonSendMut<GameState>,
    board: Option<Res<Board>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    if !mouse_button_input.just_released(MouseButton::Left) || board.is_none() {
        return;
    }

    let board = board.unwrap();

    // The issue was that the code expected pixel perfect placement.
    // Add a 5% acceptance factor.
    // We could put this in a method to clean up the code ?
    let adjusted_min_x = board.min_x * 0.95;
    let adjusted_min_y = board.min_y * 0.95;
    let adjusted_max_x = board.max_x * 1.05;
    let adjusted_max_y = board.max_y * 1.05;

    // We take the first moving piece
    // TODO: This could be improved
    let moving_piece_optional = game_state.0.iter_mut().find(|piece| piece.is_moving());
    if moving_piece_optional.is_none() {
        return;
    }
    let moving_piece = moving_piece_optional.unwrap();

    // TODO: Find the exact board position that is being filled
    let in_board = moving_piece.positions().iter().all(|t| {
        adjusted_min_x <= t.x
            && t.x <= adjusted_max_x
            && adjusted_min_y <= t.y
            && t.y <= adjusted_max_y
    });

    if in_board {
        moving_piece.snap();
        // TODO: we are once again iterating over the transform. This is not efficient.
        // TODO: Save the board squares that are filled.
    }
}
