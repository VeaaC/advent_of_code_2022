use core::str::FromStr;

pub fn run(input: &str) -> (usize, [char; 10000]) {
    let mut reg = 1;
    let mut cycle = 0;
    let mut next_part1 = 20;
    let mut part1 = 0;
    let mut part2 = [' '; 10000];
    let mut part2_pos = 0;
    let mut add_cycle = |current: isize| {
        if cycle + 1 == next_part1 {
            part1 += (cycle + 1) * current;
            next_part1 += 40;
        }
        if ((cycle % 40) - current).abs() <= 1 {
            part2[part2_pos] = '#';
        } else {
            part2[part2_pos] = '.';
        }
        part2_pos += 1;
        cycle += 1;
        if (cycle % 40) == 0 {
            part2[part2_pos] = '\n';
            part2_pos += 1;
        }
    };
    for line in input.lines() {
        if line == "noop" {
            add_cycle(reg);
            continue;
        }
        if let Some(value) = line.strip_prefix("addx ") {
            let value = isize::from_str(value).unwrap();
            add_cycle(reg);
            add_cycle(reg);
            reg += value;
        }
    }
    (part1.try_into().unwrap(), part2)
}
