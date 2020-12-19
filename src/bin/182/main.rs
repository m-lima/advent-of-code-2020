use pest::iterators::{Pair, Pairs};
use pest::prec_climber::{Assoc, Operator, PrecClimber};
use pest::Parser;

const INPUT: &str = include_str!("input.txt");

#[derive(pest_derive::Parser)]
#[grammar = "bin/182/syntax.peg"]
struct PestCalculator;

fn calc_climber() -> PrecClimber<Rule> {
    PrecClimber::new(vec![
        Operator::new(Rule::multiply, Assoc::Left),
        Operator::new(Rule::add, Assoc::Left),
    ])
}

fn eval(expression: Pairs<Rule>) -> usize {
    calc_climber().climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::number => pair.as_str().trim().parse().unwrap(),
            Rule::expression => eval(pair.into_inner()),
            _ => unreachable!(),
        },
        |lhs: usize, op: Pair<Rule>, rhs: usize| match op.as_rule() {
            Rule::add => lhs + rhs,
            Rule::multiply => lhs * rhs,
            _ => unreachable!(),
        },
    )
}

fn main() {
    let answer: usize = INPUT
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            PestCalculator::parse(Rule::expression, line)
                .map(eval)
                .unwrap()
        })
        .sum();

    println!("{}", answer);
}
