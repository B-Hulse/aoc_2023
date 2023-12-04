use std::{str::FromStr, collections::HashSet};


const INPUT: &str = include_str!("input/day04.txt");

#[derive(Debug)]
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

        let Some(id) = header.split_whitespace().last().and_then(|s| s.parse::<i32>().ok()) else {
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
    fn get_matching_count(&self) -> usize {
        self.winning_numbers.intersection(&self.present_numbers).count()
    }

    fn get_score(&self) -> i32 {
        let won_nums = self.get_matching_count();

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

struct CardCollection {
    card: Card,
    count: i32
}

pub fn part2() -> i32 {
    let mut cards : Vec<_> = INPUT.lines()
                    .filter_map(|l| Card::from_str(l).ok())
                    .map(|c| {
                        CardCollection {
                            card: c,
                            count: 1
                        }
                    })
                    .collect();
    
    for card_i in 0..cards.len() {
        let card_count = cards[card_i].count;
        let card_match_count = cards[card_i].card.get_matching_count();

        if card_match_count > 0 {
            for match_i in 1..=card_match_count {
                cards.get_mut(card_i + match_i).unwrap().count += card_count;
            }
        }
    }

    let sum = cards.iter().map(|c| c.count).sum();

    sum
}