use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{alpha0, newline, space1, u32};
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::IResult;

#[derive(Debug)]
pub enum Cd<'a> {
    Root,
    Up,
    Down(&'a str),
}

#[derive(Debug)]
pub enum Command<'a> {
    Cd(Cd<'a>),
    Ls(Vec<Path>),
}

#[derive(Debug)]
pub enum Path {
    File { size: u32, name: String },
    Dir(String),
}

fn parse_directory(input: &str) -> IResult<&str, Path> {
    let (rest, _) = tag("dir ")(input)?;
    let (rest, dir_name) = alpha0(rest)?;

    Ok((rest, Path::Dir(String::from(dir_name))))
}

fn parse_file(input: &str) -> IResult<&str, Path> {
    let (rest, size) = terminated(u32, space1)(input)?;
    let (rest, name) = take_while(|x: char| x.is_alphabetic() || x == '.')(rest)?;

    Ok((
        rest,
        Path::File {
            size,
            name: String::from(name),
        },
    ))
}

fn parse_cd(input: &str) -> IResult<&str, Command> {
    let (rest, _) = tag("$ cd ")(input)?;
    let (rest, cmd) = alt((tag("/"), tag(".."), take_while(|x: char| x.is_alphabetic())))(rest)?;

    let cmd = match cmd {
        "/" => Command::Cd(Cd::Root),
        ".." => Command::Cd(Cd::Up),
        name => Command::Cd(Cd::Down(name)),
    };

    Ok((rest, cmd))
}

fn parse_ls(input: &str) -> IResult<&str, Command> {
    let (rest, _) = tag("$ ls")(input)?;
    let (rest, _) = newline(rest)?;
    let (rest, paths) = separated_list1(newline, alt((parse_file, parse_directory)))(rest)?;

    Ok((rest, Command::Ls(paths)))
}

pub fn parse_commands(input: &str) -> IResult<&str, Vec<Command>> {
    let (input, commands) = separated_list1(newline, alt((parse_ls, parse_cd)))(input)?;
    Ok((input, commands))
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn parse_cd_works() {
        let (_, parsed) = parse_cd("$ cd e").unwrap();
        assert!(matches!(parsed, Command::Cd(Cd::Down("e"))));

        let (_, parsed) = parse_cd("$ cd ..").unwrap();
        assert!(matches!(parsed, Command::Cd(Cd::Up)));

        let (_, parsed) = parse_cd("$ cd /").unwrap();
        assert!(matches!(parsed, Command::Cd(Cd::Root)));
    }

    #[test]
    fn parse_ls_works() {
        const CMD: &str = "$ ls
dir e
62596 h.lst";

        let (_, parsed) = parse_ls(CMD).unwrap();

        let paths = match parsed {
            Command::Ls(paths) => paths,
            _ => panic!("No paths"),
        };

        let dir_name = match &paths[0] {
            Path::Dir(name) => name,
            _ => panic!("No dir name"),
        };

        let (file_name, file_size) = match &paths[1] {
            Path::File { name, size } => (name, size),
            _ => panic!("No dir name"),
        };

        assert_eq!(file_name, "h.lst");
        assert_eq!(*file_size, 62596);
        assert_eq!(dir_name, "e");
    }

    #[test]
    fn parse_directory_works() {
        const LINE: &str = "dir e";
        let (_, parsed) = parse_directory(LINE).unwrap();
        let name = match parsed {
            Path::Dir(name) => name,
            _ => panic!("Invalid directory"),
        };

        assert_eq!("e", name);
    }

    #[test]
    fn parse_file_works() {
        const LINE: &str = "8033020 d.log";

        let (_, parsed) = parse_file(LINE).unwrap();
        let name = match parsed {
            Path::File { name, size: _ } => name,
            _ => panic!("Invalid name"),
        };

        assert_eq!("d.log", name)
    }

    #[test]
    fn parse_commands_works() {
        const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
7214296 k";

        let (_, cmds) = parse_commands(INPUT).unwrap();

        assert_eq!(cmds.len(), 2);
    }
}
