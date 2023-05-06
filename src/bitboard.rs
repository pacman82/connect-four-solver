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
    /// `true` if the column indentified by the index contains six stones.
    pub fn is_full(self, column: u8) -> bool {
        // Highest cell in the specified column. Shift "up" and then "right"
        self.0 & cell(5, column) != 0
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
}

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
