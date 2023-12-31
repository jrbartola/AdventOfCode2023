use common::filereader;
use common::formatting::format_grid;
use std::cmp::{min, Reverse};
use std::collections::{BinaryHeap, HashMap};

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
) -> Vec<PQueueCoord> {
    let mut neighbors = Vec::new();

    let left_coord = get_coord_in_direction(grid, coords, direction.left());
    let right_coord = get_coord_in_direction(grid, coords, direction.right());

    if let Some(left_coord) = left_coord {
        neighbors.push(PQueueCoord {
            coords: left_coord,
            direction: direction.left(),
        })
    }

    if let Some(right_coord) = right_coord {
        neighbors.push(PQueueCoord {
            coords: right_coord,
            direction: direction.right(),
        })
    }

    neighbors
}

fn dijkstras(grid: &Vec<Vec<u8>>, start: (usize, usize), goal: (usize, usize)) -> u32 {
    let mut distances: HashMap<(usize, usize, Direction), u32> = HashMap::new();
    let mut predecessors: HashMap<(usize, usize, Direction), (usize, usize, Direction)> =
        HashMap::new();
    let mut priority_queue: BinaryHeap<Reverse<(u32, PQueueCoord)>> = BinaryHeap::new();

    priority_queue.push(Reverse((
        0,
        PQueueCoord {
            coords: start,
            direction: Direction::Right,
        },
    )));

    while let Some(Reverse((
        dist,
        PQueueCoord {
            coords: (r, c),
            direction,
        },
    ))) = priority_queue.pop()
    {
        let curr_dist = *distances.get(&(r, c, direction)).unwrap_or(&u32::MAX);
        if dist > curr_dist {
            // Outdated; skip
            continue;
        }

        let neighbors = get_neighbors(grid, (r, c), direction);

        for PQueueCoord {
            direction: neighbor_direction,
            ..
        } in neighbors
        {
            let mut next_dist = dist;
            let mut next_coord = get_coord_in_direction(grid, (r, c), neighbor_direction);

            for _ in 0..3 {
                if next_coord.is_none() {
                    break;
                }

                let (next_row, next_col) = (
                    next_coord.as_ref().unwrap().0,
                    next_coord.as_ref().unwrap().1,
                );
                next_coord = get_coord_in_direction(grid, next_coord.unwrap(), neighbor_direction);
                next_dist += grid[next_row][next_col] as u32;
                if next_dist
                    < *distances
                        .get(&(next_row, next_col, neighbor_direction))
                        .unwrap_or(&u32::MAX)
                {
                    distances.insert((next_row, next_col, neighbor_direction), next_dist);
                    predecessors
                        .insert((next_row, next_col, neighbor_direction), (r, c, direction));
                    priority_queue.push(Reverse((
                        next_dist,
                        PQueueCoord {
                            coords: (next_row, next_col),
                            direction: neighbor_direction,
                        },
                    )))
                }
            }
        }
    }

    let mut curr = (goal.0, goal.1, Direction::Right);
    let mut cloned_grid: Vec<Vec<char>> = grid
        .clone()
        .iter()
        .map(|r| {
            r.iter()
                .map(|&c| char::from_digit(c as u32, 10).unwrap())
                .collect()
        })
        .collect();

    cloned_grid[goal.0][goal.1] = '.';

    while let Some(&(r, c, dir)) = predecessors.get(&curr) {
        cloned_grid[r][c] = '.';
        curr = (r, c, dir);
    }

    println!("{}", format_grid(&cloned_grid));

    let maybe_right_dist = distances.get(&(goal.0, goal.1, Direction::Right));
    let maybe_down_dist = distances.get(&(goal.0, goal.1, Direction::Down));

    if maybe_down_dist.is_some() && maybe_down_dist.is_some() {
        return min::<u32>(*maybe_down_dist.unwrap(), *maybe_right_dist.unwrap());
    } else if maybe_down_dist.is_some() {
        return *maybe_down_dist.unwrap();
    } else if maybe_right_dist.is_some() {
        return *maybe_right_dist.unwrap();
    }

    unreachable!()
}

fn dijkstra(grid: &[&[u8]], minstep: isize, maxstep: isize) -> i64 {
    let mut dists = HashMap::new();
    let mut predecessors: HashMap<(usize, usize, (isize, isize)), (usize, usize, (isize, isize))> =
        HashMap::new();
    let mut q = BinaryHeap::from_iter([(0, (0, 0, (0, 0)))]);
    while let Some((cost, (r, c, d))) = q.pop() {
        if (r, c) == (grid.len() - 1, grid[0].len() - 1) {
            // println!("{:?}", dists);
            let goal = (grid.len() - 1, grid[0].len() - 1);
            let mut curr = (goal.0, goal.1, (0, 1));
            let mut cloned_grid: Vec<Vec<char>> = grid
                .clone()
                .iter()
                .map(|r| r.iter().map(|&c| c as char).collect())
                .collect();

            cloned_grid[goal.0][goal.1] = '.';

            while let Some(&(r, c, dir)) = predecessors.get(&curr) {
                println!("Predecessor of ({}, {}) is ({}, {})", curr.0, curr.1, r, c);
                cloned_grid[r][c] = '.';
                curr = (r, c, dir);
            }

            println!("{}", format_grid(&cloned_grid));
            return -cost;
        }
        if dists.get(&(r, c, d)).is_some_and(|&c| -cost > c) {
            continue;
        }
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if d == (dr, dc) || d == (-dr, -dc) {
                continue;
            }
            let mut next_cost = -cost;
            for dist in 1..=maxstep {
                let rr = (r as isize + dr * dist) as usize;
                let cc = (c as isize + dc * dist) as usize;
                if rr >= grid.len() || cc >= grid[0].len() {
                    break;
                }
                next_cost += (grid[rr][cc] - b'0') as i64;
                if dist < minstep {
                    continue;
                }
                let key = (rr, cc, (dr, dc));
                if next_cost < *dists.get(&key).unwrap_or(&i64::MAX) {
                    // println!("Pushing ({rr}, {cc}) with cost of {next_cost}");
                    dists.insert(key, next_cost);
                    predecessors.insert(key, (r, c, d));
                    q.push((-next_cost, key));
                }
            }
        }
    }
    unreachable!()
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

    let grid_bytes: &[&[u8]] = &lines.iter().map(|s| str::as_bytes(s)).collect::<Vec<_>>();

    let expected = dijkstra(&grid_bytes, 1, 3);
    println!("-----------------------------------");
    let actual = dijkstras(&grid, (0, 0), (grid.len() - 1, grid[0].len() - 1));

    println!("Expected: {}; Actual: {}", expected, actual);

    actual
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
