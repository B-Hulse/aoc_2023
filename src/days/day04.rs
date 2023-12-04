use std::{str::FromStr, collections::HashSet};


const INPUT: &str = include_str!("input/day04.txt");

struct Card {
    _id: i32,
    winning_numbers: HashSet<i32>,
    present_numbers: HashSet<i32>
}

fn get_num_list(s: &str) -> HashSet<i32> {
    let mut nums = HashSet::new();

    for num_str in s.split(" ") {
        if let Ok(num) = num_str.parse::<i32>() {
            nums.insert(num);
        }
    }
    
    nums
}

impl FromStr for Card {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((header, body)) = s.split_once(":") else {
            return Err("Failed to find header");
        };

        let Some(id) = header.split_whitespace().nth(1).and_then(|s| s.parse::<i32>().ok()) else {
            return Err("Failed to get ID");
        };

        let Some((winning_nums_str, present_nums_str)) = body.split_once("|") else {
            return Err("Failed to find numbers");
        };

        let winning_nums = get_num_list(winning_nums_str);
        let present_nums = get_num_list(present_nums_str);

        Ok(Self {
            _id: id,
            winning_numbers: winning_nums,
            present_numbers: present_nums
        })
    }
}

impl Card {
    fn get_score(&self) -> i32 {
        let won_nums = self.winning_numbers.intersection(&self.present_numbers).count();

        if won_nums <= 0 {
            return 0;
        }
        else {
            2_i32.pow((won_nums - 1) as u32)
        }
    }
}

pub fn part1() -> i32 {
    INPUT.lines()
        .filter_map(|l| Card::from_str(l).ok())
        .map(|c| c.get_score())
        .sum()
}

pub fn part2() -> i32 {
    0
}