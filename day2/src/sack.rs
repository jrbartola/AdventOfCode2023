use std::cmp::{max, min};
use std::io::{Error, ErrorKind};
use std::num::IntErrorKind::InvalidDigit;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct Sack {
    red: i8,
    green: i8,
    blue: i8,
}

#[derive(Debug)]
pub enum SackError {
    ParseError(ParseIntError),
    InvalidColor(String),
}

impl From<ParseIntError> for SackError {
    fn from(err: ParseIntError) -> Self {
        SackError::ParseError(err)
    }
}

impl Sack {
    pub const fn new(red: i8, green: i8, blue: i8) -> Self {
        Sack { red, green, blue }
    }

    pub fn get_min_sack(sacks: Vec<Sack>) -> Sack {
        sacks.iter().fold(Sack::new(0, 0, 0), |mut min_sack, sack| {
            min_sack.red = max(min_sack.red, sack.red);
            min_sack.green = max(min_sack.green, sack.green);
            min_sack.blue = max(min_sack.blue, sack.blue);
            min_sack
        })
    }

    pub fn from(sack_str: &str) -> Result<Self, SackError> {
        let mut new_sack = Sack {
            red: 0,
            green: 0,
            blue: 0,
        };

        for color_str in sack_str.split(", ") {
            let mut splitted = color_str.split_whitespace();

            let num_str = splitted.next().unwrap(); // In real code, handle this Option better
            let color_str = splitted.next().unwrap(); // Same as above

            let num = num_str.parse::<i8>()?;

            match color_str {
                "red" => new_sack.red = num,
                "green" => new_sack.green = num,
                "blue" => new_sack.blue = num,
                _ => return Err(SackError::InvalidColor(color_str.to_string())),
            };
        }

        Ok(new_sack)
    }

    pub fn is_valid(&self, sacks: &Vec<Sack>) -> bool {
        let mut predicted_sack = Sack::new(self.red, self.green, self.blue);

        sacks.iter().for_each(|s| {
            predicted_sack.red = min(predicted_sack.red, self.red - s.red);
            predicted_sack.green = min(predicted_sack.green, self.green - s.green);
            predicted_sack.blue = min(predicted_sack.blue, self.blue - s.blue);
        });

        return predicted_sack.red >= 0 && predicted_sack.green >= 0 && predicted_sack.blue >= 0;
    }

    pub fn power(&self) -> i32 {
        (self.red as i32) * (self.green as i32) * (self.blue as i32)
    }
}
