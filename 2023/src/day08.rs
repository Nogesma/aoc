use crate::{Input, Output};
use std::collections::HashMap;
use std::hint::unreachable_unchecked;

fn parse_input(input: Input) -> (Vec<u8>, HashMap<u32, usize>, Vec<(usize, usize)>) {
    let (instructions, nodes) = input.0.split_once("\n\n").unwrap();

    let (names, actions): (Vec<_>, Vec<_>) = nodes
        .lines()
        .map(|node| {
            let node = node.replace(' ', "");
            let (name, elements) = node.split_once('=').unwrap();
            let (left, right) = elements.split_once(',').unwrap();

            let to_u32 = |v: &str| {
                let v = v.as_bytes();
                u32::from_be_bytes([0, v[0], v[1], v[2]])
            };
            (to_u32(name), (to_u32(&left[1..]), to_u32(&right[..3])))
        })
        .unzip();

    let names = names
        .into_iter()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect::<HashMap<u32, usize>>();

    let actions = actions
        .iter()
        .map(|(l, r)| (names[l], names[r]))
        .collect::<Vec<_>>();

    (instructions.as_bytes().to_vec(), names, actions)
}

fn find_cycles(
    start: usize,
    instructions: &[u8],
    actions: &[(usize, usize)],
    stop: &[usize],
) -> usize {
    let mut pos = start;

    for (idx, instruction) in instructions.iter().cycle().enumerate() {
        if stop.contains(&pos) {
            return idx;
        }

        if instruction == &b'L' {
            pos = actions[pos].0;
        } else {
            pos = actions[pos].1;
        }
    }

    unsafe { unreachable_unchecked() }
}

const AAA: u32 = u32::from_be_bytes([0, b'A', b'A', b'A']);
const ZZZ: u32 = u32::from_be_bytes([0, b'Z', b'Z', b'Z']);

pub fn main(input: Input) -> Output<usize, usize> {
    let (instructions, names, actions) = parse_input(input);

    let apos = names[&AAA];
    let zpos = names[&ZZZ];

    let first = find_cycles(apos, &instructions, &actions, &[zpos]);

    let start_pos = names.iter().filter_map(|(n, v)| {
        if n & 0xFF == b'A' as u32 && *n != AAA {
            Some(*v)
        } else {
            None
        }
    });

    let end_pos = names
        .iter()
        .filter_map(|(n, v)| {
            if n & 0xFF == b'Z' as u32 && *n != ZZZ {
                Some(*v)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let second = start_pos
        .map(|v| find_cycles(v, &instructions, &actions, &end_pos) / instructions.len())
        .reduce(|acc, e| acc * e)
        .unwrap()
        * first;

    Output(first, second)
}
