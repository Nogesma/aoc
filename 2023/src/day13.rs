use crate::{Input, Output};

fn parse_input(input: Input) -> Vec<Vec<Vec<u8>>> {
    input
        .0
        .split("\n\n")
        .map(|m| m.lines().map(|l| l.as_bytes().to_vec()).collect())
        .collect()
}

fn find_reflection(map: &Vec<Vec<u8>>) -> (usize, usize) {
    let mut first = 0;
    let mut second = 0;

    'outer: for i in 0..map.len() - 1 {
        let mut count = 0;
        for j in 0..=i {
            if i + 1 + j == map.len() {
                break;
            }
            for k in 0..map[0].len() {
                if map[i - j][k] != map[i + 1 + j][k] {
                    count += 1;
                    if count > 1 {
                        continue 'outer;
                    }
                }
            }
        }

        if count == 1 && second == 0 {
            second = (i + 1) * 100;
        } else if count == 0 && first == 0 {
            first = (i + 1) * 100;
        }

        if first != 0 && second != 0 {
            return (first, second);
        }
    }

    'outer: for i in 0..map[0].len() - 1 {
        let mut count = 0;
        for j in 0..=i {
            if i + 1 + j == map[0].len() {
                break;
            }
            for row in map {
                if row[i - j] != row[i + 1 + j] {
                    count += 1;
                    if count > 1 {
                        continue 'outer;
                    }
                }
            }
        }

        if count == 1 && second == 0 {
            second = i + 1;
        } else if count == 0 && first == 0 {
            first = i + 1;
        }

        if first != 0 && second != 0 {
            return (first, second);
        }
    }

    panic!()
}

pub fn main(input: Input) -> Output<usize, usize> {
    let maps = parse_input(input);

    let (first, second) = maps
        .iter()
        .map(find_reflection)
        .reduce(|(facc, sacc), (f, s)| (facc + f, sacc + s))
        .unwrap();

    Output(first, second)
}
