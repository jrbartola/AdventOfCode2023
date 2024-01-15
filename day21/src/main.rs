use common::{filereader, filewriter};
use std::cell::Cell;
use std::collections::{HashSet, VecDeque};
use std::ops::Div;
use std::time::Instant;

pub struct CellFillRequest {
    start_coords: (usize, usize),
    num_cells: usize,
    steps: usize,
}

impl CellFillRequest {
    pub fn new(start_coords: (usize, usize), num_cells: usize, steps: usize) -> Self {
        CellFillRequest {
            start_coords,
            num_cells,
            steps,
        }
    }
}

fn print_grid(grid: &Vec<Vec<char>>, coords: &HashSet<(usize, usize)>) {
    let mut matrix = vec![vec!['.'; grid[0].len()]; grid.len()];

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if coords.contains(&(i, j)) {
                matrix[i][j] = 'O';
            } else {
                matrix[i][j] = grid[i][j];
            }
        }
    }

    let to_print: Vec<String> = matrix
        .iter()
        .map(|r| r.iter().fold(String::new(), |acc, &c| format!("{acc}{c}")))
        .collect();

    filewriter::write_file("./day21/resources/output.txt", to_print);
}

fn get_neighbors(grid: &Vec<Vec<char>>, coords: (usize, usize)) -> Vec<(usize, usize)> {
    let (r, c) = coords;
    let mut neighbors = Vec::with_capacity(8);

    if r > 0 {
        // Top
        neighbors.push((r - 1, c));
    }

    if c > 0 {
        // Left
        neighbors.push((r, c - 1));
    }

    if r < grid.len() - 1 {
        // Bottom
        neighbors.push((r + 1, c));
    }

    if c < grid[0].len() - 1 {
        // Right
        neighbors.push((r, c + 1));
    }

    neighbors
}

fn find_coords_where(grid: &Vec<Vec<char>>, pred: fn(char) -> bool) -> Option<(usize, usize)> {
    let row = grid
        .iter()
        .position(|r| r.iter().find(|&&c| pred(c)).is_some());

    let col =
        (0..grid[0].len()).position(|c| (0..grid.len()).find(|&r| pred(grid[r][c])).is_some());

    match (row, col) {
        (Some(row), Some(col)) => Some((row, col)),
        _ => None,
    }
}

fn bfs_propagate(grid: &Vec<Vec<char>>, start: (usize, usize), distance: isize) -> usize {
    let timer = Instant::now();

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start.0, start.1, 0));

    while let Some(&(r, c, dist)) = queue.front() {
        if visited.contains(&(dist, (r, c))) {
            queue.pop_front();
            continue;
        }

        if dist >= distance {
            break;
        }

        queue.pop_front();

        visited.insert((dist, (r, c)));

        for (new_row, new_col) in get_neighbors(grid, (r, c)) {
            if grid[new_row][new_col] != '#' {
                queue.push_back((new_row, new_col, dist + 1));
            }
        }
    }

    let mut heap_coords: HashSet<(usize, usize)> =
        queue.into_iter().map(|(r, c, _)| (r, c)).collect();

    let heap_len = heap_coords.len();

    print_grid(grid, &heap_coords.into_iter().collect());

    println!("Time elapsed: {:?}", timer.elapsed());

    heap_len
}

fn solve(lines: Vec<String>) -> u32 {
    let grid: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();

    if let Some((row, col)) = find_coords_where(&grid, |c| c == 'S') {
        return bfs_propagate(&grid, (row, col), 6) as u32;
    }

    unreachable!()
}

