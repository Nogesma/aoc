use crate::{Input, Output};

fn parse_input(input: Input) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|x| {
            x.split_ascii_whitespace()
                .map(|v| v.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn extrapolate(history: Vec<isize>) -> (isize, isize) {
    let gen_sequence = |v: &[isize]| {
        let mut r = Vec::with_capacity(v.len() - 1);
        for i in 0..(v.len() - 1) {
            r.push(v[i + 1] - v[i]);
        }
        r
    };

    let mut sequences = vec![history];
    loop {
        sequences.push(gen_sequence(sequences.last().unwrap()));

        if sequences.last().unwrap().windows(2).all(|w| w[0] == w[1]) {
            break;
        }
    }

    let mut forward = *sequences.last().unwrap().first().unwrap();
    let mut backwards = forward;
    sequences.iter().rev().skip(1).for_each(|l| {
        forward += l.last().unwrap();
        backwards = l.first().unwrap() - backwards;
    });

    (forward, backwards)
}

pub fn main(input: Input) -> Output<isize, isize> {
    let history = parse_input(input);

    let (first, second) = history
        .into_iter()
        .map(extrapolate)
        .reduce(|(facc, sacc), (f, s)| (facc + f, sacc + s))
        .unwrap();

    Output(first, second)
}
