use core::str::FromStr;
struct Monkey<'a> {
    items: [usize; 100],
    num_items: usize,
    operation: &'a str,
    test: usize,
    target_pos: usize,
    target_neg: usize,
    num_inspections: usize,
}

fn parse(input: &str) -> [Monkey; 10] {
    let mut lines = input.lines().fuse();
    core::array::from_fn(|_| {
        let mut result = Monkey {
            items: [0; 100],
            num_items: 0,
            operation: "nothing",
            test: 1,
            target_pos: 0,
            target_neg: 0,
            num_inspections: 0,
        };

        if let Some(header) = lines.next() {
            assert!(header.starts_with("Monkey "));
            let items = lines.next().unwrap();
            let items = items.strip_prefix("  Starting items: ").unwrap();
            for item in items.split(", ") {
                result.items[result.num_items] = usize::from_str(item).unwrap();
                result.num_items += 1;
            }
            let operation = lines.next().unwrap();
            result.operation = operation.strip_prefix("  Operation: new = old ").unwrap();
            let test = lines.next().unwrap();
            let test = test.strip_prefix("  Test: divisible by ").unwrap();
            result.test = usize::from_str(test).unwrap();
            let target_pos = lines.next().unwrap();
            let target_pos = target_pos
                .strip_prefix("    If true: throw to monkey ")
                .unwrap();
            result.target_pos = usize::from_str(target_pos).unwrap();
            let target_neg = lines.next().unwrap();
            let target_neg = target_neg
                .strip_prefix("    If false: throw to monkey ")
                .unwrap();
            result.target_neg = usize::from_str(target_neg).unwrap();
            lines.next();
        }
        result
    })
}

pub fn run_impl(input: &str, part1: bool, num_rounds: usize) -> usize {
    let mut monkeys = parse(input);

    let base: usize = monkeys.iter().map(|x| x.test).product();

    for _ in 0..num_rounds {
        for i in 0..monkeys.len() {
            let items = monkeys[i].items;
            let num_items = monkeys[i].num_items;
            monkeys[i].num_inspections += monkeys[i].num_items;
            monkeys[i].num_items = 0;
            for item in &items[..num_items] {
                let (op, other) = monkeys[i].operation.split_once(' ').unwrap();
                let other = match other {
                    "old" => *item,
                    _ => usize::from_str(other).unwrap(),
                };
                let mut new_item = match op {
                    "+" => item + other,
                    "*" => item * other,
                    _ => panic!("invalid input"),
                };
                if part1 {
                    new_item /= 3;
                } else {
                    new_item %= base;
                }
                let target = if new_item % monkeys[i].test == 0 {
                    monkeys[i].target_pos
                } else {
                    monkeys[i].target_neg
                };
                monkeys[target].items[monkeys[target].num_items] = new_item;
                monkeys[target].num_items += 1;
            }
        }
    }

    monkeys.sort_unstable_by_key(|x| x.num_inspections);

    monkeys[8].num_inspections * monkeys[9].num_inspections
}

pub fn run(input: &str) -> (usize, usize) {
    (run_impl(input, true, 20), run_impl(input, false, 10000))
}
