use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

pub struct GalaxyCell {
    value: char,
    adjusted_distance: u32,
}

impl GalaxyCell {
    pub fn new(value: char, adjusted_distance: u32) -> Self {
        GalaxyCell {
            value,
            adjusted_distance,
        }
    }
}

pub struct Galaxies {
    cells: Vec<Vec<GalaxyCell>>,
}

impl fmt::Debug for Galaxies {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str_builder = String::new();

        for i in 0..self.cells.len() {
            // println!("len: i={}, j={}", i, self.cells[i].len());
            for j in 0..self.cells[i].len() {
                str_builder.push(self.cells[i][j].value);
            }
            str_builder.push('\n');
        }

        write!(f, "{}", str_builder)
    }
}

impl Galaxies {
    pub fn new(cells: Vec<Vec<GalaxyCell>>) -> Self {
        Galaxies { cells }
    }

    pub fn get_shortest_distances(&self) -> HashMap<(usize, usize), HashMap<(usize, usize), u32>> {
        let mut mappings = HashMap::new();

        let galaxies = self.get_galaxies();

        // First populate the mappings with coordinates of all galaxies
        galaxies.iter().for_each(|coord| {
            mappings.insert(coord, HashMap::from([(coord, 0)]));
        });

        galaxies.iter().fold(HashMap::new(), |mut acc, &coords| {
            acc.insert(coords, self.bfs(coords));
            acc
        })
    }

    fn bfs(&self, coords: (usize, usize)) -> HashMap<(usize, usize), u32> {
        let mut mappings = HashMap::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back((coords.0, coords.1, 0));

        while let Some((r, c, distance)) = queue.pop_front() {
            // If already visited skip that ish
            if visited.contains(&(r, c)) {
                continue;
            }

            if self.is_galaxy(r, c) {
                mappings.insert((r, c), distance);
            }

            visited.insert((r, c));

            self.get_neighbors((r, c))
                .iter()
                .for_each(|&(next_row, next_col)| {
                    if !visited.contains(&(next_row, next_col)) {
                        queue.push_back((
                            next_row,
                            next_col,
                            distance + self.cells[next_row][next_col].adjusted_distance,
                        ));
                    }
                });
        }

        mappings
    }

    fn is_galaxy(&self, r: usize, c: usize) -> bool {
        self.cells[r][c].value == '#'
    }

    fn get_galaxies(&self) -> Vec<(usize, usize)> {
        self.cells
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter().enumerate().filter_map(move |(j, &ref cell)| {
                    if cell.value == '#' {
                        Some((i, j))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    fn get_neighbors(&self, coords: (usize, usize)) -> Vec<(usize, usize)> {
        let (r, c) = coords;
        let mut neighbors = Vec::with_capacity(8);

        if r > 0 {
            // Top
            neighbors.push((r - 1, c));
        }

        if c > 0 {
            // Left
            neighbors.push((r, c - 1));
        }

        if r < self.cells.len() - 1 {
            // Bottom
            neighbors.push((r + 1, c));
        }

        if c < self.cells[0].len() - 1 {
            // Right
            neighbors.push((r, c + 1));
        }

        neighbors
    }
}
