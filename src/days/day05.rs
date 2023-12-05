use std::str::FromStr;

const INPUT: &str = include_str!("input/day05.txt");

#[derive(Debug)]
struct Almanac {
    map_sets: Vec<MappingSet>
}

impl FromStr for Almanac {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map_sets : Vec<_> = s.split("\r\n\r\n")
            .map(|m_str| MappingSet::from_str(m_str))
            .collect();

        if let Some(Err(e)) = map_sets.iter().find(|m| m.is_err()) {
            return Err(e);
        }

        let map_sets : Vec<_> = map_sets.into_iter()
                                        .map(|m| m.unwrap())
                                        .collect();

        Ok(Self { map_sets })
    }
}

impl Almanac {
    fn get_location(&self, seed: i64) -> i64 {
        let mut dest = seed;
        for map_set in &self.map_sets {
            dest = map_set.apply(dest);
        }
        dest
    }
}

#[derive(Debug)]
struct MappingSet {
    mappings: Vec<Mapping>
}

impl FromStr for MappingSet {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mappings: Vec<_> = s.lines()
                                .skip(1)
                                .map(|m_str| Mapping::from_str(m_str))
                                .collect();

        if let Some(Err(e)) = mappings.iter().find(|m| m.is_err()) {
            return Err(e);
        }

        let mappings: Vec<_> = mappings.into_iter()
                                .map(|m| m.unwrap())
                                .collect();

        Ok(Self{mappings})
    }
}

impl MappingSet {
    fn apply(&self, input: i64) -> i64 {
        let dests : Vec<_> = self.mappings.iter()
            .filter_map(|m| {
                let d = m.apply(input);
                if d != input {
                    Some(d)
                }
                else {
                    None
                }
            })
            .collect();
        
        *dests.first().unwrap_or(&input)
    }
}

#[derive(Debug)]
struct Mapping {
    source: i64,
    offset: i64, 
    range: i64
}

impl FromStr for Mapping {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums : Vec<_> = s.split_ascii_whitespace()
                             .collect();

        let dest_start = nums[0].parse::<i64>();
        let source_start = nums[1].parse::<i64>();
        let range = nums[2].parse::<i64>();

        if dest_start.is_err() | source_start.is_err() | range.is_err() {
            return Err(stringify!(format!("Failed to parse mapping: {}", s)));
        }

        let source_start = source_start.unwrap();
        let offset = dest_start.unwrap() - source_start;
        let range = range.unwrap();

        Ok(Self { 
            source: source_start, 
            offset: offset, 
            range: range 
        })
    }
}

impl Mapping {
    fn apply(&self, input: i64) -> i64 {
        if input >= self.source && input < self.source + self.range {
            input + self.offset
        }
        else {
            input
        }
    }
}

pub fn part1() -> i64 {
    let Some((seeds_str, maps_str)) = INPUT.split_once("\r\n\r\n") else {
        return 0;
    };

    let seeds : Vec<_> = seeds_str.split_ascii_whitespace().filter_map(|s| s.parse::<i64>().ok()).collect();

    let almanac = Almanac::from_str(maps_str).unwrap();

    let min_location = seeds.iter().map(|&s| almanac.get_location(s)).min();

    min_location.unwrap_or(0)
}

pub fn part2() -> i64 {
    let Some((seeds_str, maps_str)) = INPUT.split_once("\r\n\r\n") else {
        return 0;
    };

    let seed_nums : Vec<_> = seeds_str.split_ascii_whitespace().filter_map(|s| s.parse::<i64>().ok()).collect();

    // Brute force does not work

    println!("{:?}",seeds.len());
    
    let almanac = Almanac::from_str(maps_str).unwrap();

    let min_location = seeds.iter().map(|&s| almanac.get_location(s)).min();

    min_location.unwrap_or(0)
}