use crate::{Input, Output};
use std::cmp::max;

pub fn main(input: Input) -> Output<usize, usize> {
    let games = input
        .lines()
        .map(|line| {
            let (game, draws) = line.split_once(':').unwrap();
            (game, draws.split(';'))
        })
        .collect::<Vec<_>>();

    let first = games
        .iter()
        .map(|(game, draws)| {
            if draws
                .clone()
                .try_for_each(|d| {
                    d.split(',').try_for_each(|color| {
                        let (v, name) = color.trim().split_once(' ').unwrap();
                        let v: u8 = v.parse().unwrap();
                        if v > 14 {
                            return Err(());
                        }
                        match name {
                            "red" => {
                                if v > 12 {
                                    Err(())
                                } else {
                                    Ok(())
                                }
                            }
                            "green" => {
                                if v > 13 {
                                    Err(())
                                } else {
                                    Ok(())
                                }
                            }
                            _ => Ok(()),
                        }
                    })
                })
                .is_ok()
            {
                game.split_once(' ').unwrap().1.parse().unwrap()
            } else {
                0
            }
        })
        .sum::<usize>();

    let second = games
        .into_iter()
        .map(|(_, draws)| {
            let min = draws
                .map(|d| {
                    let mut n = (0, 0, 0);
                    for color in d.split(',') {
                        let (v, name) = color.trim().split_once(' ').unwrap();
                        let v: usize = v.parse().unwrap();
                        match name {
                            "red" => n.0 = v,
                            "green" => n.1 = v,
                            _ => n.2 = v,
                        }
                    }
                    n
                })
                .reduce(|acc, e| (max(acc.0, e.0), max(acc.1, e.1), max(acc.2, e.2)))
                .unwrap();

            min.0 * min.1 * min.2
        })
        .sum::<usize>();

    Output(first, second)
}
