use std::{str::FromStr, collections::HashMap};
use itertools::Itertools;

const INPUT : &str = include_str!("input/day07.txt");

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0
}

impl FromStr for HandType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut card_counts: Vec<_> = s.chars()
                                        .unique()
                                        .map(|u| s.chars()
                                                        .filter(|c| u==*c)
                                                        .count())
                                        .collect();
        card_counts.sort_by(|a,b| b.cmp(a));
        let card_counts = card_counts;

        Ok(match card_counts.get(0) {
            Some(5) => Self::FiveOfAKind,
            Some(4) => Self::FourOfAKind,
            Some(3) => {
                if card_counts.get(1) == Some(&2) {
                    Self::FullHouse
                }
                else {
                    Self::ThreeOfAKind
                }
            },
            Some(2) => {
                if card_counts.get(1) == Some(&2) {
                    Self::TwoPair
                }
                else {
                    Self::OnePair
                }
            }
            _ => Self::HighCard
        })
    }
    
}

#[derive(Debug, Eq, Ord)]
struct Hand {
    _cards: String,
    bet: i64,
    hand_type: HandType,
    score: i64
}

impl FromStr for Hand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((cards, bet)) = s.split_once(" ") else {
            return Err("Failed to parse hand");
        };

        let card_vals = HashMap::from([
            ('A',14),
            ('K',13),
            ('Q',12),
            ('J',11),
            ('T',10),
            ('9',9),
            ('8',8),
            ('7',7),
            ('6',6),
            ('5',5),
            ('4',4),
            ('3',3),
            ('2',2),
            ('1',1),
            ('0',0)
        ]);

        let mut score = 0;
        let card_count = cards.chars().count();
        for (i, c) in cards.chars().enumerate() {
            let power: u32 = (card_count - i) as u32;
            let index_val = card_vals.iter().count().pow(power) as i64;
            let card_val = *card_vals.get(&c).unwrap() as i64;
            score += index_val * card_val;
        }

        Ok(Self {
            _cards: cards.to_string(),
            bet: bet.parse().unwrap(),
            hand_type: HandType::from_str(cards).unwrap(),
            score, 
        })
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.bet == other.bet
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.hand_type != other.hand_type {
            self.hand_type.partial_cmp(&other.hand_type)
        }
        else { 
            self.score.partial_cmp(&other.score)
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandTypeWithJoker {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0
}

impl FromStr for HandTypeWithJoker {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let joker_count = s.chars()
                                    .filter(|c| *c=='J')
                                    .count();

        let mut card_counts: Vec<_> = s.chars()
                                        .unique()
                                        .filter(|u| *u != 'J')
                                        .map(|u| s.chars()
                                                        .filter(|c| u==*c)
                                                        .count())
                                        .collect();

        card_counts.sort_by(|a,b| b.cmp(a));
        let card_counts = card_counts;

        let card_1_count = *card_counts.get(0).unwrap_or(&0);
        let card_2_count = *card_counts.get(1).unwrap_or(&0);

        if card_1_count == 5 - joker_count {
            return Ok(Self::FiveOfAKind);
        }

        if card_1_count == 4 - joker_count {
            return Ok(Self::FourOfAKind);
        }

        if card_1_count == 3 - joker_count {
            if card_2_count == 2 {
                return Ok(Self::FullHouse);
            }
            else {
                return Ok(Self::ThreeOfAKind);
            }
        }

        if card_1_count == 2 - joker_count {
            if card_2_count == 2 {
                return Ok(Self::TwoPair);
            }
            else {
                return Ok(Self::OnePair);
            }
        }

        return Ok(Self::HighCard);
    }
    
}

#[derive(Debug, Eq, Ord)]
struct HandWithJoker {
    _cards: String,
    bet: i64,
    hand_type: HandTypeWithJoker,
    score: i64
}

impl FromStr for HandWithJoker {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((cards, bet)) = s.split_once(" ") else {
            return Err("Failed to parse hand");
        };

        let card_vals = HashMap::from([
            ('A',14),
            ('K',13),
            ('Q',12),
            ('T',11),
            ('9',10),
            ('8',9),
            ('7',8),
            ('6',7),
            ('5',6),
            ('4',5),
            ('3',4),
            ('2',3),
            ('1',2),
            ('0',1),
            ('J',0)
        ]);

        let mut score = 0;
        let card_count = cards.chars().count();
        for (i, c) in cards.chars().enumerate() {
            let power: u32 = (card_count - i) as u32;
            let index_val = card_vals.iter().count().pow(power) as i64;
            let card_val = *card_vals.get(&c).unwrap() as i64;
            score += index_val * card_val;
        }

        Ok(Self {
            _cards: cards.to_string(),
            bet: bet.parse().unwrap(),
            hand_type: HandTypeWithJoker::from_str(cards).unwrap(),
            score, 
        })
    }
}

impl PartialEq for HandWithJoker {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.bet == other.bet
    }
}

impl PartialOrd for HandWithJoker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.hand_type != other.hand_type {
            self.hand_type.partial_cmp(&other.hand_type)
        }
        else { 
            self.score.partial_cmp(&other.score)
        }
    }
}

pub fn part1() -> i64 {
    let mut hands:Vec<_> = INPUT.lines()
                        .map(|l| Hand::from_str(l).unwrap())
                        .collect();

    hands.sort();

    hands.iter().enumerate().map(|(i, h)| (i+1) as i64 * h.bet).sum()
}

pub fn part2() -> i64 {
    let mut hands:Vec<_> = INPUT.lines()
                        .map(|l| HandWithJoker::from_str(l).unwrap())
                        .collect();

    hands.sort();

    hands.iter().enumerate().map(|(i, h)| (i+1) as i64 * h.bet).sum()
}