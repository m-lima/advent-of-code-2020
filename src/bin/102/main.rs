const INPUT: [usize; 107] = include!("input.txt");

fn main() {
    let mut sorted = INPUT.iter().map(Clone::clone).collect::<Vec<_>>();
    sorted.sort();
    sorted.push(*sorted.last().unwrap() + 3);

    let combinations = |len: Option<usize>| len.filter(|l| l > &0).map(|n| 1 + (n.pow(2) + n) / 2);

    let combinations: usize = sorted
        .iter()
        .scan((0, None), |(ref mut prev, ref mut count), curr| {
            let sequence_end = curr - *prev != 1;
            *prev = *curr;

            if sequence_end {
                let mut combinations = None;
                std::mem::swap(&mut combinations, count);
                Some(combinations)
            } else {
                *count = count.map(|v| v + 1).or(Some(0));
                Some(None)
            }
        })
        .flat_map(combinations)
        .product();

    println!("{:?}", combinations);
}
