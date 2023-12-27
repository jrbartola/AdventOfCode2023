use common::filereader;

fn compute_differences(values: &Vec<i64>) -> Vec<i64> {
    let mut differences = Vec::with_capacity(values.len() - 1);

    for i in 0..(values.len() - 1) {
        differences.push(values[i + 1] - values[i])
    }

    differences
}

fn get_extrapolated_value(values: Vec<i64>) -> i64 {
    if values.iter().all(|&v| v == 0) {
        return 0;
    }

    let differences = compute_differences(&values);
    let &last_difference = values.last().unwrap();

    let extrapolated = last_difference + get_extrapolated_value(differences);
    println!("{:?}: {:?}", values, extrapolated);
    return extrapolated;
}

fn get_historical_extrapolated_value(values: Vec<i64>) -> i64 {
    if values.iter().all(|&v| v == 0) {
        return 0;
    }

    let differences = compute_differences(&values);
    let &first_difference = values.first().unwrap();

    let extrapolated = first_difference - get_historical_extrapolated_value(differences);
    println!("{:?}: {:?}", values, extrapolated);
    return extrapolated;
}

fn solve(lines: Vec<String>) -> i64 {
    lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|splitted| splitted.parse::<i64>().unwrap())
                .collect()
        })
        .fold(0, |acc, values| acc + get_extrapolated_value(values))
}

fn solve2(lines: Vec<String>) -> i64 {
    lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|splitted| splitted.parse::<i64>().unwrap())
                .collect()
        })
        .fold(0, |acc, values| {
            acc + get_historical_extrapolated_value(values)
        })
}

fn main() {
    match filereader::read_file("./day9/resources/input.txt") {
        Ok(lines) => {
            let result = solve2(lines);
            println!("{:?}", result);
        }
        Err(e) => panic!("{}", e),
    }
}
