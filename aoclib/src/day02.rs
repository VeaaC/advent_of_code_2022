pub fn run(input: &str) -> (usize, usize) {
    let mut score_part1 = 0;
    let mut score_part2 = 0;

    #[allow(clippy::identity_op)]
    for line in input.lines() {
        let round = line.split_once(' ').unwrap();
        score_part1 += match round {
            ("A", "X") => 3 + 1,
            ("A", "Y") => 6 + 2,
            ("A", "Z") => 0 + 3,
            ("B", "X") => 0 + 1,
            ("B", "Y") => 3 + 2,
            ("B", "Z") => 6 + 3,
            ("C", "X") => 6 + 1,
            ("C", "Y") => 0 + 2,
            ("C", "Z") => 3 + 3,
            _ => panic!("invalid input"),
        };
        score_part2 += match round {
            ("A", "X") => 0 + 3,
            ("A", "Y") => 3 + 1,
            ("A", "Z") => 6 + 2,
            ("B", "X") => 0 + 1,
            ("B", "Y") => 3 + 2,
            ("B", "Z") => 6 + 3,
            ("C", "X") => 0 + 2,
            ("C", "Y") => 3 + 3,
            ("C", "Z") => 6 + 1,
            _ => panic!("invalid input"),
        };
    }

    (score_part1, score_part2)
}
