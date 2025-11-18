use std::collections::HashSet;

use super::{corner::Corner, l::L, rectangle::Rectangle, square::Square, z::Z, SQUARE_WIDTH};
use super::piece::Piece;

/// Represents the type of piece to place on the board
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceType {
    Square,
    Rectangle,
    L,
    Z,
    Corner,
}

impl PieceType {
    /// Get all available piece types
    pub fn all_types() -> Vec<Self> {
        vec![
            Self::Square,
            Self::Rectangle,
            Self::L,
            Self::Z,
            Self::Corner,
        ]
    }

    /// Get the shape of this piece as relative grid offsets (row, col) from the anchor point
    /// Based on the test comments in piece_builder.rs
    pub fn shape_offsets(&self) -> Vec<(i32, i32)> {
        match self {
            // Single square: *
            Self::Square => vec![(0, 0)],

            // Vertical rectangle (3 squares):
            // *
            // *
            // *
            Self::Rectangle => vec![(0, 0), (1, 0), (2, 0)],

            // L-shape (4 squares):
            // *
            // *
            // * *
            Self::L => vec![(0, 0), (1, 0), (2, 0), (2, 1)],

            // Z-shape (4 squares):
            // * *
            //   * *
            Self::Z => vec![(0, 0), (0, 1), (1, 1), (1, 2)],

            // Corner (3 squares):
            // *
            // * *
            Self::Corner => vec![(0, 0), (1, 0), (1, 1)],
        }
    }

    /// Get the number of squares this piece occupies
    pub fn size(&self) -> usize {
        self.shape_offsets().len()
    }

    /// Create an instance of this piece type at the given pixel position
    pub fn instantiate(&self, pixel_x: i32, pixel_y: i32) -> Box<dyn Piece> {
        match self {
            Self::Square => Box::new(Square::new(pixel_x, pixel_y)),
            Self::Rectangle => Box::new(Rectangle::new(pixel_x, pixel_y)),
            Self::L => Box::new(L::new(pixel_x, pixel_y)),
            Self::Z => Box::new(Z::new(pixel_x, pixel_y)),
            Self::Corner => Box::new(Corner::new(pixel_x, pixel_y)),
        }
    }
}

/// Represents a placed piece on the board at a specific grid position
#[derive(Debug, Clone, PartialEq)]
pub struct PlacedPiece {
    pub piece_type: PieceType,
    pub grid_row: i32,
    pub grid_col: i32,
}

impl PlacedPiece {
    /// Get all grid positions occupied by this piece
    pub fn occupied_positions(&self) -> Vec<(i32, i32)> {
        self.piece_type
            .shape_offsets()
            .iter()
            .map(|(row_offset, col_offset)| {
                (self.grid_row + row_offset, self.grid_col + col_offset)
            })
            .collect()
    }

    /// Convert grid position to pixel coordinates for the piece anchor point
    pub fn to_pixel_position(&self, board_start_x: i32, board_start_y: i32) -> (i32, i32) {
        let pixel_x = board_start_x + (self.grid_col * SQUARE_WIDTH);
        let pixel_y = board_start_y + (self.grid_row * SQUARE_WIDTH);
        (pixel_x, pixel_y)
    }

    /// Create an instance of the actual piece at the correct pixel position
    pub fn instantiate(&self, board_start_x: i32, board_start_y: i32) -> Box<dyn Piece> {
        let (pixel_x, pixel_y) = self.to_pixel_position(board_start_x, board_start_y);
        self.piece_type.instantiate(pixel_x, pixel_y)
    }
}

/// Tracks the state of the board grid
#[derive(Debug, Clone)]
pub struct BoardState {
    rows: usize,
    cols: usize,
    filled: HashSet<(i32, i32)>,
}

impl BoardState {
    /// Create a new empty board
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            filled: HashSet::new(),
        }
    }

    /// Check if a position is within board bounds
    pub fn is_within_bounds(&self, row: i32, col: i32) -> bool {
        row >= 0 && row < self.rows as i32 && col >= 0 && col < self.cols as i32
    }

    /// Check if a position is already filled
    pub fn is_filled(&self, row: i32, col: i32) -> bool {
        self.filled.contains(&(row, col))
    }

    /// Check if a piece can be placed at the given position
    pub fn can_place_piece(&self, piece: &PlacedPiece) -> bool {
        for (row, col) in piece.occupied_positions() {
            // Check bounds
            if !self.is_within_bounds(row, col) {
                return false;
            }
            // Check overlap
            if self.is_filled(row, col) {
                return false;
            }
        }
        true
    }

    /// Place a piece on the board (marks positions as filled)
    pub fn place_piece(&mut self, piece: &PlacedPiece) {
        for pos in piece.occupied_positions() {
            self.filled.insert(pos);
        }
    }

    /// Remove a piece from the board (marks positions as empty)
    pub fn remove_piece(&mut self, piece: &PlacedPiece) {
        for pos in piece.occupied_positions() {
            self.filled.remove(&pos);
        }
    }

    /// Check if the board is completely filled
    pub fn is_complete(&self) -> bool {
        self.filled.len() == self.rows * self.cols
    }

    /// Get the number of empty squares
    pub fn empty_count(&self) -> usize {
        (self.rows * self.cols) - self.filled.len()
    }

    /// Find the first empty position (for systematic filling)
    pub fn first_empty_position(&self) -> Option<(i32, i32)> {
        for row in 0..self.rows as i32 {
            for col in 0..self.cols as i32 {
                if !self.is_filled(row, col) {
                    return Some((row, col));
                }
            }
        }
        None
    }
}

