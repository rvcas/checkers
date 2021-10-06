use crate::player::Player;

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Move {
    pub initial: Position,
    pub destination: Position,
    pub line: usize,
    pub src: String,
}

impl Move {
    pub fn is_valid(&self, player: &Player) -> bool {
        if self.initial.x > 7
            || self.initial.y > 7
            || self.destination.x > 7
            || self.destination.y > 7
        {
            return false;
        }

        let x_diff = self.destination.x - self.initial.x;
        let y_diff = self.destination.y - self.initial.y;

        match player {
            Player::Red => {
                (x_diff.abs() == 1 && y_diff == -1) || (x_diff.abs() == 2 && y_diff == -2)
            }
            Player::White => {
                (x_diff.abs() == 1 && y_diff == 1) || (x_diff.abs() == 2 && y_diff == 2)
            }
        }
    }

    pub fn is_jump(&self, player: &Player) -> bool {
        let x_diff = self.destination.x - self.initial.x;
        let y_diff = self.destination.y - self.initial.y;

        match player {
            Player::Red => x_diff.abs() == 2 && y_diff == -2,
            Player::White => x_diff.abs() == 2 && y_diff == 2,
        }
    }

    pub fn jumped_position(&self, player: &Player) -> Option<Position> {
        if self.is_jump(player) {
            let x = if self.destination.x > self.initial.x {
                self.initial.x + 1
            } else {
                self.initial.x - 1
            };

            let y = match player {
                Player::Red => self.initial.y - 1,
                Player::White => self.initial.y + 1,
            };

            Some(Position { x, y })
        } else {
            None
        }
    }
}
