use std::time::Instant;

mod days;

use days::*;

#[derive(Clone)]
struct Day {
    name: String,
    part1: fn() -> i64,
    part2: fn() -> i64
}

impl Day {
    fn new(name: &str, part1: fn()->i64, part2: fn()->i64) -> Day {
        Day { 
            name: name.to_string(),
            part1: part1, 
            part2: part2
        }
    }
}

struct DayRunner {
    day: Day
}

impl DayRunner {
    fn new(day: Day) -> DayRunner {
        DayRunner { 
            day: day
        }
    }

    fn run(&self) {
        let start = Instant::now();
        let ret = (self.day.part1)();
        let duration = start.elapsed();
        println!("{} Part 1: {} ({:?})", self.day.name, ret, duration);

        let start = Instant::now();
        let ret = (self.day.part2)();
        let duration = start.elapsed();
        println!("{} Part 2: {} ({:?})", self.day.name, ret, duration);
    }
}

fn main()
{
    let args: Vec<String> = std::env::args().collect();

    let days = vec![
        Day::new("Day 1", day1_part1,  day1_part2),
        Day::new("Day 2", day2_part1,  day2_part2),
        Day::new("Day 3", day3_part1,  day3_part2),
        Day::new("Day 4", day4_part1,  day4_part2),
        Day::new("Day 5", day5_part1,  day5_part2)
    ];

    if args.len() > 1 {
        let day_i = args[1].parse::<usize>().unwrap() - 1;
        
        if let Some(day) = days.get(day_i) {
            DayRunner::new(day.clone()).run();
        }
        else {
            println!("Invalid Day")
        }
    }
    else {
        run_all_days(&days);
    }
}

fn run_all_days(days: &Vec<Day>) {
    days.iter().for_each(|day| {
        DayRunner::new(day.clone()).run();
    });
}