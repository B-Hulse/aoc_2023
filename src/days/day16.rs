use std::collections::HashSet;

const INPUT : &str = include_str!("input/day16.txt");

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn get_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Beam {
    position: (i64, i64),
    direction: Direction,
}

fn make_move(grid: &Vec<Vec<char>>, beam: &Beam) -> Vec<Beam> {
    let mut new_pos = beam.position;

    match beam.direction {
        Direction::Up => {new_pos.1-=1;},
        Direction::Down => {new_pos.1+=1;},
        Direction::Left => {new_pos.0-=1;},
        Direction::Right => {new_pos.0+=1;},
    }

    if new_pos.0 >= 0 && 
    new_pos.0 < grid[0].len() as i64 && 
    new_pos.1 >= 0 && 
    new_pos.1 < grid.len() as i64 {
        match grid[new_pos.1 as usize][new_pos.0 as usize] {
            '|' => {
                if beam.direction == Direction::Right || beam.direction == Direction::Left {
                    vec![ Beam {position:new_pos, direction: Direction::Up}, Beam{position:new_pos, direction:Direction::Down}]
                }
                else {
                    vec![ Beam {position:new_pos, direction: beam.direction}]
                }

            },
            '-' => {
                if beam.direction == Direction::Up || beam.direction == Direction::Down {
                    vec![ Beam {position:new_pos, direction: Direction::Right}, Beam{position:new_pos, direction:Direction::Left}]
                }
                else {
                    vec![ Beam {position:new_pos, direction: beam.direction}]
                }
            },
            '/' => {
                match beam.direction {
                    Direction::Up => vec![Beam{position:new_pos, direction:Direction::Right}],
                    Direction::Down => vec![Beam{position:new_pos, direction:Direction::Left}],
                    Direction::Left => vec![Beam{position:new_pos, direction:Direction::Down}],
                    Direction::Right => vec![Beam{position:new_pos, direction:Direction::Up}],
                }
            },
            '\\' => {
                match beam.direction {
                    Direction::Up => vec![Beam{position:new_pos, direction:Direction::Left}],
                    Direction::Down => vec![Beam{position:new_pos, direction:Direction::Right}],
                    Direction::Left => vec![Beam{position:new_pos, direction:Direction::Up}],
                    Direction::Right => vec![Beam{position:new_pos, direction:Direction::Down}],
                }
            },
            '.' => vec![Beam {position:new_pos, direction: beam.direction}],
            _ => panic!("Grid Invalid")
        }
    }
    else {
        vec![]
    }
}

fn get_energized_for_beam(grid: &Vec<Vec<char>>, beam: &Beam) -> i64 {
    let mut energized = HashSet::new();
    let mut seen_beams = HashSet::new();

    let mut beams = vec![*beam];

    energized.insert(beam.position);
    seen_beams.insert(*beam);

    while !beams.is_empty() {
        if let Some(beam) = beams.pop() {
            make_move(&grid, &beam).into_iter().for_each(|b| {
                if !seen_beams.contains(&b) {
                    energized.insert(b.position);
                    seen_beams.insert(b);
                    beams.push(b);
                }
            });
        }
    }

    energized.len() as i64
}

pub fn part1() -> i64 {
    let grid = get_grid(INPUT);
    
    get_energized_for_beam(&grid, &Beam { position:(0,0), direction: Direction::Right})
}

pub fn part2() -> i64 {
    let grid = get_grid(INPUT);
    let mut start_beams = Vec::new();

    // Top & Bottom
    for i in 0..grid.get(0).unwrap().len() {
        start_beams.push(Beam {position:(i as i64, 0), direction: Direction::Down});
        start_beams.push(Beam {position:(i as i64, grid.len() as i64), direction: Direction::Up});
    }
    // Left & Right
    for i in 0..grid.len() {
        start_beams.push(Beam {position:(0, i as i64), direction: Direction::Right});
        start_beams.push(Beam {position:(grid.get(0).unwrap().len() as i64,i as i64), direction: Direction::Left});
    }

    start_beams.iter().map(|b| get_energized_for_beam(&grid, b)).max().unwrap_or(0)
}