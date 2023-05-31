use std::fs;

use nom::bytes::complete::tag;
use nom::character::complete;
use nom::IResult;

struct CrateMove {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_move_line(input: &str) -> IResult<&str, CrateMove> {
    let (input, _) = tag("move ")(input)?;
    let (input, move_count) = complete::u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from_idx) = complete::u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to_idx) = complete::u32(input)?;

    Ok((
        input,
        CrateMove {
            count: usize::try_from(move_count).unwrap(),
            from: usize::try_from(from_idx - 1).unwrap(),
            to: usize::try_from(to_idx - 1).unwrap(),
        },
    ))
}

fn process(file: &str) -> String {
    let mut iter = file.split("\n\n");
    let raw_stacks = iter.next().unwrap();
    let moves = iter.next().unwrap();

    let mut stacks: Vec<Vec<char>> = vec![vec![]; 30];

    // build stacks from input
    raw_stacks.lines().rev().for_each(|line| {
        let mut index = 0;

        for (i, ch) in line.chars().enumerate() {
            if i % 4 == 0 && i != 0 {
                index += 1;
            }

            if ch.is_alphabetic() {
                (&mut stacks[index]).push(ch);
            }
        }
    });

    // apply moves to stacks
    moves.lines().for_each(|l| {
        let (_, crate_move) = parse_move_line(l).unwrap();

        let from_stack = &mut stacks[crate_move.from];
        let mut items: Vec<_> = from_stack
            .drain((from_stack.len() - crate_move.count)..)
            .collect();
        (&mut stacks[crate_move.to]).append(&mut items);
    });

    let result: String = stacks
        .into_iter()
        .filter(|x| x.len() > 0)
        .map(|mut x| x.pop().unwrap())
        .collect();

    result
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

    assert_eq!(process(INPUT), "MCD");
}
