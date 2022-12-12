#![no_std]

use core::fmt::Write;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Output {
    Number((usize, usize)),
    String(([char; 100], [char; 100])),
}

impl core::fmt::Display for Output {
    fn fmt(
        &self,
        fmt: &mut core::fmt::Formatter<'_>,
    ) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Output::Number((part1, part2)) => {
                fmt.write_fmt(format_args!("Part1: {part1}\n"))?;
                fmt.write_fmt(format_args!("Part2: {part2}\n"))
            }
            Output::String((part1, part2)) => {
                fmt.write_str("Part1: ")?;
                for c in part1 {
                    fmt.write_char(*c)?;
                }
                fmt.write_char('\n')?;
                fmt.write_str("Part2: ")?;
                for c in part2 {
                    fmt.write_char(*c)?;
                }
                fmt.write_char('\n')
            }
        }
    }
}

pub fn run(day: u8, input: &str) -> Output {
    match day {
        1 => Output::Number(day01::run(input)),
        2 => Output::Number(day02::run(input)),
        3 => Output::Number(day03::run(input)),
        4 => Output::Number(day04::run(input)),
        5 => Output::String(day05::run(input)),
        6 => Output::Number(day06::run(input)),
        _ => panic!("args"),
    }
}
