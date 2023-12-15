#![allow(dead_code, unreachable_code)]

use std::{collections::HashMap, str::FromStr};

const INPUT: &str = include_str!("input/day12_example.txt");

struct Puzzle {
    springs: String,
    blocks: Vec<i64>,
}

impl FromStr for Puzzle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((springs, blocks_str)) = s.split_once(" ") else {
            return Err("Invalid input");
        };

        let blocks = blocks_str
            .split(",")
            .filter_map(|count_str| count_str.parse::<i64>().ok())
            .collect();
        Ok(Puzzle {
            springs: springs.to_string(),
            blocks,
        })
    }
}

fn remove_block_from_vals(
    vals: &mut Vec<i64>,
    block_size: i64,
) -> bool {
    if let Some(v) = vals.first() {
        if block_size != *v {
            return false;
        }
        vals.remove(0);
    } else {
        return false;
    }
    true
}

impl Puzzle {
    fn unfold(&self) -> Puzzle {
        let mut new_springs = String::new();
        let mut new_blocks = Vec::new();

        for _ in 0..5 {
            new_springs.push_str(&self.springs);
            new_springs.push('?');
            new_blocks.append(&mut self.blocks.clone());
        }

        new_springs.pop();

        return Puzzle {
            springs: new_springs,
            blocks: new_blocks,
        };
    }

    fn recursive_count_valid_permutations(
        &self,
        solved_cache: &mut HashMap<(usize, i64), i64>,
        current_spring: String,
        solved_block_count: i64,
    ) -> i64 {
        let unknown_count = current_spring.chars().filter(|c| *c == '?').count();

        if unknown_count == 0 {
            return if self.is_valid(&current_spring) {
                1
            } else {
                0
            };
        }

        if let Some(permutations) = solved_cache.get(&(unknown_count, solved_block_count)) {
            return *permutations;
        }

        let mut permutations = 0;

        // Dot
        let dot_input = current_spring.replacen('?', ".", 1);
        if let Some(dot_solved_block_count) = self.count_solved_blocks(&dot_input) {
            permutations += self.recursive_count_valid_permutations(
                solved_cache,
                dot_input,
                dot_solved_block_count,
            );
        }

        // Hash
        let hash_input = current_spring.replacen('?', "#", 1);
        if let Some(hash_solved_block_count) = self.count_solved_blocks(&hash_input) {
            permutations += self.recursive_count_valid_permutations(
                solved_cache,
                hash_input,
                hash_solved_block_count,
            );
        }

        solved_cache.insert((unknown_count, solved_block_count), permutations);

        permutations
    }

    fn count_solved_blocks(&self, input: &String) -> Option<i64> {
        let mut filled = input.chars().peekable();
        let mut vals = self.blocks.clone();

        let mut solved_block_count = 0;

        let mut current_block_size = 0;
        while let Some(c) = filled.next() {
            if c == '#' {
                current_block_size += 1;

                let next = filled.peek();

                if next.is_none() || *next.unwrap() == '.' {
                    if remove_block_from_vals(&mut vals, current_block_size) {
                        solved_block_count += 1;
                    }
                    else {
                        return None;
                    }
                    current_block_size = 0;
                }
            }
            else if c == '?' {
                break;
            }
        }

        Some(solved_block_count)
    }

    fn is_valid(&self, input: &String) -> bool {
        let mut filled = input.chars().peekable();
        let mut vals = self.blocks.clone();

        let mut current_block_size = 0;
        while let Some(c) = filled.next() {
            if c == '#' {
                current_block_size += 1;

                let next = filled.peek();

                if next.is_none() || *next.unwrap() == '.' {
                    if !remove_block_from_vals(&mut vals, current_block_size) {
                        return false;
                    }
                    current_block_size = 0;
                }
            }
        }

        if vals.is_empty() {
            return true;
        }
        return false;
    }
}

pub fn part1() -> i64 {
    // TODO: This is not working yet
    return 0;
    
    INPUT
        .lines()
        .filter_map(|l| Puzzle::from_str(l).ok())
        .map(|puzzle| {
            puzzle.recursive_count_valid_permutations(&mut HashMap::new(), puzzle.springs.clone(), 0)
        })
        .sum::<i64>()
}

pub fn part2() -> i64 {
    // TODO: This is not working yet
    return 0;

    INPUT
        .lines()
        .filter_map(|l| Puzzle::from_str(l).ok())
        .map(|puzzle| puzzle.unfold())
        .map(|p| {
            let temp = p.recursive_count_valid_permutations(&mut HashMap::new(), p.springs.clone(), 0);
            temp
        })
        .sum::<i64>()
}
