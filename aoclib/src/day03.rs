pub fn run(input: &str) -> (usize, usize) {
    let mut result_part1 = 0;
    let mut result_part2 = 0;

    let priority = |c| match c {
        'a'..='z' => c as usize - 'a' as usize + 1,
        'A'..='Z' => c as usize - 'A' as usize + 27,
        _ => panic!("input invalid"),
    };

    for line in input.lines() {
        let count = line.chars().count() / 2;
        result_part1 += ('a'..='z')
            .chain('A'..='Z')
            .filter_map(|item| line.chars().take(count).find(|x| *x == item))
            .filter_map(|item| line.chars().skip(count).find(|x| *x == item))
            .map(priority)
            .sum::<usize>();
    }

    let mut lines = input.lines();
    while let Some(first) = lines.next() {
        let second = lines.next().unwrap();
        let third = lines.next().unwrap();
        result_part2 += ('a'..='z')
            .chain('A'..='Z')
            .filter(|item| first.find(*item).is_some())
            .filter(|item| second.find(*item).is_some())
            .filter(|item| third.find(*item).is_some())
            .map(priority)
            .sum::<usize>();
    }

    (result_part1, result_part2)
}
