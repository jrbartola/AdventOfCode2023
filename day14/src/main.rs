use crate::Direction::North;
use common::filereader;
use common::formatting::format_grid;
use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    North,
    West,
    South,
    East,
}

const CYCLE_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::West,
    Direction::South,
    Direction::East,
];

fn get_new_rock_coords(
    lines: &Vec<Vec<char>>,
    direction: &Direction,
    row: usize,
    col: usize,
) -> (usize, usize) {
    let mut new_row = row;
    let mut new_col = col;

    match direction {
        Direction::North => {
            for i in (0..row).rev() {
                match lines[i][col] {
                    '.' => {
                        new_row = i;
                    }
                    _ => break,
                }
            }
        }
        Direction::West => {
            for j in (0..col).rev() {
                match lines[row][j] {
                    '.' => {
                        new_col = j;
                    }
                    _ => break,
                }
            }
        }
        Direction::South => {
            for i in (row + 1)..lines.len() {
                match lines[i][col] {
                    '.' => {
                        new_row = i;
                    }
                    _ => break,
                }
            }
        }
        Direction::East => {
            for j in (col + 1)..lines[0].len() {
                match lines[row][j] {
                    '.' => {
                        new_col = j;
                    }
                    _ => break,
                }
            }
        }
    }

    (new_row, new_col)
}

fn roll(mut lines: Vec<Vec<char>>, direction: &Direction) -> Vec<Vec<char>> {
    match direction {
        North => {
            for i in 0..lines.len() {
                for j in 0..lines[0].len() {
                    if lines[i][j] == 'O' {
                        let (new_i, new_j) = get_new_rock_coords(&lines, direction, i, j);
                        lines[i][j] = '.';
                        lines[new_i][new_j] = 'O';
                    }
                }
            }

            lines
        }
        Direction::West => {
            for j in 0..lines[0].len() {
                for i in 0..lines.len() {
                    if lines[i][j] == 'O' {
                        let (new_i, new_j) = get_new_rock_coords(&lines, direction, i, j);
                        lines[i][j] = '.';
                        lines[new_i][new_j] = 'O';
                    }
                }
            }

            lines
        }
        Direction::South => {
            for i in (0..lines.len()).rev() {
                for j in 0..lines[0].len() {
                    if lines[i][j] == 'O' {
                        let (new_i, new_j) = get_new_rock_coords(&lines, direction, i, j);
                        lines[i][j] = '.';
                        lines[new_i][new_j] = 'O';
                    }
                }
            }

            lines
        }
        Direction::East => {
            for j in (0..lines[0].len()).rev() {
                for i in 0..lines.len() {
                    if lines[i][j] == 'O' {
                        let (new_i, new_j) = get_new_rock_coords(&lines, direction, i, j);
                        lines[i][j] = '.';
                        lines[new_i][new_j] = 'O';
                    }
                }
            }

            lines
        }
    }
}

fn run_cycles(mut lines: Vec<Vec<char>>, num_cycles: u32) -> Vec<Vec<char>> {
    for cycle in 0..num_cycles {
        println!("Running cycle {}", cycle);
        for direction in CYCLE_DIRECTIONS {
            lines = roll(lines, &direction);
        }
    }

    println!("{}", format_grid(&lines));

    lines
}

fn solve(lines: Vec<String>) -> u32 {
    let char_vec: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let rolled_forward = roll(char_vec, &North);

    rolled_forward
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let num_rollable_rocks = row.iter().filter(|&&c| c == 'O').count() as u32;
            (rolled_forward.len() - i) as u32 * num_rollable_rocks
        })
        .sum()
}

fn solve2(lines: Vec<String>) -> u32 {
    let char_vec: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let cycled_out = run_cycles(char_vec, 10000);

    cycled_out
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let num_rollable_rocks = row.iter().filter(|&&c| c == 'O').count() as u32;
            (cycled_out.len() - i) as u32 * num_rollable_rocks
        })
        .sum()
}

fn main() {
    match filereader::read_file("./day14/resources/input.txt") {
        Ok(lines) => {
            let result = solve2(lines);
            println!("{:?}", result);
        }
        Err(e) => panic!("{}", e),
    }
}
