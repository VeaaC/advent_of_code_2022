use core::str::FromStr;
pub fn run(input: &str) -> (usize, usize) {
    let mut result_part1 = 0;
    let mut result_part2 = 0;

    let to_range = |x: &str| {
        let (start, end) = x.split_once('-').unwrap();
        usize::from_str(start).unwrap()..=usize::from_str(end).unwrap()
    };
    let parsed_input = input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(left, right)| (to_range(left), to_range(right)));

    for (left, right) in parsed_input.clone() {
        result_part1 += (left.contains(right.start()) && left.contains(right.end())
            || right.contains(left.start()) && right.contains(left.end()))
            as usize;
        result_part2 += (left.contains(right.start())
            || left.contains(right.end())
            || right.contains(left.start())
            || right.contains(left.end())) as usize;
    }

    (result_part1, result_part2)
}
