use crate::Solution;
use std::collections::HashSet;

pub struct Day06;

impl Day06 {
    fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
        grid.iter()
            .enumerate()
            .filter_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(c, &ch)| {
                        if ch == '^' || ch == '>' || ch == '<' || ch == 'v' {
                            Some((r, c))
                        } else {
                            None
                        }
                    })
                    .next()
            })
            .next()
            .unwrap()
    }

    fn turn_right(dr: isize, dc: isize) -> (isize, isize) {
        match (dr, dc) {
            (0, 1) => (1, 0),
            (1, 0) => (0, -1),
            (0, -1) => (-1, 0),
            (-1, 0) => (0, 1),
            _ => panic!("Invalid character"),
        }
    }

    fn patrol(grid: &Vec<Vec<char>>) -> (HashSet<(usize, usize)>, HashSet<(usize, usize)>) {
        let (rows, cols) = (grid.len(), grid[0].len());
        let (r, c) = Self::find_start(grid);
        let (dr, dc): (isize, isize) = match grid[r][c] {
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            '^' => (-1, 0),
            _ => panic!("Invalid character"),
        };
        let mut stack = Vec::new();
        stack.push(((r, c, dr, dc), HashSet::new(), None));
        let mut positions = HashSet::new(); // positions visited when no candidate
        let mut obstructions = HashSet::new(); // candidates that cause a loop
        while let Some(state) = stack.pop() {
            let ((r, c, mut dr, mut dc), mut visited, candidate) = state;
            if visited.contains(&(r, c, dr, dc)) {
                obstructions.insert(candidate.unwrap());
                continue; // found a loop
            }
            visited.insert((r, c, dr, dc));
            if candidate.is_none() {
                positions.insert((r, c)); // normal path
            }
            if (r == 0 && dr < 0)
                || (c == 0 && dc < 0)
                || (r == rows - 1 && dr > 0)
                || (c == cols - 1 && dc > 0)
            {
                continue; // leaving the grid
            }
            let (nr, nc) = ((r as isize + dr) as usize, (c as isize + dc) as usize);
            if grid[nr][nc] == '#' || candidate == Some((nr, nc)) {
                // encountered an obstacle
                (dr, dc) = Self::turn_right(dr, dc);
                stack.push(((r, c, dr, dc), visited.clone(), candidate));
            } else {
                stack.push(((nr, nc, dr, dc), visited.clone(), candidate));
                // pretend there's an obstacle
                if candidate.is_none()
                    && grid[nr][nc] != '#'
                    && !visited.contains(&(nr, nc, 0, 1))
                    && !visited.contains(&(nr, nc, 0, -1))
                    && !visited.contains(&(nr, nc, 1, 0))
                    && !visited.contains(&(nr, nc, -1, 0))
                {
                    (dr, dc) = Self::turn_right(dr, dc);
                    stack.push(((r, c, dr, dc), visited.clone(), Some((nr, nc))));
                }
            }
        }
        println!("{:?} {:?}", positions.len(), obstructions.len());
        (positions, obstructions)
    }
}

impl Solution for Day06 {
    type ParsedInput = Vec<Vec<char>>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<char>>>()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let grid = _parsed_input;
        Self::patrol(grid).0.len().to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let grid = _parsed_input;
        Self::patrol(grid).1.len().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn check_day06_part1_case1() {
        assert_eq!(Day06::solve_part_one(TEST_INPUT), "41".to_string())
    }

    #[test]
    fn check_day06_part2_case1() {
        assert_eq!(Day06::solve_part_two(TEST_INPUT), "6".to_string())
    }
}
