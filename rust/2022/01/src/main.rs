use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let mut result = file
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    result.sort_by(|a, b| b.cmp(a));
    let sum: u32 = result.iter().take(3).sum();

    print!("{}", sum.to_string())
}
