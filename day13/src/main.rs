use common::filereader;
use common::formatting::format_grid;

#[derive(Debug, PartialEq)]
pub enum MirrorKind {
    Row(usize),
    Col(usize),
}

fn parse_grids(lines: Vec<String>) -> Vec<Vec<Vec<char>>> {
    let mut grids = Vec::new();

    let mut curr_grid = Vec::new();

    for line in lines {
        if line.len() == 0 {
            grids.push(curr_grid);
            curr_grid = Vec::new();
        } else {
            curr_grid.push(line.chars().collect());
        }
    }

    // Make sure we get the last grid since there's no newline at the end of the file
    if curr_grid.len() > 0 {
        grids.push(curr_grid);
    }

    grids
}

fn try_find_mirrors_cols(grid: &Vec<Vec<char>>) -> Vec<MirrorKind> {
    let mut mirrors = Vec::new();

    for i in 0..(grid[0].len() - 1) {
        // Attempt to place the mirror between every ith and (i+1)th col
        let mut left = i as isize;
        let mut right = i + 1;
        let mut is_mirror = true;

        'main: while left >= 0 && right < grid[0].len() {
            for r in 0..grid.len() {
                if grid[r][left as usize] != grid[r][right] {
                    is_mirror = false;
                    break 'main;
                }
            }

            left -= 1;
            right += 1;
        }

        if is_mirror {
            mirrors.push(MirrorKind::Col(i + 1));
        }
    }

    mirrors
}

fn try_find_mirrors_rows(grid: &Vec<Vec<char>>) -> Vec<MirrorKind> {
    let mut mirrors = Vec::new();

    for i in 0..(grid.len() - 1) {
        // Attempt to place the mirror between every ith and (i+1)th row
        let mut top = i as isize;
        let mut bottom = i + 1;
        let mut is_mirror = true;

        while top >= 0 && bottom < grid.len() {
            if grid[top as usize]
                .iter()
                .zip(grid[bottom].iter())
                .any(|(&top_cell, &bottom_cell)| top_cell != bottom_cell)
            {
                is_mirror = false;
                break;
            } else {
                top -= 1;
                bottom += 1;
            }
        }

        if is_mirror {
            mirrors.push(MirrorKind::Row(i + 1));
        }
    }

    mirrors
}

fn find_mirrors(grid: &Vec<Vec<char>>) -> Vec<MirrorKind> {
    let mirrors_rows = try_find_mirrors_rows(grid);
    let mirrors_cols = try_find_mirrors_cols(grid);

    mirrors_rows
        .into_iter()
        .chain(mirrors_cols.into_iter())
        .collect()
}

fn find_mirror_with_smudge(grid: &Vec<Vec<char>>) -> MirrorKind {
    let mut local_grid = grid.clone();
    let initial_mirror = &find_mirrors(grid)[0];

    for i in 0..local_grid.len() {
        for j in 0..local_grid[i].len() {
            let initial_char = local_grid[i][j];

            // Swap the smudge
            if initial_char == '.' {
                local_grid[i][j] = '#';
            } else {
                local_grid[i][j] = '.';
            }
            let new_mirrors = find_mirrors(&local_grid);

            if let Some(mirror) = new_mirrors.into_iter().find(|m| m != initial_mirror) {
                println!("Fixed smudge at {i},{j}: {:?}", mirror);
                return mirror;
            }

            local_grid[i][j] = initial_char;
        }
    }

    panic!(
        "Bad state. Could not find alternative mirror for smudge on grid:\n{}",
        format_grid(grid)
    );
}

fn solve(lines: Vec<String>) -> u32 {
    let grids = parse_grids(lines);

    let results: Vec<u32> = grids
        .iter()
        .map(|grid| match find_mirrors(grid).get(0) {
            Some(&MirrorKind::Col(cols_left)) => cols_left as u32,
            Some(&MirrorKind::Row(rows_above)) => (rows_above as u32) * 100,
            _ => panic!("Bad state. Shouldn't have a missing mirror for part 1"),
        })
        .collect();

    results.into_iter().sum()
}

fn solve2(lines: Vec<String>) -> u32 {
    let grids = parse_grids(lines);

    let results: Vec<u32> = grids
        .iter()
        .map(|grid| match find_mirror_with_smudge(grid) {
            MirrorKind::Col(cols_left) => cols_left as u32,
            MirrorKind::Row(rows_above) => (rows_above as u32) * 100,
        })
        .collect();

    results.into_iter().sum()
}

fn main() {
    match filereader::read_file("./day13/resources/input.txt") {
        Ok(lines) => {
            let result = solve2(lines);
            println!("{:?}", result);
        }
        Err(e) => panic!("{}", e),
    }
}
