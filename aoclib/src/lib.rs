#![no_std]

mod day01;

pub fn run(day: u8, input: &str) -> (usize, usize) {
    match day {
        1 => day01::run(input),
        _ => panic!("args"),
    }
}
