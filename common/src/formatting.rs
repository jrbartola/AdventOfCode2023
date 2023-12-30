pub fn format_grid(grid: &Vec<Vec<char>>) -> String {
    let mut grid_str = String::with_capacity(grid.len() & grid[0].len());

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            grid_str.push(grid[i][j]);
        }
        grid_str.push('\n')
    }

    grid_str
}
