use std::collections::HashSet;
use std::fs;

// first 4 chars that are different
fn process(file: &str) -> usize {
    let distinct_len = 14;
    let chars = file.chars().collect::<Vec<char>>();
    let position = chars
        .windows(distinct_len)
        .position(|seq| seq.iter().collect::<HashSet<&char>>().len() == seq.len())
        .unwrap();

    position + distinct_len
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let result = process(&file);
    println!("{result}");
}

#[test]
fn passes() {
    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const INPUT_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";

    assert_eq!(process(INPUT), 19);
    assert_eq!(process(INPUT_2), 23);
}
