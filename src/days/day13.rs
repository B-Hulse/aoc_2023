const INPUT: &str = include_str!("input/day13.txt");

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

fn find_vertical_mirror(input: &Vec<Vec<char>>) -> Option<i64> {
    // Switch the rows and the columns in the input
    let transposed_input = transpose(input.clone());

    find_horizontal_mirror(&transposed_input)
}

fn find_vertical_mirror_one_off(input: &Vec<Vec<char>>) -> Option<i64> {
    // Switch the rows and the columns in the input
    let transposed_input = transpose(input.clone());

    find_horizontal_mirror_one_off(&transposed_input)
}

fn find_horizontal_mirror(input: &Vec<Vec<char>>) -> Option<i64> {
    
    for i in 1..input.len() {
        let mut up_rows = (&input[0..i]).to_vec();
        up_rows.reverse();
        let down_rows = (&input[i..input.len()]).to_vec();

        let min_row_i = up_rows.len().min(down_rows.len());

        let mut mirrored = true;

        for i in 0..min_row_i {
            if up_rows[i] != down_rows[i] {
                mirrored = false;
                break;
            }
        }
        if mirrored {
            return Some(i as i64);
        }
    }

    None
}

fn find_horizontal_mirror_one_off(input: &Vec<Vec<char>>) -> Option<i64> {
    for i in 1..input.len() {
        let mut up_rows = (&input[0..i]).to_vec();
        up_rows.reverse();
        let down_rows = (&input[i..input.len()]).to_vec();

        let min_row_i = up_rows.len().min(down_rows.len());

        let mut flaws = 0;

        for i in 0..min_row_i {
            if up_rows[i] != down_rows[i] {
                for j in 0..up_rows[i].len() {
                    let up_rows_j = up_rows[i][j];
                    let down_rows_j = down_rows[i][j];

                    if up_rows_j != down_rows_j {
                        flaws+=1;
                    }
                }

                if flaws > 1 {
                    break;
                }
            }
        }

        if flaws == 1 {
            return Some(i as i64);
        }
    }

    None
}

pub fn part1() -> i64 {
    let patterns: Vec<_> = INPUT.split("\r\n\r\n").collect();

    let patterns: Vec<_> = patterns
        .iter()
        .map(|p| {
            p.lines()
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect();

    let vertical_mirror_sums: i64 = patterns
        .iter()
        .filter_map(|p| find_vertical_mirror(p))
        .sum();

    let horizontal_mirror_sums: i64 = patterns
        .iter()
        .filter_map(|p| find_horizontal_mirror(p))
        .sum();
    
    vertical_mirror_sums + (horizontal_mirror_sums * 100) 
}

pub fn part2() -> i64 {
    let patterns: Vec<_> = INPUT.split("\r\n\r\n").collect();

    let patterns: Vec<_> = patterns
        .iter()
        .map(|p| {
            p.lines()
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect();

    let vertical_mirror_sums: i64 = patterns
        .iter()
        .filter_map(|p| find_vertical_mirror_one_off(p))
        .sum();

    let horizontal_mirror_sums: i64 = patterns
        .iter()
        .filter_map(|p| find_horizontal_mirror_one_off(p))
        .sum();
    
    vertical_mirror_sums + (horizontal_mirror_sums * 100) 
}
