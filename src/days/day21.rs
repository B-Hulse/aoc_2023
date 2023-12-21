use std::collections::HashSet;

use graph::prelude::*;

const INPUT: &str = include_str!("input/day21.txt");

fn coords_to_index(x: usize, y: usize, w: usize) -> usize {
    y * w + x
}

fn get_graph(input: &str) -> (UndirectedCsrGraph<usize>, usize) {
    let grid = input
        .lines()
        .map(|line| line.chars().map(|c| (c != '#', c=='S')).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let grid_width = grid[0].len();
    let grid_height = grid.len();

    let mut edges = Vec::new();

    let mut starting_i = None;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let c = grid[y][x];

            if !c.0 {
                continue;
            }

            let c_i = coords_to_index(x, y, grid_width);
            if x < grid_width - 1 && grid[y][x + 1].0 {
                let d_i = coords_to_index(x + 1, y, grid_width);
                edges.push((c_i, d_i));
            }
            if y < grid_height - 1 && grid[y + 1][x].0 {
                let d_i = coords_to_index(x, y + 1, grid_width);
                edges.push((c_i, d_i));
            }

            if c.1 {
                starting_i = Some(c_i);
            }
        }
    }

    let Some(starting_i) = starting_i else {
        panic!("No starting point found");
    };

    (GraphBuilder::new().csr_layout(CsrLayout::Sorted).edges(edges).build(), starting_i)
}

pub fn part1() -> i64 {
    let (graph, starting_i) = get_graph(INPUT);

    let mut current_set = HashSet::from([starting_i]);

    for _ in 0..64 {
        let mut next_set = HashSet::new();

        for i in current_set {
            for j in graph.neighbors(i) {
                next_set.insert(*j);
            }
        }

        current_set = next_set;
    }

    current_set.len() as i64
}
pub fn part2() -> i64 {
    0
}
