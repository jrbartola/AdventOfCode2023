use common::filereader;
use regex::Regex;

fn parse_line_iterative(line: &String) -> u8 {
    let mut first_digit = None;
    let mut second_digit = None;

    for c in line.chars() {
        match c.to_digit(10) {
            Some(d) => {
                if first_digit.is_none() {
                    first_digit = Some(d);
                } else {
                    second_digit = Some(d)
                }
            }
            None => {}
        }
    }

    if second_digit.is_none() {
        let digit = first_digit.unwrap() as u8;

        return digit * 10 + digit;
    }

    return (first_digit.unwrap() * 10 + second_digit.unwrap()) as u8;
}

fn get_int_for_string(str: String) -> u8 {
    match str.as_str() {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("Bad string {}", str),
    }
}

fn parse_line_regex(line: &String) -> u8 {
    let one_digit_regex =
        Regex::new(r"^.*?(\d|one|two|three|four|five|six|seven|eight|nine).*$").unwrap();
    let two_digits_regex = Regex::new(r"^.*?(\d|one|two|three|four|five|six|seven|eight|nine).*(\d|one|two|three|four|five|six|seven|eight|nine).*$").unwrap();

    let parse_digit = |capture: Option<regex::Match>| {
        capture
            .map(|m| m.as_str().to_string())
            .map(|s| {
                if s.len() == 1 {
                    return s.chars().next().unwrap().to_digit(10).unwrap() as u8;
                } else {
                    return get_int_for_string(s);
                }
            })
            .unwrap()
    };

    two_digits_regex.captures(line).map_or_else(
        || {
            one_digit_regex.captures(line).map_or_else(
                || panic!("Invalid input string {}", line),
                |caps| {
                    let first_digit = parse_digit(caps.get(1));

                    first_digit * 10 + first_digit
                },
            )
        },
        |caps| {
            let first_digit = parse_digit(caps.get(1));

            if caps.get(2).is_none() {
                first_digit * 10 + first_digit
            } else {
                let second_digit = parse_digit(caps.get(2));
                first_digit * 10 + second_digit
            }
        },
    )
}

fn solve(lines: Vec<String>) -> u32 {
    lines
        .iter()
        .fold(0, |acc, line| acc + (parse_line_regex(line) as u32))
}

fn main() {
    match filereader::read_file("./day1/resources/input.txt") {
        Ok(lines) => {
            let result = solve(lines);
            println!("{:?}", result)
        }
        Err(e) => panic!("{}", e),
    }
}
