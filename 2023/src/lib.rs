pub mod day01;

pub trait Solution {
    type ParsedInput;
    /// Parse the input into the type used by the solution.
    /// You may wish to parse as you go rather than ahead of your part_one and part_two functions.
    /// If so, just return input_lines in your implementation of parse_input and do the parsing later.
    fn parse_input(input_lines: &str) -> Self::ParsedInput;
    fn part_one(parsed_input: &mut Self::ParsedInput) -> String;
    fn part_two(parsed_input: &mut Self::ParsedInput) -> String;
    fn solve_part_one(input_lines: &str) -> String {
        Self::part_one(&mut Self::parse_input(input_lines))
    }
    fn solve_part_two(input_lines: &str) -> String {
        Self::part_two(&mut Self::parse_input(input_lines))
    }
    /// Solve the problem and print the solutions to stdout, optionally include wall-clock execution time for this run.
    fn solve(input_lines: &str, include_time: bool) -> (String, String) {
        if include_time {
            Self::solve_with_time(input_lines)
        } else {
            let mut input = Self::parse_input(input_lines);
            let p1 = Self::part_one(&mut input);
            let p2 = Self::part_two(&mut input);
            println!("----------");
            println!("Part 1: {}\nPart 2: {}", p1, p2);
            (p1, p2)
        }
    }
    fn solve_with_time(input_lines: &str) -> (String, String) {
        let start_time = std::time::Instant::now();
        let mut input = Self::parse_input(input_lines);
        let parse_time = start_time.elapsed().as_micros();
        let start_time = std::time::Instant::now();
        let p1 = Self::part_one(&mut input);
        let p1_time = start_time.elapsed().as_micros();
        let start_time = std::time::Instant::now();
        let p2 = Self::part_two(&mut input);
        let p2_time = start_time.elapsed().as_micros();
        println!("----------");
        println!("Parsing... ({} μs)", parse_time);
        println!("Part 1: {} ({} μs)", p1, p1_time);
        println!("Part 2: {} ({} μs)", p2, p2_time);
        (p1, p2)
    }
}

pub fn solve_day(day: &i32, include_time: bool) {
    match day {
        1 => day01::Day01::solve(include_str!("../inputs/01"), include_time),
        _ => panic!("Day not found"),
    };
}
