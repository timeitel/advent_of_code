use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{newline, space1, u32};
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, terminated};
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
    Ls(Vec<&'a str>),
}

#[derive(Debug)]
pub enum Path<'a> {
    File { size: u32, name: &'a str },
    Dir(&'a str),
}

pub fn parse_directory(input: &str) -> IResult<&str, Path> {
    let (rest, _) = tag("dir ")(input)?;

    Ok((input, Path::Dir(rest)))
}

pub fn parse_file(input: &str) -> IResult<&str, Path> {
    let (rest, size) = terminated(u32, space1)(input)?;

    Ok((input, Path::File { size, name: rest }))
}

pub fn parse_cd(input: &str) -> IResult<&str, Command> {
    let (rest, _) = tag("$ cd ")(input)?;

    let cmd = match rest {
        "/" => Command::Cd(Cd::Root),
        ".." => Command::Cd(Cd::Up),
        name => Command::Cd(Cd::Down(name)),
    };

    Ok((input, cmd))
}

pub fn parse_ls(input: &str) -> IResult<&str, Command> {
    let (rest, _) = tag("$ ls")(input)?;
    let (rest, _) = newline(rest)?;
    // let (rest, files) = separated_list1(newline, alt((parse_file, parse_directory)))(rest)?;

    Ok((rest, Command::Ls(vec![])))
}

// pub fn parse_cmd(input: &str) -> IResult<&str, Command> {
//     let (input, cmd) = separated_list1(newline, alt((parse_ls, parse_cd)))(input)?;
//     Ok((input, Command::Ls(vec![])))
// }

#[cfg(test)]
mod tests {
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
4060174 j
7214296 k";
        let _expected_parsed = Command::Ls(vec!["4060174 j", "7214296 k"]);

        let (_, parsed) = parse_ls(CMD).unwrap();

        assert!(matches!(parsed, _expected_parsed));
    }

    #[test]
    fn parse_directory_works() {
        const LINE: &str = "dir e";
        let _expected_parsed = Path::Dir("e");

        let (_, parsed) = parse_directory(LINE).unwrap();

        assert!(matches!(parsed, _expected_parsed));
    }

    #[test]
    fn parse_file_works() {
        const LINE: &str = "8033020 d.log";
        let _expected_parsed = Path::File {
            name: "d.log",
            size: 8033020,
        };

        let (_, parsed) = parse_file(LINE).unwrap();

        assert!(matches!(parsed, _expected_parsed));
    }
}
