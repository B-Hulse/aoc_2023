const INPUT : &str = include_str!("input/day09.txt");

fn recursive_find_next_num(sequence: &Vec<i64>) -> i64 {
    if sequence.iter().all(|n| *n == 0) {
        return 0;
    }

    let derivatives : Vec<_> = sequence.windows(2).map(|n| n[1] - n[0]).collect();

    let next_derivative = recursive_find_next_num(&derivatives);

    sequence.last().unwrap() + next_derivative
}

fn recursive_find_prev_num(sequence: &Vec<i64>) -> i64 {
    if sequence.iter().all(|n| *n == 0) {
        return 0;
    }

    let derivatives : Vec<_> = sequence.windows(2).map(|n| n[1] - n[0]).collect();

    let next_derivative = recursive_find_prev_num(&derivatives);

    sequence.first().unwrap() - next_derivative
}

pub fn part1() -> i64 {
    let sequences: Vec<Vec<_>> = INPUT.lines()
                                .map(|l| {
                                    l.split_whitespace()
                                     .map(|n| n.parse::<i64>().unwrap())
                                     .collect()
                                })
                                .collect();

    let next_values : Vec<_> = sequences.iter()
                                    .map(|s| recursive_find_next_num(s))
                                    .collect();

    next_values.iter().sum()
}

pub fn part2() -> i64 {
    let sequences: Vec<Vec<_>> = INPUT.lines()
                                .map(|l| {
                                    l.split_whitespace()
                                     .map(|n| n.parse::<i64>().unwrap())
                                     .collect()
                                })
                                .collect();

    let prev_values : Vec<_> = sequences.iter()
                                    .map(|s| recursive_find_prev_num(s))
                                    .collect();

    prev_values.iter().sum()
}