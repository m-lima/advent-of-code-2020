use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

#[derive(Eq, PartialEq, Copy, Clone)]
enum Value {
    Literal(char),
    Reference(usize),
}

struct Rule(Vec<Vec<Value>>);

impl Rule {
    fn stringify_value(value: &Value, rules: &HashMap<usize, Self>, depth: u8) -> String {
        match value {
            Value::Literal(c) => String::from(*c),
            Value::Reference(r) => Self::stringify(rules.get(&r).unwrap(), &rules, depth),
        }
    }

    fn stringify_group(values: &[Value], rules: &HashMap<usize, Self>, depth: u8) -> String {
        values
            .iter()
            .map(|v| Self::stringify_value(v, rules, depth))
            .collect()
    }

    fn stringify(&self, rules: &HashMap<usize, Self>, depth: u8) -> String {
        let first = self.0.first().unwrap();

        let rgx = self
            .0
            .iter()
            .skip(1)
            .fold(Self::stringify_group(first, rules, depth), |acc, curr| {
                format!("(?:{}|{})", acc, Self::stringify_group(curr, rules, depth))
            });

        if self.0 == rules.get(&8).unwrap().0 {
            format!("{}+", rgx)
        } else if self.0 == rules.get(&11).unwrap().0 {
            (0..depth).fold(format!("{}", rgx), |acc, _| {
                format!(
                    "{}(?:{})?{}",
                    rules.get(&42).unwrap().stringify(rules, depth),
                    acc,
                    rules.get(&31).unwrap().stringify(rules, depth),
                )
            })
        } else {
            rgx
        }
    }
}

fn main() {
    let rules: HashMap<usize, Rule> = INPUT
        .split('\n')
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split(':');
            let key = parts.next().unwrap().trim().parse::<usize>().unwrap();
            let value = parts
                .next()
                .unwrap()
                .trim()
                .split('|')
                .map(|p| {
                    p.split_whitespace()
                        .map(|s| {
                            if s.starts_with('"') {
                                Value::Literal(s.chars().nth(1).unwrap())
                            } else {
                                Value::Reference(s.parse().unwrap())
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            (key, Rule(value))
        })
        .collect();

    let mut answer = 0_usize;
    let mut depth = 0;

    loop {
        let rgx = regex::Regex::new(
            format!("^{}$", rules.get(&0).unwrap().stringify(&rules, depth)).as_str(),
        )
        .unwrap();

        let count = INPUT
            .split('\n')
            .skip_while(|line| !line.is_empty())
            .filter(|line| !line.is_empty())
            .filter_map(|line| if rgx.is_match(line) { Some(line) } else { None })
            .count();
        if answer == count {
            break;
        } else {
            answer = count;
            depth += 1;
        }
    }

    println!("{}", answer);
}
