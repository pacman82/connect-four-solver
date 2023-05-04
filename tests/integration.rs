use connect_four_solver::{score, ConnectFour};

#[test]
fn should_detect_win_of_player_one() {
    // | | | | | | | |
    // | | | | | | | |
    // | | | | | |O| |
    // | | | |O|O|O| |
    // | | |X|X|X|X| |
    // | |O|X|X|X|O| |
    // ---------------
    //  0 1 2 3 4 5 6
    let game = ConnectFour::from_move_list("4544552531332");

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
    //  0 1 2 3 4 5 6
    let game = ConnectFour::from_move_list("1141465142351133000452254232560240330566");
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
    //  0 1 2 3 4 5 6
    let game = ConnectFour::from_move_list("114146514235113300045225423256024033056");
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
    //  0 1 2 3 4 5 6
    let game = ConnectFour::from_move_list("1141465142351133000452254232560240330");

    assert_eq!(-1, score(&game))
}