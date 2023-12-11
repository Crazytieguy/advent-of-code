#![feature(associated_type_defaults)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
};

pub trait BasicSolution {
    type Shared: Debug + Clone = &'static str;
    type Answer: Debug + Display + PartialEq<Self::TestAnswer>;
    type TestAnswer: Debug = Self::Answer;
    const INPUT: &'static str;
    const SAMPLE_INPUT: &'static str;
    const SAMPLE_INPUT_B: &'static str = Self::SAMPLE_INPUT;
    const SAMPLE_ANSWER_A: Self::TestAnswer;
    const SAMPLE_ANSWER_B: Self::TestAnswer;

    fn shared(input: &'static str) -> anyhow::Result<Self::Shared>;
    fn part_a(shared: Cow<Self::Shared>) -> anyhow::Result<Self::Answer>;
    fn part_b(shared: Self::Shared) -> anyhow::Result<Self::Answer>;
}

impl<T: BasicSolution> Solution for T {
    type Shared = <Self as BasicSolution>::Shared;
    type SharedTest = Self::Shared;
    type Answer = <Self as BasicSolution>::Answer;
    type TestAnswer = <Self as BasicSolution>::TestAnswer;
    const INPUT: &'static str = <Self as BasicSolution>::INPUT;
    const SAMPLE_INPUT: &'static str = <Self as BasicSolution>::SAMPLE_INPUT;
    const SAMPLE_INPUT_B: &'static str = <Self as BasicSolution>::SAMPLE_INPUT_B;
    const SAMPLE_ANSWER_A: <Self as BasicSolution>::TestAnswer =
        <Self as BasicSolution>::SAMPLE_ANSWER_A;
    const SAMPLE_ANSWER_B: <Self as BasicSolution>::TestAnswer =
        <Self as BasicSolution>::SAMPLE_ANSWER_B;

    fn shared(input: &'static str) -> anyhow::Result<Self::Shared> {
        <Self as BasicSolution>::shared(input)
    }

    fn part_a(data: Cow<Self::Shared>) -> anyhow::Result<Self::Answer> {
        <Self as BasicSolution>::part_a(data)
    }

    fn part_b(data: Self::Shared) -> anyhow::Result<Self::Answer> {
        <Self as BasicSolution>::part_b(data)
    }

    fn shared_test(input: &'static str) -> anyhow::Result<Self::SharedTest> {
        Self::shared(input)
    }
    fn part_a_test(data: Self::SharedTest) -> anyhow::Result<Self::Answer> {
        Self::part_a(Cow::Owned(data))
    }
    fn part_b_test(data: Self::SharedTest) -> anyhow::Result<Self::Answer> {
        Self::part_b(data)
    }
}

pub trait Solution {
    type Shared: Debug + Clone = &'static str;
    type SharedTest: Debug + Clone = Self::Shared;
    type Answer: Debug + Display + PartialEq<Self::TestAnswer>;
    type TestAnswer: Debug = Self::Answer;
    const INPUT: &'static str;
    const SAMPLE_INPUT: &'static str;
    const SAMPLE_INPUT_B: &'static str = Self::SAMPLE_INPUT;
    const SAMPLE_ANSWER_A: Self::TestAnswer;
    const SAMPLE_ANSWER_B: Self::TestAnswer;

    fn shared(input: &'static str) -> anyhow::Result<Self::Shared>;
    fn part_a(shared: Cow<Self::Shared>) -> anyhow::Result<Self::Answer>;
    fn part_b(shared: Self::Shared) -> anyhow::Result<Self::Answer>;
    fn shared_test(input: &'static str) -> anyhow::Result<Self::SharedTest>;
    fn part_a_test(shared: Self::SharedTest) -> anyhow::Result<Self::Answer>;
    fn part_b_test(shared: Self::SharedTest) -> anyhow::Result<Self::Answer>;

    fn test_part_a() -> anyhow::Result<()> {
        assert_eq!(
            Self::shared_test(Self::SAMPLE_INPUT).and_then(Self::part_a_test)?,
            Self::SAMPLE_ANSWER_A
        );
        let shared = Cow::Owned(Self::shared(Self::INPUT)?);
        println!("a: {}", Self::part_a(shared)?);
        Ok(())
    }

    fn test_part_b() -> anyhow::Result<()> {
        assert_eq!(
            Self::shared_test(Self::SAMPLE_INPUT_B).and_then(Self::part_b_test)?,
            Self::SAMPLE_ANSWER_B
        );
        let shared = Self::shared(Self::INPUT)?;
        println!("b: {}", Self::part_b(shared)?);
        Ok(())
    }

    fn main() -> anyhow::Result<()> {
        let shared = time("Shared", || Self::shared(Self::INPUT))?;
        let arg = std::env::args().nth(1);
        match arg.as_deref() {
            Some("a") => {
                let a = time("Part a", || Self::part_a(Cow::Owned(shared)))?;
                println!("a: {a}");
            }
            Some("b") => {
                let b = time("Part b", || Self::part_b(shared))?;
                println!("b: {b}");
            }
            _ => {
                let a = time("Part a", || Self::part_a(Cow::Borrowed(&shared)))?;
                let b = time("Part b", || Self::part_b(shared))?;
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
