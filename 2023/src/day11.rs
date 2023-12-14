use crate::{Input, Output};

fn parse_input(input: Input) -> (Vec<(usize, usize)>, (usize, usize)) {
    let mut max = (0, 0);

    let galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(x, l)| {
            l.bytes()
                .enumerate()
                .filter_map(|(y, c)| {
                    if c == b'#' {
                        if x > max.0 {
                            max.0 = x;
                        }
                        if y > max.1 {
                            max.1 = y;
                        }
                        Some((x, y))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (galaxies, max)
}

fn sorted(a: usize, b: usize) -> (usize, usize) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

pub fn main(input: Input) -> Output<usize, usize> {
    let (galaxies, size) = parse_input(input);

    let mut expanded_rows = vec![true; size.0 + 1];
    let mut expanded_cols = vec![true; size.1 + 1];

    galaxies.iter().for_each(|x| {
        expanded_rows[x.0] = false;
        expanded_cols[x.1] = false;
    });

    let generate_indexes = |v: Vec<bool>| {
        let mut i = 0;
        v.into_iter()
            .map(|v| {
                if v {
                    i += 1
                }
                i
            })
            .collect::<Vec<_>>()
    };

    let expanded_rows = generate_indexes(expanded_rows);
    let expanded_cols = generate_indexes(expanded_cols);

    let mut total_distance = 0;
    let mut total_expansion = 0;

    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let (g1, g2) = (galaxies[i], galaxies[j]);

            let (min_row, max_row) = sorted(g1.0, g2.0);
            let (min_col, max_col) = sorted(g1.1, g2.1);

            total_distance += max_row - min_row + max_col - min_col;
            total_expansion += expanded_rows[max_row] - expanded_rows[min_row]
                + expanded_cols[max_col]
                - expanded_cols[min_col];
        }
    }

    let first = total_distance + total_expansion;
    Output(first, first + total_expansion * (1000000 - 2))
}
