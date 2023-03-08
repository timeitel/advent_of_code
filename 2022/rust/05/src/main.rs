use std::fs;

use nom::bytes::complete::tag;
use nom::character::complete;
use nom::IResult;

fn parse_move_line(input: &str) -> IResult<&str, (u32, usize, usize)> {
    let (input, _) = tag("move ")(input)?;
    let (input, move_count) = complete::u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from_idx) = complete::u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to_idx) = complete::u32(input)?;

    Ok((
        input,
        (
            move_count,
            usize::try_from(from_idx - 1).unwrap(),
            usize::try_from(to_idx - 1).unwrap(),
        ),
    ))
}

fn process(file: &str) -> String {
    let mut iter = file.split("\n\n");
    let raw_stacks = iter.next().unwrap();
    let moves = iter.next().unwrap();

    let mut stacks: Vec<Vec<char>> = vec![vec![]];

    // build stacks from input
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

    // apply moves to stacks
    moves.lines().for_each(|l| {
        let (_, (move_count, from_idx, to_idx)) = parse_move_line(l).unwrap();

        (0..move_count - 1).for_each(|_| {
            let item = &mut stacks[from_idx].pop().unwrap();
            (&mut stacks[to_idx]).push(*item);
        });
    });

    let result: String = stacks
        .into_iter()
        .map(|mut x| {
            print!("{:?}", x);
            let item = x.pop().unwrap();
            item
        })
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

    assert_eq!(process(INPUT), "CMZ");
}
