use std::{
    fmt, fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context, Result};
use structopt::StructOpt;

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

#[derive(StructOpt, Debug)]
#[structopt(name = "checkers", about = "Valid checker's moves")]
struct Opt {
    #[structopt(name = "FILE", parse(from_os_str))]
    input: PathBuf,
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Move {
    initial: Position,
    destination: Position,
    line: usize,
    src: String,
}

#[derive(Debug)]
enum Player {
    Red,
    White,
}

#[derive(Debug)]
struct Board {
    coords: Vec<Vec<Option<Player>>>,
}

#[derive(Debug)]
struct Game<'a> {
    current_player: Player,
    board: Board,
    moves: &'a Vec<Move>,
}

enum Validation<'a> {
    Winner(Player),
    IncompleteGame,
    Illegal(&'a Move),
}

impl fmt::Display for Validation<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Winner(player) => match player {
                Player::Red => write!(f, "red"),
                Player::White => write!(f, "white"),
            },
            Self::IncompleteGame => write!(f, "incomplete game"),
            Self::Illegal(mov) => write!(f, "line {} illegal move: {}", mov.line, mov.src),
        }
    }
}

impl<'a> Game<'a> {
    fn new(moves: &'a Vec<Move>) -> Self {
        Self {
            current_player: Player::White,
            board: Board::new(),
            moves,
        }
    }

    fn validate(&self) -> Validation<'a> {
        todo!()
    }

    fn next_player(&mut self) {
        if let Player::White = self.current_player {
            self.current_player = Player::Red
        } else {
            self.current_player = Player::White
        }
    }
}

impl Board {
    fn new() -> Self {
        Self {
            coords: (0..8).fold(vec![], |mut cols, x| {
                let col = (0..8).fold(vec![], |mut col, y| {
                    let coord = if INITIAL_WHITE_POSITIONS.contains(&(x, y)) {
                        Some(Player::White)
                    } else if INITIAL_RED_POSITIONS.contains(&(x, y)) {
                        Some(Player::Red)
                    } else {
                        None
                    };

                    col.push(coord);

                    col
                });

                cols.push(col);

                cols
            }),
        }
    }
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let mut input = read_file(&opt.input)?;

    let answer = validate_input(&mut input, &opt.input)?;

    println!("{}", answer);

    Ok(())
}

fn read_file(path: &Path) -> Result<String> {
    fs::read_to_string(path)
        .with_context(|| format!("failed to read moves from {}", path.display()))
}

fn validate_input(input: &mut String, path: &Path) -> Result<String> {
    clean_input(input);

    let moves = parse_moves(&input)?;

    let game = Game::new(&moves);

    let validation = game.validate();

    Ok(format!("{} - {}", path.display(), validation))
}

fn clean_input(input: &mut String) {
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }

    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }
}

fn parse_moves(input: &str) -> Result<Vec<Move>> {
    input
        .split("\n")
        .enumerate()
        .fold(Ok(vec![]), |acc, (index, input_line)| {
            let mut input_line = input_line.to_string();

            clean_input(&mut input_line);

            let move_line = input_line
                .split(",")
                .map(|n| {
                    Ok(n.parse::<i32>().with_context(|| {
                        format!(
                            "line {}: failed to parse move {} because {} is not a number",
                            index + 1,
                            input_line,
                            n
                        )
                    })?)
                })
                .collect::<Result<Vec<i32>>>()?;

            if move_line.len() == 4 {
                let mut acc = acc?;

                let initial = Position {
                    x: move_line[0],
                    y: move_line[1],
                };

                let destination = Position {
                    x: move_line[2],
                    y: move_line[3],
                };

                acc.push(Move {
                    initial,
                    destination,
                    line: index + 1,
                    src: input_line,
                });

                Ok(acc)
            } else {
                Err(anyhow!(
                    "line {}: failed to parse move {} because it is invalid",
                    index + 1,
                    input_line
                ))
            }
        })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::validate_input;

    #[test]
    fn red() {
        let mut input = include_str!("../inputs/red.txt").to_string();

        let answer = validate_input(&mut input, &PathBuf::from("red.txt"));

        assert_eq!(answer.unwrap().as_str(), "red.txt - red");
    }

    #[test]
    fn white() {
        let mut input = include_str!("../inputs/white.txt").to_string();

        let answer = validate_input(&mut input, &PathBuf::from("white.txt"));

        assert_eq!(answer.unwrap().as_str(), "white.txt - white");
    }

    #[test]
    fn illegal_move() {
        let mut input = include_str!("../inputs/illegal_move.txt").to_string();

        let answer = validate_input(&mut input, &PathBuf::from("illegal_move.txt"));

        assert_eq!(
            answer.unwrap().as_str(),
            "illegal_move.txt - line 15 illegal move: 1,0,0,5"
        );
    }

    #[test]
    fn incomplete() {
        let mut input = include_str!("../inputs/incomplete.txt").to_string();

        let answer = validate_input(&mut input, &PathBuf::from("incomplete.txt"));

        assert_eq!(answer.unwrap().as_str(), "incomplete.txt - incomplete game");
    }
}
