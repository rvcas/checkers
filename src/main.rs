use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context, Result};
use structopt::StructOpt;

use checkers::{
    game::Game,
    r#move::{Move, Position},
};

/// Valid checker's moves
#[derive(StructOpt, Debug)]
#[structopt(name = "checkers")]
struct Opt {
    /// Input files containing moves to validate
    #[structopt(name = "FILE", parse(from_os_str))]
    input: Vec<PathBuf>,

    /// Print the Current Move and Board
    #[structopt(short, long)]
    debug: bool,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    for path in &opt.input {
        let mut input = read_file(path)?;

        let answer = validate_input(&mut input, opt.debug)?;

        println!("{}", answer);
    }

    Ok(())
}

fn read_file(path: &Path) -> Result<String> {
    fs::read_to_string(path)
        .with_context(|| format!("failed to read moves from {}", path.display()))
}

fn validate_input(input: &mut String, debug: bool) -> Result<String> {
    clean_input(input);

    let moves = parse_moves(&input)?;

    let mut game = Game::new(&moves);

    if debug {
        game.toggle_debug();
    }

    let validation = game.validate();

    Ok(format!("{}", validation))
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
    use crate::validate_input;

    #[test]
    fn red() {
        let mut input = include_str!("../inputs/red.txt").to_string();

        let answer = validate_input(&mut input, false);

        assert_eq!(answer.unwrap().as_str(), "red");
    }

    #[test]
    fn white() {
        let mut input = include_str!("../inputs/white.txt").to_string();

        let answer = validate_input(&mut input, false);

        assert_eq!(answer.unwrap().as_str(), "white");
    }

    #[test]
    fn illegal_move() {
        let mut input = include_str!("../inputs/illegal_move.txt").to_string();

        let answer = validate_input(&mut input, false);

        assert_eq!(answer.unwrap().as_str(), "line 15 illegal move: 1,0,0,5");
    }

    #[test]
    fn incomplete() {
        let mut input = include_str!("../inputs/incomplete.txt").to_string();

        let answer = validate_input(&mut input, false);

        assert_eq!(answer.unwrap().as_str(), "incomplete game");
    }
}
