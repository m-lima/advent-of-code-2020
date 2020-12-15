const INPUT: &str = include_str!("input.txt");

fn calculate_slope((x, y): &(usize, usize)) -> usize {
    INPUT
        .split('\n')
        .filter(|line| !line.is_empty())
        .step_by(*y)
        .scan(0, |state, curr| {
            let tree_hit = curr.chars().nth(*state).unwrap() == '#';
            *state = (*state + x) % 31;
            Some(tree_hit)
        })
        .filter(|hit| *hit)
        .count()
}

fn main() {
    let total: usize = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(calculate_slope)
        .product();
    println!("{}", total);
}
