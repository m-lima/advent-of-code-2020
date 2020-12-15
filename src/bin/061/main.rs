use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let count: usize = INPUT
        .split("\n\n")
        .map(|line| {
            line.split('\n')
                .map(|string| string.chars().collect::<HashSet<_>>())
                .fold(HashSet::new(), |mut acc, curr| {
                    acc.extend(curr);
                    acc
                })
        })
        .map(|set| set.len())
        .sum();

    println!("{}", count);
}
