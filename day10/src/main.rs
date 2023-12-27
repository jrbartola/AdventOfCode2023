mod pipe;

use crate::pipe::{CellKind, PipeCell, PipeGraph};
use common::filereader;
use std::collections::HashSet;

fn build_graph(lines: &Vec<String>) -> PipeGraph {
    let mut pipe_cells: Vec<_> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        pipe_cells.push(Vec::new());
        for (j, char) in line.chars().enumerate() {
            let pipe_cell = PipeCell::new(char, (i, j));
            pipe_cells[i].push(pipe_cell);
        }
    }

    PipeGraph::new(pipe_cells)
}

fn solve(lines: Vec<String>) -> u32 {
    let graph = build_graph(&lines);

    let animal_cell = graph.find_cell(CellKind::Animal);

    *graph
        .search_bfs((animal_cell.coords.0, animal_cell.coords.1))
        .values()
        .max()
        .unwrap()
}

fn solve2(lines: Vec<String>) -> u32 {
    let mut graph = build_graph(&lines);
    let animal_cell = graph.find_cell(CellKind::Animal);

    let path_coords: HashSet<(usize, usize)> = graph
        .search_bfs((animal_cell.coords.0, animal_cell.coords.1))
        .keys()
        .map(|x| *x)
        .collect();

    graph.remove_non_path(&path_coords);

    let expanded_graph = graph.to_expanded();

    println!("{:?}", expanded_graph);

    let outside_coords_expanded: Vec<(usize, usize)> =
        expanded_graph.bfs((0, 0)).keys().copied().collect();

    let outside_coords: Vec<_> = outside_coords_expanded
        .iter()
        .filter_map(|&(r, c)| {
            if r % 3 == 1 && c % 3 == 1 {
                Some((r / 3, c / 3))
            } else {
                None
            }
        })
        .collect();

    (lines.len() * lines[0].len()) as u32 - (path_coords.len() + outside_coords.len()) as u32
}

fn main() {
    match filereader::read_file("./day10/resources/input.txt") {
        Ok(lines) => {
            let result = solve2(lines);
            println!("{:?}", result);
        }
        Err(e) => panic!("{}", e),
    }
}
