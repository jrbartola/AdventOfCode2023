use common::filewriter;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum CellKind {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Animal,
}

pub struct PipeCell {
    kind: CellKind,
    pub coords: (usize, usize),
}

impl fmt::Debug for PipeCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Type={}, Coords=({},{})",
            self.to_char(),
            self.coords.0,
            self.coords.1
        )
    }
}

impl PipeCell {
    pub fn new(kind_char: char, coords: (usize, usize)) -> Self {
        PipeCell {
            kind: match kind_char {
                '|' => CellKind::Vertical,
                '-' => CellKind::Horizontal,
                'L' => CellKind::NorthEast,
                'J' => CellKind::NorthWest,
                '7' => CellKind::SouthWest,
                'F' => CellKind::SouthEast,
                '.' => CellKind::Ground,
                'S' => CellKind::Animal,
                _ => panic!("Valid cell character: {}", kind_char),
            },
            coords,
        }
    }

    pub fn can_connect(&self, coords: (usize, usize)) -> bool {
        let (row, col) = coords;

        if coords == self.coords {
            return false;
        }

        let (curr_row, curr_col) = self.coords;

        match self.kind {
            CellKind::Vertical => col == curr_col && (row as i32 - curr_row as i32).abs() <= 1,
            CellKind::Horizontal => row == curr_row && (col as i32 - curr_col as i32).abs() <= 1,
            CellKind::NorthEast => {
                (curr_row > 0 && row == curr_row - 1 && col == curr_col)
                    || (row == curr_row && col == curr_col + 1)
            }
            CellKind::NorthWest => {
                (curr_row > 0 && row == curr_row - 1 && col == curr_col)
                    || (curr_col > 0 && row == curr_row && col == curr_col - 1)
            }
            CellKind::SouthWest => {
                (row == curr_row + 1 && col == curr_col)
                    || (curr_col > 0 && row == curr_row && col == curr_col - 1)
            }
            CellKind::SouthEast => {
                (row == curr_row + 1 && col == curr_col) || (row == curr_row && col == curr_col + 1)
            }
            CellKind::Ground => false,
            CellKind::Animal => true,
        }
    }

