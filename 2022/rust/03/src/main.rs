use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .into_iter()
        .enumerate()
        .map(|(i, char)| (char, i + 1))
        .collect::<HashMap<char, usize>>();

    let result = file
        .lines()
        .tuples::<(_, _, _)>()
        .map(|(bag_a, bag_b, bag_c)| {
            let common_char = bag_a
                .chars()
                .find(|ch| bag_b.contains(*ch) && bag_c.contains(*ch))
                .unwrap();
            letter_scores.get(&common_char).unwrap()
        })
        .sum::<usize>();

    println!("{}", result);
}
