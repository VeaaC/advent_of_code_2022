use core::str::FromStr;

fn to_int(input: &str) -> (&str, Option<usize>) {
    let (left, right) = input.split_at(input.find([',', ']']).unwrap_or(input.len()));
    match left {
        "" => (input, None),
        number => {
            let result = usize::from_str(number).unwrap();
            (right, Some(result))
        }
    }
}

fn cmp<'a>(left: &'a str, right: &'a str) -> (&'a str, &'a str, core::cmp::Ordering) {
    let left_rest = left.strip_prefix('[').unwrap_or(left);
    let left_is_list = left != left_rest;

    let right_rest = right.strip_prefix('[').unwrap_or(right);
    let right_is_list = right != right_rest;

    if !left_is_list && !right_is_list {
        let left_int = to_int(left_rest);
        let right_int = to_int(right_rest);
        return (left_int.0, right_int.0, left_int.1.cmp(&right_int.1));
    }

    let mut data = (left_rest, right_rest, core::cmp::Ordering::Equal);
    loop {
        let left_item = left_is_list && !data.0.starts_with(']')
            || !left_is_list && !data.0.starts_with([',', ']']);
        let right_item = right_is_list && !data.1.starts_with(']')
            || !right_is_list && !data.1.starts_with([',', ']']);
        data.2 = data.2.then(left_item.cmp(&right_item));
        if !left_item && !right_item {
            if left_is_list {
                data.0 = data.0.strip_prefix(']').unwrap_or(data.0);
            }
            if right_is_list {
                data.1 = data.1.strip_prefix(']').unwrap_or(data.1);
            }
            return (data.0, data.1, data.2);
        }
        let next = cmp(data.0, data.1);
        data = (next.0, next.1, data.2.then(next.2));
        if left_is_list {
            data.0 = data.0.strip_prefix(',').unwrap_or(data.0);
        }
        if right_is_list {
            data.1 = data.1.strip_prefix(',').unwrap_or(data.1);
        }
    }
}

pub fn run(input: &str) -> (usize, usize) {
    let mut part1 = 0;
    let mut data = [""; 1000];
    let mut data_size = 0;
    for (index, pair) in input.split("\n\n").enumerate() {
        let (first, second) = pair.split_once('\n').unwrap();
        part1 += cmp(first, second).2.is_le() as usize * (index + 1);
        data[data_size] = first;
        data[data_size + 1] = second;
        data_size += 2;
    }
    data[data_size] = "[[2]]";
    data[data_size + 1] = "[[6]]";
    data_size += 2;
    data[..data_size].sort_unstable_by(|left, right| cmp(left, right).2);
    let part2 = (data.iter().position(|&x| x == "[[2]]").unwrap() + 1)
        * (data.iter().position(|&x| x == "[[6]]").unwrap() + 1);
    (part1, part2)
}
