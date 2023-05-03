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

// #[test]
// fn score_should_be_18_for_4455() {
//     // | | | | | | | |
//     // | | | | | | | |
//     // | | | | | | | |
//     // | | | | | | | |
//     // | | | |O|O| | |
//     // | | | |X|X| | |
//     // ---------------
//     //  0 1 2 3 4 5 6
//     let game = ConnectFour::from_move_list("3344");

//     // `X` can win in two turns with its fourth stone
//     assert_eq!(18, score(&game))
// }

#[test]
fn score_minus_one() {
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
