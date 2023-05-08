use std::io::{stdin, stdout, self, BufRead};

use connect_four_solver::{ConnectFour, score};

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
            println!("Calculating score ...");
            let score = score(&game);
            println!("Score is {score}");
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