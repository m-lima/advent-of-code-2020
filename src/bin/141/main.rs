use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

#[derive(Default)]
struct Mask {
    set: u64,
    unset: u64,
}

impl std::ops::ShlAssign<u32> for Mask {
    fn shl_assign(&mut self, rhs: u32) {
        self.set <<= rhs;
        self.unset <<= rhs;
    }
}

impl Mask {
    fn parse(line: &str) -> Self {
        line.split_at(7)
            .1
            .chars()
            .fold(Mask::default(), |mut acc, curr| {
                acc <<= 1;
                match curr {
                    '0' => acc.unset |= 1,
                    '1' => acc.set |= 1,
                    'X' => {}
                    _ => unreachable!(),
                }
                acc
            })
    }

    fn apply(&self, value: u64) -> u64 {
        (value | self.set) & (!self.unset)
    }
}

struct Assign {
    address: u64,
    value: u64,
}

impl Assign {
    fn parse(line: &str, mask: &Mask) -> Self {
        lazy_static! {
            static ref PATTERN: Regex = Regex::new(r"mem\[([0-9]+)] = ([0-9]+)").unwrap();
        }

        let captures = PATTERN.captures_iter(line).next().unwrap();
        Self {
            address: captures.get(1).unwrap().as_str().parse().unwrap(),
            value: mask.apply(captures.get(2).unwrap().as_str().parse().unwrap()),
        }
    }
}

fn main() {
    let sum: u64 = INPUT
        .split('\n')
        .filter(|line| !line.is_empty())
        .scan(Mask::default(), |state, curr| {
            if curr.starts_with("mask = ") {
                *state = Mask::parse(curr);
                Some(None)
            } else {
                Some(Some(Assign::parse(curr, state)))
            }
        })
        .filter_map(|v| v.map(|a| (a.address, a.value)))
        .collect::<HashMap<_, _>>()
        .values()
        .sum();

    println!("{}", sum);
}
