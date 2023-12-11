use crate::{Input, Output};

fn find(line: &[u8], range: impl Iterator<Item = usize>) -> u8 {
    for i in range {
        if line[i].is_ascii_digit() {
            return line[i] - b'0';
        }
        match &line[i..] {
            l if l.starts_with("one".as_bytes()) => return 1,
            l if l.starts_with("two".as_bytes()) => return 2,
            l if l.starts_with("three".as_bytes()) => return 3,
            l if l.starts_with("four".as_bytes()) => return 4,
            l if l.starts_with("five".as_bytes()) => return 5,
            l if l.starts_with("six".as_bytes()) => return 6,
            l if l.starts_with("seven".as_bytes()) => return 7,
            l if l.starts_with("eight".as_bytes()) => return 8,
            l if l.starts_with("nine".as_bytes()) => return 9,
            _ => (),
        }
    }
    panic!("No number in line: {}", std::str::from_utf8(line).unwrap());
}

pub fn main(input: Input) -> Output<usize, usize> {
    let first = input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let first = chars.clone().find(|c: &char| c.is_ascii_digit()).unwrap();
            let last = chars.rfind(|c: &char| c.is_ascii_digit()).unwrap();
            format!("{first}{last}").parse::<usize>().unwrap()
        })
        .sum::<usize>();

    let second = input
        .lines()
        .map(|line| {
            let line = line.as_bytes();
            let first = find(line, 0..line.len());
            let last = find(line, (0..line.len()).rev());

            format!("{first}{last}").parse::<usize>().unwrap()
        })
        .sum::<usize>();

    Output(first, second)
}
