use connect_four_solver::ConnectFour;

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
    
    assert!(game.is_player_one_victory());
}