use std::{collections::HashMap, str::FromStr};

const INPUT : &str = include_str!("input/day08.txt");

fn encode_str_to_int(s: &str) -> i64 {
    let mut encoding = 0;

    for (i, c) in s.chars().enumerate() {
        let val = (c as u8 - 'A' as u8) as i64;

        let index_pow = 26_i64.pow(i as u32);

        encoding += val * index_pow;
    }

    encoding
}

fn encode_str_to_int_2(s: &str) -> i64 {
    let mut encoding = 0;

    for (i, c) in s.chars().rev().enumerate() {
        let val = if c.is_numeric() {
            c as u8 - '0' as u8 + 26
        }
        else {
            c as u8 - 'A' as u8 
        } as i64;
        
        let index_pow = 36_i64.pow(i as u32);

        encoding += val * index_pow;
    }

    encoding
}

#[derive(Clone, Debug)]
struct Node {
    name: i64,
    left: i64,
    right: i64
}

impl FromStr for Node {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((name, paths)) = s.split_once(" = ") else {
            return Err("Could not split node");
        };

        let Some((left_node, right_node)) = paths.split_once(", ") else {
            return Err("Could not split paths");
        };

        let mut left_node = left_node.chars();
        left_node.next();
        let left_node = left_node.as_str();

        let mut right_node = right_node.chars();
        right_node.next_back();
        let right_node = right_node.as_str();

        return Ok(Self { 
            name: encode_str_to_int(name), 
            left: encode_str_to_int(left_node), 
            right: encode_str_to_int(right_node) 
        });
    }
}

#[derive(Clone, Debug)]
struct Node2 {
    name: i64,
    left: i64,
    right: i64
}

impl FromStr for Node2 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((name, paths)) = s.split_once(" = ") else {
            return Err("Could not split node");
        };

        let Some((left_node, right_node)) = paths.split_once(", ") else {
            return Err("Could not split paths");
        };

        let mut left_node = left_node.chars();
        left_node.next();
        let left_node = left_node.as_str();

        let mut right_node = right_node.chars();
        right_node.next_back();
        let right_node = right_node.as_str();

        return Ok(Self { 
            name: encode_str_to_int_2(name), 
            left: encode_str_to_int_2(left_node), 
            right: encode_str_to_int_2(right_node) 
        });
    }
}

pub fn part1() -> i64 {
    let Some((instruction_str, nodes_str)) = INPUT.split_once("\r\n\r\n") else {
        return -1;
    };

    let nodes : HashMap<i64, Node> = nodes_str.lines()
                                            .map(|n| {
                                                let node = Node::from_str(n).unwrap();
                                                return (node.name, node);
                                            })
                                            .collect();

    let mut complete = false;
    let mut current = encode_str_to_int("AAA");
    let encoded_z = encode_str_to_int("ZZZ");
    let mut steps = 0;
    while !complete {
        for instruction in instruction_str.chars() {
            let current_node = nodes.get(&current).unwrap();

            if instruction == 'L' {
                current = current_node.left;
                steps += 1;
            }
            else if instruction == 'R' {
                current = current_node.right;
                steps += 1;
            }

            if current == encoded_z {
                complete = true;
            }
        }
    }

    steps
}
 
fn lcm(first: i64, second: i64) -> i64 {
    first * second / gcd(first, second)
}

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

pub fn part2() -> i64 {
    let Some((instruction_str, nodes_str)) = INPUT.split_once("\r\n\r\n") else {
        return -1;
    };

    let nodes : HashMap<i64, Node2> = nodes_str.lines()
                                            .map(|n| {
                                                let node = Node2::from_str(n).unwrap();
                                                return (node.name, node);
                                            })
                                            .collect();
                                            
    let current : Vec<_> = nodes.iter()
                                .filter_map(|n| {
                                    if n.0 % 36 == 0 {
                                        return Some(*n.0);
                                    }
                                    else {
                                        return None;
                                    }
                                })
                                .collect();

    let cycle_counts : Vec<_> = current.iter().map(|c| {
        let mut steps = 0;
        let mut current = *c;
        loop {
            for instruction in instruction_str.chars() {
                let current_node = nodes.get(&current).unwrap();
                if instruction == 'L' {
                    current = current_node.left;
                }
                else if instruction == 'R' {
                    current = current_node.right;
                }
        
                steps += 1;
        
                if current % 36 == ('Z' as u8 - 'A' as u8) as i64 {
                    return steps;
                }
            }
        }
    })
    .collect();

    cycle_counts.iter().fold(1, |acc, c| {
        lcm(acc, *c)
    })
}