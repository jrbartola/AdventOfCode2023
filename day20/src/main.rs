mod pulse;

use common::filereader;

fn solve(lines: Vec<String>) -> u32 {
    23
}

fn main() {
    match filereader::read_file("./day20/resources/input.txt") {
        Ok(lines) => {
            let result = solve(lines);
            println!("{:?}", result);
        }
        Err(e) => panic!("{}", e),
    }
}
