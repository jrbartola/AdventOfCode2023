use common::filereader;
use common::formatting::format_grid;
use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque};
use std::ops::Sub;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn from(c: char) -> Self {
        match c {
            'U' => Direction::Up,
            'L' => Direction::Left,
            'D' => Direction::Down,
            'R' => Direction::Right,
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

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct Point(isize, isize);

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point(self.0 - other.0, self.1 - other.1)
    }
}

fn adjust_vertex(a: Point, b: Point) -> (Point, Point) {
    let (mut ax, mut ay) = (a.0, a.1);
    let (mut bx, mut by) = (b.0, b.1);

    if ax == bx {
        // Vertical edge
        by += if ay < by { 1 } else { -1 };
    } else if ay == by {
        // Horizontal edge
        bx += if ax < bx { 1 } else { -1 };
    }

    (Point(ax, ay), Point(bx, by))
}

fn determine_bounding_coords(
    instructions: &Vec<(Direction, u8)>,
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

fn polygon_area(vertices: &[(usize, usize)]) -> f64 {
    fn add_segment(
        segments: &mut HashMap<isize, Vec<(isize, isize)>>,
        key: isize,
        start: isize,
        end: isize,
    ) {
        let entry = segments.entry(key).or_insert_with(Vec::new);
        entry.push((start.min(end), start.max(end)));
    }

    fn group_vertices(
        polygon: &[Point],
    ) -> (
        HashMap<isize, Vec<(isize, isize)>>,
        HashMap<isize, Vec<(isize, isize)>>,
    ) {
        let mut horizontal_segments = HashMap::new();
        let mut vertical_segments = HashMap::new();

        for i in 0..polygon.len() {
            let p1 = polygon[i];
            let p2 = polygon[(i + 1) % polygon.len()]; // Loop back to the start for the last segment

            if p1.1 == p2.1 {
                // Horizontal segment
                add_segment(&mut horizontal_segments, p1.1, p1.0, p2.0);
            } else if p1.0 == p2.0 {
                // Vertical segment
                add_segment(&mut vertical_segments, p1.0, p1.1, p2.1);
            }
        }

        (horizontal_segments, vertical_segments)
    }

    fn count_vertices_in_polygon(polygon: &[Point]) -> usize {
        fn merge_segments(segments: &mut Vec<(isize, isize)>) {
            segments.sort_unstable_by(|a, b| a.0.cmp(&b.0));
            let mut merged: Vec<(isize, isize)> = Vec::new();

            for segment in segments.iter() {
                if let Some(last) = merged.last_mut() {
                    if segment.0 <= last.1 {
                        last.1 = last.1.max(segment.1);
                        continue;
                    }
                }
                merged.push(*segment);
            }

            *segments = merged;
        }

        let (mut horizontal_segments, mut vertical_segments) = group_vertices(polygon);

        let mut count = 0;
        for segments in horizontal_segments.values_mut() {
            merge_segments(segments);
            count += segments
                .iter()
                .map(|&(start, end)| end - start + 1)
                .sum::<isize>();
        }
        for segments in vertical_segments.values_mut() {
            merge_segments(segments);
            count += segments
                .iter()
                .map(|&(start, end)| end - start + 1)
                .sum::<isize>();
        }

        // Adjust for corner points
        let corner_points = horizontal_segments
            .keys()
            .flat_map(|&y| vertical_segments.keys().map(move |&x| Point(x, y)))
            .collect::<Vec<_>>();
        for corner in corner_points.iter() {
            if polygon.contains(corner) {
                count -= 1; // Subtract 1 for each corner point in the polygon
            }
        }

        count as usize
    }

    count_vertices_in_polygon(
        &vertices
            .iter()
            .map(|&(x, y)| Point(x as isize, y as isize))
            .collect::<Vec<_>>(),
    ) as f64

    // lt mut adjusted_vertices = VecDeque::new();
    // let mut next_delta = (0, 0);
    //
    // adjusted_vertices.push_back(Point(vertices[0].0 as isize, vertices[0].1 as isize));
    //
    // for i in 0..(vertices.len() - 1) {
    //     let current = Point(
    //         vertices[i].0 as isize + next_delta.0,
    //         vertices[i].1 as isize + next_delta.1,
    //     );
    //     let next = Point(
    //         vertices[(i + 1) % vertices.len()].0 as isize + next_delta.0,
    //         vertices[(i + 1) % vertices.len()].1 as isize + next_delta.1,
    //     );
    //     let (adjusted_current, adjusted_next) = adjust_vertex(current, next);
    //
    //     next_delta = (
    //         next_delta.0 + adjusted_next.0 - next.0,
    //         next_delta.1 + adjusted_next.1 - next.1,
    //     );
    //
    //     // println!(
    //     //     "{i}: current: {:?}, next: {:?}, adjusted_current: {:?}, adjusted_next: {:?}",
    //     //     current, next, adjusted_current, adjusted_next
    //     // );
    //     //
    //     // // adjusted_vertices.push_back(adjusted_current);
    //     adjusted_vertices.push_back(adjusted_next);
    // }
    //
    // println!("adjusted: {:?}", adjusted_vertices);
    //
    // let mut area = 0.0;
    // for i in 0..adjusted_vertices.len() {
    //     let (x0, y0) = (adjusted_vertices[i].0, adjusted_vertices[i].1);
    //     let (x1, y1) = (
    //         adjusted_vertices[(i + 1) % adjusted_vertices.len()].0,
    //         adjusted_vertices[(i + 1) % adjusted_vertices.len()].1,
    //     );
    //     area += (x0 * y1) as f64 - (y0 * x1) as f64;
    // }
    //
    // area.abs() / 2.0
}

fn parse_line(line: String) -> (Direction, u8) {
    let splitted = line.split_whitespace().collect::<Vec<_>>();

    (
        Direction::from(splitted[0].chars().find(|_| true).unwrap()),
        splitted[1].parse::<u8>().unwrap(),
    )
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

    println!("{}", format_grid(&terrain));
    // println!(" {}", polygon_area(&[(0, 0), (0, 5), (1, 5), (1, 0)]));
    println!(
        " {}",
        polygon_area(&[(0, 0), (0, 5), (3, 5), (3, 8), (5, 8), (5, 0)])
    );
    // vertices.reverse();
    polygon_area(&vertices[..]) as u32
    // 232
}

fn main() {
    match filereader::read_file("./day18/resources/input.txt") {
        Ok(lines) => {
            let result = solve(lines);
            println!("{:?}", result);
        }
        Err(e) => panic!("{}", e),
    }
}
