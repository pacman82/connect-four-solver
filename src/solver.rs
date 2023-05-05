use crate::{Column, ConnectFour};

/// Calculates the score of a connect four game. The score is set up so always picking the move with
/// the lowest score results in perfect play. Perfect meaning winning as fast as possible, drawing
/// or loosing as late as possible.
///
/// A positive score means the player who can put in the next stone can win. Positions which can be
/// one faster are scored higher. The score is 1 if the current player can win with his last stone.
/// Two if he can win with his second to last stone and so on. A score of zero means the game will
/// end in a draw if both players play perfectly. A negative score means the opponent (the player
/// which is not putting in the next stone) is winnig. It is `-1` if the opponent is winning with
/// his last stone. `-2` if he is winning second to last stone and so on.
pub fn score(game: &ConnectFour) -> i32 {
    // Draw game
    if game.stones() == 42 {
        return 0;
    }

    // Check if current player can win with next move
    let current_player_can_win_in_next_move = (0..7).filter_map(|col| {
        let mut next = *game;
        next.play_move(&Column::from_index(col)).then_some(next)
    }).any(|next| next.is_victory());

    if current_player_can_win_in_next_move {
        return (42 + 1 - game.stones() as i32) / 2;
    }

    // We play the position which is the worst for our opponent
    let worst_score_for_opponent = (0..7).filter_map(|col| {
        let mut next = *game;
        next.play_move(&Column::from_index(col)).then(|| score(&next))
    }).min().expect("There must be at least one legal move");

    // Score from the perspective of the current player is the negative of the opponents.
    -worst_score_for_opponent
}

pub fn score_slow(game: &ConnectFour) -> i32 {
    if game.is_victory() {
        let player_stones_left = (42 - game.stones() as i32) / 2;
        return -(player_stones_left + 1);
    }
    // Draw game
    if game.stones() == 42 {
        return 0;
    }

    let worst_score_for_opponent = (0..7).filter_map(|col| {
        let mut next = *game;
        next.play_move(&Column::from_index(col)).then(|| score_slow(&next))
    }).min().expect("There must be at least one legal move");

    // Score from the perspective of the current player is the negative of the opponents.
    -worst_score_for_opponent
}