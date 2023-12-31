use common::filereader;
use std::cmp::{min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy, Ord, PartialOrd)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy, Ord, PartialOrd)]
struct PQueueCoord {
    coords: (usize, usize),
    direction: Direction,
    path_len: usize,
}

fn get_coord_in_direction(
    grid: &Vec<Vec<u8>>,
    coords: (usize, usize),
    direction: Direction,
) -> Option<(usize, usize)> {
    let (r, c) = coords;

    match direction {
        Direction::Up => {
            if r > 0 {
                Some((r - 1, c))
            } else {
                None
            }
        }
        Direction::Left => {
            if c > 0 {
                Some((r, c - 1))
            } else {
                None
            }
        }
        Direction::Down => {
            if r < grid.len() - 1 {
                Some((r + 1, c))
            } else {
                None
            }
        }
        Direction::Right => {
            if c < grid[0].len() - 1 {
                Some((r, c + 1))
            } else {
                None
            }
        }
    }
}

fn get_neighbors(
    grid: &Vec<Vec<u8>>,
    coords: (usize, usize),
    direction: Direction,
    path_len: usize,
) -> Vec<PQueueCoord> {
    let mut neighbors = Vec::new();

    let front_coord = get_coord_in_direction(grid, coords, direction);
    let left_coord = get_coord_in_direction(grid, coords, direction.left());
    let right_coord = get_coord_in_direction(grid, coords, direction.right());

    // Try front only if the current path is less than 3
    if let Some(front_coord) = front_coord {
        if path_len < 3 {
            neighbors.push(PQueueCoord {
                coords: front_coord,
                direction,
                path_len: path_len + 1,
            })
        }
    }

    if let Some(left_coord) = left_coord {
        neighbors.push(PQueueCoord {
            coords: left_coord,
            direction: direction.left(),
            path_len: 1,
        })
    }

    if let Some(right_coord) = right_coord {
        neighbors.push(PQueueCoord {
            coords: right_coord,
            direction: direction.right(),
            path_len: 1,
        })
    }

    neighbors
}

fn dijkstras(grid: &Vec<Vec<u8>>, start: (usize, usize), goal: (usize, usize)) -> u32 {
    let mut distances: HashMap<(usize, usize), u32> = HashMap::new();
    let mut predecessors: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut priority_queue: BinaryHeap<Reverse<(u32, PQueueCoord)>> = BinaryHeap::new();

    priority_queue.push(Reverse((
        0,
        PQueueCoord {
            coords: start,
            direction: Direction::Right,
            path_len: 0,
        },
    )));

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let dist = if (i, j) != start { u32::MAX } else { 0 };

            distances.insert((i, j), dist);
        }
    }

    while let Some(Reverse((
        dist,
        PQueueCoord {
            coords: (r, c),
            direction,
            path_len,
        },
    ))) = priority_queue.pop()
    {
        let curr_dist = *distances.get(&(r, c)).unwrap();
        if dist > curr_dist {
            // Outdated; skip
            continue;
        }

        let neighbors = get_neighbors(grid, (r, c), direction, path_len);

        for PQueueCoord {
            coords: (neighbor_row, neighbor_col),
            direction,
            path_len,
        } in neighbors
        {
            let curr_dist = dist + grid[neighbor_row][neighbor_col] as u32;
            if curr_dist < *distances.get(&(neighbor_row, neighbor_col)).unwrap() {
                distances.insert((neighbor_row, neighbor_col), curr_dist);
                predecessors.insert((neighbor_row, neighbor_col), (r, c));
                priority_queue.push(Reverse((
                    curr_dist,
                    PQueueCoord {
                        coords: (neighbor_row, neighbor_col),
                        direction,
                        path_len,
                    },
                )))
            }
        }
    }

    *distances.get(&goal).unwrap()
}

fn solve(lines: Vec<String>) -> u32 {
    let grid: Vec<Vec<u8>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    dijkstras(&grid, (0, 0), (grid.len() - 1, grid[0].len() - 1))
}

fn main() {
    match filereader::read_file("./day17/resources/input.txt") {
        Ok(lines) => {
            let result = solve(lines);
            println!("{:?}", result);
        }
        Err(e) => panic!("{}", e),
    }
}
