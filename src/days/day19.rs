use std::{collections::HashMap, str::FromStr};

const INPUT: &str = include_str!("input/day19_example.txt");

struct Rule {
    prop: String,
    operator: char,
    value: i64,
    dest: String,
}

impl Rule {
    fn eval(&self, input: &Item) -> Option<String> {
        let Some(input_value) = input.props.get(&self.prop) else {
            return None;
        };

        match self.operator {
            '>' => {
                if *input_value > self.value {
                    Some(self.dest.clone())
                } else {
                    None
                }
            }
            '<' => {
                if *input_value < self.value {
                    Some(self.dest.clone())
                } else {
                    None
                }
            }
            _ => panic!("Invalid oprator in rule"),
        }
    }

    fn eval_range(
        &self,
        item_range: &ItemRange,
    ) -> (Option<ItemRange>, Option<(String, ItemRange)>) {
        let min_val = item_range.prop_mins[&self.prop];
        let max_val = item_range.prop_maxs[&self.prop];

        match self.operator {
            '>' => {
                if min_val > self.value {
                    (None, Some((self.dest.clone(), item_range.clone())))
                } else if max_val > self.value {
                    let mut mapped = item_range.clone();
                    let mut unmapped = item_range.clone();

                    *unmapped.prop_maxs.get_mut(&self.prop).unwrap() = self.value;
                    *mapped.prop_mins.get_mut(&self.prop).unwrap() = self.value + 1;

                    (Some(unmapped), Some((self.dest.clone(), mapped)))
                } else {
                    (Some(item_range.clone()), None)
                }
            }
            '<' => {
                if max_val < self.value {
                    (None, Some((self.dest.clone(), item_range.clone())))
                } else if min_val < self.value {
                    let mut mapped = item_range.clone();
                    let mut unmapped = item_range.clone();

                    *unmapped.prop_mins.get_mut(&self.prop).unwrap() = self.value;
                    *mapped.prop_maxs.get_mut(&self.prop).unwrap() = self.value - 1;

                    (Some(unmapped), Some((self.dest.clone(), mapped)))
                } else {
                    (Some(item_range.clone()), None)
                }
            }
            _ => panic!("Invalid oprator in rule"),
        }
    }
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let operator = if s.contains('<') {
            '<'
        } else if s.contains('>') {
            '>'
        } else {
            return Err(format!("Failed to find operator for {}", s));
        };

        let Some((prop, rhs)) = s.split_once(operator) else {
            return Err(format!("Failed to find property for rule {}", s));
        };

        let Some((val, dest)) = rhs.split_once(':') else {
            return Err(format!("Failed to find value/destination for rule {}", s));
        };

        Ok(Rule {
            prop: prop.to_string(),
            operator: operator,
            value: val.parse::<i64>().unwrap(),
            dest: dest.to_string(),
        })
    }
}

struct Workflow {
    name: String,
    rule_map: Vec<Rule>,
    fallthrough: String,
}

impl Workflow {
    fn apply(&self, item: &Item) -> String {
        for rule in &self.rule_map {
            let Some(dest) = rule.eval(item) else {
                continue;
            };

            return dest;
        }

        self.fallthrough.clone()
    }

    fn apply_range(&self, item_range: &ItemRange) -> Vec<(String, ItemRange)> {
        let mut unmapped = item_range.clone();

        let mut mappings = Vec::new();

        for rule in &self.rule_map {
            let (new_unmapped, mapping) = rule.eval_range(&unmapped);

            if let Some(new_unmapped) = new_unmapped {
                unmapped = new_unmapped;
            }

            if let Some(mapping) = mapping {
                mappings.push(mapping);
            }
        }

        return mappings;
    }
}

impl FromStr for Workflow {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((name, rhs)) = s.split_once('{') else {
            return Err(format!("Failed to find name in {}", s));
        };

        let rhs = rhs.replace('}', "");

        let mut rules = rhs.split(',');
        let Some(fallthrough) = rules.next_back() else {
            return Err(format!("Failed to find fallthrough value for {}", s));
        };

        let rule_map = rules.filter_map(|r_s| Rule::from_str(r_s).ok()).collect();

        Ok(Self {
            name: name.to_string(),
            rule_map: rule_map,
            fallthrough: fallthrough.to_string(),
        })
    }
}

#[derive(Clone)]
struct Item {
    props: HashMap<String, i64>,
}

impl FromStr for Item {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = &s[1..s.len() - 1];

