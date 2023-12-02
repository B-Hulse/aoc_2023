use std::str::FromStr;

#[derive(Debug)]
enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32)
}

impl FromStr for Cube {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((num_str, col_str)) = s.trim().split_once(" ") else {
            return Err("Could not parse Cube");
        };

        let num = num_str.parse::<u32>();

        if let Err(e) = num {
            return Err("Coult not parse Cube count");
        }

        let num = num.unwrap();
        
        match col_str {
            "red" => Ok(Self::Red(num)),
            "blue" => Ok(Self::Blue(num)),
            "green" => Ok(Self::Green(num)),
            _ => Err("Invalid color found")
        }
    }
}

#[derive(Debug)]
struct Hand {
    cubes: Vec::<Cube>
}

impl FromStr for Hand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cubes = Vec::new();

        for c_str in s.split(",") {
            match Cube::from_str(c_str) {
                Ok(c) => cubes.push(c),
                Err(e) => return Err(e)
            }
        }

        Ok(Self {
            cubes: cubes
        })
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    hands: Vec::<Hand>
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((header, body)) = s.split_once(":") else { 
            return Err("Failed to get header");
        };

        let Some(id) = header.split_whitespace().nth(1).and_then(|s| s.parse::<u32>().ok()) else {
            return Err("Failed to get ID");
        };

        let mut hands = Vec::new();

        for h_str in body.split(";") {
            match Hand::from_str(h_str) {
                Ok(h) => hands.push(h),
                Err(e) => return Err(e),
            }
        }

        Ok(Self {
            id: id,
            hands: hands
        })
    }
}

pub fn day2() {
    //let input = include_str!("input/day02.txt");
    let input = include_str!("input/day02.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut games = Vec::new();
    for line in input.split("\r\n") {
        match Game::from_str(line) {
            Ok(g) => games.push(g),
            Err(e) => println!("ERROR: Could not parse \"{}\": {}", line, e)
        }
    }

    let sum: u32 = games.iter().filter(|&g| {
        g.hands.iter().all(|h| {
            h.cubes.iter().all(|c| {
                match c {
                    Cube::Blue(count) => {return *count <= 14;},
                    Cube::Green(count) => {return *count <= 13;},
                    Cube::Red(count) => {return *count <= 12;},
                }
            })
        })
    }).map(|g| g.id).sum();

    println!("Part 1: {}", sum);
}

fn part2(input: &str) {
    let mut games = Vec::new();
    for line in input.split("\r\n") {
        match Game::from_str(line) {
            Ok(g) => games.push(g),
            Err(e) => println!("ERROR: Could not parse \"{}\": {}", line, e)
        }
    }

    let sum: u32= games.iter().map(|g| {
        let mut min_green = 0;
        let mut min_red = 0;
        let mut min_blue = 0;

        for h in &g.hands {
            for c in &h.cubes {
                match c {
                    Cube::Blue(count) => {if count > &min_blue {min_blue = *count}},
                    Cube::Green(count) => {if count > &min_green {min_green = *count}},
                    Cube::Red(count) => {if count > &min_red {min_red = *count}},
                }
            }
        }

        min_blue * min_green * min_red
    }).sum();

    println!("Part 2: {}", sum);
}