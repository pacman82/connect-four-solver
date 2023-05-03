use crate::{Column, ConnectFour};

/// Calculates the score of a connect four game. ~~The score is set up so always picking the move with
/// the highest score results in perfect play. Perfect meaning winning as fast as possible, drawing
/// or loosing as late as possible.~~
///
/// A positive score means the player who can put in the next stone can win. Positions which can be
/// one faster are scored higher. The score is 1 if the current player can win with his last stone.
/// Two if he can win with his second to last stone and so on. A score of zero means the game will
/// end in a draw if both players play perfectly. A negative score means the opponent (the player
/// which is not putting in the next stone) is winnig. It is `-1` if the opponent is winning with
/// his last stone. `-2` if he is winning second to last stone and so on.
pub fn score(game: &ConnectFour) -> i32 {
    // Draw game
    if game.stones() == 7 * 6 {
        return 0;
    }

    // Check if current player can win with next move
    let current_player_can_win_in_next_move = (0..7).filter_map(|col| {
        let mut next = *game;
        next.play_move(&Column::from_index(col)).then_some(next)
    }).any(|next| next.is_victory());

    if current_player_can_win_in_next_move {
        return (7 * 6 + 1) - game.stones() as i32 / 2;
    }

    // Opponent gets
    let best_score_for_opponent = (0..7).filter_map(|col| {
        let mut next = *game;
        next.play_move(&Column::from_index(col)).then(|| score(&next))
    }).max().expect("There must be at least one legal move");

    // Score from the perspective of the current player is the negative of the opponents.
    -best_score_for_opponent
}
