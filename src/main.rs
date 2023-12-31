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
        Day::new("Day 5", day5_part1,  day5_part2),
        Day::new("Day 6", day6_part1,  day6_part2),
        Day::new("Day 7", day7_part1,  day7_part2),
        Day::new("Day 8", day8_part1,  day8_part2),
        Day::new("Day 9", day9_part1,  day9_part2),
        Day::new("Day 10", day10_part1, day10_part2),
        Day::new("Day 11", day11_part1, day11_part2),
        Day::new("Day 12", day12_part1, day12_part2),
        Day::new("Day 13", day13_part1, day13_part2),
        Day::new("Day 14", day14_part1, day14_part2),
        Day::new("Day 15", day15_part1, day15_part2),
        Day::new("Day 16", day16_part1, day16_part2),
        Day::new("Day 17", day17_part1, day17_part2),
        Day::new("Day 18", day18_part1, day18_part2),
        Day::new("Day 19", day19_part1, day19_part2),
        Day::new("Day 20", day20_part1, day20_part2),
        Day::new("Day 21", day21_part1, day21_part2),
        Day::new("Day 22", day22_part1, day22_part2),
        Day::new("Day 23", day23_part1, day23_part2),
        Day::new("Day 24", day24_part1, day24_part2),
        Day::new("Day 25", day25_part1, day25_part2),
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