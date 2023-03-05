use std::fs;

fn get_round_score(elf_move: &str, required_result: &str) -> Result<u32, String> {
    let shape_score = match (elf_move, required_result) {
        ("A", "X") => 3,
        ("A", "Y") => 1,
        ("A", "Z") => 2,

        ("B", "X") => 1,
        ("B", "Y") => 2,
        ("B", "Z") => 3,

        ("C", "X") => 2,
        ("C", "Y") => 3,
        ("C", "Z") => 1,
        _ => return Err("Invalid my_move".to_string()),
    };

    let outcome_score = match required_result {
        "X" => 0, //lose
        "Y" => 3, //draw
        "Z" => 6, //win
        _ => {
            return Err(format!(
                "Invalid outcome for elf move: {elf_move} and result: {required_result}"
            ))
        }
    };

    Ok(shape_score + outcome_score)
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut count = 0;

    file.lines().for_each(|line| {
        let moves = line.split_whitespace().collect::<Vec<_>>();
        let elf_move = moves.get(0).unwrap();
        let my_move = moves.get(1).unwrap();
        count += get_round_score(elf_move, my_move).unwrap();
    });

    println!("{count}")
}
