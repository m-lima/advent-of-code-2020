const INPUT: &str = include_str!("input.txt");

enum Operation {
    Add,
    Multiply,
}

fn calculate(operation: Option<Operation>, left: usize, right: usize) -> usize {
    if let Some(operation) = operation {
        match operation {
            Operation::Add => left + right,
            Operation::Multiply => left * right,
        }
    } else {
        right
    }
}

fn compute(reader: &mut std::str::SplitWhitespace<'_>) -> usize {
    let mut output = 0;
    let mut operation = None;

    while let Some(c) = reader.next() {
        match c {
            ")" => return output,
            "+" => operation = Some(Operation::Add),
            "*" => operation = Some(Operation::Multiply),
            "(" => output = calculate(operation.take(), output, compute(reader)),
            n => output = calculate(operation.take(), output, n.parse().unwrap()),
        }
    }

    output
}

fn prepare_line(line: &str) -> String {
    line.chars()
        .map(|c| {
            if c.is_ascii_digit() {
                c.to_string()
            } else {
                format!(" {} ", c)
            }
        })
        .collect()
}

fn main() {
    let answer: usize = INPUT
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(prepare_line)
        .map(|calc| compute(&mut calc.split_whitespace()))
        .sum();

    println!("{}", answer);
}
