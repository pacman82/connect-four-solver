use std::{fs::File, io::{BufReader, BufRead}};

use connect_four_solver::{score, ConnectFour, score2};

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
fn end_easy() {
    let test_data = "./tests/Test_L3_R1";
    // Verify we give the correct score for each line in the dataset
    verify_test_data(test_data);
}

#[test]
#[ignore = "takes long"]
fn middle_easy() {
    let test_data = "./tests/Test_L2_R1";
    // Verify we give the correct score for each line in the dataset
    verify_test_data(test_data);
}

#[test]
#[ignore = "takes long"]
fn middle_medium() {
    let test_data = "./tests/Test_L2_R2";
    // Verify we give the correct score for each line in the dataset
    verify_test_data(test_data);
}

fn verify_test_data(test_data: &str) {
    let input = BufReader::new(File::open(test_data).unwrap());

    for line in input.lines() {
        let line = line.unwrap();
        let mut line_it = line.split_whitespace();
        let game = ConnectFour::from_move_list(line_it.next().unwrap());
        let expected_score: i32 = line_it.next().unwrap().parse().unwrap();

        let actual_score = score(&game);

        assert_eq!(expected_score, actual_score)
    }
}

#[test]
#[ignore = "much slower"]
fn end_easy_score_2() {
    // Verify we give the correct score for each line in the dataset
    let input = BufReader::new(File::open("./tests/Test_L3_R1").unwrap());

    for line in input.lines() {
        let line = line.unwrap();
        let mut line_it = line.split_whitespace();
        let game = ConnectFour::from_move_list(line_it.next().unwrap());
        let expected_score: i32 = line_it.next().unwrap().parse().unwrap();

        let actual_score = -score2(&game);

        assert_eq!(expected_score, actual_score)
    }
}