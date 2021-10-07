use std::fmt;

use crate::{
    player::Player,
    r#move::{Move, Position},
};

#[derive(Debug)]
pub struct Board {
    pub coords: Vec<Vec<Option<Player>>>,
}

const INITIAL_WHITE_POSITIONS: [(i32, i32); 12] = [
    (1, 0),
    (3, 0),
    (5, 0),
    (7, 0),
    (0, 1),
    (2, 1),
    (4, 1),
    (6, 1),
    (1, 2),
    (3, 2),
    (5, 2),
    (7, 2),
];

const INITIAL_RED_POSITIONS: [(i32, i32); 12] = [
    (0, 5),
    (2, 5),
    (4, 5),
    (6, 5),
    (1, 6),
    (3, 6),
    (5, 6),
    (7, 6),
    (0, 7),
    (2, 7),
    (4, 7),
    (6, 7),
];

impl Board {
    pub fn new() -> Self {
        Self {
            coords: (0..8).fold(vec![], |mut rows, x| {
                let row = (0..8).fold(vec![], |mut row, y| {
                    let coord = if INITIAL_WHITE_POSITIONS.contains(&(x, y)) {
                        Some(Player::White)
                    } else if INITIAL_RED_POSITIONS.contains(&(x, y)) {
                        Some(Player::Red)
                    } else {
                        None
                    };

                    row.push(coord);

                    row
                });

                rows.push(row);

                rows
            }),
        }
    }

    pub fn make_move(&mut self, current_player: &Player, mov: &Move) -> bool {
        let Move {
            initial,
            destination,
            ..
        } = mov;

        if !mov.is_valid(current_player) {
            return false;
        }

        let opt_player = self.get(initial.x as usize, initial.y as usize);

        if opt_player.is_none() {
            return false;
        }

        let player = opt_player.unwrap();

        if current_player != player {
            return false;
        }

        let opt_dest_player = self.get(destination.x as usize, destination.y as usize);

        if opt_dest_player.is_some() {
            return false;
        }

        if !mov.is_jump(current_player) && self.is_jumping_possible(current_player, mov) {
            return false;
        }

        if let Some(pos) = mov.jumped_position(current_player) {
            let Position { x, y } = pos;

            let opt_jumped_player = self.get(x as usize, y as usize);

            if opt_jumped_player.is_none() {
                return false;
            }

            let jumped_player = opt_jumped_player.unwrap();

            if current_player == jumped_player {
                return false;
            }

            self.set(x as usize, y as usize, None);
        }

        self.set(initial.x as usize, initial.y as usize, None);
        self.set(
            destination.x as usize,
            destination.y as usize,
            Some(*current_player),
        );

        return true;
    }

    fn set(&mut self, x: usize, y: usize, value: Option<Player>) {
        self.coords[x][y] = value;
    }

    pub fn get(&mut self, x: usize, y: usize) -> Option<&Player> {
        self.coords[x][y].as_ref()
    }

    pub fn red_score(&self) -> usize {
        self.coords
            .iter()
            .flatten()
            .filter(|coord| coord.is_some())
            .fold(0, |acc, opt_coord| {
                if opt_coord.unwrap().is_red() {
                    acc + 1
                } else {
                    acc
                }
            })
    }

    pub fn white_score(&self) -> usize {
        self.coords
            .iter()
            .flatten()
            .filter(|coord| coord.is_some())
            .fold(0, |acc, opt_coord| {
                if opt_coord.unwrap().is_white() {
                    acc + 1
                } else {
                    acc
                }
            })
    }

