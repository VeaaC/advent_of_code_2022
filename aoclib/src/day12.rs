pub fn run_impl(input: &str, part1: bool) -> usize {
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();
    assert!(height < 1000);
    assert!(width < 1000);

    let mut field = [0_usize; 1000 * 1000];
    let index = |(x, y): (usize, usize)| x + y * width;
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start = (x, y);
                    field[index((x, y))] = 0;
                }
                'E' => {
                    end = (x, y);
                    field[index((x, y))] = 'z' as usize - 'a' as usize;
                }
                _ => field[index((x, y))] = c as usize - 'a' as usize,
            }
        }
    }

    if !part1 {
        core::mem::swap(&mut start, &mut end);
    }
    let cmp = |them, us| {
        if part1 {
            them <= us + 1
        } else {
            us <= them + 1
        }
    };

    let mut dist = [usize::MAX; 1000 * 1000];
    dist[index(start)] = 0;
    let mut q = [start; 1000 * 1000];
    let mut q_pos = 0;
    let mut q_size = 1;
    while q_pos < q_size {
        let pos = q[q_pos];
        q_pos += 1;
        for neighbor in [
            (pos.0, (pos.1 + 1).clamp(0, height - 1)),
            (pos.0, pos.1.saturating_sub(1)),
            ((pos.0 + 1).clamp(0, width - 1), pos.1),
            (pos.0.saturating_sub(1), pos.1),
        ] {
            if cmp(field[index(neighbor)], field[index(pos)])
                && dist[index(neighbor)] > dist[index(pos)] + 1
            {
                q[q_size] = neighbor;
                q_size += 1;
                dist[index(neighbor)] = dist[index(pos)] + 1;
            }
        }
    }

    if part1 {
        dist[index(end)]
    } else {
        (0..height)
            .flat_map(|y| (0..width).map(move |x| (x, y)))
            .filter(|pos| field[index(*pos)] == 0)
            .map(|pos| dist[index(pos)])
            .min()
            .unwrap()
    }
}

pub fn run(input: &str) -> (usize, usize) {
    (run_impl(input, true), run_impl(input, false))
}
