use common::filereader;
use common::filewriter::write_file;
use common::formatting::format_grid;
use regex::Regex;
use std::cmp::{max, min};
use std::collections::{HashSet, VecDeque};
use std::ops::Add;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'U' => Direction::Up,
            'L' => Direction::Left,
            'D' => Direction::Down,
            'R' => Direction::Right,
            _ => unreachable!(),
        }
    }

    fn from_digit(d: u8) -> Self {
        match d {
            3 => Direction::Up,
            2 => Direction::Left,
            1 => Direction::Down,
            0 => Direction::Right,
            _ => unreachable!(),
        }
    }

    fn to_unit_coord(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Left => (0, -1),
            Direction::Down => (1, 0),
            Direction::Right => (0, 1),
        }
    }
}

fn fill_bfs(grid: &mut Vec<Vec<char>>, start_coords: Vec<(usize, usize)>) {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue = VecDeque::new();

    queue.extend(start_coords);

    while let Some((r, c)) = queue.pop_front() {
        if visited.contains(&(r, c)) {
            continue;
        }

        let curr_char = grid[r][c];

        if curr_char == '.' {
            grid[r][c] = 'X';
        } else {
            continue;
        }

        for (r_new, c_new) in get_neighbors(grid, (r, c)) {
            if grid[r_new][c_new] != '#' && !visited.contains(&(r_new, c_new)) {
                queue.push_back((r_new, c_new));
            }
        }
    }
}

fn get_neighbors(grid: &Vec<Vec<char>>, coords: (usize, usize)) -> Vec<(usize, usize)> {
    let (r, c) = coords;
    // let cell = self.get_cell(coords.0, coords.1);
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

fn get_border_coords(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let rows = grid.len();
    let cols = grid[0].len();

    (0..rows)
        .map(|r| (r, 0))
        .chain((0..rows).map(|r| (r, cols - 1)))
        .chain((0..cols).map(|c| (0, c)))
        .chain((0..cols).map(|c| (rows - 1, c)))
        .collect()
}

fn determine_bounding_coords(
    instructions: &Vec<(Direction, u64)>,
) -> ((usize, usize), impl Fn((isize, isize)) -> (usize, usize)) {
    let mut row: isize = 0;
    let mut col: isize = 0;

    let mut min_row: isize = row;
    let mut max_row: isize = row;

    let mut min_col: isize = col;
    let mut max_col: isize = col;

    for &(direction, steps) in instructions {
        let (r_dir, c_dir) = direction.to_unit_coord();

        row += steps as isize * r_dir;
        col += steps as isize * c_dir;

        min_row = min(min_row, row);
        max_row = max(max_row, row);

        min_col = min(min_col, col);
        max_col = max(max_col, col);
    }

    let top_left = (min_row, min_col);
    let bottom_right = (max_row as usize, max_col as usize);

    (
        (
            (bottom_right.0 as isize - top_left.0) as usize,
            (bottom_right.1 as isize - top_left.1) as usize,
        ),
        move |(r, c)| ((r - min_row) as usize, (c - min_col) as usize),
    )
}

fn parse_line(line: String) -> (Direction, u64) {
    let splitted = line.split_whitespace().collect::<Vec<_>>();

    (
        Direction::from_char(splitted[0].chars().find(|_| true).unwrap()),
        splitted[1].parse::<u64>().unwrap(),
    )
}

fn parse_line_hex(line: String) -> (Direction, u64) {
    let reg = Regex::new(r"\(#([a-z0-9]{5})(\d)\)").unwrap();
    if let Some(caps) = reg.captures(line.as_str()) {
        let distance_hex = caps.get(1).unwrap().as_str();
        let direction = caps.get(2).unwrap().as_str().parse::<u8>().unwrap();
        return (
            Direction::from_digit(direction),
            u64::from_str_radix(distance_hex, 16).unwrap(),
        );
    }

    unreachable!()
}

fn solve(lines: Vec<String>) -> u32 {
    let instructions: Vec<_> = lines.into_iter().map(|line| parse_line(line)).collect();

    let ((max_row, max_col), map_to_nonzero_coords) = determine_bounding_coords(&instructions);

    let mut terrain = vec![vec!['.'; max_col + 1]; max_row + 1];
    let mut vertices = vec![(0, 0)];

    let mut row = 0_isize;
    let mut col = 0_isize;

    for (direction, steps) in instructions {
        let (dir_row, dir_col) = direction.to_unit_coord();

        for _ in 0..steps {
            row += dir_row;
            col += dir_col;

            let (actual_row, actual_col) = map_to_nonzero_coords((row, col));

            terrain[actual_row][actual_col] = '#';
        }

        vertices.push(map_to_nonzero_coords((row, col)));
    }

    // println!("{}", format_grid(&terrain));

    for coords in get_border_coords(&terrain) {
        fill_bfs(&mut terrain, vec![coords]);
    }

    write_file(
        "./day18/resources/output.txt",
        format_grid(&terrain)
            .split("\n")
            .map(|s| s.to_owned())
            .collect(),
    )
    .expect("Couldn't write to file for whatever reason");

    (terrain.len() * terrain[0].len()) as u32
        - terrain
            .iter()
            .flatten()
            .fold(0, |acc, &c| acc + (if c == 'X' { 1 } else { 0 }))
}

fn solve2(lines: Vec<String>) -> u64 {
    let instructions: Vec<_> = lines.into_iter().map(|line| parse_line_hex(line)).collect();

    let ((max_row, max_col), map_to_nonzero_coords) = determine_bounding_coords(&instructions);
    println!("rows: {}, cols: {}", max_row, max_col);
    let mut terrain = vec![vec!['.'; max_col + 1]; max_row + 1];
    let mut vertices = vec![(0, 0)];

    let mut row = 0_isize;
    let mut col = 0_isize;

    println!("before instructions");
    for (direction, steps) in instructions {
        let (dir_row, dir_col) = direction.to_unit_coord();

        for _ in 0..steps {
            row += dir_row;
            col += dir_col;

            let (actual_row, actual_col) = map_to_nonzero_coords((row, col));

            terrain[actual_row][actual_col] = '#';
        }

        vertices.push(map_to_nonzero_coords((row, col)));
    }

    println!("rows: {}, cols: {}", terrain.len(), terrain[0].len());

    let border_coords = get_border_coords(&terrain);

    fill_bfs(&mut terrain, border_coords);

    // write_file(
    //     "./day18/resources/output.txt",
    //     format_grid(&terrain)
    //         .split("\n")
    //         .map(|s| s.to_owned())
    //         .collect(),
    // )
    // .expect("Couldn't write to file for whatever reason");

    (terrain.len() * terrain[0].len()) as u64
        - terrain
            .iter()
            .flatten()
            .fold(0, |acc, &c| acc + (if c == 'X' { 1 } else { 0 }))
}

fn main() {
    match filereader::read_file("./day18/resources/input.txt") {
        Ok(lines) => {
            let result = solve2(lines);
            println!("{:?}", result);
        }
        Err(e) => panic!("{}", e),
    }
}
