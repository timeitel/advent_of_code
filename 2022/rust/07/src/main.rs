use parsers::parse_commands;
use std::fs;

mod parsers;

fn process(file: &str) -> usize {
    let (_, cmds) = parse_commands(file).unwrap();

    let _pwd: Vec<&str> = vec![];
    dbg!(cmds.len());

    0
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let result = process(&file);
    println!("{result}")
}

#[test]
#[ignore]
fn passes() {
    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    assert_eq!(process(INPUT), 95437);
}
