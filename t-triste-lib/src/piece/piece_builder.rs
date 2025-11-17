use bevy::prelude::*;

use crate::piece::SQUARE_WIDTH;

pub struct PieceBuilder {
    #[allow(dead_code)]
    pub positions: Vec<Vec3>,
}

impl PieceBuilder {
    pub fn new_horizontal_rectangle(
        start_x: i32,
        start_y: i32,
        length: i32,
        z_index: f32,
    ) -> Vec<Vec3> {
        let mut squares = vec![];
        for i in 0..length {
            squares.push(Vec3::new(
                (start_x + i * SQUARE_WIDTH) as f32,
                start_y as f32,
                z_index,
            ))
        }
        squares
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece::{
        board::Board, corner::Corner, l::L, piece::Piece, square::Square, z::Z, SQUARE_WIDTH,
    };

    #[test]
    fn test_build_l_piece() {
        // Given / When
        // *
        // *
        // * *
        let piece = L::new(0, 0);

        // Then
        let results = piece.positions();
        assert_eq!(
            results,
            vec![
                Vec3::new(0., 0., 1.),
                Vec3::new(SQUARE_WIDTH as f32, 0., 1.),
                Vec3::new(0., SQUARE_WIDTH as f32, 1.),
                Vec3::new(0., 2. * (SQUARE_WIDTH as f32), 1.),
            ]
        );
    }

    #[test]
    fn test_build_z_piece() {
        // Given / When
        // * *
        //   * *
        let piece = Z::new(0, 0);

        // Then
        let results = piece.positions();
        assert_eq!(
            results,
            vec![
                Vec3::new(0., 0., 1.),
                Vec3::new(SQUARE_WIDTH as f32, 0., 1.),
                Vec3::new(SQUARE_WIDTH as f32, SQUARE_WIDTH as f32, 1.),
                Vec3::new(2. * SQUARE_WIDTH as f32, SQUARE_WIDTH as f32, 1.)
            ]
        );
    }

    #[test]
    fn test_build_corner_piece() {
        // Given / When
        // *
        // * *
        let piece = Corner::new(0, 0);

        // Then
        let results = piece.positions();
        assert_eq!(
            results,
            vec![
                Vec3::new(0., 0., 1.),
                Vec3::new(SQUARE_WIDTH as f32, 0., 1.),
                Vec3::new(0., SQUARE_WIDTH as f32, 1.),
            ]
        );
    }

    #[test]
    fn test_build_dot_square_piece() {
        // Given / When
        // *
        let piece = Square::new(0, 0);

        // Then
        let results = piece.positions();
        assert_eq!(results, vec![Vec3::new(0., 0., 1.),]);
    }

    #[test]
    fn test_board_positions() {
        // Given
        let start_x = 100;
        let start_y = 50;

        // When
        let board = Board::new_for_tests(start_x, start_y);

        // Then - Check that board has correct bounds
        assert_eq!(board.min_x, start_x as f32);
        assert_eq!(board.min_y, start_y as f32);
        // Board is 3 rows x 5 cols, so max values should account for that
        // Max is calculated as start + (count * SQUARE_WIDTH), not (count - 1) * SQUARE_WIDTH
        assert_eq!(board.max_x, (start_x + 5 * SQUARE_WIDTH) as f32);
        assert_eq!(board.max_y, (start_y + 3 * SQUARE_WIDTH) as f32);

        // Check that we have correct number of positions (3 rows * 5 cols = 15)
        assert_eq!(board.positions.len(), 15);
    }
}
