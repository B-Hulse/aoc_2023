use std::str::FromStr;

const INPUT: &str = include_str!("input/day12.txt");

struct Puzzle {
    springs: Vec<u8>,
    blocks: Vec<usize>,
}

impl FromStr for Puzzle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((springs, blocks_str)) = s.split_once(" ") else {
            return Err("Invalid input");
        };

        let blocks = blocks_str
            .split(",")
            .filter_map(|count_str| count_str.parse::<usize>().ok())
            .collect();
        Ok(Puzzle {
            springs: springs.as_bytes().to_vec(),
            blocks,
        })
    }
}

impl Puzzle {
    fn unfold(&self) -> Puzzle {
        let mut new_springs = Vec::new();
        let mut new_blocks = Vec::new();

        for _ in 0..5 {
            new_springs.extend(self.springs.clone());
            new_springs.push(b'?');
            new_blocks.append(&mut self.blocks.clone());
        }

        new_springs.pop();

        return Puzzle {
            springs: new_springs,
            blocks: new_blocks,
        };
    }

    fn solve_dp(&self) -> i64 {
        let mut springs = self.springs.clone();
        springs.push(b'.');

        let mut dp_table = vec![vec![0;springs.len()]; self.blocks.len()];
    
        // For each i in springs, max_hash_for_i[i] is the number of # and ? before i.
        let mut sum = 0;

        let mut max_hash_for_i = springs.iter().map(|&b| {
            if b != b'.' {
                sum += 1;
            }
            sum
        }).collect::<Vec<_>>();
        max_hash_for_i.insert(0, 0);
        let max_hash_for_i = max_hash_for_i;
    
        //  How many dots will there be in a valid solution
        let wiggle_room = springs.len() - self.blocks.iter().sum::<usize>() - self.blocks.len() + 1;
    
        // Count combinations, handling the first row as a special case.
        let block_0_width = self.blocks[0];
        let mut combination_count:i64 = 0;
        let mut valid = true;
    
        for starting_i in 0..wiggle_room {
            // In order to be a broken spring, an interval must only contains `#` or `?`
            // characters and not have a '#' character immediately before or after.
            if springs[starting_i + block_0_width] == b'#' {
                combination_count = 0;
            } else if valid && max_hash_for_i[starting_i + block_0_width] - max_hash_for_i[starting_i] == block_0_width {
                combination_count += 1;
            }
    
            dp_table[0][starting_i + block_0_width] = combination_count;
    
            // All following patters are invalid if our window is preceded by a '#'.
            valid &= springs[starting_i] != b'#';
        }
    
        let mut next_i = block_0_width + 1;
    
        for (block_i, &block_size) in self.blocks.iter().enumerate().skip(1) {
            let mut combination_count = 0;
    
            for i in next_i..next_i + wiggle_room {
                if springs[i + block_size] == b'#' {
                    combination_count = 0;
                } else if dp_table[block_i - 1][i - 1] > 0
                    && springs[i - 1] != b'#'
                    && max_hash_for_i[i + block_size] - max_hash_for_i[i] == block_size
                {
                    combination_count += dp_table[block_i - 1][i - 1];
                }
    
                dp_table[block_i][i + block_size] = combination_count;
            }
    
            next_i += block_size + 1;
        }
    
        *dp_table.last().unwrap().last().unwrap()
    }
}

pub fn part1() -> i64 {
    let puzzles : Vec<_> = INPUT.lines().filter_map(|l| {
        Puzzle::from_str(l).ok()
    })
    .collect();

    puzzles.into_iter().map(|p| p.solve_dp()).sum::<i64>()
}

pub fn part2() -> i64 {
    let puzzles : Vec<_> = INPUT.lines().filter_map(|l| {
        if let Ok(p) = Puzzle::from_str(l) {
            Some(p.unfold())
        }
        else {
            None
        }
    })
    .collect();

    puzzles.into_iter().map(|p| p.solve_dp()).sum::<i64>()
}
