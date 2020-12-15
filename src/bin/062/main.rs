use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let full_set = ('a'..='z').collect::<HashSet<_>>();

    let count: usize = INPUT
        .split("\n\n")
        .map(|line| {
            line.split('\n')
                .map(|string| string.chars().collect::<HashSet<_>>())
                .fold(full_set.clone(), |acc, curr| {
                    acc.intersection(&curr).map(Clone::clone).collect()
                })
        })
        .map(|set| set.len())
        .sum();

    println!("{}", count);
}
