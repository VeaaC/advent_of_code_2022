use core::str::FromStr;

fn compute_ranges(data: &[(isize, isize, isize)], row: isize) -> ([(isize, isize); 100], usize) {
    let mut ranges = [(0, 0); 100];
    let mut ranges_size = 0;
    for (x, y, max_distance) in data {
        let distance = max_distance - (row - y).abs();
        if distance >= 0 {
            ranges[ranges_size] = (x - distance, x + distance);
            ranges_size += 1;
        }
    }
    ranges[..ranges_size].sort_unstable();

    (ranges, ranges_size)
}

pub fn run(input: &str) -> (usize, usize) {
    let mut data = [(0, 0, 0); 100];
    let mut data_size = 0;
    let mut beacons = [(0, 0); 100];
    let mut beacons_size = 0;
    for line in input.lines() {
        let (sensor, beacon) = line.split_once(": closest beacon is at x=").unwrap();
        let sensor = sensor.strip_prefix("Sensor at x=").unwrap();
        let (sensor_x, sensor_y) = sensor.split_once(", y=").unwrap();
        let sensor_x = isize::from_str(sensor_x).unwrap();
        let sensor_y = isize::from_str(sensor_y).unwrap();
        let (beacon_x, beacon_y) = beacon.split_once(", y=").unwrap();
        let beacon_x = isize::from_str(beacon_x).unwrap();
        let beacon_y = isize::from_str(beacon_y).unwrap();
        let distance = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs();
        data[data_size] = (sensor_x, sensor_y, distance);
        data_size += 1;
        beacons[data_size] = (beacon_x, beacon_y);
    }
    beacons[..data_size].sort_unstable();
    for i in 0..data_size {
        if i == 0 || beacons[i - 1] != beacons[i] {
            beacons[beacons_size] = beacons[i];
            beacons_size += 1;
        }
    }

    let (ranges, ranges_size) = compute_ranges(&data[..data_size], 2000000);

    let mut part1 = 0;
    let mut next_start = isize::MIN;
    for &(start, end) in &ranges[..ranges_size] {
        part1 += (end + 1 - start.max(next_start)).clamp(0, isize::MAX);
        next_start = next_start.max(end + 1);
    }
    for &(_, y) in &beacons[..beacons_size] {
        if y == 2000000 {
            part1 -= 1;
        }
    }

    for row in 0..=4000000 {
        let (ranges, ranges_size) = compute_ranges(&data[..data_size], row);

        let mut next_start = 0;
        for &(start, end) in &ranges[..ranges_size] {
            if start > next_start {
                let part2 = row + next_start * 4000000;
                return (part1.try_into().unwrap(), part2.try_into().unwrap());
            }
            let clamped_end = end.min(4000000);
            next_start = next_start.max(clamped_end + 1);
        }
    }

    panic!("No result found");
}
