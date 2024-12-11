use std::collections::HashMap;

use advent_2022::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, not_line_ending, u32},
    multi::{fold_many0, fold_many1},
    Parser,
};
use nom_supreme::ParserExt;

boilerplate!(Day);

const TOTAL_DISK_SPACE: u32 = 70000000;
const NEEDED_DISK_SPACE: u32 = 30000000;

impl BasicSolution for Day {
    type Parsed = HashMap<Vec<&'static str>, u32>;
    type Answer = u32;
    const SAMPLE_ANSWER_A: Self::TestAnswer = 95437;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 24933642;

    fn parse(data: &'static str) -> IResult<'static, Self::Parsed> {
        let mut current_dir = vec![];
        fold_many1(
            parse_command,
            HashMap::new,
            move |mut directory_sizes, command| {
                compute_command(command, &mut current_dir, &mut directory_sizes);
                directory_sizes
            },
        )(data)
    }

    fn a(data: Self::Parsed) -> Self::Answer {
        data.into_values().filter(|&size| size < 100000).sum()
    }

    fn b(data: Self::Parsed) -> Self::Answer {
        let root_directory_size = data[&vec![]];
        let need_to_free =
            (root_directory_size + NEEDED_DISK_SPACE).saturating_sub(TOTAL_DISK_SPACE);
        data.into_values()
            .filter(|&size| size >= need_to_free)
            .min()
            .expect("At least one directory should be larger than the missing space")
    }
}

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

use CDTarget::*;
use Command::*;

fn compute_command<'a>(
    command: Command<'a>,
    current_dir: &mut Vec<&'a str>,
    directory_sizes: &mut HashMap<Vec<&'a str>, u32>,
) {
    match command {
        LS(current_dir_size) => {
            for i in 0..=current_dir.len() {
                let path = &current_dir[..i];
                if let Some(size) = directory_sizes.get_mut(path) {
                    *size += current_dir_size;
                } else {
                    directory_sizes.insert(path.to_vec(), current_dir_size);
                }
            }
        }
        CD(target) => match target {
            Root => current_dir.clear(),
            Parent => {
                current_dir.pop();
            }
            Child(name) => current_dir.push(name),
        },
    }
}

fn parse_cd(data: &str) -> IResult<Command> {
    tag("cd ")
        .precedes(alt((
            tag("/").value(Root),
            tag("..").value(Parent),
            not_line_ending.map(Child),
        )))
        .map(CD)
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
        .map(LS)
        .parse(data)
}

fn parse_command(data: &str) -> IResult<Command> {
    tag("$ ").precedes(alt((parse_cd, parse_ls))).parse(data)
}
