/// Stores the score of board positions, so we do not need to recompute it, if the same position
/// comes up again.
pub struct TranspositionTable {
    // Stores the last 32 bits of the board, i.e. board modulo 2 ^ 32.
    keys: Vec<u32>,
    scores: Vec<i8>,
}

impl TranspositionTable {
    pub fn new(capacity: usize) -> Self {
        // Capacity must be odd, so it is a coprime (i.e. it has no common prime factors) a power of
        // two.
        assert!(capacity % 2 == 1);
        // 49 Bits uniquely encode the board. => Max key is 2 ^ 49.
        // capacity is coprime to 2 ^ 32, and S * 2 ^ 32 greater than the max possible full key, the
        // chinese remainder theorem guarantees that the index, key pair is unique. 
        assert!(capacity * (2 ^ 32) > 2 ^ 49);
        Self {
            // We use 0, to represent a cache miss
            keys: vec![0; capacity],
            scores: vec![0; capacity],
        }
    }

    pub fn put(&mut self, board: u64, score: i8) {
        let index = self.index(board);
        self.keys[index] = Self::key(board);
        self.scores[index] = score;
    }

    pub fn get(&self, board: u64) -> Option<i8> {
        let index = self.index(board);
        let found_key = self.keys[index];
        if found_key == Self::key(board) {
            // Hit
            Some(self.scores[index])
        } else {
            // Miss
            None
        }
    }

    fn key(board: u64) -> u32 {
        board as u32
    }

    fn index(&self, board: u64) -> usize {
        (board % self.keys.len() as u64) as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::ConnectFour;
    use super::TranspositionTable;

    #[test]
    fn cache_hit() {
        let position = ConnectFour::from_move_list("5655663642443");
        let score = 15;

        // 131101 next prime after 131073 which is the smallest valid number for the transposition
        // table to work correctly.
        let mut cache = TranspositionTable::new(131101);
        cache.put(position.encode(), score);

        assert_eq!(cache.get(position.encode()), Some(score));
    }

    #[test]
    fn cache_miss() {
        let position = ConnectFour::from_move_list("5655663642443");
        let other_position = ConnectFour::from_move_list("5655663642442");
        let score = 15;

        let mut cache = TranspositionTable::new(131101);
        cache.put(position.encode(), score);

        assert_eq!(cache.get(other_position.encode()), None);
    }
}
