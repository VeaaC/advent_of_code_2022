fn run_impl(input: &str, num_different: usize) -> usize {
    let mut counts = [0; 26];
    let mut num_set = 0;
    for (pos, (added, removed)) in input
        .chars()
        .zip(
            core::iter::repeat(None)
                .take(num_different)
                .chain(input.chars().map(Some)),
        )
        .enumerate()
    {
        num_set += (counts[added as usize - 'a' as usize] == 0) as usize;
        counts[added as usize - 'a' as usize] += 1;
        if let Some(removed) = removed {
            counts[removed as usize - 'a' as usize] -= 1;
            num_set -= (counts[removed as usize - 'a' as usize] == 0) as usize;
        }
        if num_set == num_different {
            return pos + 1;
        }
    }
    panic!("Not found");
}

pub fn run(input: &str) -> (usize, usize) {
    (run_impl(input, 4), run_impl(input, 14))
}
