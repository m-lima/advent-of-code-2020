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

struct Field {
    name: String,
    range: Range,
}

#[derive(Default)]
struct Fields {
    fields: Vec<Field>,
    possible_indices: Vec<Vec<usize>>,
}

impl Fields {
    fn new(fields: Vec<Field>, tickets: &[Ticket]) -> Self {
        let possible_indices = fields
            .iter()
            .map(|field| {
                (0..fields.len())
                    .filter_map(|index| {
                        if tickets
                            .iter()
                            .map(|ticket| ticket.0[index])
                            .find(|value| !field.range.valid(*value))
                            .is_some()
                        {
                            None
                        } else {
                            Some(index)
                        }
                    })
                    .collect()
            })
            .collect();
        Self {
            fields,
            possible_indices,
        }
    }

    fn hone_uncertainty(&mut self) -> bool {
        let mut uncertain = false;

        for index in 0..self.possible_indices.len() {
            if self.possible_indices[index].len() == 0 {
                panic!("Completely invalid field!");
            }
            if self.possible_indices[index].len() == 1 {
                let unique = self.possible_indices[index][0];
                self.possible_indices
                    .iter_mut()
                    .enumerate()
                    .filter_map(|(i, v)| if i == index { None } else { Some(v) })
                    .for_each(|indices| indices.retain(|i| *i != unique));
            } else {
                uncertain = true;
            }
        }
        uncertain
    }
}

struct Ticket(Vec<usize>);

impl Ticket {
    fn parse(line: &str) -> Self {
        Self(
            line.split(',')
                .map(|value| value.trim().parse().unwrap())
                .collect(),
        )
    }

    fn invalid_for(&self, fields: &[Field]) -> bool {
        self.0
            .iter()
            .find(|value| {
                fields
                    .iter()
                    .find(|field| field.range.valid(**value))
                    .is_none()
            })
            .is_some()
    }
}

fn main() {
    let mut parser = INPUT.split("\n\n").filter(|line| !line.is_empty());

    let raw_fields = to_lines(parser.next()).fold(Vec::new(), |mut acc, curr| {
        let (field, value) = curr.split(':').fold((None, None), to_pair);
        acc.push(Field {
            name: field.unwrap().to_owned(),
            range: Range::parse(value.unwrap()),
        });

        acc
    });

    let my_ticket = Ticket::parse(to_lines(parser.next()).nth(1).unwrap());
    let tickets: Vec<_> = to_lines(parser.next())
        .skip(1)
        .map(Ticket::parse)
        .filter(|ticket| !ticket.invalid_for(&raw_fields))
        .collect();

    let mut fields = Fields::new(raw_fields, &tickets);

    while fields.hone_uncertainty() {}

    let answer: usize = fields
        .fields
        .iter()
        .enumerate()
        .filter_map(|(index, field)| {
            if field.name.starts_with("departure") {
                Some(my_ticket.0[fields.possible_indices[index][0]])
            } else {
                None
            }
        })
        .product();

    println!("{}", answer);
}
