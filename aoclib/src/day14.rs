use core::str::FromStr;

pub fn run(input: &str) -> (usize, usize) {
    let mut data = [' '; 1000 * 1000];
    let index = |x, y| {
        assert!(x < 1000);
        assert!(y < 1000);
        x + y * 1000
    };

    for line in input.lines() {
        let coords = line.split(" -> ").map(|coord| {
            let (x, y) = coord.split_once(',').unwrap();
            (usize::from_str(x).unwrap(), usize::from_str(y).unwrap())
        });
        for (from, to) in coords.clone().zip(coords.skip(1)) {
            for x in from.0.min(to.0)..=from.0.max(to.0) {
                data[index(x, from.1)] = '#';
            }
            for y in from.1.min(to.1)..=from.1.max(to.1) {
                data[index(from.0, y)] = '#';
            }
        }
    }

    let max_y = (0..1000)
        .rfind(|&y| (0..1000).any(|x| data[index(x, y)] != ' '))
        .unwrap();
    for x in 0..1000 {
        data[index(x, max_y + 2)] = 'X';
    }

    let mut part1 = 0;
    for i in 0.. {
        if data[index(500, 0)] != ' ' {
            return (part1, i);
        }
        let mut pos = (500, 0);
        loop {
            if part1 == 0 && pos.1 > max_y {
                part1 = i;
            }
            if data[index(pos.0, pos.1 + 1)] == ' ' {
                pos = (pos.0, pos.1 + 1);
            } else if data[index(pos.0 - 1, pos.1 + 1)] == ' ' {
                pos = (pos.0 - 1, pos.1 + 1);
            } else if data[index(pos.0 + 1, pos.1 + 1)] == ' ' {
                pos = (pos.0 + 1, pos.1 + 1);
            } else {
                data[index(pos.0, pos.1)] = '#';
                break;
            }
        }
    }
    panic!("Endless loop");
}
