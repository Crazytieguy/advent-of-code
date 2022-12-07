use std::{collections::BTreeMap, error::Error};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending, not_line_ending, u64},
    multi::{separated_list0, separated_list1},
    sequence::separated_pair,
    Parser,
};
use nom_supreme::ParserExt;

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;
type IResult<'a, T> = nom::IResult<&'a str, T>;

type Parsed<'a> = BTreeMap<FilePath<'a>, u64>;

#[derive(Debug, Clone, Copy)]
enum CDTarget<'a> {
    Root,
    Parent,
    Child(&'a str),
}

fn parse_cd_target(data: &str) -> IResult<CDTarget> {
    alt((
        tag("/").value(CDTarget::Root),
        tag("..").value(CDTarget::Parent),
        not_line_ending.map(CDTarget::Child),
    ))(data)
}

#[derive(Debug, Clone, Copy)]
enum Output<'a> {
    Dir(&'a str),
    File { name: &'a str, size: u64 },
}

fn parse_ls_output(data: &str) -> IResult<Output> {
    alt((
        tag("dir ").precedes(not_line_ending).map(Output::Dir),
        separated_pair(u64, char(' '), not_line_ending)
            .map(|(size, name)| Output::File { name, size }),
    ))(data)
}

#[derive(Debug, Clone)]
enum Command<'a> {
    LS(Vec<Output<'a>>),
    CD(CDTarget<'a>),
}

fn parse_command(data: &str) -> IResult<Command> {
    alt((
        tag("ls\n")
            .precedes(separated_list0(line_ending, parse_ls_output))
            .map(Command::LS),
        tag("cd ").precedes(parse_cd_target).map(Command::CD),
    ))
    .preceded_by(tag("$ "))
    .parse(data)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct FilePath<'a> {
    path: Vec<&'a str>,
    name: &'a str,
}

fn commands_to_file_system(data: Vec<Command>) -> Parsed {
    let mut current_dir = vec![];
    let mut file_system = BTreeMap::new();
    for command in data {
        match command {
            Command::LS(outputs) => {
                for output in outputs {
                    if let Output::File { name, size } = output {
                        file_system.insert(
                            FilePath {
                                path: current_dir.clone(),
                                name,
                            },
                            size,
                        );
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
    }
    file_system
}

fn parse(data: &str) -> IResult<Parsed> {
    separated_list1(line_ending, parse_command)
        .map(commands_to_file_system)
        .parse(data)
}

fn directory_sizes<'a>(data: &'a Parsed) -> BTreeMap<&'a [&'a str], u64> {
    let mut total_sizes = BTreeMap::new();
    for (file_path, size) in data {
        for i in 0..=file_path.path.len() {
            let path = &file_path.path[..i];
            *total_sizes.entry(path).or_insert(0) += size;
        }
    }
    total_sizes
}

fn part_a(data: &Parsed) -> u64 {
    let total_sizes = directory_sizes(data);
    total_sizes
        .into_values()
        .filter(|&size| size < 100000)
        .sum()
}

const TOTAL_DISK_SPACE: u64 = 70000000;
const NEEDED_DISK_SPACE: u64 = 30000000;

fn part_b(data: &Parsed) -> u64 {
    let total_sizes = directory_sizes(data);
    let &root_directory_size = total_sizes
        .values()
        .next()
        .expect("There should be at least one file");
    let minumim_directory_size_to_delete =
        (root_directory_size + NEEDED_DISK_SPACE).saturating_sub(TOTAL_DISK_SPACE);
    total_sizes
        .into_values()
        .filter(|&size| size >= minumim_directory_size_to_delete)
        .min()
        .expect("At least one directory should be large enough to delete")
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
