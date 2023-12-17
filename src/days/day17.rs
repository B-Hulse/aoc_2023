use pathfinding::prelude::dijkstra;

const INPUT: &str = include_str!("input/day17.txt");

fn get_grid(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .filter_map(|c| c.to_string().parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn move_pos_in_direction(
    pos: (usize, usize),
    dir: Direction,
    grid: &Vec<Vec<usize>>
) -> Option<(usize, usize)> {
    if pos.0 == 0 && dir == Direction::Left
        || pos.1 == 0 && dir == Direction::Up
        || pos.0 == grid[0].len() - 1 && dir == Direction::Right
        || pos.1 == grid.len() - 1 && dir == Direction::Down
    {
        None
    } else {
        match dir {
            Direction::Up => Some((pos.0, pos.1 - 1)),
            Direction::Right => Some((pos.0 + 1, pos.1)),
            Direction::Left => Some((pos.0 - 1, pos.1)),
            Direction::Down => Some((pos.0, pos.1 + 1)),
            Direction::None => None
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Direction {
    None,
    Up,
    Right,
    Left,
    Down,
}

impl Direction {
    fn get_rotated(&self) -> Vec<Direction> {
        match self {
            Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
            Direction::Right | Direction::Left => vec![Direction::Up, Direction::Down],
            Direction::None => vec![Direction::Left, Direction::Right, Direction::Up, Direction::Down]
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct CruciblePos {
    pos: (usize, usize),
    dir: Direction,
    straight: usize
}

impl CruciblePos {
    fn successors(&self, grid:&Vec<Vec<usize>>, ultra: bool) -> Vec<(CruciblePos, usize)> {
        let mut dirs = Vec::new();

        if !ultra && self.dir != Direction::None {
            if self.straight < 3 {
                dirs.push(self.dir);
            }
    
            for d in self.dir.get_rotated() {
                dirs.push(d);
            }
        }
        else if self.dir != Direction::None {
            if self.straight < 10 {
                dirs.push(self.dir);
            }
    
            if self.straight >= 4 {
                for d in self.dir.get_rotated() {
                    dirs.push(d);
                }
            }
        }
        else {
            dirs = vec![Direction::Up, Direction::Right, Direction::Down, Direction:: Left];
        }

        let mut ret = Vec::new();

        for dir in dirs {
            if let Some(new_pos) = move_pos_in_direction(self.pos, dir, grid) {
                let new_node = CruciblePos{
                    pos: new_pos,
                    dir,
                    straight: if dir == self.dir {self.straight + 1} else {1} 
                };

                ret.push((new_node, grid[new_pos.1][new_pos.0]));
            }
        }

        ret
    }
}

pub fn part1() -> i64 {
    let grid = get_grid(INPUT);

    let start = CruciblePos {
        pos: (0,0),
        dir: Direction::None,
        straight: 0
    };

    let result = dijkstra(&start, |p| p.successors(&grid, false), |p| p.pos == (grid.len()-1, grid[0].len()-1));

    result.unwrap().1 as i64
}

pub fn part2() -> i64 {
    let grid = get_grid(INPUT);
    
    let start = CruciblePos {
        pos: (0,0),
        dir: Direction::None,
        straight: 0
    };

    let result = dijkstra(&start, |p| p.successors(&grid, true), |p| p.pos == (grid.len()-1, grid[0].len()-1) && p.straight >= 4);

    result.unwrap().1 as i64
}
