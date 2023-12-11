const INPUT: &str = include_str!("input/day11.txt");

fn get_grid(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|l| l.chars().collect()).collect()
}

fn get_rows_to_expand(grid: &mut Vec<Vec<char>>) -> Vec<usize> {
    let mut rows_to_expand = Vec::new();
    for row_i in 0..grid.len() {
        let row = grid.get(row_i).unwrap();

        if row.iter().all(|&c| c == '.') {
            rows_to_expand.push(row_i);
        }
    }

    rows_to_expand
}

fn get_cols_to_expand(grid: &mut Vec<Vec<char>>) -> Vec<usize> {
    let mut cols_to_expand = Vec::new();
    for col_i in 0..grid.get(0).unwrap().len() {
        let mut column_empty = true;

        for row_i in 0..grid.len() {
            let c = grid.get(row_i).unwrap().get(col_i).unwrap();

            if *c != '.' {
                column_empty = false;
            }
        }

        if column_empty {
            cols_to_expand.push(col_i);
        }
    }

    cols_to_expand
}

fn index_to_coords(i: usize, width: usize) -> (usize, usize) {
    (i % width, i / width)
}

fn num_is_between(a: usize, b: usize, c: usize) -> bool {
    let first = if a < b {a} else {b};
    let second = if a < b {b} else {a};
    
    first < c && c < second
}

pub fn part1() -> i64 {
    let mut grid = get_grid(INPUT);

    let cols_to_expand = get_cols_to_expand(&mut grid);
    let rows_to_expand = get_rows_to_expand(&mut grid);

    let star_locations: Vec<_> = grid.iter()
                                    .flatten()
                                    .enumerate()
                                    .filter_map(|(i, &c)| if c == '#' {Some(i)} else {None})
                                    .collect();

    let mut sum = 0;
    for i in 0..star_locations.len() {
        for j in 0..star_locations.len() {
            if i > j {
                let (x1, y1) = index_to_coords(star_locations[i], grid.get(0).unwrap().len());
                let (x2, y2) = index_to_coords(star_locations[j], grid.get(0).unwrap().len());

                let dx = (x2 as i64 - x1 as i64).abs();
                let dy = (y2 as i64 - y1 as i64).abs();

                let dy = dy + rows_to_expand.iter().filter(|&row_i| num_is_between(y1, y2, *row_i)).count() as i64;
                let dx = dx + cols_to_expand.iter().filter(|&col_i| num_is_between(x1, x2, *col_i)).count() as i64;

                sum += dx + dy;
            }
        }
    }

    sum
}

pub fn part2() -> i64 {
    let mut grid = get_grid(INPUT);

    let cols_to_expand = get_cols_to_expand(&mut grid);
    let rows_to_expand = get_rows_to_expand(&mut grid);

    let star_locations: Vec<_> = grid.iter()
                                    .flatten()
                                    .enumerate()
                                    .filter_map(|(i, &c)| if c == '#' {Some(i)} else {None})
                                    .collect();

    let mut sum = 0;
    for i in 0..star_locations.len() {
        for j in 0..star_locations.len() {
            if i > j {
                let (x1, y1) = index_to_coords(star_locations[i], grid.get(0).unwrap().len());
                let (x2, y2) = index_to_coords(star_locations[j], grid.get(0).unwrap().len());

                let dx = (x2 as i64 - x1 as i64).abs();
                let dy = (y2 as i64 - y1 as i64).abs();

                let dy = dy + (rows_to_expand.iter().filter(|&row_i| num_is_between(y1, y2, *row_i)).count() * 999_999) as i64;
                let dx = dx + (cols_to_expand.iter().filter(|&col_i| num_is_between(x1, x2, *col_i)).count() * 999_999) as i64;

                sum += dx + dy;
            }
        }
    }

    sum
}