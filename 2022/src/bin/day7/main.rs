use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, not_line_ending, u32},
    multi::{fold_many0, fold_many1},
    Parser,
};
use nom_supreme::ParserExt;
use std::{collections::HashMap, error::Error};

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;
type IResult<'a, T> = nom::IResult<&'a str, T>;

type Parsed<'a> = HashMap<Vec<&'a str>, u32>;

#[derive(Debug, Clone, Copy)]
enum CDTarget<'a> {
    Root,
    Parent,
    Child(&'a str),
}

#[derive(Debug, Clone, Copy)]
enum Command<'a> {
    LS(u32),
    CD(CDTarget<'a>),
}

fn parse_cd(data: &str) -> IResult<Command> {
    tag("cd ")
        .precedes(alt((
            tag("/").value(CDTarget::Root),
            tag("..").value(CDTarget::Parent),
            not_line_ending.map(CDTarget::Child),
        )))
        .map(Command::CD)
        .terminated(line_ending.opt())
        .parse(data)
}

fn parse_ls_output_line(data: &str) -> IResult<u32> {
    u32.or(tag("dir").value(0))
        .terminated(not_line_ending.and(line_ending.opt()))
        .parse(data)
}

fn parse_ls(data: &str) -> IResult<Command> {
    tag("ls\n")
        .precedes(fold_many0(parse_ls_output_line, || 0, |acc, cur| acc + cur))
        .map(Command::LS)
        .parse(data)
}

fn parse_command(data: &str) -> IResult<Command> {
    tag("$ ").precedes(alt((parse_cd, parse_ls))).parse(data)
}

fn parse(data: &str) -> IResult<Parsed> {
    let mut current_dir = vec![];
    fold_many1(
        parse_command,
        HashMap::new,
        move |mut directory_sizes, command| {
            match command {
                Command::LS(current_dir_size) => {
                    for i in 0..=current_dir.len() {
                        let path = &current_dir[..i];
                        if let Some(size) = directory_sizes.get_mut(path) {
                            *size += current_dir_size;
                        } else {
                            directory_sizes.insert(path.to_vec(), current_dir_size);
                        }
                    }
                }
                Command::CD(target) => match target {
                    CDTarget::Root => current_dir.clear(),
                    CDTarget::Parent => {
                        current_dir.pop();
                    }
                    CDTarget::Child(name) => current_dir.push(name),
                },
            }
            directory_sizes
        },
    )(data)
}

fn part_a(data: &Parsed) -> u32 {
    data.values().copied().filter(|&size| size < 100000).sum()
}

const TOTAL_DISK_SPACE: u32 = 70000000;
const NEEDED_DISK_SPACE: u32 = 30000000;

fn part_b(data: &Parsed) -> u32 {
    let root_directory_size = data[&vec![]];
    let need_to_free = (root_directory_size + NEEDED_DISK_SPACE).saturating_sub(TOTAL_DISK_SPACE);
    data.values()
        .copied()
        .filter(|&size| size >= need_to_free)
        .min()
        .expect("At least one directory should be larger than the missing space")
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() -> OutResult {
        assert_eq!(part_a(&parse(SAMPLE_DATA)?.1), 95437);
        println!("part a: {}", part_a(&parse(DATA)?.1));
        Ok(())
    }

    #[test]
    fn test_b() -> OutResult {
        assert_eq!(part_b(&parse(SAMPLE_DATA)?.1), 24933642);
        println!("part b: {}", part_b(&parse(DATA)?.1));
        Ok(())
    }
}

fn main() -> OutResult {
    let parsed = parse(DATA)?.1;
    println!("part a: {}", part_a(&parsed));
    println!("part b: {}", part_b(&parsed));
    Ok(())
}
