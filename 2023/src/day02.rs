use crate::Solution;

pub struct Day02;

pub struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

type Game = Vec<CubeSet>;

impl Day02 {
    fn possible(game: &Game, contents: &CubeSet) -> bool {
        game.iter().all(|cs| {
            cs.red <= contents.red && cs.green <= contents.green && cs.blue <= contents.blue
        })
    }

    fn minimal_cube_set(game: &Game) -> CubeSet {
        CubeSet {
            red: game.iter().map(|cs| cs.red).max().unwrap(),
            green: game.iter().map(|cs| cs.green).max().unwrap(),
            blue: game.iter().map(|cs| cs.blue).max().unwrap(),
        }
    }

    fn power(cs: &CubeSet) -> u32 {
        cs.red * cs.green * cs.blue
    }
}

impl Solution for Day02 {
    type ParsedInput = Vec<Game>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let mut games = Vec::<Game>::new();
        for line in input_lines.lines() {
            let mut game = Game::new();
            for s in line.split_once(": ").unwrap().1.split("; ") {
                let (mut r, mut g, mut b) = (0, 0, 0);
                for t in s.split(", ") {
                    if let Some((count_str, color)) = t.split_once(' ') {
                        let count = count_str.parse::<u32>().unwrap();
                        if color == "red" {
                            r = count;
                        } else if color == "green" {
                            g = count;
                        } else if color == "blue" {
                            b = count;
                        }
                    }
                }
                game.push(CubeSet {
                    red: r,
                    green: g,
                    blue: b,
                });
            }
            games.push(game)
        }
        games
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let games = _parsed_input;
        let contents = CubeSet {
            red: 12,
            green: 13,
            blue: 14,
        };
        games
            .iter()
            .enumerate()
            .filter(|(_, game)| Self::possible(game, &contents))
            .map(|(i, _)| i + 1)
            .sum::<usize>()
            .to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let games = _parsed_input;
        games
            .iter()
            .map(Self::minimal_cube_set)
            .map(|cs| Self::power(&cs))
            .sum::<u32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn check_day02_part1_case1() {
        assert_eq!(Day02::solve_part_one(TEST_INPUT), "8".to_string())
    }

    #[test]
    fn check_day02_part2_case1() {
        assert_eq!(Day02::solve_part_two(TEST_INPUT), "2286".to_string())
    }
}
