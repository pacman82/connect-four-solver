/// Stores the score of board positions, so we do not need to recompute it, if the same position
/// comes up again.
pub struct TranspositionTable {
    entries: Vec<Entry>,
}

impl TranspositionTable {
    pub fn new(capacity: usize) -> Self {
        Self {
            entries: vec![Entry::EMPTY; capacity],
        }
    }

    pub fn put(&mut self, board: u64, score: i8) {
        let index = self.index(board);
        self.entries[index] = Entry::new(board, score);
    }

    pub fn get(&self, board: u64) -> Option<i8> {
        let entry = self.entries[self.index(board)];
        if entry.board() == board {
            Some(entry.score())
        } else {
            None
        }
    }

    fn index(&self, board: u64) -> usize {
        (board % self.entries.len() as u64) as usize
    }
}

/// We store both the position and the score in the same 64Bit integer.
#[derive(Clone, Copy)]
pub struct Entry([u8; 8]);

impl Entry {
    /// Lower 56 Bits are reserved for the board.
    const MASK_BOARD: u64 = (1 << 56) - 1;

    /// An entry encoding an invalid board which could never occurr during normal play, and can
    /// therfore be used to represent a cache miss.
    const EMPTY: Entry = Entry::new(Self::MASK_BOARD, 0);

    pub const fn new(position: u64, score: i8) -> Self {
        let mut bytes = position.to_le_bytes();
        debug_assert!(bytes[7] == 0);
        bytes[7] = score.to_ne_bytes()[0];
        Entry(bytes)
    }

    pub fn score(self) -> i8 {
        i8::from_ne_bytes([self.0[7]])
    }

    pub fn board(self) -> u64 {
        let mut bytes = self.0;
        bytes[7] = 0;
        u64::from_le_bytes(bytes)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ConnectFour, score};

    use super::{Entry, TranspositionTable};

    #[test]
    fn save_and_load() {
        // Just a random number I came up with, does most likely not represent a valid position, but
        // all that matters for this tests, is that it fits in 56 bits.
        let board: u64 = 60_115_128_075_855_851;
        let score = -12;
        let entry = Entry::new(board, score);

        assert_eq!(score, entry.score());
        assert_eq!(board, entry.board());
    }

    #[test]
    fn cache_hit() {
        let position = ConnectFour::from_move_list("5655663642443");
        let score = score(&position);

        let mut cache = TranspositionTable::new(1024);
        cache.put(position.encode(), score);

        assert_eq!(cache.get(position.encode()), Some(score));
    }

    #[test]
    fn cache_miss() {
        let position = ConnectFour::from_move_list("5655663642443");
        let other_position = ConnectFour::from_move_list("5655663642442");
        let score = score(&position);

        let mut cache = TranspositionTable::new(1024);
        cache.put(position.encode(), score);

        assert_eq!(cache.get(other_position.encode()), None);
    }
}
