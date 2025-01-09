//! Precalculate the scores for starting postitions. See `precalculated.rs` for more information.

use std::{
    fs::File,
    io::{BufWriter, Write},
};

use connect_four_solver::{score, ConnectFour};
use rayon::iter::{IntoParallelRefIterator, ParallelExtend, ParallelIterator};

const PRECALULATE_UP_TO_NUM_STONES: usize = 5;

fn main() {
    // Hold all unique game positions for `n` stones at index n.
    let mut unique_boards: Vec<ConnectFour> = Vec::new();
    let mut scores = Vec::new();

    for num_stones in 0..PRECALULATE_UP_TO_NUM_STONES {
        let mut new_boards = Vec::new();
        for board in &unique_boards {
            for col in board.legal_moves() {
                let mut new_board = *board;
                new_board.play(col);
                new_boards.push(new_board);
            }
        }
        // For the first iteration we start a new empty game.
        if unique_boards.is_empty() {
            new_boards.push(ConnectFour::new());
        };
        eprintln!(
            "For {num_stones} stones: Checked {} permutations",
            new_boards.len()
        );
        new_boards.sort_by_key(ConnectFour::encode);
        new_boards.dedup();
        eprintln!("Unique boards: {}", new_boards.len());
        unique_boards = new_boards;

        eprintln!("Calculating scores ...");
        scores.par_extend(
            unique_boards
                .par_iter()
                .map(|board| (board.encode(), score(board))),
        );
    }

    eprintln!("NUM_STONES_PRECALCULATED: {PRECALULATE_UP_TO_NUM_STONES}");
    eprintln!("NUM_SCORES_PRECALCULATED: {}", scores.len());

    let file = File::create("scores.dat").unwrap();
    let mut out = BufWriter::new(file);

    scores.sort_by_key(|(key, _)| *key);

    for (key, score) in scores {
        out.write_all(&key.to_le_bytes()).unwrap();
        out.write_all(&score.to_le_bytes()).unwrap();
    }
    out.flush().unwrap();
}
