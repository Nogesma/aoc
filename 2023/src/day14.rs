use crate::{Input, Output};

fn parse_input(input: Input) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.as_bytes().to_vec()).collect()
}

fn roll_row_west(row: &mut [u8]) {
    let mut bottom = 0;
    for x in 0..row.len() {
        match row[x] {
            b'#' => bottom = x + 1,
            b'O' => {
                if bottom != x {
                    row[bottom] = b'O';
                    row[x] = b'.';
                }
                bottom += 1;
            }
            _ => {}
        }
    }
}

fn roll_row_east(row: &mut [u8]) {
    let mut bottom = row.len() - 1;
    for x in (0..row.len()).rev() {
        match row[x] {
            b'#' => bottom = x.saturating_sub(1),
            b'O' => {
                if bottom != x {
                    row[bottom] = b'O';
                    row[x] = b'.';
                }
                bottom = bottom.saturating_sub(1);
            }
            _ => {}
        }
    }
}

fn roll_north(map: &mut [Vec<u8>]) {
    for x in 0..map[0].len() {
        let mut bottom = 0;
        for y in 0..map.len() {
            match map[y][x] {
                b'#' => bottom = y + 1,
                b'O' => {
                    if bottom != y {
                        map[bottom][x] = b'O';
                        map[y][x] = b'.';
                    }
                    bottom += 1;
                }
                _ => {}
            }
        }
    }
}

fn roll_south(map: &mut [Vec<u8>]) {
    for x in 0..map[0].len() {
        let mut bottom = map.len() - 1;
        for y in (0..map.len()).rev() {
            match map[y][x] {
                b'#' => bottom = y.saturating_sub(1),
                b'O' => {
                    if bottom != y {
                        map[bottom][x] = b'O';
                        map[y][x] = b'.';
                    }
                    bottom = bottom.saturating_sub(1);
                }
                _ => {}
            }
        }
    }
}

fn roll_west(map: &mut [Vec<u8>]) {
    map.iter_mut().for_each(|x| roll_row_west(x));
}

fn roll_east(map: &mut [Vec<u8>]) {
    map.iter_mut().for_each(|x| roll_row_east(x));
}

fn calculate_load(map: &[Vec<u8>]) -> usize {
    map.iter()
        .rev()
        .enumerate()
        .map(|(v, r)| (v + 1) * r.iter().filter(|&&x| x == b'O').count())
        .sum()
}

pub fn main(input: Input) -> Output<usize, usize> {
    let mut map = parse_input(input);

    let mut states = vec![map.clone()];

    roll_north(&mut map);

    let first = calculate_load(&map);

    roll_west(&mut map);
    roll_south(&mut map);
    roll_east(&mut map);

    const N_LOOPS: usize = 1000000000;

    for _ in 1..N_LOOPS {
        if states.contains(&map) {
            break;
        }
        states.push(map.clone());
        roll_north(&mut map);
        roll_west(&mut map);
        roll_south(&mut map);
        roll_east(&mut map);
    }

    let cycle_pos = states.iter().position(|x| *x == map).unwrap();
    let cycle_len = states.len() - cycle_pos;

    let final_map = &states[cycle_pos + ((N_LOOPS - cycle_pos) % cycle_len)];

    let second = calculate_load(final_map);

    Output(first, second)
}