    fn is_jumping_possible(&self, player: &Player, mov: &Move) -> bool {
        let Position { x, y } = mov.initial;

        match player {
            Player::Red => {
                // check right
                let right = match self.coords.get((x + 1) as usize) {
                    // Out of bounds but that's ok we just keep going
                    None => false,
                    Some(row) => match row.get((y - 1) as usize) {
                        // Out of bounds but that's ok we just keep going
                        None => false,
                        Some(spot) => match spot {
                            // Empty Spot, the original x, y could move here
                            None => false,
                            Some(other_player) => {
                                // We might be able to jump in this case
                                if other_player != player {
                                    match self.coords.get((x + 2) as usize) {
                                        // Out of bounds but that's ok we just keep going
                                        None => false,
                                        Some(row) => match row.get((y - 2) as usize) {
                                            // Out of bounds but that's ok we just keep going
                                            None => false,
                                            Some(spot) => match spot {
                                                None => true,
                                                // Occupied
                                                Some(_) => false,
                                            },
                                        },
                                    }
                                } else {
                                    false
                                }
                            }
                        },
                    },
                };

                // check left
                let left = match self.coords.get((x - 1) as usize) {
                    // Out of bounds but that's ok we just keep going
                    None => false,
                    Some(row) => match row.get((y - 1) as usize) {
                        // Out of bounds but that's ok we just keep going
                        None => false,
                        Some(spot) => match spot {
                            // Empty Spot, the original x, y could move here
                            None => false,
                            Some(other_player) => {
                                // We might be able to jump in this case
                                if other_player != player {
                                    match self.coords.get((x - 2) as usize) {
                                        // Out of bounds but that's ok we just keep going
                                        None => false,
                                        Some(row) => match row.get((y - 2) as usize) {
                                            // Out of bounds but that's ok we just keep going
                                            None => false,
                                            Some(spot) => match spot {
                                                None => true,
                                                // Occupied
                                                Some(_) => false,
                                            },
                                        },
                                    }
                                } else {
                                    false
                                }
                            }
                        },
                    },
                };

                left || right
            }
            Player::White => {
                // check right
                let right = match self.coords.get((x + 1) as usize) {
                    None => false,
                    Some(row) => match row.get((y + 1) as usize) {
                        None => false,
                        Some(spot) => match spot {
                            None => false,
                            Some(other_player) => {
                                // We might be able to jump in this case
                                if other_player != player {
                                    match self.coords.get((x + 2) as usize) {
                                        None => false,
                                        Some(row) => match row.get((y + 2) as usize) {
                                            None => false,
                                            Some(spot) => match spot {
                                                None => true,
                                                // Occupied
                                                Some(_) => false,
                                            },
                                        },
                                    }
                                } else {
                                    false
                                }
                            }
                        },
                    },
                };

                // check left
                let left = match self.coords.get((x - 1) as usize) {
                    None => false,
                    Some(row) => match row.get((y + 1) as usize) {
                        None => false,
                        Some(spot) => match spot {
                            None => false,
                            Some(other_player) => {
                                // We might be able to jump in this case
                                if other_player != player {
                                    match self.coords.get((x - 2) as usize) {
                                        None => false,
                                        Some(row) => match row.get((y + 2) as usize) {
                                            None => false,
                                            Some(spot) => match spot {
                                                None => true,
                                                // Occupied
                                                Some(_) => false,
                                            },
                                        },
                                    }
                                } else {
                                    false
                                }
                            }
                        },
                    },
                };

                left || right
            }
        }
    }

