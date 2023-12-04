use std::time::Instant;

mod days;

use days::*;

// Struct that takes a function and runs that function, outputting the time it took to run.
struct PartRunner {
    func: fn() -> i32,
    name: String,
}

impl PartRunner {
    fn new(func: fn() -> i32, name: String) -> PartRunner {
        PartRunner { 
            func: func,
            name: name
         }
    }

    fn run(&self) {
        let start = Instant::now();
        let ret = (self.func)();
        let duration = start.elapsed();
        println!("{}: {} ({:?})", self.name, ret, duration);
    }
}

fn main()
{
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let day = args[1].parse::<u8>().unwrap();
        match day {
            1 => {
                PartRunner::new(day1_part1, "Day 1 Part 1".to_string()).run();
                PartRunner::new(day1_part2, "Day 1 Part 2".to_string()).run();
            },
            2 => {
                PartRunner::new(day2_part1, "Day 2 Part 1".to_string()).run();
                PartRunner::new(day2_part2, "Day 2 Part 2".to_string()).run();
            },
            3 => {
                PartRunner::new(day3_part1, "Day 3 Part 1".to_string()).run();
                PartRunner::new(day3_part2, "Day 3 Part 2".to_string()).run();
            },
            4 => {
                PartRunner::new(day4_part1, "Day 4 Part 1".to_string()).run();
                PartRunner::new(day4_part2, "Day 4 Part 2".to_string()).run();
            },
            _ => println!("Invalid day"),
        }
    }
    else {
        run_all_days()
    }
}

fn run_all_days() {
    PartRunner::new(day1_part1, "Day 1 Part 1".to_string()).run();
    PartRunner::new(day1_part2, "Day 1 Part 2".to_string()).run();
    PartRunner::new(day2_part1, "Day 2 Part 1".to_string()).run();
    PartRunner::new(day2_part2, "Day 2 Part 2".to_string()).run();
    PartRunner::new(day3_part1, "Day 3 Part 1".to_string()).run();
    PartRunner::new(day3_part2, "Day 3 Part 2".to_string()).run();
    PartRunner::new(day4_part1, "Day 4 Part 1".to_string()).run();
    PartRunner::new(day4_part2, "Day 4 Part 2".to_string()).run();
}