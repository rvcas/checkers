use std::{fs, path::PathBuf};

use anyhow::{anyhow, Context, Result};
use structopt::StructOpt;

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

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let path = opt.input.as_path();

    let mut input = fs::read_to_string(path)
        .with_context(|| format!("failed to read moves from {}", path.display()))?;

    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }

    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }

    let moves = parse_moves(&input)?;

    println!("{:?}", moves);

    Ok(())
}

fn parse_moves(input: &str) -> Result<Vec<Move>> {
    input
        .split("\n")
        .enumerate()
        .fold(Ok(vec![]), |acc, (index, line)| {
            let move_line = line
                .split(",")
                .map(|n| {
                    let mut n = n.to_string();

                    if let Some('\r') = n.chars().next_back() {
                        n.pop();
                    }

                    Ok(n.parse::<i32>().with_context(|| {
                        format!(
                            "line {}: failed to parse move {} because {} is not a number",
                            index + 1,
                            line,
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
                    src: line.to_string(),
                });

                Ok(acc)
            } else {
                Err(anyhow!(
                    "line {}: failed to parse move {} because it is invalid"
                ))
            }
        })
}
