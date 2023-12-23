fn is_valid_number(
    lines: &Vec<String>,
    start_col_idx: usize,
    end_col_idx: usize,
    row_index: usize,
) -> bool {
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

    return all_adjacent_cells.iter().any(|&(r, c)| {
        let cell: char = lines.get(r).and_then(|row| row.chars().nth(c)).unwrap();
        if !cell.is_numeric() && cell != '.' {
            return true;
        }

        return false;
    });
}

pub(crate) fn solve(lines: Vec<String>) -> u32 {
    let mut total = 0;

    for row in 0..lines.len() {
        let mut char_index = 0;
        let mut curr_num_string = String::with_capacity(142);

        while char_index < lines.get(row).unwrap().len() {
            if let Some(c) = lines.get(row).unwrap().chars().nth(char_index) {
                if c.is_numeric() {
                    curr_num_string.push(c);
                } else if curr_num_string.len() > 0 {
                    if is_valid_number(
                        &lines,
                        char_index - curr_num_string.len(),
                        char_index - 1,
                        row,
                    ) {
                        println!(
                            "Adding: {} at indices ({}, {})",
                            curr_num_string,
                            row,
                            char_index - curr_num_string.len()
                        );
                        total += curr_num_string.parse::<u32>().unwrap();
                    }
                    curr_num_string = String::with_capacity(142);
                }
            }

            char_index += 1;
        }

        if curr_num_string.len() > 0
            && is_valid_number(
                &lines,
                char_index - curr_num_string.len(),
                char_index - 1,
                row,
            )
        {
            println!(
                "Adding: {} at indices ({}, {})",
                curr_num_string,
                row,
                char_index - curr_num_string.len()
            );
            total += curr_num_string.parse::<u32>().unwrap();
        }
    }

    total
}
