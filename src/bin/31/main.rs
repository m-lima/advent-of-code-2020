const INPUT: &str = include_str!("input.txt");

fn main() {
    let tree_count = INPUT
        .split('\n')
        .filter(|line| !line.is_empty())
        .scan(0, |state, curr| {
            let tree_hit = curr.chars().nth(*state).unwrap() == '#';
            *state = (*state + 3) % 31;
            Some(tree_hit)
        })
        .filter(|x| *x)
        .count();

    println!("{}", tree_count);
}
