use parsers::*;
use std::collections::HashMap;
use std::fs;

mod parsers;

fn get_total_size_for_folder(paths: Vec<Path>) -> u32 {
    paths.iter().fold(0, |acc, x| match x {
        Path::File { name: _, size } => acc + *size,
        _ => acc,
    })
}

fn update_present_and_parent_dir_sizes(
    dirs: &mut HashMap<String, u32>,
    pwd: &Vec<&str>,
    size: u32,
) {
    // dbg!(&dirs);
    for i in 0..pwd.len() {
        let dir = pwd[..=i].join("");
        *dirs.entry(dir).or_insert(0) += size;
    }
    // dbg!(&dirs);
}

fn process(file: &str) -> (u32, u32) {
    const MAX_DIR_SIZE: u32 = 100000;
    let (_, cmds) = parse_commands(file).unwrap();

    let mut pwd: Vec<&str> = vec![];
    let mut dirs: HashMap<String, u32> = HashMap::new();

    for c in cmds {
        match c {
            Command::Cd(Cd::Root) => {
                pwd.clear();
                pwd.push("/");
            }
            Command::Cd(Cd::Up) => {
                pwd.pop();
            }
            Command::Cd(Cd::Down(n)) => {
                pwd.push(n);
            }
            Command::Ls(paths) => {
                let size = get_total_size_for_folder(paths);
                update_present_and_parent_dir_sizes(&mut dirs, &pwd, size);
            }
        }
    }

    // part 1
    let part_1_result = dirs.iter().fold(0, |acc, (_, size)| {
        if *size <= MAX_DIR_SIZE {
            acc + *size
        } else {
            acc
        }
    });

    // part 2
    let total_size = 70_000_000;
    let update_size = 30_000_000;
    let used_size = dirs.get("/").unwrap();
    let available_size = total_size - used_size;
    let required_size_to_delete = update_size - available_size;

    let mut valid_sizes: Vec<u32> = dirs
        .into_iter()
        .filter_map(|(_, size)| {
            if size >= required_size_to_delete {
                Some(size)
            } else {
                None
            }
        })
        .collect();
    valid_sizes.sort();

    (part_1_result, valid_sizes[0])
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let (part1, part2) = process(&file);
    println!("Part 1: {part1}, Part 2: {part2}")
}

#[test]
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

    let (size, _) = process(INPUT);
    assert_eq!(size, 95437);
}
