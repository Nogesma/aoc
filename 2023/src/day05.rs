type Seeds = Vec<usize>;
type Map = Vec<(usize, usize, usize)>;

fn parse_input(input: String) -> (Seeds, Vec<Map>) {
    let mut blocks = input.split("\n\n").filter(|l| !l.is_empty());
    let seeds = blocks
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split(' ')
        .filter_map(|v| v.trim().parse::<usize>().ok())
        .collect::<Vec<usize>>();

    let maps = blocks
        .map(|m| {
            m.split('\n')
                .filter(|r| !r.is_empty())
                .skip(1)
                .map(|r| {
                    match &r
                        .split(' ')
                        .filter_map(|v| v.trim().parse::<usize>().ok())
                        .collect::<Vec<_>>()[..]
                    {
                        &[a, b, c] => (a, b, c),
                        _ => panic!(),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (seeds, maps)
}

fn get_next(n: usize, map: &[(usize, usize, usize)]) -> (usize, usize) {
    let mut min = usize::MAX;
    for &(dst, src, range) in map {
        if n >= src && n < src + range {
            return (dst + n - src, src + range - n);
        }
        if n < src && src - n < min {
            min = src - n;
        }
    }

    (n, min)
}

fn get_location(seed: usize, maps: &[Vec<(usize, usize, usize)>]) -> (usize, usize) {
    let mut seed = seed;
    let mut range = usize::MAX;
    for map in maps {
        let (a, b) = get_next(seed, map);

        seed = a;
        if b < range {
            range = b;
        }
    }

    (seed, range)
}

pub fn main(input: String) -> (String, String) {
    let (seeds, maps) = parse_input(input);

    let first = seeds
        .iter()
        .map(|&s| get_location(s, &maps).0)
        .min()
        .unwrap()
        .to_string();

    let second = seeds
        .chunks_exact(2)
        .map(|s| {
            if let &[start, size, ..] = s {
                let mut i = start;
                let mut min = usize::MAX;
                while i < start + size {
                    let (res, skip) = get_location(i, &maps);
                    if res < min {
                        min = res;
                    }
                    i = i.saturating_add(skip);
                }
                min
            } else {
                panic!();
            }
        })
        .min()
        .unwrap()
        .to_string();

    (first, second)
}