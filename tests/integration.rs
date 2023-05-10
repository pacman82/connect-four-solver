use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use connect_four_solver::{score, ConnectFour};

/// Construct game state from move list and print it correctly.
#[test]
fn construct_from_move_list() {
    let game = ConnectFour::from_move_list("5655663642443");
    let mut out = Vec::new();
    game.print_to(&mut out).unwrap();
    let out = String::from_utf8(out).unwrap();

    let expected = "\
        | | | | | | | |\n\
        | | | | | | | |\n\
        | | | | | |O| |\n\
        | | | |O|O|O| |\n\
        | | |X|X|X|X| |\n\
        | |O|X|X|X|O| |\n\
        ---------------\n \
         1 2 3 4 5 6 7\n\
    ";
    assert_eq!(expected, out);
}

#[test]
fn should_detect_win_of_player_one() {
    // | | | | | | | |
    // | | | | | | | |
    // | | | | | |O| |
    // | | | |O|O|O| |
    // | | |X|X|X|X| |
    // | |O|X|X|X|O| |
    // ---------------
    //  1 2 3 4 5 6 7
    let game = ConnectFour::from_move_list("5655663642443");

    assert!(game.is_victory());
}

#[test]
fn can_not_win_with_next_move() {
    // |X|O|O|O|X|O| |
    // |O|X|O|X|X|X| |
    // |X|O|O|X|O|O| |
    // |X|O|X|O|X|X|X|
    // |O|O|O|X|X|O|O|
    // |X|X|O|X|X|X|O|
    // ---------------
    //  1 2 3 4 5 6 7
    let game = ConnectFour::from_move_list("225257625346224411156336534367135144167");

    assert!(!game.can_win_in_next_move())
}

#[test]
fn can_win_with_next_move() {
    // | | | | | | | |
    // | | | | | | | |
    // | | | | | | | |
    // | |O|O|X|X| | |
    // | |X|X|O|O| |X|
    // | |X|X|O|O|X|O|
    // ---------------
    //  1 2 3 4 5 6 7
    let game = ConnectFour::from_move_list("253733227554644");

    assert!(game.can_win_in_next_move())
}

#[test]
fn score_depth_one_victory() {
    // |X|O|O|O|X|O| |
    // |O|X|O|X|X|X| |
    // |X|O|O|X|O|O|O|
    // |X|O|X|O|X|X|X|
    // |O|O|O|X|X|O|O|
    // |X|X|O|X|X|X|O|
    // ---------------
    //  1 2 3 4 5 6 7
    let game = ConnectFour::from_move_list("2252576253462244111563365343671351441677");
    assert_eq!(1, score(&game))
}

#[test]
fn score_depth_two_victory() {
    // |X|O|O|O|X|O| |
    // |O|X|O|X|X|X| |
    // |X|O|O|X|O|O| |
    // |X|O|X|O|X|X|X|
    // |O|O|O|X|X|O|O|
    // |X|X|O|X|X|X|O|
    // ---------------
    //  1 2 3 4 5 6 7
    let game = ConnectFour::from_move_list("225257625346224411156336534367135144167");
    assert_eq!(-1, score(&game))
}

#[test]
fn score_depth_four_victory() {
    // |X|O|O|O|X| | |
    // |O|X|O|X|X|X| |
    // |X|O|O|X|O|O| |
    // |X|O|X|O|X|X| |
    // |O|O|O|X|X|O|O|
    // |X|X|O|X|X|X|O|
    // ---------------
    //  1 2 3 4 5 6 7
    let game = ConnectFour::from_move_list("2252576253462244111563365343671351441");

    assert_eq!(-1, score(&game))
}

#[test]
fn score_begin_easy() {
    let game = ConnectFour::from_move_list("253733227554");
    assert_eq!(-9, score(&game))
}

#[test]
fn score_begin_medium() {
    let game = ConnectFour::from_move_list("13555111322723");
    assert_eq!(-1, score(&game))

}

#[test]
fn middle_medium() {
    let game = ConnectFour::from_move_list("533772466715155");
    assert_eq!(-1, score(&game));
}

#[test]
// #[ignore = "too slow"]
fn thousand_end_easy() {
    let test_data = "./tests/Test_L3_R1";
    // Verify we give the correct score for each line in the dataset
    verify_test_data(test_data);
}

#[test]
// #[ignore = "too slow"]
fn thousand_middle_easy() {
    let test_data = "./tests/Test_L2_R1";
    // Verify we give the correct score for each line in the dataset
    verify_test_data(test_data);
}

#[test]
// #[ignore = "too slow"]
fn thousand_middle_medium() {
    let test_data = "./tests/Test_L2_R2";
    // Verify we give the correct score for each line in the dataset
    verify_test_data(test_data);
}

#[test]
// #[ignore = "too slow"]
fn thousand_begin_easy() {
    let test_data = "./tests/Test_L1_R1";
    // Verify we give the correct score for each line in the dataset
    verify_test_data(test_data);
}

fn verify_test_data(test_data: &str) {
    let input = BufReader::new(File::open(test_data).unwrap());

    for line in input.lines() {
        let line = line.unwrap();
        let mut line_it = line.split_whitespace();
        let game = ConnectFour::from_move_list(line_it.next().unwrap());
        let expected_score: i8 = line_it.next().unwrap().parse().unwrap();

        let actual_score = score(&game);

        assert_eq!(expected_score, actual_score)
    }
}
