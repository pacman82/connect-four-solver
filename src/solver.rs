use std::cmp::{max, min};

use crate::{Column, ConnectFour};

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
pub fn score(game: &ConnectFour) -> i32 {
    alpha_beta(game, -21, 21)
}

/// Score of the position with alepha beta pruning.
///
/// * If actual score is smaller than alpha then: actual score <= return value <= alpha
/// * If actual score is bigger than beta then: actual score >= return value >= beta
/// * If score is within alpha beta window precise score is returned
fn alpha_beta(game: &ConnectFour, mut alpha: i32, mut beta: i32) -> i32 {
    // Draw game
    if game.stones() == 42 {
        return 0;
    }

    // Check if current player can win with next move
    let current_player_can_win_in_next_move = (0..7)
        .filter_map(|col| {
            let mut next = *game;
            next.play(&Column::from_index(col)).then_some(next)
        })
        .any(|next| next.is_victory());

    let score_if_current_player_wins_next_move = (42 + 1 - game.stones() as i32) / 2;
    if current_player_can_win_in_next_move {
        return score_if_current_player_wins_next_move;
    }

    // Current player can not win in one move. This gives us an upper bound for the score
    let max_score = score_if_current_player_wins_next_move - 1;
    // Narrow the search window with new upper bound
    beta = min(beta, max_score);
    // Check if search window is empty. Prune exploration, if so.
    if beta <= alpha {
        return beta;
    }

    // We play the position which is the worst for our opponent
    for col in 0..7 {
        let mut next = *game;
        if next.play(&Column::from_index(col)) {
            // Score from the perspective of the current player is the negative of the opponents.
            let score = -alpha_beta(&next, -beta, -alpha);
            if score >= beta {
                return score;
            }
            // We only need to search for positions, which are better than the best so far.
            alpha = max(alpha, score);
        }
    }

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
            next.play(&Column::from_index(col))
                .then(|| score2(&next))
        })
        .max()
        .expect("There must be at least one legal move");

    // Score from the perspective of the current player is the negative of the opponents.
    -best_score_for_current_player
}
