use core::str::FromStr;

pub fn run(input: &str) -> (usize, usize) {
    let mut data = [0_usize; 10000];
    let mut index = [0_usize; 10000];
    let mut sum = [0_usize; 10000];

    let mut size = 0;
    for line in input.lines() {
        if line.is_empty() {
            size += 1;
            index[size + 1] = index[size];
        } else {
            data[index[size + 1]] = usize::from_str(line).unwrap();
            sum[size] += data[index[size + 1]];
            index[size + 1] += 1;
        }
    }
    index[size + 1] = index[size];
    size += 1;

    sum[0..size].sort_unstable_by(|left, right| left.cmp(right).reverse());
    (sum[0], sum[0..3].iter().cloned().sum())
}
