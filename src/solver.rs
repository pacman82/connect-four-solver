use std::cmp::{max, min, Ordering};

use crate::{
    precalculated::precalculated_score, transposition_table::TranspositionTable, Column, ConnectFour
};

/// Reusing the same solver instead of repeatedly running score in order to calculate similar
/// positions, may have performance benefits, because we can reuse the transposition table.
pub struct Solver {
    transposition_table: TranspositionTable,
}

impl Default for Solver {
    fn default() -> Self {
        Self::new()
    }
}

impl Solver {
    pub fn new() -> Solver {
        // 64Bit per entry. Let's hardcode it to use a prime close to 16777213 which multiplied by 8
        // Byte should be close to 128MiB.
        let transposition_table = TranspositionTable::new(16777213);
        Solver {
            transposition_table,
        }
    }

    /// Calculates the score of a connect four game. The score is set up so always picking the move with
    /// the lowest score results in perfect play. Perfect meaning winning as fast as possible, drawing
    /// or loosing as late as possible.
    ///
    /// A positive score means the player who can put in the next stone can win. Positions which can be
    /// won faster are scored higher. The score is 1 if the current player can win with his last stone.
    /// Two if he can win with his second to last stone and so on. A score of zero means the game will
    /// end in a draw if both players play perfectly. A negative score means the opponent (the player
    /// which is not putting in the next stone) is winnig. It is `-1` if the opponent is winning with
    /// his last stone. `-2` if he is winning second to last stone and so on.
    pub fn score(&mut self, game: &ConnectFour) -> i8 {
        precalculated_score(game)
            .unwrap_or_else(|| self.score_without_precalculated(game))
    }

    fn score_without_precalculated(&mut self, game: &ConnectFour) -> i8 {
        if game.is_victory() {
            return score_from_num_stones(game.stones() as i8);
        }

        // Check if we can win in the next move because `alpha_beta` assumes that the next move can not
        // win the game.
        if game.can_win_in_next_move() {
            return -score_from_num_stones(game.stones() as i8 + 1);
        }

        // 64Bit per entry. Let's hardcode it to use a prime close to 16777213 which multiplied by 8
        // Byte should be close to 128MiB.
        let mut min = -(42 - game.stones() as i8) / 2;
        let mut max = (42 + 1 - game.stones() as i8) / 2;

        // Iterative deepening
        while min < max {
            let median = min + (max - min) / 2;
            let alpha = if median <= 0 && min / 2 < median {
                // Explore loosing path deeper
                min / 2
            } else if median >= 0 && max / 2 > median {
                // Explore winning path deeper
                max / 2
            } else {
                median
            };
            let result = alpha_beta(game, alpha, alpha + 1, &mut self.transposition_table);
            if result <= alpha {
                max = result;
            } else {
                min = result;
            }
        }
        debug_assert_eq!(min, max);
        min
    }

    /// Fills `best_moves` with all the legal moves, which have the best strong score.
    pub fn best_moves(&mut self, game: &ConnectFour, best_moves: &mut Vec<Column>) {
        if game.is_over() {
            return;
        }
        let mut min = i8::MAX;
        for column in game.legal_moves() {
            let mut board = *game;
            board.play(column);
            let score = self.score(&board);
            match score.cmp(&min) {
                Ordering::Less => {
                    min = score; 
                    best_moves.clear();
                    best_moves.push(column);
                },
                Ordering::Equal => {
                    best_moves.push(column);
                },
                Ordering::Greater => (),
            };
        }
    }
}

/// Calculates the score of a connect four game. The score is set up so always picking the move with
/// the lowest score results in perfect play. Perfect meaning winning as fast as possible, drawing
/// or loosing as late as possible.
///
/// A positive score means the player who can put in the next stone can win. Positions which can be
/// won faster are scored higher. The score is 1 if the current player can win with his last stone.
/// Two if he can win with his second to last stone and so on. A score of zero means the game will
/// end in a draw if both players play perfectly. A negative score means the opponent (the player
/// which is not putting in the next stone) is winnig. It is `-1` if the opponent is winning with
/// his last stone. `-2` if he is winning second to last stone and so on.
pub fn score(game: &ConnectFour) -> i8 {
    Solver::new().score(game)
}

