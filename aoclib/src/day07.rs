use core::str::FromStr;

fn subtree_size<'a, 'b>(
    mut input: &'a str,
    callback: &'b mut impl FnMut(usize),
) -> (usize, &'a str) {
    let mut sum = 0;
    while let Some((line, rest)) = input.split_once('\n') {
        input = rest;
        if line.starts_with("$ cd ..") {
            break;
        }
        if let Some(_name) = line.strip_prefix("$ cd ") {
            let sub_folder = subtree_size(input, callback);
            sum += sub_folder.0;
            input = sub_folder.1;
        } else if line == "$ ls" {
            while input.starts_with(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])
                || input.starts_with("dir ")
            {
                let (ls_line, ls_rest) = input.split_once('\n').unwrap_or((input, ""));
                input = ls_rest;
                if !ls_line.starts_with("dir ") {
                    sum +=
                        usize::from_str(ls_line.split_once(' ').expect(ls_line).0).expect(ls_line);
                }
            }
        } else {
            panic!("{}", line);
        }
    }
    callback(sum);
    (sum, input)
}

pub fn run(input: &str) -> (usize, usize) {
    let mut part1 = 0;
    let (total_size, _) = subtree_size(input.strip_prefix("$ cd /\n").unwrap(), &mut |size| {
        if size <= 100000 {
            part1 += size;
        }
    });
    let threshold = total_size - (70000000 - 30000000);
    let mut part2 = None;
    subtree_size(input.strip_prefix("$ cd /\n").unwrap(), &mut |size| {
        if size > threshold && (part2.is_none() || Some(size) < part2) {
            part2 = Some(size);
        }
    });
    (part1, part2.unwrap())
}