        let props: Vec<_> = s
            .split(',')
            .map(|prop_val_pair| {
                let Some((prop, val)) = prop_val_pair.split_once('=') else {
                    return Err(format!(
                        "Failed to find property value pair in {}",
                        prop_val_pair
                    ));
                };

                let Ok(val) = val.parse::<i64>() else {
                    return Err(format!("Failed to parse value as i64 in {}", prop_val_pair));
                };

                Ok((prop.to_string(), val))
            })
            .collect();

        if let Some(Err(e)) = props.iter().find(|p| p.is_err()) {
            return Err(e.to_string());
        }

        let props = props.into_iter().map(|p| p.unwrap()).collect();

        Ok(Self { props })
    }
}

#[derive(Clone)]
struct ItemRange {
    prop_mins: HashMap<String, i64>,
    prop_maxs: HashMap<String, i64>,
}

impl ItemRange {
    fn start_range() -> Self {
        Self { 
            prop_mins: HashMap::from([
                ("x".to_string(), 1),
                ("m".to_string(), 1),
                ("a".to_string(), 1),
                ("s".to_string(), 1),
            ]), 
            prop_maxs: HashMap::from([
                ("x".to_string(), 4000),
                ("m".to_string(), 4000),
                ("a".to_string(), 4000),
                ("s".to_string(), 4000),
            ]), 
        }
    }
}

pub fn part1() -> i64 {
    let Some((workflows, items)) = INPUT.split_once("\r\n\r\n") else {
        panic!("Failed to find sections");
    };

    let workflows: Vec<_> = workflows
        .lines()
        .filter_map(|l| Workflow::from_str(l).ok())
        .collect();

    let mut bins = vec![Vec::<Item>::new(); workflows.len()];

    let bin_map: HashMap<&str, usize> = workflows
        .iter()
        .enumerate()
        .map(|(i, w)| (w.name.as_str(), i))
        .collect();

    let start_bin_i = *bin_map.get("in").unwrap();

    for item in items.lines().filter_map(|l| Item::from_str(l).ok()) {
        bins[start_bin_i].push(item);
    }

    let mut accepted = Vec::new();

    while let Some(wf_i) = bins.iter_mut().enumerate().position(|(_, b)| !b.is_empty()) {
        let Some(item) = bins[wf_i].pop() else {
            panic!("No item found");
        };
        let wf = &workflows[wf_i];

        let dest = wf.apply(&item);

        if dest == "R" {
            continue;
        } else if dest == "A" {
            accepted.push(item);
        } else {
            let dest_i = bin_map[dest.as_str()];

            bins[dest_i].push(item);
        }
    }

    accepted
        .iter()
        .map(|i| i.props.values().sum::<i64>())
        .sum::<i64>()
}

pub fn part2() -> i64 {
    let Some((workflows, _)) = INPUT.split_once("\r\n\r\n") else {
        panic!("Failed to find sections");
    };

    let workflows: Vec<_> = workflows
        .lines()
        .filter_map(|l| Workflow::from_str(l).ok())
        .collect();

    let mut bins = vec![Vec::<ItemRange>::new(); workflows.len()];

    let bin_map: HashMap<&str, usize> = workflows
        .iter()
        .enumerate()
        .map(|(i, w)| (w.name.as_str(), i))
        .collect();

    let start_bin_i = *bin_map.get("in").unwrap();

    bins[start_bin_i].push(ItemRange::start_range());

    let mut accepted = Vec::new();

    while let Some(wf_i) = bins.iter_mut().enumerate().position(|(_, b)| !b.is_empty()) {
        let Some(item_range) = bins[wf_i].pop() else {
            panic!("No item found");
        };
        let wf = &workflows[wf_i];

        for (dest, range) in wf.apply_range(&item_range) {
            if dest == "R" {
                continue;
            } else if dest == "A" {
                accepted.push(range);
            } else {
                let dest_i = bin_map[dest.as_str()];
                
                bins[dest_i].push(range);
            }
        }
    }

    accepted.iter().map(|range| {
        let min_x = range.prop_mins["x"];
        let max_x = range.prop_maxs["x"];
        let min_m = range.prop_mins["m"];
        let max_m = range.prop_maxs["m"];
        let min_a = range.prop_mins["a"];
        let max_a = range.prop_maxs["a"];
        let min_s = range.prop_mins["s"];
        let max_s = range.prop_maxs["s"];

        let x_range = max_x - min_x + 1;
        let m_range = max_m - min_m + 1;
        let a_range = max_a - min_a + 1;
        let s_range = max_s - min_s + 1;

        x_range * m_range * a_range * s_range
    })
    .sum::<i64>()
}
