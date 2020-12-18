use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

struct Mask {
    set: u64,
    float: Vec<u64>,
    float_mask: u64,
}

impl Mask {
    fn new() -> Self {
        Self {
            set: 0,
            float: vec![0],
            float_mask: 0,
        }
    }

    fn parse(line: &str) -> Self {
        line.split_at(7)
            .1
            .chars()
            .enumerate()
            .fold(Mask::new(), |mut acc, curr| {
                acc.set <<= 1;
                match curr.1 {
                    '0' => {}
                    '1' => acc.set |= 1,
                    'X' => {
                        let bit = 2_u64.pow(35 - curr.0 as u32);
                        acc.float_mask |= bit;
                        acc.float = acc
                            .float
                            .into_iter()
                            .flat_map(|v| vec![v, v | bit])
                            .collect();
                    }
                    _ => unreachable!(),
                }
                acc
            })
    }

    fn apply(&self, addr: u64) -> Vec<u64> {
        let base = (addr | self.set) & !self.float_mask;
        self.float.iter().map(|v| *v | base).collect()
    }
}

struct Assign {
    addr: Vec<u64>,
    value: u64,
}

impl Assign {
    fn parse(line: &str, mask: &Mask) -> Self {
        lazy_static! {
            static ref PATTERN: Regex = Regex::new(r"mem\[([0-9]+)] = ([0-9]+)").unwrap();
        }

        let captures = PATTERN.captures_iter(line).next().unwrap();
        Self {
            addr: mask.apply(captures.get(1).unwrap().as_str().parse().unwrap()),
            value: captures.get(2).unwrap().as_str().parse().unwrap(),
        }
    }
}

fn main() {
    let sum: u64 = INPUT
        .split('\n')
        .filter(|line| !line.is_empty())
        .scan(Mask::new(), |state, curr| {
            if curr.starts_with("mask = ") {
                *state = Mask::parse(curr);
                Some(vec![])
            } else {
                let assign = Assign::parse(curr, state);
                Some(
                    assign
                        .addr
                        .iter()
                        .map(|a| (*a, assign.value))
                        .collect::<Vec<_>>(),
                )
            }
        })
        .flatten()
        .collect::<HashMap<_, _>>()
        .values()
        .sum();

    println!("{}", sum);
}
