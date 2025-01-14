use connect_four_solver::{Column, ConnectFour, Solver};

#[test]
fn pick_best_move() {
    let mut solver = Solver::new();
    let game = ConnectFour::new();
    let mut best_moves = Vec::new();
    solver.best_moves(&game, &mut best_moves);

    assert_eq!(&[Column::from_index(3)][..], &best_moves);
}