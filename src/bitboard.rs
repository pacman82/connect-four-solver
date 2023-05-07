// Idea from: https://stackoverflow.com/questions/7033165/algorithm-to-check-a-connect-four-field

/// Bitboard containing stones of one player:
/// .  .  .  .  .  .  .  TOP
/// 5 12 19 26 33 40 47
/// 4 11 18 25 32 39 46
/// 3 10 17 24 31 38 45
/// 2  9 16 23 30 37 44
/// 1  8 15 22 29 36 43
/// 0  7 14 21 28 35 42  BOTTOM
/// The bits 6, 13, 20, 27, 34, 41, >= 48 have to be 0
///
/// `1` represents a stone of one player. `0` is an empty field, or a stone of the other player.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct PlayerStones(u64);

impl PlayerStones {
    /// Create empty Bitboard
    pub fn new() -> PlayerStones {
        PlayerStones(0)
    }

    /// Tells if the board has a stone in the specified place. The bottom row and the leftmost
    /// column are `0`.
    pub fn is_empty(self, row: u8, column: u8) -> bool {
        (cell(row, column) & self.0) == 0
    }

    /// Place a stone a the specified position
    #[cfg(test)]
    pub fn place_stone(&mut self, row: u8, column: u8) {
        self.0 |= cell(row, column)
    }

    pub fn is_win(self) -> bool {
        let y = self.0 & (self.0 >> 6);
        if (y & (y >> (2 * 6))) != 0 {
            // check \ diagonal
            return true;
        }
        let y = self.0 & (self.0 >> 7);
        if (y & (y >> (2 * 7))) != 0 {
            // check horizontal
            return true;
        }
        let y = self.0 & (self.0 >> 8);
        if (y & (y >> (2 * 8))) != 0 {
            // check / diagonal
            return true;
        }
        let y = self.0 & (self.0 >> 1);
        if (y & (y >> 2)) != 0 {
            // check vertical
            return true;
        }
        false
    }

    /// Changes the bitmask to represent the stones of the other player
    pub fn flip(&mut self, mask: AllStones) {
        self.0 ^= mask.0
    }

    /// A unique key encoding the board. Starting from bit 49 everything is guaranteed to be zero.
    /// Two different boards are guaranteed to have to different keys.
    pub fn key(self, mask: AllStones) -> u64 {
        self.0 + mask.0
    }

    /// Bitmask with `1`s in all positions in which would imply victory for the current player if he
    /// can place a stone in them.
    pub fn winning_positions(self) -> u64 {
        // Vertical (These can only be won by adding one stone on top)
        let mut winning = (self.0 << 1) & (self.0 << 2) & (self.0 << 3);

        let add_left_right_gaps = |shift| {
            let mut w = 0u64;
            // All but the vertical one can be one by adding one to the "left" of three stones, one
            // two the "right", or filling gaps in the middle. We generalize our definition of left
            // and right with the shift variable
            let two_to_the_left = self.0 << shift & self.0 << (2 * shift);
            // Two to the left, and also a third one
            w |= two_to_the_left & self.0 << (3 * shift);
            // Two to the left, and also one to the right
            w |= two_to_the_left & self.0 >> shift;
            let two_to_the_right = self.0 >> shift & self.0 >> (2 * shift);
            // Two to the right and one to the left
            w |= two_to_the_right & self.0 << shift;
            // Two to the right and also a third one
            w |= two_to_the_right & self.0 >> (3 * shift);
            w
        };

        // Horizontal; Can be won by adding a stone left, right, but also by filling a gap.
        winning |= add_left_right_gaps(6 + 1);

        // Diagonal; Bottom left to top right
        winning |= add_left_right_gaps(6 + 1 + 1);

        // Diagonal; Top left to bottom right
        winning |= add_left_right_gaps(6 + 1 - 1);

        winning & FULL
    }
}

