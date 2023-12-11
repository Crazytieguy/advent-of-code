#![feature(associated_type_defaults)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
};

pub trait BasicSolution {
    type Common: Debug + Clone = &'static str;
    type Answer: Debug + Display + PartialEq<Self::TestAnswer>;
    type TestAnswer: Debug = Self::Answer;
    const INPUT: &'static str;
    const SAMPLE_INPUT: &'static str;
    const SAMPLE_INPUT_B: &'static str = Self::SAMPLE_INPUT;
    const SAMPLE_ANSWER_A: Self::TestAnswer;
    const SAMPLE_ANSWER_B: Self::TestAnswer;

    fn common(input: &'static str) -> anyhow::Result<Self::Common>;
    fn part_a(common: Cow<Self::Common>) -> anyhow::Result<Self::Answer>;
    fn part_b(common: Self::Common) -> anyhow::Result<Self::Answer>;
}

impl<T: BasicSolution> Solution for T {
    type Common = <Self as BasicSolution>::Common;
    type CommonTest = Self::Common;
    type Answer = <Self as BasicSolution>::Answer;
    type TestAnswer = <Self as BasicSolution>::TestAnswer;
    const INPUT: &'static str = <Self as BasicSolution>::INPUT;
    const SAMPLE_INPUT: &'static str = <Self as BasicSolution>::SAMPLE_INPUT;
    const SAMPLE_INPUT_B: &'static str = <Self as BasicSolution>::SAMPLE_INPUT_B;
    const SAMPLE_ANSWER_A: <Self as BasicSolution>::TestAnswer =
        <Self as BasicSolution>::SAMPLE_ANSWER_A;
    const SAMPLE_ANSWER_B: <Self as BasicSolution>::TestAnswer =
        <Self as BasicSolution>::SAMPLE_ANSWER_B;

    fn common(input: &'static str) -> anyhow::Result<Self::Common> {
        <Self as BasicSolution>::common(input)
    }

    fn part_a(data: Cow<Self::Common>) -> anyhow::Result<Self::Answer> {
        <Self as BasicSolution>::part_a(data)
    }

    fn part_b(data: Self::Common) -> anyhow::Result<Self::Answer> {
        <Self as BasicSolution>::part_b(data)
    }

    fn common_test(input: &'static str) -> anyhow::Result<Self::CommonTest> {
        Self::common(input)
    }
    fn part_a_test(data: Self::CommonTest) -> anyhow::Result<Self::Answer> {
        Self::part_a(Cow::Owned(data))
    }
    fn part_b_test(data: Self::CommonTest) -> anyhow::Result<Self::Answer> {
        Self::part_b(data)
    }
}

pub trait Solution {
    type Common: Debug + Clone = &'static str;
    type CommonTest: Debug + Clone = Self::Common;
    type Answer: Debug + Display + PartialEq<Self::TestAnswer>;
    type TestAnswer: Debug = Self::Answer;
    const INPUT: &'static str;
    const SAMPLE_INPUT: &'static str;
    const SAMPLE_INPUT_B: &'static str = Self::SAMPLE_INPUT;
    const SAMPLE_ANSWER_A: Self::TestAnswer;
    const SAMPLE_ANSWER_B: Self::TestAnswer;

    fn common(input: &'static str) -> anyhow::Result<Self::Common>;
    fn part_a(common: Cow<Self::Common>) -> anyhow::Result<Self::Answer>;
    fn part_b(common: Self::Common) -> anyhow::Result<Self::Answer>;
    fn common_test(input: &'static str) -> anyhow::Result<Self::CommonTest>;
    fn part_a_test(common: Self::CommonTest) -> anyhow::Result<Self::Answer>;
    fn part_b_test(common: Self::CommonTest) -> anyhow::Result<Self::Answer>;

    fn test_part_a() -> anyhow::Result<()> {
        assert_eq!(
            Self::common_test(Self::SAMPLE_INPUT).and_then(Self::part_a_test)?,
            Self::SAMPLE_ANSWER_A
        );
        let common = Cow::Owned(Self::common(Self::INPUT)?);
        println!("a: {}", Self::part_a(common)?);
        Ok(())
    }

    fn test_part_b() -> anyhow::Result<()> {
        assert_eq!(
            Self::common_test(Self::SAMPLE_INPUT_B).and_then(Self::part_b_test)?,
            Self::SAMPLE_ANSWER_B
        );
        let common = Self::common(Self::INPUT)?;
        println!("b: {}", Self::part_b(common)?);
        Ok(())
    }

    fn main() -> anyhow::Result<()> {
        let common = time("Common", || Self::common(Self::INPUT))?;
        let arg = std::env::args().nth(1);
        match arg.as_deref() {
            Some("a") => {
                let a = time("Part a", || Self::part_a(Cow::Owned(common)))?;
                println!("a: {a}");
            }
            Some("b") => {
                let b = time("Part b", || Self::part_b(common))?;
                println!("b: {b}");
            }
            _ => {
                let a = time("Part a", || Self::part_a(Cow::Borrowed(&common)))?;
                let b = time("Part b", || Self::part_b(common))?;
                println!("a: {a}");
                println!("b: {b}");
            }
        }
        Ok(())
    }
}

fn time<T>(tag: &str, f: impl FnOnce() -> T) -> T {
    let start = std::time::Instant::now();
    let ans = f();
    println!("{tag} took {:?}", start.elapsed());
    ans
}
