use std::cmp::{max, min};

use crate::{transposition_table::TranspositionTable, Column, ConnectFour};

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
    if game.is_victory() {
        return score_from_num_stones(game.stones() as i8);
    }

    // Check if we can win in the next move because `alpha_beta` assumes that the next move can not
    // win the game.
    if game.can_win_in_next_move() {
        return -score_from_num_stones(game.stones() as i8 + 1);
    }

    // 64Bit per entry. Let's hardcode it to use a prime close to 64MB.
    let mut cached_beta = TranspositionTable::new(8388593);
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
        let result = alpha_beta(game, alpha, alpha + 1, &mut cached_beta);
        if result <= alpha {
            max = result;
        } else {
            min = result;
        }
    }
    debug_assert_eq!(min, max);
    min
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

    // Explore center moves first. These are better on average. This allows for faster pruning.
    const MOVE_EXPLORATION_ORDER: [u8; 7] = [3, 4, 2, 5, 1, 6, 0];

    let possibilities = game.non_loosing_moves();
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

    // We play the position which is the worst for our opponent
    for col in MOVE_EXPLORATION_ORDER {
        if !possibilities.contains(col) {
            // Discard any move which would loose the game immediatly
            continue;
        }
        let mut next = *game;
        let is_legal = next.play(Column::from_index(col));
        debug_assert!(is_legal);

        // Score from the perspective of the current player is the negative of the opponents.
        let score = -alpha_beta(&next, -beta, -alpha, cached_beta);
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

/// Calculates the score of a connect four game. The score is set up so always picking the move with
/// the highest score results in perfect play. Perfect meaning winning as fast as possible, drawing
/// or loosing as late as possible.
///
/// A positive score means the player who did put in the last stone can win. Positions which can be
/// won faster are scored higher. The score is one if the last player can win with his last stone.
/// Two if he can win with his second to last stone and so on. A score of zero means the game will
/// end in a draw if both players play perfectly. A negative score means the opponent (the player
/// which is not putting the next stone) is winnig. It is `-1` if the opponent is winning with his
/// last stone. `-2` if he is winning second to last stone and so on.
pub fn score2(game: &ConnectFour) -> i32 {
    if game.is_victory() {
        let player_stones_left = (42 - game.stones() as i32) / 2;
        return player_stones_left + 1;
    }
    // Draw game
    if game.stones() == 42 {
        return 0;
    }

    let best_score_for_current_player = (0..7)
        .filter_map(|col| {
            let mut next = *game;
            next.play(Column::from_index(col)).then(|| score2(&next))
        })
        .max()
        .expect("There must be at least one legal move");

    // Score from the perspective of the current player is the negative of the opponents.
    -best_score_for_current_player
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
