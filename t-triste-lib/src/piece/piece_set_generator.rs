use bevy::math::Vec3;
use std::collections::HashSet;

/// Represents the shape of a piece as relative grid positions
/// Each position is (row_offset, col_offset) from the piece's anchor point
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PieceShape {
    pub offsets: Vec<(i32, i32)>,
}

impl PieceShape {
    /// Create a new piece shape from relative offsets
    pub fn new(offsets: Vec<(i32, i32)>) -> Self {
        Self { offsets }
    }

    /// Single square (1x1)
    pub fn square() -> Self {
        Self::new(vec![(0, 0)])
    }

    /// Vertical rectangle (3x1)
    pub fn vertical_rectangle() -> Self {
        Self::new(vec![(0, 0), (1, 0), (2, 0)])
    }

    /// Horizontal rectangle (1x3)
    pub fn horizontal_rectangle() -> Self {
        Self::new(vec![(0, 0), (0, 1), (0, 2)])
    }

    /// L-shape (vertical variant)
    pub fn l_shape() -> Self {
        Self::new(vec![(0, 0), (1, 0), (2, 0), (2, 1)])
    }

    /// L-shape (horizontal variant)
    pub fn l_shape_horizontal() -> Self {
        Self::new(vec![(0, 0), (0, 1), (0, 2), (1, 0)])
    }

    /// Z-shape (horizontal)
    pub fn z_shape() -> Self {
        Self::new(vec![(0, 0), (0, 1), (1, 1), (1, 2)])
    }

    /// Corner shape (L with 3 squares)
    pub fn corner() -> Self {
        Self::new(vec![(0, 0), (0, 1), (1, 0)])
    }

    /// Get all available piece shapes
    pub fn all_shapes() -> Vec<Self> {
        vec![
            Self::square(),
            Self::vertical_rectangle(),
            Self::horizontal_rectangle(),
            Self::l_shape(),
            Self::l_shape_horizontal(),
            Self::z_shape(),
            Self::corner(),
        ]
    }

    /// Get the number of squares this piece occupies
    pub fn size(&self) -> usize {
        self.offsets.len()
    }
}

/// Represents a placed piece on the board
#[derive(Debug, Clone, PartialEq)]
pub struct PlacedPiece {
    pub shape: PieceShape,
    pub row: i32,
    pub col: i32,
}

impl PlacedPiece {
    /// Get all grid positions occupied by this piece
    pub fn occupied_positions(&self) -> Vec<(i32, i32)> {
        self.shape
            .offsets
            .iter()
            .map(|(row_offset, col_offset)| (self.row + row_offset, self.col + col_offset))
            .collect()
    }

