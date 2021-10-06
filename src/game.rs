use std::fmt;

use crate::{board::Board, player::Player, r#move::Move};

#[derive(Debug)]
pub struct Game<'a> {
    current_player: Player,
    board: Board,
    moves: &'a Vec<Move>,
    debug: bool,
}

pub enum Validation<'a> {
    Illegal(&'a Move),
    IncompleteGame,
    Tie,
    Winner(Player),
}

impl fmt::Display for Validation<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Illegal(mov) => write!(f, "line {} illegal move: {}", mov.line, mov.src),
            Self::IncompleteGame => write!(f, "incomplete game"),
            Self::Tie => write!(f, "tie"),
            Self::Winner(player) => write!(f, "{}", player),
        }
    }
}

impl<'a> Game<'a> {
    pub fn new(moves: &'a Vec<Move>) -> Self {
        Self {
            current_player: Player::White,
            board: Board::new(),
            moves,
            debug: false,
        }
    }

    pub fn validate(&mut self) -> Validation<'a> {
        if self.debug {
            println!("{}", self.board);
        }

        let mut moves_iter = self.moves.iter().peekable();

        while let Some(mov) = moves_iter.next() {
            if self.debug {
                println!("{}", self.current_player);
                println!("{:?}", mov);
            }

            if !self.board.make_move(&self.current_player, mov) {
                return Validation::Illegal(mov);
            }

            if self.debug {
                println!("{}", self.board);
            }

            if let Some(next_mov) = moves_iter.peek() {
                let opt_player = self
                    .board
                    .get(next_mov.initial.x as usize, next_mov.initial.y as usize);

                match opt_player {
                    None => {
                        return Validation::Illegal(mov);
                    }
                    Some(player) => {
                        if *player == self.current_player {
                            if !next_mov.is_jump(player) {
                                return Validation::Illegal(mov);
                            }
                        } else {
                            self.next_player();
                        }
                    }
                }
            }
        }

        let red_score = self.board.red_score();
        let white_score = self.board.white_score();

        if white_score == red_score {
            Validation::Tie
        } else if white_score > red_score {
            Validation::Winner(Player::White)
        } else {
            Validation::Winner(Player::Red)
        }
    }

    fn next_player(&mut self) {
        if let Player::White = self.current_player {
            self.current_player = Player::Red
        } else {
            self.current_player = Player::White
        }
    }

    pub fn toggle_debug(&mut self) {
        self.debug = !self.debug;
    }
}
