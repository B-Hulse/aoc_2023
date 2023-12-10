use core::panic;
use std::collections::HashMap;

const INPUT: &str = include_str!("input/day10.txt");

struct Node {
    connections: (usize, usize)
}

struct NodeBuilder {
    bounds : (usize, usize)
}

impl NodeBuilder {
    fn new(grid_width: usize, grid_height: usize) -> NodeBuilder {
        NodeBuilder {
            bounds: (grid_width, grid_height)
        }
    }

    fn build(&self, connections: (usize, usize)) -> Node {
        Node {
            connections: connections
        }
    }

    fn coords_to_index(&self, x: usize, y: usize) -> usize {
        x * self.bounds.0 + y
    }

    fn index_to_coords(&self, index: usize) -> (usize, usize) {
        (index % self.bounds.0, index / self.bounds.0)
    }
}

struct Traversal {
    current: usize,
    last: usize,
}

fn get_grid(s: &str) -> (HashMap<usize, Node>, usize) {
    let grid_height = s.lines().count();
    let grid_width = s.lines().next().unwrap().chars().count();

    let node_builder = NodeBuilder::new(grid_width, grid_height);
    let mut grid = HashMap::new();

    let mut start = None;

    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                let connections = match c {
                    '|' => {
                        if y == 0 || y == grid_height - 1 {
                            continue;
                        }

                        (
                            node_builder.coords_to_index(x, y - 1),
                            node_builder.coords_to_index(x, y + 1)
                        )
                    },
                    '-' => {
                        if x == 0 || x == grid_width - 1 {
                            continue;
                        }

                        (
                            node_builder.coords_to_index(x - 1, y),
                            node_builder.coords_to_index(x + 1, y)
                        )
                    },
                    'L' => {
                        if y == 0 || x == grid_width - 1 {
                            continue;
                        }

                        (
                            node_builder.coords_to_index(x, y - 1),
                            node_builder.coords_to_index(x + 1, y)
                        )
                    },
                    '7' => {
                        if x == 0 || y == grid_height - 1 {
                            continue;
                        }

                        (
                            node_builder.coords_to_index(x, y + 1),
                            node_builder.coords_to_index(x - 1, y)
                        )
                    },
                    'J' => {
                        if x == 0 || y == 0 {
                            continue;
                        }

                        (
                            node_builder.coords_to_index(x, y - 1),
                            node_builder.coords_to_index(x - 1, y)
                        )
                    },
                    'F' => {
                        if x == grid_width - 1 || y == grid_height - 1 {
                            continue;
                        }

                        (
                            node_builder.coords_to_index(x, y + 1),
                            node_builder.coords_to_index(x + 1, y)
                        )
                    },
                    'S' => {
                        start = Some(node_builder.coords_to_index(x, y));
                        continue;
                    }
                    _ => panic!("Invalid char")
                };

                grid.insert(node_builder.coords_to_index(x, y), node_builder.build(connections));
            }
        }
    }

    if start.is_none() {
        panic!("No start found");
    }

    let start = start.unwrap();

    let matches : Vec<_> = grid.iter()
                            .filter(|(_, node)| 
                                {
                                    node.connections.0 == start || 
                                    node.connections.1 == start
                                })
                            .collect();
    
    if matches.len() != 2 {
        panic!("Invalid number of matches");
    }

    grid.insert(start, node_builder.build((matches[0].0.clone(), matches[1].0.clone())));

    (grid, start)
}

fn get_traversal(grid: HashMap<usize, Node>, start: usize) -> (Vec<usize>, i64) {
    let mut traversal_1 = Traversal {
        current: grid[&start].connections.0,
        last: start
    };
    let mut visited_1 = vec![traversal_1.last, traversal_1.current];

    let mut traversal_2 = Traversal {
        current: grid[&start].connections.1,
        last: start
    };
    let mut visited_2 = vec![traversal_2.last, traversal_2.current];

    let mut distance = 1;

    loop {
        if traversal_1.current == traversal_2.current {
            visited_2.pop();
            break;
        }
        else if traversal_1.current == traversal_2.last && traversal_2.current == traversal_1.last  {
            visited_1.pop();
            visited_2.pop();
            break;
        }

        let temp_traversal_1_last = traversal_1.last;
        let traversal_1_node = &grid[&traversal_1.current];
        traversal_1.last = traversal_1.current;
        if temp_traversal_1_last != traversal_1_node.connections.0 {
            traversal_1.current = traversal_1_node.connections.0;
        }
        else {
            traversal_1.current = traversal_1_node.connections.1;
        }
        visited_1.push(traversal_1.current);

        let temp_traversal_2_last = traversal_2.last;
        let traversal_2_node = &grid[&traversal_2.current];
        traversal_2.last = traversal_2.current;
        if temp_traversal_2_last != traversal_2_node.connections.0 {
            traversal_2.current = traversal_2_node.connections.0;
        }
        else {
            traversal_2.current = traversal_2_node.connections.1;
        }
        visited_2.push(traversal_2.current);

        distance += 1;
    }

    visited_2.reverse();
    visited_1.append(&mut visited_2);
    (visited_1, distance)
}

pub fn part1() -> i64 {
    let (grid, start) = get_grid(INPUT);

    get_traversal(grid, start).1
}

pub fn part2() -> i64 {
    let (grid, start) = get_grid(INPUT);

    let (path, distance) = get_traversal(grid, start);

    let grid_height = INPUT.lines().count();
    let grid_width = INPUT.lines().next().unwrap().chars().count();

    let node_builder = NodeBuilder::new(grid_width, grid_height);

    let path : Vec<_> = path.iter().map(|i| node_builder.index_to_coords(*i)).collect();

    let mut left_lace = 0;
    let mut right_lace = 0;
    for i in 0..(path.len()-1) {
        let (x1, y1) = path[i];
        let (x2, y2) = path[i + 1];

        left_lace += x1 * y2;
        right_lace += x2 * y1;
    }

    let area = (left_lace as i64 - right_lace as i64).abs() / 2;

    area - distance + 1
}