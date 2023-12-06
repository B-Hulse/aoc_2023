const INPUT: &str = include_str!("input/day06.txt");

struct Race {
    time:i64,
    distance:i64
}

impl Race {
    fn find_ways_to_beat(&self) -> i64 {
        let mut lowest_win = None;
        let mut highest_win = None;

        for charge_time in 0..self.time {
            let winning = ((self.time - charge_time) * charge_time) > self.distance;
            if lowest_win.is_none() && winning {
                lowest_win = Some(charge_time);
            }
            else if lowest_win.is_some() && highest_win.is_none() && !winning {
                highest_win = Some(charge_time - 1);
            }
        }

        if lowest_win.is_some() && highest_win.is_some() {
            highest_win.unwrap() - lowest_win.unwrap() + 1
        }
        else if lowest_win.is_some() {
            self.time - lowest_win.unwrap()
        }
        else {
            0
        }
    }
}

pub fn part1() -> i64 {
    let nums : Vec<_> = INPUT.lines()
                            .map(|l| l.split_ascii_whitespace()
                                            .skip(1)    
                                            .filter_map(|n| n.parse::<i64>().ok())
                                            .collect::<Vec<_>>()
                            )
                            .collect();

    let mut races = Vec::new();

    for i in 0..nums[0].len() {
        races.push(Race { time: nums[0][i], distance: nums[1][i]});
    }

    races.iter().map(|r| r.find_ways_to_beat()).product()
}

pub fn part2() -> i64 {
    let lines : Vec<_> = INPUT.lines().collect();

    let time = lines[0].chars()
                            .filter(|c| c.is_digit(10))
                            .collect::<String>()
                            .parse::<i64>()
                            .unwrap();
    let dist = lines[1].chars()
                            .filter(|c| c.is_digit(10))
                            .collect::<String>()
                            .parse::<i64>()
                            .unwrap();

    let race = Race {
        time: time, 
        distance: dist
    };

    race.find_ways_to_beat()
}