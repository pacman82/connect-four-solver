use connect_four_solver::{Column, ConnectFour, Solver};

#[test]
fn pick_best_move() {
    let mut solver = Solver::new();
    let game = ConnectFour::new();
    let mut best_moves = Vec::new();
    solver.best_moves(&game, &mut best_moves);

    assert_eq!(&[Column::from_index(3)][..], &best_moves);
}

#[test]
fn non_loosing_moves() {
    let game = ConnectFour::from_move_list("123242");
    let mut moves = game.non_loosing_moves();
    assert_eq!(Some(Column::from_index(1)), moves.next());
    assert_eq!(None, moves.next());
}