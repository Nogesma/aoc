use crate::day07::HandType::{
    FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair,
};
use crate::{Input, Output};

#[derive(Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct Hand {
    t: HandType,
    cards: [u8; 5],
    bid: usize,
}

impl From<&[u8; 5]> for HandType {
    fn from(value: &[u8; 5]) -> Self {
        let mut t = HighCard;
        'outer: for i in 0..4 {
            for j in 0..i {
                if value[i] == value[j] {
                    continue 'outer;
                }
            }
            let mut count = 1;
            for j in (i + 1)..5 {
                if value[i] == value[j] {
                    count += 1;
                }
            }

            match count {
                5 => return FiveOfAKind,
                4 => return FourOfAKind,
                3 => match t {
                    OnePair => return FullHouse,
                    _ => t = ThreeOfAKind,
                },
                2 => match t {
                    ThreeOfAKind => return FullHouse,
                    OnePair => t = TwoPair,
                    _ => t = OnePair,
                },
                _ => {}
            }
        }
        t
    }
}

fn parse_input(input: Input) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            let cards = cards.as_bytes();
            let mut parsed = [0; 5];
            for i in 0..5 {
                parsed[i] = match cards[i] {
                    b'A' => 14,
                    b'K' => 13,
                    b'Q' => 12,
                    b'J' => 11,
                    b'T' => 10,
                    n => n - b'0',
                }
            }

            Hand {
                t: (&parsed).into(),
                cards: parsed,
                bid: bid.parse().unwrap(),
            }
        })
        .collect()
}

fn get_result(hands: &mut [Hand]) -> usize {
    hands.sort_unstable();

    hands
        .iter()
        .enumerate()
        .map(|(idx, Hand { bid, .. })| (idx + 1) * bid)
        .sum::<usize>()
}

pub fn main(input: Input) -> Output<usize, usize> {
    let mut hands = parse_input(input);

    let first = get_result(&mut hands);

    hands.iter_mut().for_each(|hand| {
        let count = hand.cards.iter_mut().fold(0, |acc, e| {
            if *e == 11 {
                *e = 0;
                acc + 1
            } else {
                acc
            }
        });

        match count {
            4 => hand.t = FiveOfAKind,
            3 => match hand.t {
                FullHouse => hand.t = FiveOfAKind,
                ThreeOfAKind => hand.t = FourOfAKind,
                _ => panic!(),
            },
            2 => match hand.t {
                FullHouse => hand.t = FiveOfAKind,
                TwoPair => hand.t = FourOfAKind,
                OnePair => hand.t = ThreeOfAKind,
                _ => panic!(),
            },
            1 => match hand.t {
                FourOfAKind => hand.t = FiveOfAKind,
                ThreeOfAKind => hand.t = FourOfAKind,
                TwoPair => hand.t = FullHouse,
                OnePair => hand.t = ThreeOfAKind,
                HighCard => hand.t = OnePair,
                _ => panic!(),
            },
            _ => {}
        }
    });

    let second = get_result(&mut hands);

    Output(first, second)
}