    pub fn to_expanded(&self) -> Vec<Vec<ExpandedCell>> {
        match self.kind {
            CellKind::Vertical => vec![
                vec![
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 0,
                            c: self.coords.1,
                            sub_c: 0,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Pipe,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 0,
                            c: self.coords.1,
                            sub_c: 1,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 0,
                            c: self.coords.1,
                            sub_c: 2,
                        },
                    },
                ],
                vec![
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 1,
                            c: self.coords.1,
                            sub_c: 0,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Pipe,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 1,
                            c: self.coords.1,
                            sub_c: 1,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 1,
                            c: self.coords.1,
                            sub_c: 2,
                        },
                    },
                ],
                vec![
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 2,
                            c: self.coords.1,
                            sub_c: 0,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Pipe,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 2,
                            c: self.coords.1,
                            sub_c: 1,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 2,
                            c: self.coords.1,
                            sub_c: 2,
                        },
                    },
                ],
            ],
            CellKind::Horizontal => vec![
                vec![
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 0,
                            c: self.coords.1,
                            sub_c: 0,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 0,
                            c: self.coords.1,
                            sub_c: 1,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 0,
                            c: self.coords.1,
                            sub_c: 2,
                        },
                    },
                ],
                vec![
                    ExpandedCell {
                        kind: ExpandedCellKind::Pipe,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 1,
                            c: self.coords.1,
                            sub_c: 0,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Pipe,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 1,
                            c: self.coords.1,
                            sub_c: 1,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Pipe,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 1,
                            c: self.coords.1,
                            sub_c: 2,
                        },
                    },
                ],
                vec![
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 2,
                            c: self.coords.1,
                            sub_c: 0,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 2,
                            c: self.coords.1,
                            sub_c: 1,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 2,
                            c: self.coords.1,
                            sub_c: 2,
                        },
                    },
                ],
            ],
            CellKind::NorthEast => vec![
                vec![
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 0,
                            c: self.coords.1,
                            sub_c: 0,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Pipe,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 0,
                            c: self.coords.1,
                            sub_c: 1,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 0,
                            c: self.coords.1,
                            sub_c: 2,
                        },
                    },
                ],
                vec![
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 1,
                            c: self.coords.1,
                            sub_c: 0,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Pipe,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 1,
                            c: self.coords.1,
                            sub_c: 1,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Pipe,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 1,
                            c: self.coords.1,
                            sub_c: 2,
                        },
                    },
                ],
                vec![
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 2,
                            c: self.coords.1,
                            sub_c: 0,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 2,
                            c: self.coords.1,
                            sub_c: 1,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 2,
                            c: self.coords.1,
                            sub_c: 2,
                        },
                    },
                ],
            ],
            CellKind::NorthWest => vec![
                vec![
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 0,
                            c: self.coords.1,
                            sub_c: 0,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Pipe,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 0,
                            c: self.coords.1,
                            sub_c: 1,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 0,
                            c: self.coords.1,
                            sub_c: 2,
                        },
                    },
                ],
                vec![
                    ExpandedCell {
                        kind: ExpandedCellKind::Pipe,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 1,
                            c: self.coords.1,
                            sub_c: 0,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Pipe,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 1,
                            c: self.coords.1,
                            sub_c: 1,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 1,
                            c: self.coords.1,
                            sub_c: 2,
                        },
                    },
                ],
                vec![
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 2,
                            c: self.coords.1,
                            sub_c: 0,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 2,
                            c: self.coords.1,
                            sub_c: 1,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 2,
                            c: self.coords.1,
                            sub_c: 2,
                        },
                    },
                ],
            ],
            CellKind::SouthWest => vec![
                vec![
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 0,
                            c: self.coords.1,
                            sub_c: 0,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 0,
                            c: self.coords.1,
                            sub_c: 1,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 0,
                            c: self.coords.1,
                            sub_c: 2,
                        },
                    },
                ],
                vec![
                    ExpandedCell {
                        kind: ExpandedCellKind::Pipe,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 1,
                            c: self.coords.1,
                            sub_c: 0,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Pipe,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 1,
                            c: self.coords.1,
                            sub_c: 1,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 1,
                            c: self.coords.1,
                            sub_c: 2,
                        },
                    },
                ],
                vec![
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 2,
                            c: self.coords.1,
                            sub_c: 0,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Pipe,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 2,
                            c: self.coords.1,
                            sub_c: 1,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 2,
                            c: self.coords.1,
                            sub_c: 2,
                        },
                    },
                ],
            ],
            CellKind::SouthEast => vec![
                vec![
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 0,
                            c: self.coords.1,
                            sub_c: 0,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 0,
                            c: self.coords.1,
                            sub_c: 1,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 0,
                            c: self.coords.1,
                            sub_c: 2,
                        },
                    },
                ],
                vec![
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 1,
                            c: self.coords.1,
                            sub_c: 0,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Pipe,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 1,
                            c: self.coords.1,
                            sub_c: 1,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Pipe,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 1,
                            c: self.coords.1,
                            sub_c: 2,
                        },
                    },
                ],
                vec![
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 2,
                            c: self.coords.1,
                            sub_c: 0,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Pipe,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 2,
                            c: self.coords.1,
                            sub_c: 1,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 2,
                            c: self.coords.1,
                            sub_c: 2,
                        },
                    },
                ],
            ],
            CellKind::Ground => vec![
                vec![
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 0,
                            c: self.coords.1,
                            sub_c: 0,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 0,
                            c: self.coords.1,
                            sub_c: 1,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 0,
                            c: self.coords.1,
                            sub_c: 2,
                        },
                    },
                ],
                vec![
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 1,
                            c: self.coords.1,
                            sub_c: 0,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 1,
                            c: self.coords.1,
                            sub_c: 1,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 1,
                            c: self.coords.1,
                            sub_c: 2,
                        },
                    },
                ],
                vec![
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 2,
                            c: self.coords.1,
                            sub_c: 0,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 2,
                            c: self.coords.1,
                            sub_c: 1,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 2,
                            c: self.coords.1,
                            sub_c: 2,
                        },
                    },
                ],
            ],
            CellKind::Animal => vec![
                vec![
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 0,
                            c: self.coords.1,
                            sub_c: 0,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Pipe,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 0,
                            c: self.coords.1,
                            sub_c: 1,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 0,
                            c: self.coords.1,
                            sub_c: 2,
                        },
                    },
                ],
                vec![
                    ExpandedCell {
                        kind: ExpandedCellKind::Pipe,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 1,
                            c: self.coords.1,
                            sub_c: 0,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Pipe,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 1,
                            c: self.coords.1,
                            sub_c: 1,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Pipe,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 1,
                            c: self.coords.1,
                            sub_c: 2,
                        },
                    },
                ],
                vec![
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 2,
                            c: self.coords.1,
                            sub_c: 0,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Pipe,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 2,
                            c: self.coords.1,
                            sub_c: 1,
                        },
                    },
                    ExpandedCell {
                        kind: ExpandedCellKind::Ground,
                        coords: ExpandedCoord {
                            r: self.coords.0,
                            sub_r: 2,
                            c: self.coords.1,
                            sub_c: 2,
                        },
                    },
                ],
            ],
        }
    }

    pub fn to_char(&self) -> char {
        match self.kind {
            CellKind::Vertical => '|',
            CellKind::Horizontal => '-',
            CellKind::NorthEast => 'L',
            CellKind::NorthWest => 'J',
            CellKind::SouthWest => '7',
            CellKind::SouthEast => 'F',
            CellKind::Ground => '.',
            CellKind::Animal => 'S',
        }
    }
}

