#![no_std]

mod day01;
mod day02;

pub fn run(day: u8, input: &str) -> (usize, usize) {
    match day {
        1 => day01::run(input),
        2 => day02::run(input),
        _ => panic!("args"),
    }
}
