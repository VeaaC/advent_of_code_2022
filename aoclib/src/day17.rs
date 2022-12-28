fn handle_block<const WIDTH: usize, const HEIGHT: usize>(
    input_pos: &mut usize,
    input: &[u8],
    field: &mut [[u8; 9]; 100000000],
    field_height: &mut usize,
    block: [[u8; WIDTH]; HEIGHT],
) {
    let mut y = *field_height - 3 - HEIGHT;
    let mut x = 3;
    loop {
        let dir = input[*input_pos % input.len()];
        *input_pos += 1;

        let space = |pos_x: usize, pos_y: usize| {
            for sub_x in 0..WIDTH {
                for sub_y in 0..HEIGHT {
                    if block[sub_y][sub_x] == b'#' && field[pos_y + sub_y][pos_x + sub_x] == b'#' {
                        return false;
                    }
                }
            }
            true
        };
        let new_x = match dir {
            b'>' => x + 1,
            b'<' => x - 1,
            _ => panic!("Invalid input"),
        };
        if space(new_x, y) {
            x = new_x;
        }
        if space(x, y + 1) {
            y += 1;
        } else {
            for sub_x in 0..WIDTH {
                for sub_y in 0..HEIGHT {
                    if block[sub_y][sub_x] == b'#' {
                        assert!(field[y + sub_y][x + sub_x] != b'#');
                        field[y + sub_y][x + sub_x] = b'#';
                    }
                }
            }
            *field_height = (*field_height).min(y);
            return;
        }
    }
}

fn run_impl(input: &str, limit: usize) -> usize {
    let input = input.as_bytes();
    let cross: [[u8; 3]; 3] = [
        *b".#.", //
        *b"###", //
        *b".#.", //
    ];
    let corner: [[u8; 3]; 3] = [
        *b"..#", //
        *b"..#", //
        *b"###", //
    ];
    let square: [[u8; 2]; 2] = [
        *b"##", //
        *b"##", //
    ];
    let line: [[u8; 4]; 1] = [
        *b"####", //
    ];
    let bar: [[u8; 1]; 4] = [
        *b"#", //
        *b"#", //
        *b"#", //
        *b"#", //
    ];

    let mut field = [[b' '; 9]; 100000000];
    for x in field.last_mut().unwrap() {
        *x = b'#';
    }
    for row in field.iter_mut() {
        *row.first_mut().unwrap() = b'#';
        *row.last_mut().unwrap() = b'#';
    }
    let mut field_height = field.len() - 1;

    let mut repetition = [None; 5 * 100000];
    let mut input_pos = 0;
    let mut i = 0;
    let mut field_adjustment = 0;
    while i < limit {
        if field_adjustment == 0 {
            if let Some((prev, prev_height)) = repetition[5 * (input_pos % input.len()) + (i % 5)] {
                let matches = || {
                    for y in 0..100 {
                        if field[field_height + y] != field[prev_height + y] {
                            return false;
                        }
                    }
                    true
                };
                if matches() {
                    let num_cycles = (limit - i - 1) / (i - prev) - 1;
                    field_adjustment = num_cycles * (prev_height - field_height);
                    i += num_cycles * (i - prev);
                }
            } else if field_height < field.len() - 100 {
                repetition[5 * (input_pos % input.len()) + (i % 5)] = Some((i, field_height));
            }
        }
        match i % 5 {
            0 => handle_block(&mut input_pos, input, &mut field, &mut field_height, line),
            1 => handle_block(&mut input_pos, input, &mut field, &mut field_height, cross),
            2 => handle_block(&mut input_pos, input, &mut field, &mut field_height, corner),
            3 => handle_block(&mut input_pos, input, &mut field, &mut field_height, bar),
            4 => handle_block(&mut input_pos, input, &mut field, &mut field_height, square),
            _ => unreachable!(),
        }
        i += 1;
    }
    field.len() - 1 - field_height + field_adjustment
}

pub fn run(input: &str) -> (usize, usize) {
    (run_impl(input, 2022), run_impl(input, 1000000000000))
}