/// Generate a valid set of pieces that completely fills the board
/// Uses backtracking algorithm to find a solution
pub fn generate_piece_set(rows: usize, cols: usize) -> Option<Vec<PlacedPiece>> {
    let mut board = BoardState::new(rows, cols);
    let mut pieces = Vec::new();
    let piece_types = PieceType::all_types();

    if backtrack(&mut board, &mut pieces, &piece_types) {
        Some(pieces)
    } else {
        None
    }
}

/// Backtracking helper function
fn backtrack(
    board: &mut BoardState,
    pieces: &mut Vec<PlacedPiece>,
    piece_types: &[PieceType],
) -> bool {
    // Base case: board is complete
    if board.is_complete() {
        return true;
    }

    // Find the first empty position to fill
    let (start_row, start_col) = match board.first_empty_position() {
        Some(pos) => pos,
        None => return false,
    };

    // Try each piece type
    for &piece_type in piece_types {
        let offsets = piece_type.shape_offsets();

        // Try placing the piece at positions that would cover the empty square
        // We try different anchor positions relative to the empty square
        for &(offset_row, offset_col) in &offsets {
            let anchor_row = start_row - offset_row;
            let anchor_col = start_col - offset_col;

            let piece = PlacedPiece {
                piece_type,
                grid_row: anchor_row,
                grid_col: anchor_col,
            };

            // Check if we can place this piece
            if board.can_place_piece(&piece) {
                // Place the piece
                board.place_piece(&piece);
                pieces.push(piece.clone());

                // Recurse
                if backtrack(board, pieces, piece_types) {
                    return true;
                }

                // Backtrack
                board.remove_piece(&piece);
                pieces.pop();
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_type_sizes() {
        assert_eq!(PieceType::Square.size(), 1);
        assert_eq!(PieceType::Rectangle.size(), 3);
        assert_eq!(PieceType::L.size(), 4);
        assert_eq!(PieceType::Z.size(), 4);
        assert_eq!(PieceType::Corner.size(), 3);
    }

    #[test]
    fn test_piece_type_shapes() {
        // Square: *
        assert_eq!(PieceType::Square.shape_offsets(), vec![(0, 0)]);

        // Rectangle vertical (3 squares)
        assert_eq!(
            PieceType::Rectangle.shape_offsets(),
            vec![(0, 0), (1, 0), (2, 0)]
        );

        // L-shape (4 squares)
        assert_eq!(
            PieceType::L.shape_offsets(),
            vec![(0, 0), (1, 0), (2, 0), (2, 1)]
        );

        // Z-shape (4 squares)
        assert_eq!(
            PieceType::Z.shape_offsets(),
            vec![(0, 0), (0, 1), (1, 1), (1, 2)]
        );

        // Corner (3 squares)
        assert_eq!(
            PieceType::Corner.shape_offsets(),
            vec![(0, 0), (1, 0), (1, 1)]
        );
    }

    #[test]
    fn test_board_state_bounds() {
        let board = BoardState::new(3, 5);

        // Valid positions
        assert!(board.is_within_bounds(0, 0));
        assert!(board.is_within_bounds(2, 4));
        assert!(board.is_within_bounds(1, 2));

        // Invalid positions
        assert!(!board.is_within_bounds(-1, 0));
        assert!(!board.is_within_bounds(0, -1));
        assert!(!board.is_within_bounds(3, 0));
        assert!(!board.is_within_bounds(0, 5));
    }

    #[test]
    fn test_board_state_place_and_remove() {
        let mut board = BoardState::new(3, 5);
        let piece = PlacedPiece {
            piece_type: PieceType::Square,
            grid_row: 0,
            grid_col: 0,
        };

        assert!(!board.is_filled(0, 0));

        board.place_piece(&piece);
        assert!(board.is_filled(0, 0));
        assert_eq!(board.empty_count(), 14);

        board.remove_piece(&piece);
        assert!(!board.is_filled(0, 0));
        assert_eq!(board.empty_count(), 15);
    }

    #[test]
    fn test_can_place_piece_no_overlap() {
        let mut board = BoardState::new(3, 5);

        let piece1 = PlacedPiece {
            piece_type: PieceType::Square,
            grid_row: 0,
            grid_col: 0,
        };

        let piece2 = PlacedPiece {
            piece_type: PieceType::Square,
            grid_row: 0,
            grid_col: 0,
        };

        assert!(board.can_place_piece(&piece1));
        board.place_piece(&piece1);

        // piece2 overlaps with piece1 (same position)
        assert!(!board.can_place_piece(&piece2));
    }

    #[test]
    fn test_can_place_piece_out_of_bounds() {
        let board = BoardState::new(3, 5);

        // Z piece at (0, 4) would extend to col 6, which is out of bounds
        let piece = PlacedPiece {
            piece_type: PieceType::Z,
            grid_row: 0,
            grid_col: 4,
        };

        assert!(!board.can_place_piece(&piece));
    }

    #[test]
    fn test_occupied_positions() {
        let piece = PlacedPiece {
            piece_type: PieceType::Corner,
            grid_row: 1,
            grid_col: 2,
        };

        let positions = piece.occupied_positions();
        assert_eq!(positions.len(), 3);
        assert!(positions.contains(&(1, 2))); // anchor
        assert!(positions.contains(&(2, 2))); // down
        assert!(positions.contains(&(2, 3))); // down-right
    }

    #[test]
    fn test_board_complete() {
        let mut board = BoardState::new(2, 2);
        assert!(!board.is_complete());

        // Fill all 4 squares
        for row in 0..2 {
            for col in 0..2 {
                board.filled.insert((row, col));
            }
        }

        assert!(board.is_complete());
    }

    #[test]
    fn test_first_empty_position() {
        let mut board = BoardState::new(3, 5);

        assert_eq!(board.first_empty_position(), Some((0, 0)));

        board.filled.insert((0, 0));
        assert_eq!(board.first_empty_position(), Some((0, 1)));

        // Fill first row
        for col in 0..5 {
            board.filled.insert((0, col));
        }
        assert_eq!(board.first_empty_position(), Some((1, 0)));
    }

    #[test]
    fn test_to_pixel_position() {
        let piece = PlacedPiece {
            piece_type: PieceType::Square,
            grid_row: 0,
            grid_col: 0,
        };

        let (pixel_x, pixel_y) = piece.to_pixel_position(300, 250);
        assert_eq!(pixel_x, 300);
        assert_eq!(pixel_y, 250);

        let piece2 = PlacedPiece {
            piece_type: PieceType::Square,
            grid_row: 2,
            grid_col: 3,
        };

        let (pixel_x, pixel_y) = piece2.to_pixel_position(300, 250);
        // row 2: 250 + (2 * 50) = 350
        // col 3: 300 + (3 * 50) = 450
        assert_eq!(pixel_x, 450);
        assert_eq!(pixel_y, 350);
    }

    #[test]
    fn test_generate_piece_set_fills_board() {
        let pieces = generate_piece_set(3, 5).expect("Should find a solution");

        // Verify the solution is valid
        let mut board = BoardState::new(3, 5);

        for piece in &pieces {
            // Each piece should be placeable
            assert!(
                board.can_place_piece(piece),
                "Piece should be placeable: {:?}",
                piece
            );
            board.place_piece(piece);
        }

        // Board should be completely filled
        assert!(board.is_complete(), "Board should be completely filled");

        // Should have exactly 15 filled squares (3 rows × 5 cols)
        assert_eq!(board.filled.len(), 15);
    }

    #[test]
    fn test_generate_piece_set_no_overlap() {
        let pieces = generate_piece_set(3, 5).expect("Should find a solution");

        let mut all_positions = HashSet::new();
        let mut position_count = 0;

        for piece in &pieces {
            for pos in piece.occupied_positions() {
                // Each position should be unique (no overlaps)
                assert!(
                    all_positions.insert(pos),
                    "Position {:?} is occupied by multiple pieces",
                    pos
                );
                position_count += 1;
            }
        }

        // Should have exactly 15 unique positions
        assert_eq!(position_count, 15);
        assert_eq!(all_positions.len(), 15);
    }

    #[test]
    fn test_generate_piece_set_within_bounds() {
        let pieces = generate_piece_set(3, 5).expect("Should find a solution");

        for piece in &pieces {
            for (row, col) in piece.occupied_positions() {
                // All positions should be within bounds
                assert!(
                    row >= 0 && row < 3 && col >= 0 && col < 5,
                    "Position ({}, {}) is out of bounds",
                    row,
                    col
                );
            }
        }
    }

    #[test]
    fn test_instantiate_pieces() {
        let pieces = generate_piece_set(3, 5).expect("Should find a solution");

        // Test that we can instantiate all pieces
        for piece in &pieces {
            let instance = piece.instantiate(300, 250);

            // Verify the piece has the expected number of positions
            let positions = instance.positions();
            assert_eq!(
                positions.len(),
                piece.piece_type.size(),
                "Piece {:?} should have {} positions",
                piece.piece_type,
                piece.piece_type.size()
            );
        }
    }

    #[test]
    fn test_different_board_sizes() {
        // Test 2x3 board (6 squares)
        let pieces = generate_piece_set(2, 3).expect("Should find solution for 2x3 board");
        let mut board = BoardState::new(2, 3);
        for piece in &pieces {
            board.place_piece(piece);
        }
        assert!(board.is_complete());

        // Test 1x5 board (5 squares)
        let pieces = generate_piece_set(1, 5).expect("Should find solution for 1x5 board");
        let mut board = BoardState::new(1, 5);
        for piece in &pieces {
            board.place_piece(piece);
        }
        assert!(board.is_complete());
    }
}