/// Score of the position with alepha beta pruning.
///
/// Assumes that position can not be won in a single move. Assumes that position is not won position
/// already.
///
/// * If actual score is smaller than alpha then: actual score <= return value <= alpha
/// * If actual score is bigger than beta then: actual score >= return value >= beta
/// * If score is within alpha beta window precise score is returned
///
/// If alpha is higher (or equal) than the score of this position, we can prune this position,
/// because the current player would not play this route, since he is guaranteed to achieve a better
/// outcome with some other play.
///
/// Similarly if this positions score is higher than beta we can prune it, since the opponent would
/// choose a different line of play, which leavs him in a better position.
///
/// Alpha is a lower bound on what the current player can expect. Beta is as upper bound on what he
/// can expect.
fn alpha_beta(
    game: &ConnectFour,
    mut alpha: i8,
    mut beta: i8,
    cached_beta: &mut TranspositionTable,
) -> i8 {
    debug_assert!(alpha < beta);
    debug_assert!(!game.can_win_in_next_move());

    let possibilities = game.non_loosing_moves_impl();
    if possibilities.is_empty() {
        // If there are no possibilities for the current player not to loose, the opponent wins.
        return score_from_num_stones(game.stones() as i8 + 2);
    }

    // Check for draw
    if game.stones() >= 42 - 2 {
        return 0;
    }

    // Opponent can not win within one move, this gives us a lower bound for the score
    alpha = max(alpha, score_from_num_stones(game.stones() as i8 + 4));
    if alpha >= beta {
        return alpha;
    }

    // We may also find an upper bound in the cache. If not we use the fact that we know we can not
    // win with our next stone, which puts the fastest possible win at least three stones away.
    let upper_bound_beta = cached_beta
        .get(game.encode())
        .unwrap_or_else(|| -score_from_num_stones(game.stones() as i8 + 3));
    beta = min(beta, upper_bound_beta);
    if alpha >= beta {
        return beta;
    }

    let mut move_explorer = MoveExplorer::new();
    for col in 0..7 {
        if possibilities.contains(col) {
            move_explorer.add(col, game);
        }
    }
    move_explorer.sort();

    // We play the position which is the worst for our opponent
    for position in move_explorer.next_positions() {
        // Score from the perspective of the current player is the negative of the opponents.
        let score = -alpha_beta(&position, -beta, -alpha, cached_beta);
        // prune the exploration if we find a possible move better than what we were looking for.
        if score >= beta {
            return score;
        }
        // We only need to search for positions, which are better than the best so far.
        alpha = max(alpha, score);
    }

    // save the upper bound of the position
    cached_beta.put(game.encode(), alpha);
    alpha
}

/// Score from the perspective of the current player (who can no longer move, because the game is
/// over), assuming the last stone won after `num_stones`.
fn score_from_num_stones(num_stones: i8) -> i8 {
    // Remaining stones of the winning player.
    let remaining_stones = (42 - num_stones) / 2;
    // Score is from the perspective of the moving player. So if the current position is a win, it
    // is negative.
    -(remaining_stones + 1)
}

/// Stack allocated container for possible moves. Iterates over moves in a fashion which allows to
/// prune the search tree sooner.
struct MoveExplorer {
    /// Up to seven indices are possible. Store index, score and position.
    col_indices: [(u8, u32, ConnectFour); 7],
    /// Up to this index the moves are valid.
    len: usize,
}

impl MoveExplorer {
    pub fn new() -> Self {
        Self {
            col_indices: [(0, 0, ConnectFour::new()); 7],
            len: 0,
        }
    }

    pub fn add(&mut self, col_index: u8, from: &ConnectFour) {
        let mut next_position = *from;
        let is_legal = next_position.play(Column::from_index(col_index));
        debug_assert!(is_legal);
        let score = next_position.heuristic();
        self.col_indices[self.len] = (col_index, score, next_position);
        self.len += 1;
    }

    pub fn sort(&mut self) {
        /// Indices which should get explored first get smaller values. Explore center moves first.
        /// These are better on average. This allows for faster pruning.
        const COLUMN_PRIORITY: [u8; 7] = [6, 4, 2, 0, 1, 3, 5];
        self.col_indices[..self.len].sort_unstable_by(|a, b| {
            // sort by score first, then by column priority. We prefer higher scores, therfore a, b
            // are switched in order.
            b.1.cmp(&a.1)
                .then_with(|| COLUMN_PRIORITY[a.0 as usize].cmp(&COLUMN_PRIORITY[b.0 as usize]))
        });
    }

    pub fn next_positions(&self) -> impl Iterator<Item = ConnectFour> + '_ {
        self.col_indices[..self.len].iter().map(|(_, _, pos)| *pos)
    }
}
