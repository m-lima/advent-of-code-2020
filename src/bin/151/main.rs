const INPUT: &[usize] = &[13, 0, 10, 12, 1, 5, 8];

fn main() {
    let mut stack = INPUT.to_vec();
    for index in stack.len() - 1..2019 {
        let reference = stack[index];
        stack.push(
            stack
                .iter()
                .take(stack.len() - 1)
                .rposition(|v| *v == reference)
                .map_or(0, |i| index - i),
        );
    }
    println!("{}", stack[2019]);
}
