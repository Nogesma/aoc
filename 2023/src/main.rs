// mod day10;

use aoc::generate_days;
use std::fmt::Display;
use std::iter::Iterator;
use std::{env, fs};

struct Input(String);

impl From<String> for Input {
    fn from(value: String) -> Self {
        Self(fs::read_to_string(value).unwrap())
    }
}

impl Input {
    fn lines(&self) -> impl Iterator<Item = &str> {
        let mut l = self.0.split('\n');
        l.next_back();
        l
    }
}

struct Output<T: Display, U: Display>(T, U);

fn run<F: Fn(Input) -> Output<T, U>, T: Display, U: Display>(idx: &str, func: F) {
    use std::time::Instant;

    println!("Running {}::main:", idx);

    let input: Input = format!("inputs/{idx}").into();
    let start = Instant::now();
    let res = func(input);
    let elapsed = start.elapsed();

    println!("Result: {} - {}", res.0, res.1);
    println!("Execution time: {:?}", elapsed);
}

generate_days!(7);

fn main() -> Result<(), String> {
    let idx = env::args().nth(1).ok_or("Missing day argument")?;

    run_day(&idx);

    Ok(())
}
