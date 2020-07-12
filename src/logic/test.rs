use super::*;

#[test]
fn one_cell_one_board_count_liberties() {
    let mut board = Board::new(1, 1);
    board.stones[0][0] = Some(Stone::White);

    assert_eq!(0, board.count_liberties(Position(0, 0), Rc::new(RefCell::new(HashMap::new())), Stone::White));
}

#[test]
fn one_cell_count_liberties() {
    let mut board = Board::new(9, 9);
    board.stones[2][1] = Some(Stone::White);

    assert_eq!(4, board.count_liberties(Position(1, 2), Rc::new(RefCell::new(HashMap::new())), Stone::White))
}

#[test]
fn two_stones_count_liberties() {
    let mut board = Board::new(9, 9);
    board.stones[2][1] = Some(Stone::White);
    board.stones[3][1] = Some(Stone::White);

    assert_eq!(6, board.count_liberties(Position(1, 2), Rc::new(RefCell::new(HashMap::new())), Stone::White))
}

#[test]
fn three_stones_count_liberties() {
    let mut board = Board::new(9, 9);
    board.stones[3][3] = Some(Stone::White);
    board.stones[3][4] = Some(Stone::White);
    board.stones[3][5] = Some(Stone::White);

    assert_eq!(8, board.count_liberties(Position(4, 3), Rc::new(RefCell::new(HashMap::new())), Stone::White))
}

#[test]
fn play_in_eye() {
    let mut board = Board::new(9, 9);
    board.stones[3][3] = Some(Stone::Black);
    board.stones[2][3] = Some(Stone::White);
    board.stones[2][4] = Some(Stone::White);
    board.stones[4][3] = Some(Stone::White);
    board.stones[4][4] = Some(Stone::White);
    board.stones[3][2] = Some(Stone::White);
    board.stones[3][5] = Some(Stone::White);

    assert_eq!(InvalidMove::Suicide, board.play(false, Position(4, 3), Stone::Black).unwrap_err());
}

#[test]
fn ko_rule() {
    let mut board = Board::new(9, 9);
    board.stones[3][3] = Some(Stone::Black);
    board.stones[4][2] = Some(Stone::Black);
    board.stones[4][4] = Some(Stone::Black);
    board.stones[5][3] = Some(Stone::Black);
    board.stones[5][2] = Some(Stone::White);
    board.stones[5][4] = Some(Stone::White);
    board.stones[6][3] = Some(Stone::White);

    board.play(false, Position(3, 4), Stone::White).unwrap();
    assert_eq!(InvalidMove::KoRule, board.play(false, Position(3, 5), Stone::Black).unwrap_err());
}
