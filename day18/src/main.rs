use common::filereader;
use common::filewriter::write_file;
use common::formatting::format_grid;
use regex::Regex;
use std::ops::{Add, Div, Sub};

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

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    fn move_to(&self, direction: Direction, steps: u64) -> Point {
        let (dir_row, dir_col) = direction.to_unit_coord();

        Point {
            x: self.x + (dir_row as i64 * steps as i64),
            y: self.y + (dir_col as i64 * steps as i64),
        }
    }

    fn get_area(points: Vec<Point>) -> u64 {
        let mut area = 0_i64;
        for i in 0..points.len() {
            let curr = points[i];
            let next = points[(i + 1) % points.len()];
            area += (curr.y * next.x) - (curr.x * next.y);
        }
        return area.abs().div(2) as u64;
    }
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

fn solve(lines: Vec<String>) -> u64 {
    let instructions: Vec<_> = lines.into_iter().map(|line| parse_line(line)).collect();

    let mut points = vec![];
    let mut curr_point = Point::new(0, 0);

    for &(direction, steps) in instructions.iter() {
        points.push(curr_point);
        curr_point = curr_point.move_to(direction, steps);
    }

    points.push(curr_point);

    let border_points = instructions.iter().fold(0, |acc, &(_, count)| acc + count);
    let inside_area = Point::get_area(points.clone()) - border_points / 2 + 1;

    border_points + inside_area
}

fn solve2(lines: Vec<String>) -> u64 {
    let instructions: Vec<_> = lines.into_iter().map(|line| parse_line_hex(line)).collect();

    let mut points = vec![];
    let mut curr_point = Point::new(0, 0);

    for &(direction, steps) in instructions.iter() {
        points.push(curr_point);
        curr_point = curr_point.move_to(direction, steps);
    }

    points.push(curr_point);

    let border_points = instructions.iter().fold(0, |acc, &(_, count)| acc + count);
    let inside_area = Point::get_area(points.clone()) - border_points / 2 + 1;

    println!("border: {:?}, area: {:?}", border_points, inside_area);

    border_points + inside_area
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
