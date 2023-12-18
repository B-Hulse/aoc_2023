use std::str::FromStr;

const INPUT : &str = include_str!("input/day18.txt");

enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct Node {
    dir: Direction,
    dist: i64
}

impl FromStr for Node {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<_> = s.split_ascii_whitespace().collect();

        if splits.len() < 2 {
            return Err("Invalid input");
        }

        let direction = match splits[0] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err("Invalid direction")
        };

        let Ok(distance) = splits[1].parse::<i64>() else {
            return Err("Invalid distance");
        };

        Ok(Self { dir: direction, dist: distance })
    }
}

fn get_coords(nodes: &Vec<Node>) -> Vec<(i64, i64)> {
    let mut coords = vec![(0, 0)];

    nodes.iter().for_each(|node| {
        let current_coords = coords.last().unwrap();

        coords.push(
            match node.dir {
                Direction::Up => (current_coords.0, current_coords.1 - node.dist),
                Direction::Down => (current_coords.0, current_coords.1 + node.dist),
                Direction::Left => (current_coords.0 - node.dist, current_coords.1),
                Direction::Right => (current_coords.0 + node.dist, current_coords.1)
            }
        );
    });

    coords
}

fn get_enclosed_area(coords: &Vec<(i64,i64)>) -> i64 {
    // Shoelace formula
    let mut sum = 0;
    for i in 0..coords.len() - 1 {
        let coords_current = coords[i];
        let coords_next = coords[i + 1];

        sum += coords_current.0 * coords_next.1;
        sum -= coords_current.1 * coords_next.0;
    }
    sum / 2
}

fn decode_hex_instruction(s: &str) -> Result<String, &'static str> {
    fn hex_to_dec(s: &str) -> i64 {
        let mut sum = 0;

        for c in s.chars() {
            sum *= 16;
            sum += match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'a' | 'A' => 10,
                'b' | 'B' => 11,
                'c' | 'C' => 12,
                'd' | 'D' => 13,
                'e' | 'E' => 14,
                'f' | 'F' => 15,
                _ => panic!("Invalid hex character")
            };
        }

        sum
    }

    fn get_hex(s: &str) -> &str {
        let hash_i = s.find('#').unwrap();

        &s[hash_i + 1..hash_i + 7]
    }

    let hex_str = get_hex(s);

    let direction_encoded = hex_str.chars().last();
    if direction_encoded.is_none() {
        return Err("No direction hex");
    }

    let direction = match direction_encoded.unwrap() {
        '0' => 'R',
        '1' => 'D',
        '2' => 'L',
        '3' => 'U',
        _ => return Err("Invalid direction hex")
    };

    let distance = &hex_to_dec(&hex_str[0..hex_str.len() - 1]).to_string();

    Ok(format!("{} {}", direction, distance))
}

pub fn part1() -> i64 {
    let nodes: Vec<_> = INPUT.lines().map(|line| {
        let Ok(node) = Node::from_str(line) else {
            panic!("Invalid input");
        };

        node
    })
    .collect();
    
    let coords = get_coords(&nodes);

    let area_enclosed = get_enclosed_area(&coords);
    
    let distance = nodes.iter().map(|n| n.dist).sum::<i64>();
    // Pick's theorem
    area_enclosed + (distance/2) + 1
}

pub fn part2() -> i64 {
    let nodes: Vec<_> = INPUT.lines().map(|line| {
        let Ok(decoded_instructions) = decode_hex_instruction(line) else {
            panic!("Invalid hex");
        };

        let Ok(node) = Node::from_str(&decoded_instructions) else {
            panic!("Invalid input");
        };

        node
    })
    .collect();
    
    let coords = get_coords(&nodes);

    let area_enclosed = get_enclosed_area(&coords);
    
    let distance = nodes.iter().map(|n| n.dist).sum::<i64>();
    // Pick's theorem
    area_enclosed + (distance/2) + 1
}