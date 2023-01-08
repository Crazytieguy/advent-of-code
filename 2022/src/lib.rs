#![feature(associated_type_defaults)]
use nom_supreme::final_parser::final_parser;
use std::{
    fmt::{Debug, Display},
    time::Instant,
};

pub type OutResult = Result<(), Box<dyn std::error::Error>>;
pub type IResult<'a, T> = nom::IResult<&'a str, T>;

pub trait SolutionData {
    const DATA: &'static str;
    const SAMPLE_DATA: &'static str;
}

pub trait BasicSolution: SolutionData {
    type Parsed: Debug + Clone = &'static str;
    type Answer: Debug + Display + PartialEq<Self::TestAnswer>;
    type TestAnswer: Debug = Self::Answer;
    const SAMPLE_ANSWER_A: Self::TestAnswer;
    const SAMPLE_ANSWER_B: Self::TestAnswer;

    fn parse(data: &'static str) -> IResult<Self::Parsed>;
    fn a(data: Self::Parsed) -> Self::Answer;
    fn b(data: Self::Parsed) -> Self::Answer;
}

impl<T: BasicSolution> Solution for T {
    type Parsed = <Self as BasicSolution>::Parsed;
    type ParsedTest = Self::Parsed;
    type Answer = <Self as BasicSolution>::Answer;
    type TestAnswer = <Self as BasicSolution>::TestAnswer;
    const SAMPLE_ANSWER_A: <Self as BasicSolution>::TestAnswer =
        <Self as BasicSolution>::SAMPLE_ANSWER_A;
    const SAMPLE_ANSWER_B: <Self as BasicSolution>::TestAnswer =
        <Self as BasicSolution>::SAMPLE_ANSWER_B;

    fn parse(data: &'static str) -> IResult<Self::Parsed> {
        <Self as BasicSolution>::parse(data)
    }

    fn a(data: Self::Parsed) -> Self::Answer {
        <Self as BasicSolution>::a(data)
    }

    fn b(data: Self::Parsed) -> Self::Answer {
        <Self as BasicSolution>::b(data)
    }

    fn parse_test(data: &'static str) -> IResult<Self::ParsedTest> {
        Self::parse(data)
    }
    fn a_test(data: Self::ParsedTest) -> Self::Answer {
        Self::a(data)
    }
    fn b_test(data: Self::ParsedTest) -> Self::Answer {
        Self::b(data)
    }
}

pub trait Solution: SolutionData {
    type Parsed: Debug + Clone = &'static str;
    type ParsedTest: Debug + Clone = Self::Parsed;
    type Answer: Debug + Display + PartialEq<Self::TestAnswer>;
    type TestAnswer: Debug = Self::Answer;
    const SAMPLE_ANSWER_A: Self::TestAnswer;
    const SAMPLE_ANSWER_B: Self::TestAnswer;

    fn parse(data: &'static str) -> IResult<Self::Parsed>;
    fn a(data: Self::Parsed) -> Self::Answer;
    fn b(data: Self::Parsed) -> Self::Answer;
    fn parse_test(data: &'static str) -> IResult<Self::ParsedTest>;
    fn a_test(data: Self::ParsedTest) -> Self::Answer;
    fn b_test(data: Self::ParsedTest) -> Self::Answer;

    fn final_parse(data: &'static str) -> Result<Self::Parsed, nom::error::Error<&str>> {
        final_parser(Self::parse)(data)
    }

    fn final_parse_test(data: &'static str) -> Result<Self::ParsedTest, nom::error::Error<&str>> {
        final_parser(Self::parse_test)(data)
    }

    fn test_a() -> OutResult {
        assert_eq!(
            Self::a_test(Self::final_parse_test(Self::SAMPLE_DATA)?),
            Self::SAMPLE_ANSWER_A
        );
        println!("a: {}", Self::a(Self::final_parse(Self::DATA)?));
        Ok(())
    }

    fn test_b() -> OutResult {
        assert_eq!(
            Self::b_test(Self::final_parse_test(Self::SAMPLE_DATA)?),
            Self::SAMPLE_ANSWER_B
        );
        println!("b: {}", Self::b(Self::final_parse(Self::DATA)?));
        Ok(())
    }

    fn main() -> OutResult {
        let start = Instant::now();
        let parsed_a = Self::final_parse(Self::DATA)?;
        let parsing_time = start.elapsed();
        let parsed_b = parsed_a.clone();
        let start = Instant::now();
        println!("a: {}", Self::a(parsed_a),);
        let (start, part_a) = (Instant::now(), start.elapsed() + parsing_time);
        println!("b: {}", Self::b(parsed_b),);
        let part_b = start.elapsed() + parsing_time;
        println!("\na runs in {part_a:?}; b runs in {part_b:?}",);
        Ok(())
    }
}

#[macro_export]
macro_rules! boilerplate {
    ($day:ident) => {
        struct $day;

        impl SolutionData for $day {
            const DATA: &'static str = include_str!("data.txt");
            const SAMPLE_DATA: &'static str = include_str!("sample.txt");
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn a() -> OutResult {
                $day::test_a()
            }

            #[test]
            fn b() -> OutResult {
                $day::test_b()
            }
        }

        fn main() -> OutResult {
            $day::main()
        }
    };
}
