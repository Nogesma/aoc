use aoc::generate_days;
use std::iter::Iterator;
use std::{env, fs};

mod day01;
mod day02;

fn run<F: Fn(String) -> (String, String)>(idx: &str, func: F) {
    use std::time::Instant;

    println!("Running {}::main:", idx);

    let input = fs::read_to_string(format!("inputs/{idx}")).unwrap();
    let start = Instant::now();
    let res = func(input);
    let elapsed = start.elapsed();

    println!("Result: {} - {}", res.0, res.1);
    println!("Execution time: {:?}", elapsed);
}

generate_days! {2}

fn main() -> Result<(), String> {
    let idx = env::args().nth(1).ok_or("Missing day argument")?;

    run(&idx, get_day_fn(&idx));

    Ok(())
}
