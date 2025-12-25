use aoc2025::solve_day;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Selects a single day to run. If not specified, all days are run.
    day: Option<i32>,
}

fn main() {
    let cli = Cli::parse();
    let days = (1..=12).collect::<Vec<i32>>();
    let mut days_to_execute = vec![];
    if let Some(day) = cli.day {
        if !days.contains(&day) {
            panic!("Day not found");
        }
        days_to_execute.push(day);
    } else {
        days_to_execute = days
    }
    for day in days_to_execute {
        solve_day(&day, true);
    }
}
