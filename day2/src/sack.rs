use std::cmp::{max, min};

#[derive(Debug)]
pub struct Sack {
    red: i8,
    green: i8,
    blue: i8,
}

impl Sack {
    pub const fn new(red: i8, green: i8, blue: i8) -> Self {
        Sack { red, green, blue }
    }

    pub fn get_min_sack(sacks: Vec<Sack>) -> Sack {
        let mut min_sack = Sack::new(0, 0, 0);

        sacks.iter().for_each(|sack| {
            min_sack.red = max(min_sack.red, sack.red);
            min_sack.green = max(min_sack.green, sack.green);
            min_sack.blue = max(min_sack.blue, sack.blue);
        });

        min_sack
    }

    pub fn from(sack_str: &str) -> Self {
        let mut new_sack = Sack {
            red: 0,
            green: 0,
            blue: 0,
        };

        let colors = sack_str.split(", ");

        colors.for_each(|color_str| {
            let mut splitted = color_str.split_whitespace();

            let num_str = splitted.next().unwrap();
            let color_str = splitted.next().unwrap();

            match color_str {
                "red" => {
                    new_sack.red = num_str.parse::<i8>().unwrap();
                }
                "green" => {
                    new_sack.green = num_str.parse::<i8>().unwrap();
                }
                "blue" => {
                    new_sack.blue = num_str.parse::<i8>().unwrap();
                }
                _ => panic!("Bad color {}", color_str),
            };
        });

        new_sack
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
