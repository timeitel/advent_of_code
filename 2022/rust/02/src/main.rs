use std::fs;

fn get_round_score(elf_move: &str, my_move: &str) -> Result<u32, String> {
    let shape_score = match my_move {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => return Err("Invalid my_move".to_string()),
    };

    let outcome_score = match (elf_move, my_move) {
        ("A", "X") => 3,
        ("A", "Y") => 6,
        ("A", "Z") => 0,

        ("B", "X") => 0,
        ("B", "Y") => 3,
        ("B", "Z") => 6,

        ("C", "X") => 6,
        ("C", "Y") => 0,
        ("C", "Z") => 3,
        _ => {
            return Err(format!(
                "Invalid outcome for elf move: {elf_move} and my_move: {my_move}"
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
