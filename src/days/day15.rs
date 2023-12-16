use std::str::FromStr;

use itertools::Itertools;

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

enum LensOperation {
    Add(String, i64, i64), // Label, Focal Length, Box number
    Remove(String, i64),   // Label, Box number
}

impl FromStr for LensOperation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        if s.contains('-') {
            let split = s.split('-').collect_vec();
            let label = split.get(0).unwrap();
            let hash = hash_from_string(label);

            Ok(Self::Remove(label.to_string(), hash))
        } else {
            let split = s.split('=').collect_vec();

            let label = split.get(0).unwrap();
            let focal_len = split.get(1).unwrap().parse::<i64>().unwrap();
            let hash = hash_from_string(label);

            Ok(Self::Add(label.to_string(), focal_len, hash))
        }
    }
}

fn apply_operation(boxes: &mut Vec<Vec<(String, i64)>>, op: LensOperation) {
    match op {
        LensOperation::Add(label, focal_len, box_i) => {
            let box_ref = &mut boxes[box_i as usize];

            if let Some((existing_i, _)) = box_ref.iter().find_position(|(l, _)| *l == label) {
                box_ref[existing_i] = (label, focal_len);
            } else {
                box_ref.push((label, focal_len));
            }
        }
        LensOperation::Remove(label, box_i) => {
            let box_ref = &mut boxes[box_i as usize];

            if let Some((existing_i, _)) = box_ref.iter().find_position(|(l, _)| *l == label) {
                box_ref.remove(existing_i);
            }
        }
    }
}

pub fn part2() -> i64 {
    let input_no_whitespace: String = INPUT.chars().filter(|c| !c.is_whitespace()).collect();

    let operations: Vec<_> = input_no_whitespace
        .split(",")
        .filter_map(|s| LensOperation::from_str(s).ok())
        .collect();

    let mut boxes: Vec<Vec<(String, i64)>> = vec![vec![]; 256];

    operations
        .into_iter()
        .for_each(|op| apply_operation(&mut boxes, op));

    boxes
        .iter()
        .enumerate()
        .map(|(box_i, b)| {
            b.iter()
                .enumerate()
                .map(|(lens_i, (_, focal_len))| {
                    (box_i + 1) as i64 * (lens_i + 1) as i64 * focal_len
                })
                .sum::<i64>()
        })
        .sum::<i64>()
}
