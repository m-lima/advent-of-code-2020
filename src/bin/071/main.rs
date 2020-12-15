use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

struct Rules {
    color_to_index: HashMap<String, usize>,
    my_bag_index: usize,
    allowed_bags: Vec<Vec<usize>>,
    can_use: Vec<Option<bool>>,
}

impl Rules {
    fn new(known_bags: Vec<String>) -> Self {
        let color_to_index: HashMap<String, usize> = known_bags
            .iter()
            .enumerate()
            .map(|(index, value)| (value.clone(), index))
            .collect();

        let my_bag_index = *color_to_index.get("shiny gold").unwrap();

        Self {
            color_to_index,
            my_bag_index,
            allowed_bags: Vec::with_capacity(known_bags.len()),
            can_use: Vec::with_capacity(known_bags.len()),
        }
    }
}

fn fold_into_dataset(mut rules: Rules, line: &str) -> Rules {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"[0-9]+ (\w+ \w+) bags?").unwrap();
    }

    let captures = PATTERN.captures_iter(line);

    let allowed = captures
        .filter_map(|capture| capture.get(1).map(|group| String::from(group.as_str())))
        .map(|color| *rules.color_to_index.get(&color).unwrap())
        .collect::<Vec<_>>();

    let can_use = if allowed.is_empty() {
        Some(false)
    } else if allowed.contains(&rules.my_bag_index) {
        Some(true)
    } else {
        None
    };

    rules.allowed_bags.push(allowed);
    rules.can_use.push(can_use);

    rules
}

fn is_bag_allowed(bag_index: usize, rules: &mut Rules) -> bool {
    if let Some(can_use) = rules.can_use[bag_index] {
        can_use
    } else if rules.allowed_bags[bag_index].contains(&rules.my_bag_index) {
        true
    } else {
        let indices = rules.allowed_bags[bag_index].clone();

        let allowed = indices
            .iter()
            .map(|allowed_bad_index| is_bag_allowed(*allowed_bad_index, rules))
            .find(|v| *v)
            .is_some();

        rules.can_use[bag_index] = Some(allowed);
        allowed
    }
}

fn main() {
    let mut rules = {
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

    for bag in 0..rules.allowed_bags.len() {
        is_bag_allowed(bag, &mut rules);
    }

    let count = rules
        .color_to_index
        .iter()
        .filter_map(|(color, index)| {
            if rules.can_use[*index].unwrap() {
                Some(color)
            } else {
                None
            }
        })
        .count();

    println!("{}", count);
}
