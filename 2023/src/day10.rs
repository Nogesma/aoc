use crate::day10::Direction::{East, Invalid, North, South, West};
use crate::{Input, Output};
use core::hint::unreachable_unchecked;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    North,
    East,
    West,
    South,
    Invalid,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Pipe(Direction, Direction);

impl From<u8> for Pipe {
    fn from(value: u8) -> Self {
        match value {
            b'|' => Self(North, South),
            b'-' => Self(East, West),
            b'L' => Self(North, East),
            b'J' => Self(North, West),
            b'7' => Self(West, South),
            b'F' => Self(East, South),
            b'S' => Self(North, South),
            _ => Self(Invalid, Invalid),
        }
    }
}

fn parse_input(input: Input) -> (Vec<Vec<Pipe>>, (usize, usize)) {
    let mut spos = (0, 0);
    (
        input
            .lines()
            .enumerate()
            .map(|(i, l)| {
                l.bytes()
                    .enumerate()
                    .map(|(j, c)| {
                        if c == b'S' {
                            spos = (i, j);
                        }
                        c.into()
                    })
                    .collect::<Vec<_>>()
            })
            .collect(),
        spos,
    )
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Location {
    Outside,
    Loop,
    Unknown,
    VisitedLoop,
}

type Position = ((usize, usize), Direction);

fn mark_map(location_map: &mut Vec<Vec<Location>>, map: &[Vec<Pipe>]) {
    let mut positions = vec![((0, 0), Invalid)];
    positions.reserve((location_map.len() * location_map[0].len()) / 2);

    while let Some((pos, dir)) = positions.pop() {
        if pos.0 < 0 || pos.1 < 0 {
            continue;
        }
        let upos = (pos.0 as usize, pos.1 as usize);

        if upos.0 == location_map.len() || upos.1 == location_map[0].len() {
            continue;
        }

        match location_map[upos.0][upos.1] {
            Location::Outside | Location::VisitedLoop => continue,
            Location::Unknown => {
                location_map[upos.0][upos.1] = Location::Outside;
                positions.push(((pos.0 - 1, pos.1), South));
                positions.push(((pos.0 + 1, pos.1), North));
                positions.push(((pos.0, pos.1 - 1), East));
                positions.push(((pos.0, pos.1 + 1), West));
            }
            Location::Loop => {
                location_map[upos.0][upos.1] = Location::VisitedLoop;

                match map[upos.0][upos.1] {
                    Pipe(North, South) => {
                        positions.push(((pos.0 - 1, pos.1), dir));
                        positions.push(((pos.0 + 1, pos.1), dir));
                        if dir == East {
                            positions.push(((pos.0, pos.1 + 1), West));
                        } else if dir == West {
                            positions.push(((pos.0, pos.1 - 1), East));
                        }
                    }
                    Pipe(North, East) => {
                        if dir == South || dir == West {
                            positions.push(((pos.0 - 1, pos.1), West));
                            positions.push(((pos.0 + 1, pos.1), North));
                            positions.push(((pos.0, pos.1 - 1), East));
                            positions.push(((pos.0, pos.1 + 1), South));
                        }
                    }
                    Pipe(North, West) => {
                        if dir == South || dir == East {
                            positions.push(((pos.0 - 1, pos.1), East));
                            positions.push(((pos.0 + 1, pos.1), North));
                            positions.push(((pos.0, pos.1 - 1), South));
                            positions.push(((pos.0, pos.1 + 1), West));
                        }
                    }
                    Pipe(East, South) => {
                        if dir == North || dir == West {
                            positions.push(((pos.0 - 1, pos.1), South));
                            positions.push(((pos.0 + 1, pos.1), West));
                            positions.push(((pos.0, pos.1 - 1), East));
                            positions.push(((pos.0, pos.1 + 1), North));
                        }
                    }
                    Pipe(West, South) => {
                        if dir == North || dir == East {
                            positions.push(((pos.0 - 1, pos.1), South));
                            positions.push(((pos.0 + 1, pos.1), East));
                            positions.push(((pos.0, pos.1 - 1), North));
                            positions.push(((pos.0, pos.1 + 1), West));
                        }
                    }
                    Pipe(East, West) => {
                        positions.push(((pos.0, pos.1 - 1), dir));
                        positions.push(((pos.0, pos.1 + 1), dir));
                        if dir == North {
                            positions.push(((pos.0 - 1, pos.1), South));
                        } else if dir == South {
                            positions.push(((pos.0 + 1, pos.1), North));
                        }
                    }
                    _ => unsafe { unreachable_unchecked() },
                }
            }
        }
    }
}

pub fn main(input: Input) -> Output<usize, usize> {
    let (map, spos) = parse_input(input);

    let mut left: Option<Position> = None;
    let mut right = left;

    match map[spos.0][spos.1 - 1] {
        Pipe(East, _) | Pipe(_, East) => left = Some(((spos.0, spos.1 - 1), East)),
        _ => {}
    }
    if let Pipe(North, _) = map[spos.0 + 1][spos.1] {
        if left.is_none() {
            left = Some(((spos.0 + 1, spos.1), North))
        } else {
            right = Some(((spos.0 + 1, spos.1), North))
        }
    }
    match map[spos.0][spos.1 + 1] {
        Pipe(West, _) | Pipe(_, West) => {
            if left.is_none() {
                left = Some(((spos.0, spos.1 + 1), West))
            } else {
                right = Some(((spos.0, spos.1 + 1), West))
            }
        }
        _ => {}
    }
    if let Pipe(_, South) = map[spos.0 - 1][spos.1] {
        right = Some(((spos.0 - 1, spos.1), South))
    }

    let opposite = |dir: Direction, p: Pipe| match p {
        Pipe(d, x) if d == dir => x,
        Pipe(x, d) if d == dir => x,
        _ => unsafe { unreachable_unchecked() },
    };

    let next_pos = |p: &Position| match opposite(p.1, map[p.0 .0][p.0 .1]) {
        North => ((p.0 .0 - 1, p.0 .1), South),
        East => ((p.0 .0, p.0 .1 + 1), West),
        West => ((p.0 .0, p.0 .1 - 1), East),
        South => ((p.0 .0 + 1, p.0 .1), North),
        _ => unsafe { unreachable_unchecked() },
    };

    let (mut left, mut right) = (left.unwrap(), right.unwrap());
    let mut loop_tiles = vec![spos, left.0, right.0];
    let mut first: usize = 1;
    loop {
        left = next_pos(&left);
        right = next_pos(&right);

        first += 1;

        if left.0 == right.0 {
            break;
        }
        loop_tiles.push(left.0);
        loop_tiles.push(right.0);
    }

    loop_tiles.push(left.0);

    let mut location_map = vec![vec![Location::Unknown; map[0].len()]; map.len()];

    loop_tiles
        .iter()
        .for_each(|t| location_map[t.0][t.1] = Location::Loop);

    mark_map(&mut location_map, &map);

    let second = location_map
        .into_iter()
        .map(|l| l.into_iter().filter(|&v| v == Location::Unknown).count())
        .sum::<usize>();

    Output(first, second)
}