    /// Convert grid positions to pixel coordinates (Vec3)
    /// Based on the board's position and SQUARE_WIDTH constant
    pub fn to_pixel_positions(&self, board_start_x: f32, board_start_y: f32, square_width: f32) -> Vec<Vec3> {
        self.occupied_positions()
            .iter()
            .map(|(row, col)| {
                let x = board_start_x + (*col as f32 * square_width);
                let y = board_start_y + (*row as f32 * square_width);
                Vec3::new(x, y, 0.0)
            })
            .collect()
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
    let shapes = PieceShape::all_shapes();

    if backtrack(&mut board, &mut pieces, &shapes) {
        Some(pieces)
    } else {
        None
    }
}

/// Backtracking helper function
fn backtrack(board: &mut BoardState, pieces: &mut Vec<PlacedPiece>, shapes: &[PieceShape]) -> bool {
    // Base case: board is complete
    if board.is_complete() {
        return true;
    }

    // Find the first empty position to fill
    let (start_row, start_col) = match board.first_empty_position() {
        Some(pos) => pos,
        None => return false,
    };

    // Try each shape
    for shape in shapes {
        // Try placing the shape at positions that would cover the empty square
        // We try different anchor positions relative to the empty square
        for offset_row in 0..shape.offsets.len() as i32 {
            for offset_col in 0..shape.offsets.iter().map(|(_, c)| c).max().unwrap_or(&0) + 1 {
                let anchor_row = start_row - offset_row;
                let anchor_col = start_col - offset_col;

                let piece = PlacedPiece {
                    shape: shape.clone(),
                    row: anchor_row,
                    col: anchor_col,
                };

                // Check if we can place this piece
                if board.can_place_piece(&piece) {
                    // Check if this piece actually covers the target empty square
                    let positions = piece.occupied_positions();
                    if positions.contains(&(start_row, start_col)) {
                        // Place the piece
                        board.place_piece(&piece);
                        pieces.push(piece.clone());

                        // Recurse
                        if backtrack(board, pieces, shapes) {
                            return true;
                        }

                        // Backtrack
                        board.remove_piece(&piece);
                        pieces.pop();
                    }
                }
            }
        }
    }

    false
}

/// Generate a deterministic piece set for testing (simple horizontal strips)
pub fn generate_simple_piece_set(rows: usize, cols: usize) -> Vec<PlacedPiece> {
    let mut pieces = Vec::new();

    // Fill the board with horizontal rectangles (length = cols)
    for row in 0..rows as i32 {
        // Create a horizontal piece for this row
        let offsets: Vec<(i32, i32)> = (0..cols as i32).map(|c| (0, c)).collect();
        let shape = PieceShape::new(offsets);

        pieces.push(PlacedPiece {
            shape,
            row,
            col: 0,
        });
    }

    pieces
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_shape_sizes() {
        assert_eq!(PieceShape::square().size(), 1);
        assert_eq!(PieceShape::vertical_rectangle().size(), 3);
        assert_eq!(PieceShape::horizontal_rectangle().size(), 3);
        assert_eq!(PieceShape::l_shape().size(), 4);
        assert_eq!(PieceShape::z_shape().size(), 4);
        assert_eq!(PieceShape::corner().size(), 3);
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
            shape: PieceShape::square(),
            row: 0,
            col: 0,
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
            shape: PieceShape::horizontal_rectangle(),
            row: 0,
            col: 0,
        };

        let piece2 = PlacedPiece {
            shape: PieceShape::horizontal_rectangle(),
            row: 0,
            col: 1,
        };

        assert!(board.can_place_piece(&piece1));
        board.place_piece(&piece1);

        // piece2 overlaps with piece1
        assert!(!board.can_place_piece(&piece2));
    }

    #[test]
    fn test_can_place_piece_out_of_bounds() {
        let board = BoardState::new(3, 5);

        // Piece goes out of bounds (col 4 + offset 2 = col 6, but max is 4)
        let piece = PlacedPiece {
            shape: PieceShape::horizontal_rectangle(),
            row: 0,
            col: 4,
        };

        assert!(!board.can_place_piece(&piece));
    }

    #[test]
    fn test_occupied_positions() {
        let piece = PlacedPiece {
            shape: PieceShape::horizontal_rectangle(),
            row: 1,
            col: 2,
        };

        let positions = piece.occupied_positions();
        assert_eq!(positions.len(), 3);
        assert!(positions.contains(&(1, 2)));
        assert!(positions.contains(&(1, 3)));
        assert!(positions.contains(&(1, 4)));
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
    fn test_generate_simple_piece_set() {
        let pieces = generate_simple_piece_set(3, 5);

        // Should generate 3 pieces (one per row)
        assert_eq!(pieces.len(), 3);

        // Verify all pieces together fill the board
        let mut board = BoardState::new(3, 5);
        for piece in &pieces {
            assert!(board.can_place_piece(piece));
            board.place_piece(piece);
        }

        assert!(board.is_complete());
    }

    #[test]
    fn test_generate_piece_set_fills_board() {
        let pieces = generate_piece_set(3, 5).expect("Should find a solution");

        // Verify the solution is valid
        let mut board = BoardState::new(3, 5);

        for piece in &pieces {
            // Each piece should be placeable
            assert!(board.can_place_piece(piece), "Piece should be placeable: {:?}", piece);
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
    fn test_to_pixel_positions() {
        let piece = PlacedPiece {
            shape: PieceShape::horizontal_rectangle(),
            row: 0,
            col: 0,
        };

        let pixel_positions = piece.to_pixel_positions(100.0, 200.0, 50.0);

        assert_eq!(pixel_positions.len(), 3);
        assert_eq!(pixel_positions[0], Vec3::new(100.0, 200.0, 0.0));
        assert_eq!(pixel_positions[1], Vec3::new(150.0, 200.0, 0.0));
        assert_eq!(pixel_positions[2], Vec3::new(200.0, 200.0, 0.0));
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