fn solve2(lines: Vec<String>) -> u64 {
    let grid: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();

    let size = /*11_usize;*/  grid.len();
    let steps = /*49_usize; */ 26501365;

    let total_grid_width = steps.div(size) - 1;
    let even_tiles = usize::pow(total_grid_width.div(2) * 2 + 1, 2);
    let odd_tiles = usize::pow((total_grid_width + 1).div(2) * 2, 2);

    let big_tiles = total_grid_width;
    let lil_tiles = total_grid_width + 1;

    // Now that we have the counts right, let's construct the coords and step counts necessary to populate the grid cells for those types
    let even_start_coords = (size.div(2), size.div(2));
    let odd_start_coords = (size.div(2), size.div(2));

    let top_mid_start_coords = (size - 1, size.div(2));
    let left_mid_start_coords = (size.div(2), size - 1);
    let bottom_mid_start_coords = (0, size.div(2));
    let right_mid_start_coords = (size.div(2), 0);

    let top_right_coords = (size - 1, 0);
    let top_left_coords = (size - 1, size - 1);
    let bottom_left_coords = (0, size - 1);
    let bottom_right_coords = (0, 0);

    // ----------------------------- Step counts -----------------------------

    let even_steps = size * 2 + 1;
    let odd_steps = size * 2;

    let mid_steps = size - 1;

    let big_diagonal_steps = size + size.div(2) - 1;
    let small_diagonal_steps = size.div(2) - 1;

    // ------------------------------- BEGIN ------------------------------
    let cell_requests = [
        CellFillRequest::new(even_start_coords, even_tiles, even_steps),
        CellFillRequest::new(odd_start_coords, odd_tiles, odd_steps),
        CellFillRequest::new(top_mid_start_coords, 1, mid_steps),
        CellFillRequest::new(left_mid_start_coords, 1, mid_steps),
        CellFillRequest::new(bottom_mid_start_coords, 1, mid_steps),
        CellFillRequest::new(right_mid_start_coords, 1, mid_steps),
        CellFillRequest::new(top_right_coords, big_tiles, big_diagonal_steps),
        CellFillRequest::new(top_right_coords, lil_tiles, small_diagonal_steps),
        CellFillRequest::new(top_left_coords, big_tiles, big_diagonal_steps),
        CellFillRequest::new(top_left_coords, lil_tiles, small_diagonal_steps),
        CellFillRequest::new(bottom_left_coords, big_tiles, big_diagonal_steps),
        CellFillRequest::new(bottom_left_coords, lil_tiles, small_diagonal_steps),
        CellFillRequest::new(bottom_right_coords, big_tiles, big_diagonal_steps),
        CellFillRequest::new(bottom_right_coords, lil_tiles, small_diagonal_steps),
    ];

    let mut total = 0_u64;

    if let Some((row, col)) = find_coords_where(&grid, |c| c == 'S') {
        for CellFillRequest {
            start_coords,
            num_cells,
            steps,
        } in cell_requests
        {
            let available_spots = bfs_propagate(&grid, start_coords, steps as isize);
            total += (available_spots as u64) * (num_cells as u64);
        }
    }

    total
}

fn main() {
    match filereader::read_file("./day21/resources/input.txt") {
        Ok(lines) => {
            let result = solve2(lines);
            println!("{:?}", result);
        }
        Err(e) => panic!("{}", e),
    }
}

#[cfg(test)]
mod tests {
    use crate::{bfs_propagate, find_coords_where};
    use common::filereader;

    #[test]
    fn bfs_example() {
        let grid = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '#', '#', '#', '.', '#', '.'],
            vec!['.', '#', '#', '#', '.', '#', '#', '.', '.', '#', '.'],
            vec!['.', '.', '#', '.', '#', '.', '.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '.', '#', '.', '#', '.', '.', '.', '.'],
            vec!['.', '#', '#', '.', '.', 'S', '#', '#', '#', '#', '.'],
            vec!['.', '#', '#', '.', '.', '#', '.', '.', '.', '#', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '#', '#', '.', '.'],
            vec!['.', '#', '#', '.', '#', '.', '#', '#', '#', '#', '.'],
            vec!['.', '#', '#', '.', '.', '#', '#', '.', '#', '#', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];

        assert_eq!(bfs_propagate(&grid, (5, 5), 6), 16)
    }

    #[test]
    fn bfs_part_1() {
        match filereader::read_file("../day21/resources/input.txt") {
            Ok(lines) => {
                let grid: Vec<Vec<char>> = lines
                    .into_iter()
                    .map(|line| line.chars().collect())
                    .collect();

                if let Some((row, col)) = find_coords_where(&grid, |c| c == 'S') {
                    assert_eq!(bfs_propagate(&grid, (row, col), 64) as u32, 3542)
                }
            }
            Err(e) => panic!("{}", e),
        }
    }
}
