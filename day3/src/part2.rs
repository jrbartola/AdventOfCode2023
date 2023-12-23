use std::collections::HashMap;

fn get_gear(
    lines: &Vec<String>,
    start_col_idx: usize,
    end_col_idx: usize,
    row_index: usize,
) -> Option<(usize, usize)> {
    let num_rows = lines.len();
    let num_cols = lines.get(0).unwrap().len();
    let mut all_adjacent_cells: Vec<(usize, usize)> = Vec::new();

    if start_col_idx > 0 {
        all_adjacent_cells.push((row_index, start_col_idx - 1));

        if row_index > 0 {
            all_adjacent_cells.push((row_index - 1, start_col_idx - 1));
        }

        if row_index < num_rows - 1 {
            all_adjacent_cells.push((row_index + 1, start_col_idx - 1));
        }
    }

    if end_col_idx < num_cols - 1 {
        all_adjacent_cells.push((row_index, end_col_idx + 1));

        if row_index > 0 {
            all_adjacent_cells.push((row_index - 1, end_col_idx + 1));
        }

        if row_index < num_rows - 1 {
            all_adjacent_cells.push((row_index + 1, end_col_idx + 1));
        }
    }

    for col in start_col_idx..=end_col_idx {
        if row_index > 0 {
            all_adjacent_cells.push((row_index - 1, col));
        }

        if row_index < num_rows - 1 {
            all_adjacent_cells.push((row_index + 1, col));
        }
    }

    all_adjacent_cells
        .iter()
        .find(|&&(r, c)| {
            let cell: char = lines.get(r).and_then(|row| row.chars().nth(c)).unwrap();

            cell == '*'
        })
        .map(|&res| res)
}

fn compute_gear_ratios(gear_map: &HashMap<(usize, usize), Vec<u32>>) -> u32 {
    gear_map.values().fold(0, |acc, nums| {
        if nums.len() == 2 {
            return acc + nums.iter().product::<u32>();
        }

        return acc;
    })
}

pub(crate) fn solve(lines: Vec<String>) -> u32 {
    let mut gear_map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for row in 0..lines.len() {
        let mut char_index = 0;
        let mut curr_num_string = String::with_capacity(142);

        while char_index < lines.get(row).unwrap().len() {
            if let Some(c) = lines.get(row).unwrap().chars().nth(char_index) {
                if c.is_numeric() {
                    curr_num_string.push(c);
                } else if curr_num_string.len() > 0 {
                    if let Some((gear_row, gear_col)) = get_gear(
                        &lines,
                        char_index - curr_num_string.len(),
                        char_index - 1,
                        row,
                    ) {
                        println!(
                            "Number {} borders gear at indices ({}, {}), Incrementing..",
                            curr_num_string, gear_row, gear_col
                        );

                        // use the entry API- the `or_insert` method returns a mutable reference to the value we just
                        // added, which we can dereference to increment and mutate it
                        gear_map
                            .entry((gear_row, gear_col))
                            .or_insert(Vec::new())
                            .push(curr_num_string.parse::<u32>().unwrap());
                    }
                    curr_num_string = String::with_capacity(142);
                }
            }

            char_index += 1;
        }

        if let Some((gear_row, gear_col)) = get_gear(
            &lines,
            char_index - curr_num_string.len(),
            char_index - 1,
            row,
        ) {
            println!(
                "Number {} borders gear at indices ({}, {}), Incrementing..",
                curr_num_string, gear_row, gear_col
            );

            // use the entry API- the `or_insert` method returns a mutable reference to the value we just
            // added, which we can dereference to increment and mutate it
            gear_map
                .entry((gear_row, gear_col))
                .or_insert(Vec::new())
                .push(curr_num_string.parse::<u32>().unwrap());
        }
    }

    println!("{:?}", gear_map);
    compute_gear_ratios(&gear_map)
}
