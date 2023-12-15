use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &str = include_str!("input/day14.txt");

fn rotate_vec(vec: &mut Vec<Vec<char>>) {
    let mut reversed = vec.clone();
    reversed.iter_mut().for_each(|row| row.reverse());

    let transposed = transpose(reversed);

    *vec = transposed
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn roll_upwards(grid: &mut Vec<Vec<char>>) {
    grid.iter_mut().for_each(|col| {
        let mut hash_groups_shifted = Vec::new();
        let col_str = col.iter().collect::<String>();

        for hash_group in col_str.split('#') {
            let round_count = hash_group.chars().filter(|c| *c == 'O').count();

            hash_groups_shifted.push(hash_group.replace('O', ".").replacen('.', "O", round_count));
        }

        *col = hash_groups_shifted.join("#").chars().collect_vec();
    });
}

fn cycle_rotate_and_roll(grid: &mut Vec<Vec<char>>) {
    roll_upwards(grid);
    rotate_vec(grid);
    roll_upwards(grid);
    rotate_vec(grid);
    roll_upwards(grid);
    rotate_vec(grid);
    roll_upwards(grid);
    rotate_vec(grid);
}

fn get_grid_from_str(grid_str: &str) -> Vec<Vec<char>> {
    let lines: Vec<Vec<_>> = grid_str.lines().map(|l| l.chars().collect()).collect();
    let grid_h = grid_str.lines().count();
    let grid_w = lines.get(0).unwrap().len();

    let mut grid = vec![vec!['.'; grid_h]; grid_w];

    for x_i in 0..grid_w {
        for y_i in 0..grid_h {
            grid[x_i][y_i] = lines[y_i][x_i];
        }
    }

    return grid;
}

fn calculate_load_on_grid(grid: &Vec<Vec<char>>) -> i64 {
    grid.iter()
        .map(|col| {
            let h = col.len();
            col.iter()
                .enumerate()
                .filter_map(|(i, c)| if *c == 'O' {Some((h - i) as i64)} else {None}).sum::<i64>()
        })
        .sum()
}

pub fn part1() -> i64 {
    let mut grid = get_grid_from_str(INPUT);

    roll_upwards(&mut grid);
    
    calculate_load_on_grid(&grid)
}

pub fn part2() -> i64 {    
    let mut grid = get_grid_from_str(INPUT);
    
    let mut cache: HashMap<Vec<Vec<char>>, usize> = HashMap::new();

    let mut cycle_offset = None;

    for i in 0..1_000_000_000 {
        if let Some(cached_value) = cache.get(&grid) {
            let cycle_len = i - cached_value;

            cycle_offset = Some((1_000_000_000 - cached_value) % cycle_len);
            break;
        }

        cache.insert(grid.clone(), i);

        cycle_rotate_and_roll(&mut grid);
    }

    for _ in 0..cycle_offset.unwrap() {
        cycle_rotate_and_roll(&mut grid);
    }
    
    calculate_load_on_grid(&grid)
}
