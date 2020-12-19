use pest::Parser;

const INPUT: &str = include_str!("messages.txt");

#[derive(pest_derive::Parser)]
#[grammar = "bin/191/rules.peg"]
struct Validator;

fn main() {
    let answer: usize = INPUT
        .split('\n')
        .filter(|line| !line.is_empty())
        .filter_map(|line| Validator::parse(Rule::main, line).ok())
        .count();

    println!("{}", answer);
}
