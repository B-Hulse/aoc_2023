use std::collections::HashSet;

const INPUT: &str = include_str!("input/day03.txt");

pub fn part1() -> i64 {
    let mut symbol_neighbourhood = HashSet::new();

    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            if !c.is_digit(10) && c != '.' {
                symbol_neighbourhood.insert((x, y));
                symbol_neighbourhood.insert((x, y+1));
                symbol_neighbourhood.insert((x, y-1));
                symbol_neighbourhood.insert((x-1, y));
                symbol_neighbourhood.insert((x-1, y+1));
                symbol_neighbourhood.insert((x-1, y-1));
                symbol_neighbourhood.insert((x+1, y));
                symbol_neighbourhood.insert((x+1, y+1));
                symbol_neighbourhood.insert((x+1, y-1));
            }
        }
    }

    let mut sum = 0;
    for (y, line) in INPUT.lines().enumerate() {
        let mut current_num = 0;
        let mut current_num_part = false;
        for (x, c) in line.trim().chars().enumerate() {
            if c.is_digit(10) {
                current_num = (current_num * 10) + c.to_digit(10).unwrap();
                if symbol_neighbourhood.contains(&(x, y)) {
                    current_num_part = true;
                }
            }
            else if current_num > 0 {
                if current_num_part {
                    sum += current_num;
                }
                current_num = 0;
                current_num_part = false;
            }
        }

        if current_num > 0 {
            if current_num_part {
                sum += current_num;
            }
        }
    }

    sum as i64
}

pub fn part2() -> i64 {
    let mut gears = Vec::new();
    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            if !c.is_digit(10) && c != '.' {
                let mut gear = HashSet::new();
                gear.insert((x, y));
                gear.insert((x, y+1));
                gear.insert((x, y-1));
                gear.insert((x-1, y));
                gear.insert((x-1, y+1));
                gear.insert((x-1, y-1));
                gear.insert((x+1, y));
                gear.insert((x+1, y+1));
                gear.insert((x+1, y-1));
                gears.push(gear);
            }
        }
    }

    let mut gear_ratios = Vec::new();
    gear_ratios.resize(gears.len(), Vec::new());
    for (y, line) in INPUT.lines().enumerate() {
        let mut current_num = 0;
        let mut adjacent_gears = HashSet::new();

        for (x, c) in line.trim().chars().enumerate() {
            if c.is_digit(10) {
                current_num = (current_num * 10) + c.to_digit(10).unwrap();
                
                let touching_gears_i = gears.iter()
                    .enumerate()
                    .filter(|(_, g)| g.contains(&(x, y)))
                    .map(|(i, _)| i)
                    .collect::<Vec<_>>();

                for gear_i in touching_gears_i {
                    adjacent_gears.insert(gear_i);
                }
            }
            else if current_num > 0 {
                for i in adjacent_gears.iter() {
                    gear_ratios[*i].push(current_num);
                }
                current_num = 0;
                adjacent_gears.clear();
            }
        }

        if current_num > 0 {
            for i in adjacent_gears.iter() {
                gear_ratios[*i].push(current_num);
            }
        }
    }

    let sum = gear_ratios.iter()
        .filter(|g| g.len() > 1)
        .map(|g| g.iter().product::<u32>())
        .sum::<u32>();

    sum as i64
}