use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Position(usize, usize);

type NumberMap = Vec<usize>;
type SymbolMap = Vec<(Position, Vec<usize>)>;

fn is_special(c: u8) -> bool {
    c != b'.' && !c.is_ascii_digit()
}

fn parse_slice(n: &[u8]) -> usize {
    String::from_utf8_lossy(n).parse::<usize>().unwrap()
}

fn parse_input(input: String) -> Vec<Vec<u8>> {
    let lines = input.split('\n').filter(|l| !l.is_empty());
    let mut grid = lines
        .map(|line| format!(".{line}.").into_bytes())
        .collect::<Vec<_>>();

    grid.insert(0, vec![b'.'; grid[0].len()]);
    grid.push(vec![b'.'; grid[0].len()]);

    grid
}

fn parse_grid(grid: &[Vec<u8>]) -> (NumberMap, SymbolMap) {
    let mut numbers: NumberMap = Vec::new();
    let mut symbols: SymbolMap = Vec::new();

    for y in 0..grid.len() {
        let mut start = None;
        let mut chr = HashSet::new();
        for x in 0..grid[y].len() {
            if grid[y][x].is_ascii_digit() {
                if start.is_none() {
                    start = Some(x);
                    if is_special(grid[y][x - 1]) {
                        chr.insert(Position(y, x - 1));
                    }
                }

                let mut check_special = |x| {
                    if is_special(grid[y - 1][x]) {
                        chr.insert(Position(y - 1, x));
                    }
                    if is_special(grid[y + 1][x]) {
                        chr.insert(Position(y + 1, x));
                    }
                };

                check_special(x - 1);
                check_special(x);
                check_special(x + 1);
            } else if let Some(s) = start {
                if is_special(grid[y][x]) {
                    chr.insert(Position(y, x));
                }

                if !chr.is_empty() {
                    let n = parse_slice(&grid[y][s..x]);
                    numbers.push(n);
                    for c in &chr {
                        if grid[c.0][c.1] == b'*' {
                            let pos = symbols.iter_mut().rfind(|(p, _)| p == c);
                            if let Some(s) = pos {
                                s.1.push(n);
                            } else {
                                symbols.push((*c, vec![n]));
                            }
                        }
                    }
                }
                start = None;
                chr.clear();
            }
        }
    }

    (numbers, symbols)
}

pub fn main(input: String) -> (String, String) {
    let grid = parse_input(input);
    let (number, symbol) = parse_grid(&grid);

    let first = number.iter().sum::<usize>().to_string();

    let second = symbol
        .iter()
        .filter_map(|(_, n)| {
            if n.len() == 2 {
                Some(n[0] * n[1])
            } else {
                None
            }
        })
        .sum::<usize>()
        .to_string();

    (first, second)
}
