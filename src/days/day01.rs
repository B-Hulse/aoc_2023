use std::collections::HashMap;

pub fn day1() {
    let input = include_str!("input/day1.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut sum = 0;
    for line in input.split("\r\n") {
        let digit1 = line.chars().find(|c: &char| c.is_digit(10));
        let digit2 = line.chars().rfind(|c: &char| c.is_digit(10));
        
        let mut resultStr = String::new();
        if let Some(d) = digit1 {
            resultStr.push(d);
        }
        if let Some(d) = digit2 {
            resultStr.push(d);
        }

        let result = resultStr.parse::<i32>();
        if let Ok(res) = result {
            sum += res;
        }
    }
    println!("Part 1: {}", sum);
}

fn find_first_digit(input: &str, reverse: bool) -> Option<char> {
    let digit_map = HashMap::from([
        ("1", '1'),
        ("2", '2'),
        ("3", '3'),
        ("4", '4'),
        ("5", '5'),
        ("6", '6'),
        ("7", '7'),
        ("8", '8'),
        ("9", '9'),
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    let positions = digit_map.iter()
                                            .map(|(&s,&c)| {
                                                if !reverse {
                                                    (input.find(s), c)
                                                }
                                                else {
                                                    (input.rfind(s), c)
                                                }
                                            })
                                            .filter(|(v,_)| v.is_some())
                                            .map(|(v,s)| (v.unwrap(), s));

    let first = if !reverse {
        positions.min_by_key(|(v,_)| v.clone())
    } else {
        positions.max_by_key(|(v,_)| v.clone())
    } ;

    if let Some(digit) = first {
        return Some(digit.1);
    }
    else {
        return None;
    }
}

fn part2(input: &str) {
    let mut sum = 0;
    for line in input.split("\r\n") {
        let digit1 = find_first_digit(line, false);
        let digit2 = find_first_digit(line, true);
        
        let mut resultStr = String::new();
        if let Some(d) = digit1 {
            resultStr.push(d);
        }
        if let Some(d) = digit2 {
            resultStr.push(d);
        }

        let result = resultStr.parse::<i32>();
        if let Ok(res) = result {
            sum += res;
        }
    }
    println!("Part 2: {}", sum);
}