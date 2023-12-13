use aoc::generate_days;
use std::fmt::Display;
use std::iter::Iterator;
use std::str::Lines;
use std::{env, fs};

struct Input(String);

impl From<String> for Input {
    fn from(value: String) -> Self {
        Self(fs::read_to_string(value).unwrap())
    }
}

impl Input {
    fn lines(&self) -> Lines<'_> {
        self.0.lines()
    }
}

struct Output<T: Display, U: Display>(T, U);

fn run<F: Fn(Input) -> Output<T, U>, T: Display, U: Display>(idx: &str, test: bool, func: F) {
    use std::time::Instant;

    println!("Running {}::main:", idx);

    let input: Input = if test {
        format!("inputs/test/{idx}")
    } else {
        format!("inputs/{idx}")
    }
    .into();
    let start = Instant::now();
    let res = func(input);
    let elapsed = start.elapsed();

    println!("Result: {} - {}", res.0, res.1);
    println!("Execution time: {:?}", elapsed);
}

generate_days!(8);

fn main() -> Result<(), String> {
    let idx = env::args().nth(1).ok_or("Missing day argument")?;
    let test = env::args().nth(2).map_or(false, |v| v == "-t");

    run_day(&idx, test);

    Ok(())
}
