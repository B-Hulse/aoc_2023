use std::{collections::HashMap, str::FromStr};
use regex::Regex;

const INPUT : &str = include_str!("input/day08.txt");

fn encode_str_to_int(s: &str) -> i64 {
    let mut encoding = 0;

    for (i, c) in s.chars().enumerate() {
        let val = (c as u8 - 'A' as u8) as i64;

        let index_pow = ((i + 1) as i64).pow(26_u32);

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
        let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

        if let Some(captures) = re.captures(s) {
            let node_name = captures.get(1).unwrap().as_str();
            let left_name = captures.get(2).unwrap().as_str();
            let right_name = captures.get(3).unwrap().as_str();
            return Ok(Self { 
                name: encode_str_to_int(node_name), 
                left: encode_str_to_int(left_name), 
                right: encode_str_to_int(right_name) 
            });
        }
        else {
            return Err("Could not extract node names");
        }
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

pub fn part2() -> i64 {
    0
}