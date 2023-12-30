use crate::Direction::{Left, Right, Up};
use common::filereader;
use std::cmp::max;
use std::collections::{HashSet, VecDeque};

#[derive(Eq, PartialEq, Clone, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(PartialEq, Eq, Clone, Hash)]
struct Trajectory {
    cell: (usize, usize),
    direction: Direction,
}

fn print_traversed_grid(grid: &Vec<Vec<char>>, visited: &HashSet<(usize, usize)>) {
    let mut str_builder = String::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if visited.contains(&(i, j)) {
                str_builder.push('#');
            } else {
                str_builder.push(grid[i][j]);
            }
        }
        str_builder.push('\n')
    }

    println!("{}", str_builder);
}

fn is_in_bounds(grid: &Vec<Vec<char>>, coords: (usize, usize)) -> bool {
    let (r, c) = coords;
    (0..grid.len()).contains(&r) && (0..grid[0].len()).contains(&c)
}

fn get_next_trajectories(grid: &Vec<Vec<char>>, trajectory: Trajectory) -> Vec<Trajectory> {
    let mut next_trajectories = Vec::new();
    let (row, col) = trajectory.cell;
    let curr_char = grid[row][col];

    let mut handle_no_op_cell = || {
        let new_coords = match &trajectory.direction {
            Direction::Up => (row.saturating_sub(1), col),
            Direction::Left => (row, col.saturating_sub(1)),
            Direction::Down => (row + 1, col),
            Direction::Right => (row, col + 1),
        };

        if is_in_bounds(grid, new_coords) {
            next_trajectories.push(Trajectory {
                cell: new_coords,
                direction: trajectory.direction.clone(),
            });
        }
    };

    match curr_char {
        '.' => handle_no_op_cell(),
        '/' => {
            let (new_coords, new_direction) = match trajectory.direction {
                Direction::Up => (Some((row, col + 1)), Direction::Right),
                Direction::Left => (Some((row + 1, col)), Direction::Down),
                Direction::Down => (
                    if col > 0 { Some((row, col - 1)) } else { None },
                    Direction::Left,
                ),
                Direction::Right => (
                    if row > 0 { Some((row - 1, col)) } else { None },
                    Direction::Up,
                ),
            };

            if let Some(new_coords) = new_coords {
                if is_in_bounds(grid, new_coords) {
                    next_trajectories.push(Trajectory {
                        cell: new_coords,
                        direction: new_direction,
                    });
                }
            }
        }
        '\\' => {
            let (new_coords, new_direction) = match trajectory.direction {
                Direction::Up => (
                    if col > 0 { Some((row, col - 1)) } else { None },
                    Direction::Left,
                ),
                Direction::Left => (
                    if row > 0 { Some((row - 1, col)) } else { None },
                    Direction::Up,
                ),
                Direction::Down => (Some((row, col + 1)), Direction::Right),
                Direction::Right => (Some((row + 1, col)), Direction::Down),
            };

            if let Some(new_coords) = new_coords {
                if is_in_bounds(grid, new_coords) {
                    next_trajectories.push(Trajectory {
                        cell: new_coords,
                        direction: new_direction,
                    });
                }
            }
        }
        '|' => {
            if trajectory.direction == Right || trajectory.direction == Direction::Left {
                let (r, c) = trajectory.cell;
                let up_coords = if r > 0 { Some((r - 1, c)) } else { None };
                let down_coords = (r + 1, c);

                if let Some(up_coords) = up_coords {
                    if is_in_bounds(grid, up_coords) {
                        next_trajectories.push(Trajectory {
                            cell: up_coords,
                            direction: Direction::Up,
                        });
                    }
                }

                if is_in_bounds(grid, down_coords) {
                    next_trajectories.push(Trajectory {
                        cell: down_coords,
                        direction: Direction::Down,
                    });
                }
            } else {
                // If we aren't perpendicular, just proceed as normal
                handle_no_op_cell()
            }
        }
        '-' => {
            if trajectory.direction == Direction::Up || trajectory.direction == Direction::Down {
                let (r, c) = trajectory.cell;
                let left_coords = if c > 0 { Some((r, c - 1)) } else { None };
                let right_coords = (r, c + 1);

                if let Some(left_coords) = left_coords {
                    if is_in_bounds(grid, left_coords) {
                        next_trajectories.push(Trajectory {
                            cell: left_coords,
                            direction: Direction::Left,
                        });
                    }
                }

                if is_in_bounds(grid, right_coords) {
                    next_trajectories.push(Trajectory {
                        cell: right_coords,
                        direction: Direction::Right,
                    });
                }
            } else {
                // If we aren't perpendicular, just proceed as normal
                handle_no_op_cell()
            }
        }
        other_char => panic!("Invalid character found: {}", other_char),
    }

    next_trajectories
}

fn traverse_beam(grid: &Vec<Vec<char>>, trajectory: Trajectory) -> HashSet<(usize, usize)> {
    let mut visited_trajectories = HashSet::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::from([trajectory]);

    while let Some(curr_traj) = queue.pop_front() {
        visited.insert(curr_traj.cell);
        visited_trajectories.insert(curr_traj.clone());

        let next_trajs = get_next_trajectories(grid, curr_traj);
        next_trajs.into_iter().for_each(|t| {
            if !visited_trajectories.contains(&t) {
                queue.push_back(t);
            }
        })
    }

    visited
}

fn to_grid(lines: Vec<String>) -> Vec<Vec<char>> {
    lines
        .into_iter()
        .map(|line| line.chars().collect())
        .collect()
}
fn solve(lines: Vec<String>) -> u32 {
    let grid = to_grid(lines);
    let visited_cells = traverse_beam(
        &grid,
        Trajectory {
            cell: (0, 0),
            direction: Right,
        },
    );

    print_traversed_grid(&grid, &visited_cells);

    visited_cells.len() as u32
}

fn solve2(lines: Vec<String>) -> u32 {
    let grid = to_grid(lines);
    let mut max_energized = 0;

    let trajectories_to_try: Vec<Trajectory> = (0..grid.len())
        .map(|r| Trajectory {
            cell: (r, 0),
            direction: Right,
        })
        .chain(
            (0..grid[0].len())
                .map(|c| Trajectory {
                    cell: (0, c),
                    direction: Direction::Down,
                })
                .into_iter(),
        )
        .chain(
            (0..grid.len())
                .map(|r| Trajectory {
                    cell: (r, grid[0].len() - 1),
                    direction: Left,
                })
                .into_iter(),
        )
        .chain(
            (0..grid[0].len())
                .map(|c| Trajectory {
                    cell: (grid.len() - 1, c),
                    direction: Up,
                })
                .into_iter(),
        )
        .collect();

    for trajectory in trajectories_to_try {
        let visited_cells = traverse_beam(&grid, trajectory);

        max_energized = max(max_energized, visited_cells.len() as u32);
    }

    max_energized
}

fn main() {
    match filereader::read_file("./day16/resources/input.txt") {
        Ok(lines) => {
            let result = solve2(lines);
            println!("{:?}", result);
        }
        Err(e) => panic!("{}", e),
    }
}
