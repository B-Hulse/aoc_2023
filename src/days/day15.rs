const INPUT: &str = include_str!("input/day15.txt");

fn char_to_ascii(c: char) -> i64 {
    c as i64
}

fn add_to_hash(current_hash: i64, new_val: char) -> i64 {
    let new_val = char_to_ascii(new_val);
    let new_hash = (current_hash + new_val) * 17;
    new_hash % 256
}

fn hash_from_string(input: &str) -> i64 {
    input.chars().fold(0, |acc, c| add_to_hash(acc, c))
}

pub fn part1() -> i64 {
    let input_no_whitespace: String = INPUT.chars().filter(|c| !c.is_whitespace()).collect();

    let sum: i64 = input_no_whitespace
        .split(",")
        .map(|s| hash_from_string(s))
        .sum();

    sum
}

pub fn part2() -> i64 {
    0
}
