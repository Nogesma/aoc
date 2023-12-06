use std::cmp::Ordering;

fn parse_input(input: String) -> (Vec<(usize, usize)>, (usize, usize)) {
    let (times, distances) = input.split_once('\n').unwrap();

    let (_, times) = times.trim().split_once(':').unwrap();
    let (_, distances) = distances.trim().split_once(':').unwrap();

    let ctime = times.replace(' ', "").parse::<usize>().unwrap();
    let cdist = distances.replace(' ', "").parse::<usize>().unwrap();

    let times = times
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let distances = distances
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    (times.zip(distances).collect(), (ctime, cdist))
}

fn count_wins((time, distance): &(usize, usize)) -> usize {
    let optimal = time / 2;

    let mut left = 0;
    let mut right = optimal;
    let mut size = optimal;

    while left < right {
        let mid = left + size / 2;
        let dist = mid * (time - mid);
        match dist.cmp(distance) {
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid,
            Ordering::Equal => {
                left = mid + 1;
                right = left;
            }
        }
        size = right - left;
    }

    if time & 1 == 0 {
        (optimal + 1 - left) * 2 - 1
    } else {
        (optimal + 1 - left) * 2
    }
}

pub fn main(input: String) -> (String, String) {
    let (races, final_race) = parse_input(input);

    let first = races
        .iter()
        .map(count_wins)
        .reduce(|acc, v| acc * v)
        .unwrap()
        .to_string();

    let second = count_wins(&final_race).to_string();

    (first, second)
}
