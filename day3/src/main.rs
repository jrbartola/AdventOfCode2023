mod part1;
mod part2;

use common::filereader;

fn main() {
    match filereader::read_file("./day3/resources/input.txt") {
        Ok(lines) => {
            let result = part2::solve(lines);
            println!("{:?}", result)
        }
        Err(e) => panic!("{}", e),
    }
}
