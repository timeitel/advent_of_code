use std::fmt::Display;
use std::fs;
use std::str::FromStr;

enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn get_round_score(elf_move: Move, required_result: &str) -> Result<u32, String> {
    let shape_score = match (elf_move, required_result) {
        (Move::Rock, "X") => 3,
        (Move::Rock, "Y") => 1,
        (Move::Rock, "Z") => 2,

        (Move::Paper, "X") => 1,
        (Move::Paper, "Y") => 2,
        (Move::Paper, "Z") => 3,

        (Move::Scissors, "X") => 2,
        (Move::Scissors, "Y") => 3,
        (Move::Scissors, "Z") => 1,
        _ => return Err("Invalid my_move".to_string()),
    };

    let outcome_score = match required_result {
        "X" => 0, //lose
        "Y" => 3, //draw
        "Z" => 6, //win
        _ => return Err(format!("Invalid outcome for result: {required_result}")),
    };

    Ok(shape_score + outcome_score)
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            _ => Err("Not a valid move".to_string()),
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Move::Paper => write!(f, "Paper"),
            Move::Rock => write!(f, "Rock"),
            Move::Scissors => write!(f, "Scissors"),
        }
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let score: u32 = file
        .lines()
        .map(|line| {
            // TODO: why _ in Vec<_>
            let moves: Vec<_> = line.split_whitespace().collect();
            let elf_move = moves[0].parse::<Move>().unwrap();
            let required_result = moves[1];
            get_round_score(elf_move, required_result).unwrap()
        })
        .sum();

    println!("{score}")
}
