#![no_std]

use core::fmt::Write;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Output {
    Number((usize, usize)),
    String(([char; 10000], [char; 10000])),
    NumberString((usize, [char; 10000])),
    StringNumber(([char; 10000], usize)),
}

fn strip(mut x: &[char]) -> &[char] {
    while let Some(rest) = x.strip_suffix(&[' ']) {
        x = rest;
    }
    x
}

impl core::fmt::Display for Output {
    fn fmt(
        &self,
        fmt: &mut core::fmt::Formatter<'_>,
    ) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Output::Number((x, _)) | Output::NumberString((x, _)) => {
                fmt.write_fmt(format_args!("Part1:\n{x}\n"))?;
            }
            Output::String((x, _)) | Output::StringNumber((x, _)) => {
                fmt.write_str("Part1:\n")?;
                for c in strip(x) {
                    fmt.write_char(*c)?;
                }
                fmt.write_char('\n')?;
            }
        }
        match self {
            Output::Number((_, x)) | Output::StringNumber((_, x)) => {
                fmt.write_fmt(format_args!("Part2:\n{x}\n"))
            }
            Output::String((_, x)) | Output::NumberString((_, x)) => {
                fmt.write_str("Part2:\n")?;
                for c in strip(x) {
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
        7 => Output::Number(day07::run(input)),
        8 => Output::Number(day08::run(input)),
        9 => Output::Number(day09::run(input)),
        10 => Output::NumberString(day10::run(input)),
        11 => Output::Number(day11::run(input)),
        12 => Output::Number(day12::run(input)),
        13 => Output::Number(day13::run(input)),
        14 => Output::Number(day14::run(input)),
        15 => Output::Number(day15::run(input)),
        16 => Output::Number(day16::run(input)),
        17 => Output::Number(day17::run(input)),
        _ => panic!("args"),
    }
}
