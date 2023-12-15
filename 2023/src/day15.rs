use crate::{Input, Output};
use ahash::HashSet;

enum Operation {
    Remove,
    Set(u8),
}

struct Action<'a> {
    label: &'a str,
    op: Operation,
}

fn parse_steps<'a>(steps: &'a [&str]) -> Vec<Action<'a>> {
    steps
        .iter()
        .map(|s| {
            if s.rfind(|c| c == '-').is_some() {
                Action {
                    label: &s[..s.len() - 1],
                    op: Operation::Remove,
                }
            } else {
                let (label, value) = s.split_once('=').unwrap();
                Action {
                    label,
                    op: Operation::Set(value.parse().unwrap()),
                }
            }
        })
        .collect()
}

fn hash(s: &str) -> usize {
    let mut res = 0;

    s.bytes().for_each(|c| {
        res += c as usize;
        res *= 17;
        res %= 256;
    });

    res
}

fn parse_input(input: &Input) -> Vec<&str> {
    input.0.trim_end().split(',').collect()
}

pub fn main(input: Input) -> Output<usize, usize> {
    let steps: Vec<&str> = parse_input(&input);

    let first = steps.iter().map(|x| hash(x)).sum();

    let operations = parse_steps(&steps);

    const EMPTY_VEC: Vec<(&str, u8)> = vec![];

    let mut boxes: [Vec<(&str, u8)>; 256] = [EMPTY_VEC; 256];

    operations.iter().for_each(|op| {
        let b = &mut boxes[hash(op.label)];
        match op.op {
            Operation::Remove => {
                if let Some(pos) = b.iter().position(|(l, _)| *l == op.label) {
                    b.remove(pos);
                }
            }
            Operation::Set(set) => {
                if let Some(v) = b.iter_mut().find(|(l, _)| *l == op.label) {
                    v.1 = set;
                } else {
                    b.push((op.label, set))
                }
            }
        }
    });

    let lenses: HashSet<&str> = operations
        .into_iter()
        .map(|Action { label, .. }| label)
        .collect();

    let second =
        lenses
            .into_iter()
            .filter_map(|l| {
                let idx = hash(l);
                let (pos, focal_length) = boxes[idx]
                    .iter()
                    .enumerate()
                    .find_map(|(idx, (x, v))| if *x == l { Some((idx, v)) } else { None })?;

                Some((idx + 1) * (pos + 1) * *focal_length as usize)
            })
            .sum();

    Output(first, second)
}
