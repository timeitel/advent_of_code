use std::fs;

fn process(file: &str) -> &str {
    let mut iter = file.split("\n\n");
    let raw_stacks = iter.next().unwrap();
    // let moves = iter.next().unwrap();

    let mut stacks: Vec<Vec<char>> = vec![vec![]];

    raw_stacks.lines().rev().for_each(|line| {
        let mut index = 0;

        for ch in line.chars() {
            if ch.is_alphabetic() {
                if index > stacks.len() - 1 {
                    stacks.push(vec![]);
                }

                (&mut stacks[index]).push(ch);
                index += 1;
            }
        }
    });

    // let result = file
    //     .lines()
    //     .map(|l| {
    //         for (i, c) in l.chars().enumerate() {
    //             if i % 2 == 0 {
    //                 continue;
    //             }

    //             if c.is_alphabetic() {
    //                 println!("{c}")
    //             }
    //         }
    //     })
    //     .collect::<Vec<_>>();

    "test"
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let result = process(&file);
    println!("{result}")
}

#[test]
fn passes() {
    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    assert_eq!(process(INPUT), "CMZ");
}
