// Idea from: https://stackoverflow.com/questions/7033165/algorithm-to-check-a-connect-four-field

/// Bitboard stones player one:
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
pub struct Bitboard(u64);

impl Bitboard {
    /// Create empty Bitboard
    pub fn new() -> Bitboard {
        Bitboard(0)
    }

    /// Tells if the board has a stone in the specified place. The bottom row and the leftmost
    /// column are `0`.
    pub fn is_empty(self, row: u8, column: u8) -> bool {
        (Self::cell(row, column) & self.0) == 0
    }

    /// Place a stone a the specified position
    pub fn place_stone(&mut self, row: u8, column: u8) {
        self.0 |= Self::cell(row, column)
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

    pub fn stones(self) -> u8 {
        self.0.count_ones() as u8
    }

    /// Return a bitmask, with 0 everywhere but the Bit identifed by row and column
    fn cell(row: u8, column: u8) -> u64 {
        1u64 << (7 * column + row)
    }
}

#[cfg(test)]
mod test {

    use super::Bitboard;

    #[test]
    fn place_stones() {
        let mut board = Bitboard::new();
        assert!(board.is_empty(0, 2));
        board.place_stone(0, 2);
        assert!(!board.is_empty(0, 2));

        assert!(board.is_empty(3, 3));
        board.place_stone(3, 3);
        assert!(!board.is_empty(3, 3));
    }

    #[test]
    fn horizontal() {
        let mut board = Bitboard::new();
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
        let mut board = Bitboard::new();
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
        let mut board = Bitboard::new();
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
        let mut board = Bitboard::new();
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
