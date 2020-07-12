use {
    crate::networking::ServerMessage,
    std::{
        collections::HashMap,
        cell::RefCell,
        rc::Rc,
    },
    serde::{Serialize, Deserialize},
};

#[cfg(test)]
mod test;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum InvalidMove {
    PieceAlreadyPresent,
    Suicide,
    KoRule,
    InvalidTurn,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Move<T: Copy>(pub Option<(Position<T>, Option<Stone>)>, pub Option<String>);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position<T: Copy>(pub T, pub T);

impl<T: Copy> Position<T> {
    pub fn x(&self) -> T {
        self.0
    }
    
    pub fn y(&self) -> T {
        self.1
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Stone {
    White,
    Black,
}

pub fn other_cell(stone: Stone) -> Stone {
    if stone == Stone::White {
        Stone::Black
    } else {
        Stone::White
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Board {
    pub prev_stones: Vec<Vec<Option<Stone>>>,
    pub stones: Vec<Vec<Option<Stone>>>,
}

impl Board {
    pub fn new(size_x: usize, size_y: usize) -> Board {
        Board {
            prev_stones: vec![vec![None; size_x]; size_y],
            stones: vec![vec![None; size_x]; size_y],
        }
    }

    pub fn play(&mut self, update_vector: bool, position: Position<usize>, stone: Stone) -> Result<Option<Vec<ServerMessage>>, InvalidMove> {
        if let Some(_) = self.stones[position.y()][position.x()] {
            return Err(InvalidMove::PieceAlreadyPresent)
        }

        let original_board = self.stones.clone();

        self.stones[position.y()][position.x()] = Some(stone);

        let mut board_delta = Vec::new();

        board_delta.push(ServerMessage::PlaceResponse(
                Ok(Move(Some((Position(position.x() as u8, position.y() as u8), Some(stone))), None))));

        let mut original_surrounding_positions: Vec<Position<usize>> = Vec::with_capacity(4);
        let mut surrounding_positions: Vec<(bool, Position<usize>, Rc<RefCell<HashMap<Position<usize>, ()>>>)> = Vec::with_capacity(4);

        let mut piece_captured = false;

        for delta in [-1, 1].iter() {
            if position.x() as isize + delta >= 0 {
                original_surrounding_positions.push(Position((position.x() as isize + delta) as usize, position.y()));
            }

            if position.y() as isize + delta >= 0 {
                original_surrounding_positions.push(Position(position.x(), (position.y() as isize + delta) as usize));
            }
        }

        for surrounding_position in &original_surrounding_positions {
            let mut unique = true;

            for other_surrounding_position in &surrounding_positions {
                if let Some(_) = other_surrounding_position.2.as_ref().borrow().get(surrounding_position) {
                    unique = false;
                }
            }

            if unique {
                if surrounding_position.x() < self.stones[0].len() && surrounding_position.y() < self.stones.len() {
                    if let Some(other_piece) = self.stones[surrounding_position.y()][surrounding_position.x()] {
                        if other_piece != stone {
                            surrounding_positions.push((false, surrounding_position.clone(), Rc::new(RefCell::new(HashMap::new()))));
                            
                            let pos_surrounding_position = surrounding_positions.len() - 1;
                            if self.count_liberties(surrounding_position.clone(),
                                    Rc::clone(&surrounding_positions[pos_surrounding_position].2),
                                    other_cell(stone)) == 0 {
                                piece_captured = true;
                                surrounding_positions[pos_surrounding_position].0 = true;
                            }
                        }
                    }
                }
            }
        }

        if piece_captured {
            for surrounding_position in &surrounding_positions {
                if surrounding_position.0 {
                    surrounding_position.2.as_ref().borrow()
                        .iter()
                        .for_each(|(pos, _)| {
                            self.stones[pos.y()][pos.x()] = None;
                            if update_vector {
                                board_delta.push(ServerMessage::PlaceResponse(
                                        Ok(Move(Some((Position(pos.x() as u8, pos.y() as u8), None)), None))));
                            }
                        });
                }
            }

            if self.stones == self.prev_stones {
                self.stones = original_board;
                return Err(InvalidMove::KoRule);
            }
        } else {
            if self.count_liberties(position.clone(), Rc::new(RefCell::new(HashMap::new())), stone) == 0 {
                self.stones[position.y()][position.x()] = None;
                return Err(InvalidMove::Suicide);
            }
        }

        self.prev_stones = original_board;

        if update_vector {
            return Ok(Some(board_delta));
        } else {
            return Ok(None);
        }
    }

    fn count_liberties(&self, position: Position<usize>, prev_positions: Rc<RefCell<HashMap<Position<usize>, ()>>>, cell_owner: Stone) -> u32 {
        prev_positions.borrow_mut().insert(position.clone(), ());

        let mut total_liberties = 0;

        for delta in [-1, 1].iter() {
            let new_positions = [
                Position(position.x() as isize + delta, position.y() as isize),
                Position(position.x() as isize, position.y() as isize + delta),
            ];

            for new_position_signed in &new_positions {
                if new_position_signed.x() < 0 || new_position_signed.y() < 0 {
                    continue;
                }

                let new_position = Position(new_position_signed.x() as usize, new_position_signed.y() as usize);

                if let Some(_) = prev_positions.borrow().get(&new_position) {
                    continue;
                }

                if let Some(board_x) = self.stones.get(new_position.y()) {
                    if let Some(piece) = board_x.get(new_position.x()) {
                        match piece {
                            Some(new_cell) if *new_cell == cell_owner => {
                                total_liberties += self.count_liberties(new_position, Rc::clone(&prev_positions), cell_owner)
                            }

                            Some(_) => (),

                            None => total_liberties += 1,
                        }
                    } 
                }
            }
        }

        total_liberties
    }
}
