use crate::Solution;
use std::collections::HashSet;

pub struct Day16;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Beam {
    r: usize,
    c: usize,
    dr: isize,
    dc: isize,
}

impl Day16 {
    fn move_beam(beam: &Beam, bounds: (usize, usize)) -> Option<Beam> {
        if (beam.r == 0 && beam.dr < 0)
            || (beam.c == 0 && beam.dc < 0)
            || (beam.r == bounds.0 - 1 && beam.dr > 0)
            || (beam.c == bounds.1 - 1 && beam.dc > 0)
        {
            return None;
        }
        let r = (beam.r as isize + beam.dr) as usize;
        let c = (beam.c as isize + beam.dc) as usize;
        Some(Beam {
            r: r,
            c: c,
            dr: beam.dr,
            dc: beam.dc,
        })
    }

    fn energized_tiles(grid: &Vec<Vec<char>>, initial_beam: Beam) -> HashSet<(usize, usize)> {
        // apologies for the spaghetti
        let mut energized = HashSet::new();
        let mut cache = HashSet::new();
        let mut beams = Vec::from([initial_beam]);
        let bounds = (grid.len(), grid[0].len());
        while beams.len() > 0 {
            let mut beam = beams.pop().unwrap();
            if cache.contains(&beam) {
                continue;
            }
            cache.insert(beam);
            energized.insert((beam.r, beam.c));
            let tile = grid[beam.r][beam.c];
            if tile == '/' {
                (beam.dr, beam.dc) = (beam.dc * -1, beam.dr * -1);
                if let Some(new_beam) = Self::move_beam(&beam, bounds) {
                    beams.push(new_beam);
                }
            } else if tile == '\\' {
                (beam.dr, beam.dc) = (beam.dc, beam.dr);
                if let Some(new_beam) = Self::move_beam(&beam, bounds) {
                    beams.push(new_beam);
                }
            } else if tile == '|' {
                if beam.dc == 0 {
                    // up or down, keep going
                    if let Some(new_beam) = Self::move_beam(&beam, bounds) {
                        beams.push(new_beam);
                    }
                } else {
                    // left or right, split
                    let a = Beam {
                        r: beam.r,
                        c: beam.c,
                        dr: -1,
                        dc: 0,
                    };
                    let b = Beam {
                        r: beam.r,
                        c: beam.c,
                        dr: 1,
                        dc: 0,
                    };
                    if let Some(new_beam) = Self::move_beam(&a, bounds) {
                        beams.push(new_beam);
                    }
                    if let Some(new_beam) = Self::move_beam(&b, bounds) {
                        beams.push(new_beam);
                    }
                }
            } else if tile == '-' {
                if beam.dr == 0 {
                    // left or right, keep going
                    if let Some(new_beam) = Self::move_beam(&beam, bounds) {
                        beams.push(new_beam);
                    }
                } else {
                    // up or down, split
                    let a = Beam {
                        r: beam.r,
                        c: beam.c,
                        dr: 0,
                        dc: 1,
                    };
                    let b = Beam {
                        r: beam.r,
                        c: beam.c,
                        dr: 0,
                        dc: -1,
                    };
                    if let Some(new_beam) = Self::move_beam(&a, bounds) {
                        beams.push(new_beam);
                    }
                    if let Some(new_beam) = Self::move_beam(&b, bounds) {
                        beams.push(new_beam);
                    }
                }
            } else {
                // empty space
                if let Some(new_beam) = Self::move_beam(&beam, bounds) {
                    beams.push(new_beam);
                }
            }
        }
        energized
    }
}

impl Solution for Day16 {
    type ParsedInput = Vec<Vec<char>>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<char>>>()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let grid = _parsed_input;
        let initial_beam = Beam {
            r: 0,
            c: 0,
            dr: 0,
            dc: 1,
        };
        Self::energized_tiles(grid, initial_beam).len().to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        "0".to_string()
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn check_day16_part1_case1() {
        assert_eq!(Day16::solve_part_one(TEST_INPUT), "46".to_string())
    }

    #[test]
    fn check_day16_part2_case1() {
        assert_eq!(Day16::solve_part_two(TEST_INPUT), "51".to_string())
    }
}
