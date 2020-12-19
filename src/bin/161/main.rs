const INPUT: &str = include_str!("input.txt");

fn to_lines(string: Option<&str>) -> impl std::iter::Iterator<Item = &str> {
    string.unwrap().split('\n').filter(|line| !line.is_empty())
}

fn to_pair<T>(acc: (Option<T>, Option<T>), curr: T) -> (Option<T>, Option<T>) {
    if acc.0.is_none() {
        (Some(curr), None)
    } else if acc.1.is_none() {
        (acc.0, Some(curr))
    } else {
        unreachable!();
    }
}

struct Range(
    std::ops::RangeInclusive<usize>,
    std::ops::RangeInclusive<usize>,
);

impl Range {
    fn parse(string: &str) -> Self {
        let range = string
            .trim()
            .split(" or ")
            .map(|range| {
                let bounds = range
                    .split('-')
                    .map(|bound| bound.parse::<usize>().unwrap())
                    .fold((None, None), to_pair);
                bounds.0.unwrap()..=bounds.1.unwrap()
            })
            .fold((None, None), to_pair);
        Range(range.0.unwrap(), range.1.unwrap())
    }

    fn valid(&self, value: usize) -> bool {
        self.0.contains(&value) || self.1.contains(&value)
    }
}

#[derive(Default)]
struct Fields(Vec<Range>);

struct Ticket(Vec<usize>);

impl Ticket {
    fn parse(line: &str) -> Self {
        Self(
            line.split(',')
                .map(|value| value.trim().parse().unwrap())
                .collect(),
        )
    }

    fn invalid_for(&self, fields: &Fields) -> Option<usize> {
        self.0
            .iter()
            .find(|value| fields.0.iter().find(|range| range.valid(**value)).is_none())
            .map(Clone::clone)
    }
}

fn main() {
    let mut parser = INPUT.split("\n\n").filter(|line| !line.is_empty());

    let fields = to_lines(parser.next()).fold(Fields::default(), |mut acc, curr| {
        acc.0.push(Range::parse(curr.split(':').nth(1).unwrap()));
        acc
    });

    let answer: usize = to_lines(parser.nth(1))
        .skip(1)
        .map(Ticket::parse)
        .filter_map(|ticket| ticket.invalid_for(&fields))
        .sum();

    println!("{}", answer);
}