/// Return a bitmask, with 0 everywhere but the Bit identifed by row and column
const fn cell(row: u8, column: u8) -> u64 {
    1u64 << (7 * column + row)
}

/// Bitboard containing stones of both players. First seven bits represent first column, second
/// seven bits the second column and so on.
///
/// .  .  .  .  .  .  .  TOP
/// 5 12 19 26 33 40 47
/// 4 11 18 25 32 39 46
/// 3 10 17 24 31 38 45
/// 2  9 16 23 30 37 44
/// 1  8 15 22 29 36 43
/// 0  7 14 21 28 35 42  BOTTOM
/// The bits 6, 13, 20, 27, 34, 41, >= 48 have to be 0
#[derive(Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct AllStones(u64);

impl AllStones {
    /// Mask with one stone at the bottom of each column.
    #[allow(clippy::unusual_byte_groupings)] // Group by column rather than byte ;-)
    const BOTTOM: u64 = 0b0000001_0000001_0000001_0000001_0000001_0000001_0000001u64;

    /// `true` if the column indentified by the index contains six stones.
    pub fn is_full(self, column: u8) -> bool {
        !self.is_empty(5, column)
    }

    /// Add a stone into a column. User must check before if column is already full.
    pub fn insert(&mut self, column: u8) {
        self.0 |= self.0 + cell(0, column);
    }

    /// Total number of stones in the board
    pub fn stones(self) -> u8 {
        self.0.count_ones() as u8
    }

    /// Tells if the board has a stone in the specified place. The bottom row and the leftmost
    /// column are `0`.
    pub fn is_empty(self, row: u8, column: u8) -> bool {
        (cell(row, column) & self.0) == 0
    }

    /// Bitmask with ossible positions for the next stone to land in
    pub fn possible(self) -> u64 {
        (self.0 + Self::BOTTOM) & FULL
    }
}

/// Mask with one stone in each column of the board
#[allow(clippy::unusual_byte_groupings)] // Group by column rather than byte ;-)
const FULL: u64 = 0b0111111_0111111_0111111_0111111_0111111_0111111_0111111_0111111u64;

#[cfg(test)]
mod test {

    use super::PlayerStones;

    #[test]
    fn place_stones() {
        let mut board = PlayerStones::new();
        assert!(board.is_empty(0, 2));
        board.place_stone(0, 2);
        assert!(!board.is_empty(0, 2));

        assert!(board.is_empty(3, 3));
        board.place_stone(3, 3);
        assert!(!board.is_empty(3, 3));
    }

    #[test]
    fn horizontal() {
        let mut board = PlayerStones::new();
        board.place_stone(0, 1);
        assert!(!board.is_win());
        board.place_stone(0, 2);
        assert!(!board.is_win());
        board.place_stone(0, 3);
        assert!(!board.is_win());
        board.place_stone(0, 4);
        assert!(board.is_win());
    }

    #[test]
    fn vertical() {
        let mut board = PlayerStones::new();
        board.place_stone(1, 2);
        assert!(!board.is_win());
        board.place_stone(2, 2);
        assert!(!board.is_win());
        board.place_stone(3, 2);
        assert!(!board.is_win());
        board.place_stone(4, 2);
        assert!(board.is_win());
    }

    #[test]
    fn diagonal1() {
        let mut board = PlayerStones::new();
        board.place_stone(1, 1);
        assert!(!board.is_win());
        board.place_stone(2, 2);
        assert!(!board.is_win());
        board.place_stone(3, 3);
        assert!(!board.is_win());
        board.place_stone(4, 4);
        assert!(board.is_win());
    }

    #[test]
    fn diagonal2() {
        let mut board = PlayerStones::new();
        board.place_stone(1, 4);
        assert!(!board.is_win());
        board.place_stone(2, 3);
        assert!(!board.is_win());
        board.place_stone(3, 2);
        assert!(!board.is_win());
        board.place_stone(4, 1);
        assert!(board.is_win());
    }
}
