use core::str::FromStr;

pub fn run_impl(input: &str, reverse: bool) -> [char; 100] {
    let mut result = [' '; 100];
    let mut stacks = [['~'; 100]; 10];
    let mut stack_size = [0; 10];

    for line in input.lines().take_while(|line| !line.contains('1')) {
        for (index, item) in line.chars().skip(1).step_by(4).enumerate() {
            if item == ' ' {
                continue;
            }
            stacks[index][stack_size[index]] = item;
            assert!(('A'..='Z').contains(&item));
            stack_size[index] += 1;
        }
    }
    for (stack, size) in stacks.iter_mut().zip(stack_size) {
        stack[0..size].reverse();
    }
    for line in input.lines().skip_while(|line| !line.is_empty()).skip(1) {
        let (count, rest) = line.split_once(" from ").unwrap();
        let count = usize::from_str(count.strip_prefix("move ").unwrap()).unwrap();
        let (from, to) = rest.split_once(" to ").unwrap();
        let from = usize::from_str(from).unwrap() - 1;
        let to = usize::from_str(to).unwrap() - 1;
        for _ in 0..count {
            stack_size[from] -= 1;
            assert!(('A'..='Z').contains(&stacks[from][stack_size[from]]));
            stacks[to][stack_size[to]] = stacks[from][stack_size[from]];
            stack_size[to] += 1;
        }
        if reverse {
            stacks[to][stack_size[to] - count..stack_size[to]].reverse();
        }
    }
    for i in 0..10 {
        if stack_size[i] != 0 {
            result[i] = stacks[i][stack_size[i] - 1];
        }
    }

    result
}

pub fn run(input: &str) -> ([char; 100], [char; 100]) {
    (run_impl(input, false), run_impl(input, true))
}
