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

    fn get_location_range(&self, ranges: Vec<NumRange>) -> Vec<NumRange> {
        let mut seed_ranges = ranges;

        for map_set in &self.map_sets {
            seed_ranges = map_set.apply_to_range(seed_ranges);
        }

        seed_ranges
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

    fn apply_to_range(&self, input_ranges: Vec<NumRange>) -> Vec<NumRange> {
        let mut ranges = input_ranges;
        let mut modified_ranges = Vec::new();
        for mapping in &self.mappings {
            // Each number can only be modified by each MappingSet once
            // Keep ranges of numbers that have changed separate so that they're not processed by later mappings
            let mut new_modified_ranges;
            (ranges, new_modified_ranges) = mapping.apply_to_ranges(ranges);

            if new_modified_ranges.len() > 0 {
                modified_ranges.append(&mut new_modified_ranges);
            }
        }

        // Add the modified ranges back once the MappingSet is finished
        ranges.append(&mut modified_ranges);
        ranges
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

    fn apply_to_ranges(&self, input_ranges: Vec<NumRange>) -> (Vec<NumRange>, Vec<NumRange>) {
        let mut unmodified = Vec::new();
        let mut modified = Vec::new();
        for seed_range in input_ranges {
            let ranges: (Option<NumRange>, Option<NumRange>, Option<NumRange>) = seed_range.apply_mapping(self);

            if let Some(ul) = ranges.0 {
                unmodified.push(ul);
            }
            if let Some(ur) = ranges.1 {
                unmodified.push(ur);
            }
            if let Some(m) = ranges.2 {
                modified.push(m);
            }
        }

        (unmodified, modified)
    }
}

#[derive(Clone, Copy, Debug)]
struct NumRange {
    start: i64,
    len: i64
}

impl NumRange {
    fn apply_mapping(&self, mapping: &Mapping) -> (Option<NumRange>, Option<NumRange>, Option<NumRange>) {
        let old_left = self.start;
        let old_right = self.start + self.len - 1;

        let mapping_left = mapping.source;
        let mapping_right = mapping.source + mapping.range - 1;

        // Case where original is entirely inside mapping
        if mapping_left <= old_left && old_right <= mapping_right {
            return (None, None , Some(
                NumRange { 
                    start: self.start+mapping.offset, 
                    len: self.len 
                }));
        }

        let new_left = if old_left <= mapping_left && mapping_left <= old_right {mapping_left} else {old_left};
        let new_right = if old_left <= mapping_right && mapping_right <= old_right {mapping_right} else {old_right};

        // No mapping applied
        if new_left == old_left && new_right == old_right {
            return (Some(*self), None, None);
        }

        let unmodified_left = if new_left > old_left {
            Some(NumRange { 
                    start: old_left, 
                    len: new_left - old_left
                })
        }
        else {
            None
        };

        let unmodified_right = if new_right < old_right {
            Some(NumRange { 
                    start: new_right + 1, 
                    len: old_right - new_right 
                })
        } 
        else {
            None
        };

        let modified_middle = Some(
            NumRange { 
                start: new_left + mapping.offset, 
                len: new_right - new_left + 1 
            });

        (unmodified_left, unmodified_right, modified_middle)
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

    let mut seed_ranges = Vec::new();

    for seed_start_i in (0..seed_nums.len()).step_by(2) {
        let seed_start = seed_nums[seed_start_i];
        let seed_range = seed_nums[seed_start_i + 1]; 

        seed_ranges.push(
            NumRange {
                start: seed_start,
                len: seed_range
            }
        )
    }
    
    let almanac = Almanac::from_str(maps_str).unwrap();

    let loc_ranges = almanac.get_location_range(seed_ranges);

    let min_loc = loc_ranges.into_iter().min_by_key(|r| r.start);

    min_loc.unwrap().start
}