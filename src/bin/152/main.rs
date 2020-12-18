use std::collections::HashMap;

const INPUT: &[usize] = &[13, 0, 10, 12, 1, 5, 8];

fn main() {
    let mut spoken: HashMap<usize, usize> = INPUT[..INPUT.len() - 1]
        .iter()
        .enumerate()
        .map(|v| (*v.1, v.0))
        .collect();

    let mut last = *INPUT.last().unwrap();
    for index in INPUT.len()..30000000 {
        let new = spoken.get(&last).map_or(0, |v| index - *v - 1);
        spoken.insert(last, index - 1);
        last = new;
    }

    println!("{}", last);
}