pub struct PipeGraph {
    cells: Vec<Vec<PipeCell>>,
}

impl PipeGraph {
    pub fn new(cells: Vec<Vec<PipeCell>>) -> Self {
        PipeGraph { cells }
    }

    // Find the first cell of a given kind
    pub fn find_cell(&self, kind: CellKind) -> &PipeCell {
        self.cells
            .iter()
            .flatten()
            .find(|cell| cell.kind == kind)
            .unwrap()
    }

    pub fn search_bfs(&self, coords: (usize, usize)) -> HashMap<(usize, usize), u32> {
        self.bfs(coords, |&&new_coords, &&old_coords| {
            let cell = self.get_cell(old_coords.0, old_coords.1);
            let new_cell = self.get_cell(new_coords.0, new_coords.1);

            cell.can_connect(new_coords) && new_cell.can_connect((old_coords.0, old_coords.1))
        })
    }

    pub fn print_matrix(&self, coords: &HashSet<(usize, usize)>) {
        let mut matrix = vec![vec!['.'; self.cells[0].len()]; self.cells.len()];

        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if coords.contains(&(i, j)) {
                    matrix[i][j] = 'X';
                }
            }
        }

        let to_print: Vec<String> = matrix
            .iter()
            .map(|r| r.iter().fold(String::new(), |acc, &c| format!("{acc}{c}")))
            .collect();

        filewriter::write_file("./day10/resources/output.txt", to_print);
    }

    pub fn to_expanded(&self) -> ExpandedPipeGraph {
        let mut expanded_cells: Vec<Vec<ExpandedCell>> = vec![
            vec![
                ExpandedCell {
                    kind: ExpandedCellKind::Ground,
                    coords: ExpandedCoord {
                        r: 0,
                        sub_r: 0,
                        c: 0,
                        sub_c: 0
                    }
                };
                self.cells[0].len() * 3
            ];
            self.cells.len() * 3
        ];

        for i in 0..self.cells.len() {
            for j in 0..self.cells[0].len() {
                let cells = self.cells[i][j].to_expanded();

                for e_i in 0..cells.len() {
                    for e_j in 0..cells[e_i].len() {
                        let expanded_cell = &cells[e_i][e_j];
                        expanded_cells[expanded_cell.coords.r * 3 + expanded_cell.coords.sub_r]
                            [expanded_cell.coords.c * 3 + expanded_cell.coords.sub_c] =
                            expanded_cell.clone();
                    }
                }
            }
        }

        ExpandedPipeGraph::new(expanded_cells)
    }

    pub fn remove_non_path(&mut self, path_coords: &HashSet<(usize, usize)>) {
        // Turn all non-path cells into ground so we can make floor easier after transformation
        for i in 0..self.cells.len() {
            for j in 0..self.cells[0].len() {
                if !path_coords.contains(&(i, j)) {
                    self.cells[i][j] = PipeCell {
                        kind: CellKind::Ground,
                        coords: (i, j),
                    }
                }
            }
        }
    }

    fn bfs<F>(&self, coords: (usize, usize), neighbor_filter: F) -> HashMap<(usize, usize), u32>
    where
        F: Fn(&&(usize, usize), &&(usize, usize)) -> bool,
    {
        let mut mappings = HashMap::new();
        let mut visited: Vec<(usize, usize)> = Vec::new();
        let mut queue = VecDeque::new();

        queue.push_back((coords.0, coords.1, 0));

        while let Some((r, c, distance)) = queue.pop_front() {
            mappings.insert((r, c), distance);
            visited.push((r, c));

            self.get_neighbors((r, c))
                .iter()
                .filter(|new_coords| neighbor_filter(new_coords, &&(r, c)))
                .for_each(|&(next_row, next_col)| {
                    if !mappings.contains_key(&(next_row, next_col)) {
                        queue.push_back((next_row, next_col, distance + 1));
                    }
                });
        }

        mappings
    }

    fn get_cell(&self, r: usize, c: usize) -> &PipeCell {
        &self.cells[r][c]
    }

    // Get allowable neighbors for the given coordinates
    fn get_neighbors(&self, coords: (usize, usize)) -> Vec<(usize, usize)> {
        let (r, c) = coords;
        // let cell = self.get_cell(coords.0, coords.1);
        let mut neighbors = Vec::with_capacity(8);

        if r > 0 {
            // Top
            neighbors.push((r - 1, c));

            // if c > 0 {
            //     // Top left
            //     neighbors.push((r - 1, c - 1));
            // }

            // if c < self.cells[0].len() - 1 {
            //     // Top right
            //     neighbors.push((r - 1, c + 1));
            // }
        }

        if c > 0 {
            // Left
            neighbors.push((r, c - 1));
        }

        if r < self.cells.len() - 1 {
            // Bottom
            neighbors.push((r + 1, c));

            // if c < self.cells[0].len() - 1 {
            //     // Bottom right
            //     neighbors.push((r + 1, c + 1));
            // }

            // if c > 0 {
            //     // Bottom left
            //     neighbors.push((r + 1, c - 1));
            // }
        }

        if c < self.cells[0].len() - 1 {
            // Right
            neighbors.push((r, c + 1));
        }

        neighbors
    }
}

