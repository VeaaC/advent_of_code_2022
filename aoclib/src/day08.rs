pub fn run(input: &str) -> (usize, usize) {
    let mut field = [-1_isize; 128 * 128];
    let width = input.lines().next().unwrap().chars().count() + 2;
    let height = input.lines().count() + 2;
    let index = |x: usize, y: usize| y * width + x;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            field[index(x + 1, y + 1)] = c as isize - '0' as isize;
        }
    }

    let part1 = (1..width - 1)
        .flat_map(|x| (1..height - 1).map(move |y| (x, y)))
        .filter(|&(x, y)| {
            (x + 1..width - 1).all(|next_x| field[index(next_x, y)] < field[index(x, y)])
                || (1..x).all(|next_x| field[index(next_x, y)] < field[index(x, y)])
                || (y + 1..height - 1).all(|next_y| field[index(x, next_y)] < field[index(x, y)])
                || (1..y).all(|next_y| field[index(x, next_y)] < field[index(x, y)])
        })
        .count();

    let part2 = (1..width - 1)
        .flat_map(|x| (1..height - 1).map(move |y| (x, y)))
        .map(|(x, y): (usize, usize)| {
            let this_one = field[index(x, y)];
            let until_larger = |max: &mut isize, current: isize| {
                if *max >= this_one {
                    None
                } else {
                    *max = (*max).max(current);
                    Some(())
                }
            };

            let result1 = (x + 1..width - 1)
                .map(|next_x| field[index(next_x, y)])
                .scan(-1, until_larger)
                .count();
            let result2 = (1..x)
                .rev()
                .map(|next_x| field[index(next_x, y)])
                .scan(-1, until_larger)
                .count();
            let result3 = (y + 1..height - 1)
                .map(|next_y| field[index(x, next_y)])
                .scan(-1, until_larger)
                .count();
            let result4 = (1..y)
                .rev()
                .map(|next_y| field[index(x, next_y)])
                .scan(-1, until_larger)
                .count();
            result1 * result2 * result3 * result4
        })
        .max()
        .unwrap();

    (part1, part2)
}
