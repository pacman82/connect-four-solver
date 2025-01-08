use std::io::{stdin, stdout, self, BufRead};

use connect_four_solver::{score, Column, ConnectFour};

fn main() -> io::Result<()>{
    println!("\
        Place a stone in the connect four board by typing the column number 1-7. Press s to
        calculate score of current position.");

    let mut game = ConnectFour::new();
    let mut input = stdin().lock();
    let mut line = String::new();

    while !game.is_over() {
        game.print_to(stdout())?;

        line.clear();
        input.read_line(&mut line)?;
        if line.trim() == "s"{
            print_scores(game);
            continue;
        }
        if let Ok(col) = line.parse() {
            game.play(col);
        } else {
            println!("Invalid column.");
            continue;
        };
    }

    Ok(())
}

fn print_scores(game: ConnectFour) {
    for col_index in 0..7 {
        if !game.is_legal_move(Column::from_index(col_index)) {
            continue;
        }

        let mut game_copy = game;
        if game_copy.play(Column::from_index(col_index)) {
            let score = score(&game_copy);
            let col = col_index + 1;
            let stones_to_end = stones_to_end(game.stones() as i8, score);
            let result_msg = match score.signum() {
                0 => "Draw",
                1 => "Loss",
                -1 => "Win",
                _ => unreachable!()
            };
            println!("{col}: {result_msg} in {stones_to_end} stones.");
        }
    }
}

fn stones_to_end(current_turn: i8, score: i8) -> i8 {
    if score == 0 {
        return 42 - current_turn;
    }

    let remaining_stones_at_end = score.abs() - 1;
    let remaining_stones_now = 42 / 2 - current_turn / 2;
    let stones_for_winner_to_throw = remaining_stones_now - remaining_stones_at_end;
    if score.is_positive() {
        stones_for_winner_to_throw * 2
    } else {
        (stones_for_winner_to_throw - 1) * 2 + 1
    }
}