#[derive(Clone, PartialEq)]
enum ExpandedCellKind {
    Ground,
    Pipe,
}

#[derive(Clone)]
struct ExpandedCoord {
    r: usize,
    sub_r: usize,
    c: usize,
    sub_c: usize,
}

#[derive(Clone)]
pub struct ExpandedCell {
    kind: ExpandedCellKind,
    pub coords: ExpandedCoord,
}

impl ExpandedCell {
    pub fn new(kind_char: char, coords: (usize, usize, usize, usize)) -> Self {
        ExpandedCell {
            kind: match kind_char {
                '.' => ExpandedCellKind::Ground,
                '#' => ExpandedCellKind::Pipe,
                _ => panic!("Invalid expanded pipe cell character: {}", kind_char),
            },
            coords: ExpandedCoord {
                r: coords.0,
                sub_r: coords.1,
                c: coords.2,
                sub_c: coords.3,
            },
        }
    }

    pub fn to_char(&self) -> char {
        match self.kind {
            ExpandedCellKind::Ground => '.',
            ExpandedCellKind::Pipe => '#',
        }
    }
}

impl fmt::Debug for ExpandedCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char(),)
    }
}

pub struct ExpandedPipeGraph {
    cells: Vec<Vec<ExpandedCell>>,
}

impl ExpandedPipeGraph {
    pub fn new(cells: Vec<Vec<ExpandedCell>>) -> Self {
        ExpandedPipeGraph { cells }
    }

    pub(crate) fn bfs(&self, coords: (usize, usize)) -> HashMap<(usize, usize), u32> {
        let mut mappings = HashMap::new();
        let mut queue = VecDeque::new();

        queue.push_back((coords.0, coords.1, 0));

        while let Some((r, c, distance)) = queue.pop_front() {
            // Redundant keys might've been added to the queue from adjacent cells. Ignore them
            if mappings.contains_key(&(r, c)) {
                continue;
            }

            mappings.insert((r, c), distance);

            self.get_neighbors((r, c))
                .iter()
                .filter(
                    |new_coords| match self.get_cell(new_coords.0, new_coords.1).kind {
                        ExpandedCellKind::Pipe => false,
                        ExpandedCellKind::Ground => true,
                    },
                )
                .for_each(|&(next_row, next_col)| {
                    if !mappings.contains_key(&(next_row, next_col)) {
                        queue.push_back((next_row, next_col, distance + 1));
                    }
                });
        }

        mappings
    }

    fn get_cell(&self, r: usize, c: usize) -> &ExpandedCell {
        &self.cells[r][c]
    }

    // Get allowable neighbors for the given coordinates
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

impl fmt::Debug for ExpandedPipeGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut graph_str = String::new();

        for i in 0..self.cells.len() {
            for j in 0..self.cells[0].len() {
                graph_str.push(self.cells[i][j].to_char());
            }
            graph_str.push('\n');
        }

        write!(f, "{}", graph_str)
    }
}