    pub fn is_incomplete(&self) -> bool {
        if self.red_score() == 0 || self.white_score() == 0 {
            return false;
        }

        for y in 0..8 {
            for x in 0..8 {
                let x: i32 = x;
                let y: i32 = y;

                match &self.coords[x as usize][y as usize] {
                    None => (),
                    Some(player) => match player {
                        Player::Red => {
                            // check right
                            match self.coords.get((x + 1) as usize) {
                                // Out of bounds but that's ok we just keep going
                                None => (),
                                Some(row) => match row.get((y - 1) as usize) {
                                    // Out of bounds but that's ok we just keep going
                                    None => (),
                                    Some(spot) => match spot {
                                        // Empty Spot, the original x, y could move here
                                        None => return true,
                                        Some(other_player) => {
                                            // We might be able to jump in this case
                                            if other_player != player {
                                                match self.coords.get((x + 2) as usize) {
                                                    // Out of bounds but that's ok we just keep going
                                                    None => (),
                                                    Some(row) => match row.get((y - 2) as usize) {
                                                        // Out of bounds but that's ok we just keep going
                                                        None => (),
                                                        Some(spot) => match spot {
                                                            None => return true,
                                                            // Occupied
                                                            Some(_) => (),
                                                        },
                                                    },
                                                }
                                            }
                                        }
                                    },
                                },
                            };

                            // check left
                            match self.coords.get((x - 1) as usize) {
                                // Out of bounds but that's ok we just keep going
                                None => (),
                                Some(row) => match row.get((y - 1) as usize) {
                                    // Out of bounds but that's ok we just keep going
                                    None => (),
                                    Some(spot) => match spot {
                                        // Empty Spot, the original x, y could move here
                                        None => return true,
                                        Some(other_player) => {
                                            // We might be able to jump in this case
                                            if other_player != player {
                                                match self.coords.get((x - 2) as usize) {
                                                    // Out of bounds but that's ok we just keep going
                                                    None => (),
                                                    Some(row) => match row.get((y - 2) as usize) {
                                                        // Out of bounds but that's ok we just keep going
                                                        None => (),
                                                        Some(spot) => match spot {
                                                            None => return true,
                                                            // Occupied
                                                            Some(_) => (),
                                                        },
                                                    },
                                                }
                                            }
                                        }
                                    },
                                },
                            };
                        }
                        Player::White => {
                            // check right
                            match self.coords.get((x + 1) as usize) {
                                // Out of bounds but that's ok we just keep going
                                None => (),
                                Some(row) => match row.get((y + 1) as usize) {
                                    // Out of bounds but that's ok we just keep going
                                    None => (),
                                    Some(spot) => match spot {
                                        // Empty Spot, the original x, y could move here
                                        None => return true,
                                        Some(other_player) => {
                                            // We might be able to jump in this case
                                            if other_player != player {
                                                match self.coords.get((x + 2) as usize) {
                                                    // Out of bounds but that's ok we just keep going
                                                    None => (),
                                                    Some(row) => match row.get((y + 2) as usize) {
                                                        // Out of bounds but that's ok we just keep going
                                                        None => (),
                                                        Some(spot) => match spot {
                                                            None => return true,
                                                            // Occupied
                                                            Some(_) => (),
                                                        },
                                                    },
                                                }
                                            }
                                        }
                                    },
                                },
                            };

                            // check left
                            match self.coords.get((x - 1) as usize) {
                                // Out of bounds but that's ok we just keep going
                                None => (),
                                Some(row) => match row.get((y + 1) as usize) {
                                    // Out of bounds but that's ok we just keep going
                                    None => (),
                                    Some(spot) => match spot {
                                        // Empty Spot, the original x, y could move here
                                        None => return true,
                                        Some(other_player) => {
                                            // We might be able to jump in this case
                                            if other_player != player {
                                                match self.coords.get((x - 2) as usize) {
                                                    // Out of bounds but that's ok we just keep going
                                                    None => (),
                                                    Some(row) => match row.get((y + 2) as usize) {
                                                        // Out of bounds but that's ok we just keep going
                                                        None => (),
                                                        Some(spot) => match spot {
                                                            None => return true,
                                                            // Occupied
                                                            Some(_) => (),
                                                        },
                                                    },
                                                }
                                            }
                                        }
                                    },
                                },
                            };
                        }
                    },
                }
            }
        }

        false
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..8 {
            for x in 0..8 {
                match &self.coords[x][y] {
                    None => write!(f, " _")?,
                    Some(player) => match player {
                        Player::Red => write!(f, " x")?,
                        Player::White => write!(f, " o")?,
                    },
                };
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}
