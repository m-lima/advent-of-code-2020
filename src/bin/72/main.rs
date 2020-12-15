use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

struct Rules {
    color_to_index: HashMap<String, usize>,
    allowed_bags: Vec<Vec<(usize, usize)>>,
}

impl Rules {
    fn new(known_bags: Vec<String>) -> Self {
        let color_to_index: HashMap<String, usize> = known_bags
            .iter()
            .enumerate()
            .map(|(index, value)| (value.clone(), index))
            .collect();

        Self {
            color_to_index,
            allowed_bags: Vec::with_capacity(known_bags.len()),
        }
    }
}

fn fold_into_dataset(mut rules: Rules, line: &str) -> Rules {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"([0-9]+) (\w+ \w+) bags?").unwrap();
    }

    let captures = PATTERN.captures_iter(line);

    let allowed = captures
        .filter_map(|capture| {
            capture
                .get(1)
                .and_then(|count| count.as_str().parse::<usize>().ok())
                .and_then(|count| {
                    capture
                        .get(2)
                        .map(|color| (count, String::from(color.as_str())))
                })
        })
        .map(|(count, color)| (count, *rules.color_to_index.get(&color).unwrap()))
        .collect::<Vec<_>>();

    rules.allowed_bags.push(allowed);
    rules
}

fn bag_count(bag: usize, rules: &Rules) -> usize {
    rules.allowed_bags[bag]
        .iter()
        .map(|allowed_bag| allowed_bag.0 + allowed_bag.0 * bag_count(allowed_bag.1, rules))
        .sum()
}

fn main() {
    let rules = {
        let known_bags = INPUT
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut name = line.split_whitespace().take(2);
                format!("{} {}", name.next().unwrap(), name.next().unwrap())
            })
            .collect();

        INPUT
            .split('\n')
            .filter(|line| !line.is_empty())
            .fold(Rules::new(known_bags), fold_into_dataset)
    };

    let count = bag_count(*rules.color_to_index.get("shiny gold").unwrap(), &rules);
    println!("{}", count);
}
