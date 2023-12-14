use crate::{Input, Output};
use ahash::HashMapExt;
use memoize::memoize;
use rayon::prelude::*;
use std::hint::unreachable_unchecked;

fn parse_input(input: Input) -> Vec<(String, Vec<usize>)> {
    input
        .lines()
        .map(|l| {
            let (springs, n) = l.split_once(' ').unwrap();
            (
                springs.to_string(),
                n.split(',').map(|x| x.parse::<usize>().unwrap()).collect(),
            )
        })
        .collect()
}

// Check each combination in reverse, as truncating a vec is faster than creating a new one
#[memoize(CustomHasher: ahash::HashMap)]
fn count_combinations(mut springs: String, mut obj: Vec<usize>, mut in_block: bool) -> usize {
    if obj.is_empty() {
        return if in_block || springs.contains('#') {
            0
        } else {
            1
        };
    }

    let last = obj.len() - 1;

    loop {
        match springs.pop() {
            Some('.') => {
                if in_block {
                    return if obj[last] != 0 {
                        0
                    } else {
                        obj.pop();
                        count_combinations(springs, obj, false)
                    };
                }
            }
            Some('#') => {
                if obj[last] == 0 {
                    return 0;
                }
                in_block = true;
                obj[last] -= 1;
            }
            Some('?') => {
                return if in_block {
                    if obj[last] == 0 {
                        obj.pop();
                        count_combinations(springs, obj, false)
                    } else {
                        obj[last] -= 1;
                        count_combinations(springs, obj, true)
                    }
                } else {
                    let a = count_combinations(springs.clone(), obj.clone(), false);
                    obj[last] -= 1;
                    a + count_combinations(springs, obj, true)
                };
            }
            None => return if obj == [0] { 1 } else { 0 },
            _ => unsafe { unreachable_unchecked() },
        };
    }
}

pub fn main(input: Input) -> Output<usize, usize> {
    let map = parse_input(input);

    let first = map
        .iter()
        .cloned()
        .map(|(a, b)| count_combinations(a, b, false))
        .sum::<usize>();

    let second = map
        .into_par_iter()
        .map(|(mut a, b)| {
            a.push('?');
            let mut a = a.repeat(5);
            a.pop();
            count_combinations(a, b.repeat(5), false)
        })
        .sum::<usize>();

    Output(first, second)
}
