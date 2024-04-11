use std::io;
use std::io::{Write};

type DayFn = fn() -> String;

#[allow(dead_code)]
fn run_all(days: std::slice::Iter<'_, DayFn>, num_days: usize) -> Vec<String> {
    let mut results = Vec::with_capacity(num_days);

    for f in days {
        results.push(f());
    }

    results
}

fn main() {
    #[allow(unused_variables)]
    let days = [
        aoc2019::template::run,
        aoc2019::day01::run,
    ];

    let result = run_all(days.iter(), days.len());

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for result in result {
        write!(handle, "{}", result).unwrap();
    }
}