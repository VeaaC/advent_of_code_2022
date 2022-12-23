use core::str::FromStr;

pub fn run_impl(input: &str, num_knots: usize) -> usize {
    let mut visited = [(0, 0); 10000];
    let mut num_visited = 1;
    let mut rope = [(0_isize, 0_isize); 11];
    for line in input.lines() {
        let (dir, step) = line.split_once(' ').unwrap();
        let step = usize::from_str(step).unwrap();
        for _ in 0..step {
            match dir {
                "R" => rope[num_knots - 1].0 += 1,
                "L" => rope[num_knots - 1].0 -= 1,
                "U" => rope[num_knots - 1].1 -= 1,
                "D" => rope[num_knots - 1].1 += 1,
                _ => panic!("Invalid input"),
            }
            for i in (0..num_knots - 1).rev() {
                if (rope[i + 1].0 - rope[i].0).abs() > 1 || (rope[i + 1].1 - rope[i].1).abs() > 1 {
                    rope[i].0 += (rope[i + 1].0 - rope[i].0).signum();
                    rope[i].1 += (rope[i + 1].1 - rope[i].1).signum();
                    if i == 0 {
                        visited[num_visited] = rope[i];
                        num_visited += 1;
                    }
                }
            }
        }
    }
    visited[..num_visited].sort_unstable();
    1 + visited[..num_visited]
        .iter()
        .zip(&visited[1..num_visited])
        .filter(|(a, b)| a != b)
        .count()
}

pub fn run(input: &str) -> (usize, usize) {
    (run_impl(input, 2), run_impl(input, 10))
}
