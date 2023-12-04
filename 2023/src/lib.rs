#![feature(associated_type_defaults)]
use std::fmt::{Debug, Display};

pub trait BasicSolution {
    type Parsed: Debug + Clone = &'static str;
    type Answer: Debug + Display + PartialEq<Self::TestAnswer>;
    type TestAnswer: Debug = Self::Answer;
    const DATA: &'static str;
    const SAMPLE_DATA: &'static str;
    const SAMPLE_DATA_B: &'static str = Self::SAMPLE_DATA;
    const SAMPLE_ANSWER_A: Self::TestAnswer;
    const SAMPLE_ANSWER_B: Self::TestAnswer;

    fn parse(data: &'static str) -> anyhow::Result<Self::Parsed>;
    fn a(data: Self::Parsed) -> anyhow::Result<Self::Answer>;
    fn b(data: Self::Parsed) -> anyhow::Result<Self::Answer>;
}

impl<T: BasicSolution> Solution for T {
    type Parsed = <Self as BasicSolution>::Parsed;
    type ParsedTest = Self::Parsed;
    type Answer = <Self as BasicSolution>::Answer;
    type TestAnswer = <Self as BasicSolution>::TestAnswer;
    const DATA: &'static str = <Self as BasicSolution>::DATA;
    const SAMPLE_DATA: &'static str = <Self as BasicSolution>::SAMPLE_DATA;
    const SAMPLE_DATA_B: &'static str = <Self as BasicSolution>::SAMPLE_DATA_B;
    const SAMPLE_ANSWER_A: <Self as BasicSolution>::TestAnswer =
        <Self as BasicSolution>::SAMPLE_ANSWER_A;
    const SAMPLE_ANSWER_B: <Self as BasicSolution>::TestAnswer =
        <Self as BasicSolution>::SAMPLE_ANSWER_B;

    fn parse(data: &'static str) -> anyhow::Result<Self::Parsed> {
        <Self as BasicSolution>::parse(data)
    }

    fn a(data: Self::Parsed) -> anyhow::Result<Self::Answer> {
        <Self as BasicSolution>::a(data)
    }

    fn b(data: Self::Parsed) -> anyhow::Result<Self::Answer> {
        <Self as BasicSolution>::b(data)
    }

    fn parse_test(data: &'static str) -> anyhow::Result<Self::ParsedTest> {
        Self::parse(data)
    }
    fn a_test(data: Self::ParsedTest) -> anyhow::Result<Self::Answer> {
        Self::a(data)
    }
    fn b_test(data: Self::ParsedTest) -> anyhow::Result<Self::Answer> {
        Self::b(data)
    }
}

pub trait Solution {
    type Parsed: Debug + Clone = &'static str;
    type ParsedTest: Debug + Clone = Self::Parsed;
    type Answer: Debug + Display + PartialEq<Self::TestAnswer>;
    type TestAnswer: Debug = Self::Answer;
    const DATA: &'static str;
    const SAMPLE_DATA: &'static str;
    const SAMPLE_DATA_B: &'static str = Self::SAMPLE_DATA;
    const SAMPLE_ANSWER_A: Self::TestAnswer;
    const SAMPLE_ANSWER_B: Self::TestAnswer;

    fn parse(data: &'static str) -> anyhow::Result<Self::Parsed>;
    fn a(data: Self::Parsed) -> anyhow::Result<Self::Answer>;
    fn b(data: Self::Parsed) -> anyhow::Result<Self::Answer>;
    fn parse_test(data: &'static str) -> anyhow::Result<Self::ParsedTest>;
    fn a_test(data: Self::ParsedTest) -> anyhow::Result<Self::Answer>;
    fn b_test(data: Self::ParsedTest) -> anyhow::Result<Self::Answer>;

    fn test_a() -> anyhow::Result<()> {
        assert_eq!(
            Self::parse_test(Self::SAMPLE_DATA).and_then(Self::a_test)?,
            Self::SAMPLE_ANSWER_A
        );
        println!("a: {}", Self::parse(Self::DATA).and_then(Self::a)?);
        Ok(())
    }

    fn test_b() -> anyhow::Result<()> {
        assert_eq!(
            Self::parse_test(Self::SAMPLE_DATA_B).and_then(Self::b_test)?,
            Self::SAMPLE_ANSWER_B
        );
        println!("b: {}", Self::parse(Self::DATA).and_then(Self::b)?);
        Ok(())
    }

    fn main() -> anyhow::Result<()> {
        let parsed = Self::parse(Self::DATA)?;
        let arg = std::env::args().nth(1);
        match arg.as_deref() {
            Some("a") => {
                println!("a: {}", Self::a(parsed)?);
            }
            Some("b") => {
                println!("b: {}", Self::b(parsed)?);
            }
            _ => {
                println!("a: {}", Self::a(parsed.clone())?);
                println!("b: {}", Self::b(parsed)?);
            }
        }
        Ok(())
    }
}
