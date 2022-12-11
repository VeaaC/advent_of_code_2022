#![no_std]

mod day01;
mod day02;
mod day03;
mod day04;

pub fn run(day: u8, input: &str) -> (usize, usize) {
    match day {
        1 => day01::run(input),
        2 => day02::run(input),
        3 => day03::run(input),
        4 => day04::run(input),
        _ => panic!("args"),
    }
}
