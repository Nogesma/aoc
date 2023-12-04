fn parse_input(input: String) -> Vec<(Vec<usize>, Vec<usize>)> {
    let lines = input.split('\n').filter(|l| !l.is_empty());
    lines
        .map(|line| {
            let (_, numbers) = line.split_once(':').unwrap();
            let (winning, numbers) = numbers.split_once('|').unwrap();

            let winning = winning
                .split(' ')
                .filter_map(|n| n.parse::<usize>().ok())
                .collect::<Vec<usize>>();
            let numbers = numbers
                .split(' ')
                .filter_map(|n| n.parse::<usize>().ok())
                .collect::<Vec<usize>>();
            (winning, numbers)
        })
        .collect()
}

fn get_matches(matches: &[usize], cards: &mut [Option<usize>], idx: usize) -> usize {
    if let Some(v) = cards[idx] {
        return v;
    }

    let mut res = matches[idx];
    for i in 1..=matches[idx] {
        if let Some(x) = cards[idx + i] {
            res += x;
        } else {
            res += get_matches(matches, cards, idx + i);
        }
    }
    cards[idx] = Some(res);
    res
}

pub fn main(input: String) -> (String, String) {
    let cards = parse_input(input);

    let matches = cards
        .iter()
        .map(|(w, n)| n.iter().filter(|n| w.contains(n)).count())
        .collect::<Vec<_>>();

    let first = matches
        .iter()
        .map(|&v| if v > 0 { 1 << (v - 1) } else { 0 })
        .sum::<usize>()
        .to_string();

    let mut ncards = vec![None; cards.len()];

    let second = (0..cards.len())
        .map(|i| get_matches(&matches, &mut ncards, i) + 1)
        .sum::<usize>()
        .to_string();

    (first, second)
}
