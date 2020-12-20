use pest::Parser;

const INPUT: &str = include_str!("input.txt");

#[derive(pest_derive::Parser)]
#[grammar = "bin/192/rules.peg"]
struct Validator;

fn main() {
    let answer: usize = INPUT
        .split('\n')
        .skip_while(|line| !line.is_empty())
        .filter(|line| !line.is_empty())
        .filter_map(|line| Validator::parse(Rule::main, line).ok())
        .count();

    println!("{}", answer);
}
