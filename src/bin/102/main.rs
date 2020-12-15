const INPUT: [usize; 107] = include!("input.txt");

fn main() {
    let mut sorted = INPUT.iter().map(Clone::clone).collect::<Vec<_>>();
    sorted.sort();
    sorted.push(*sorted.last().unwrap() + 3);

    let combinations: usize = sorted
        .iter()
        .scan((0, None), |state, curr| {
            let sequence_end = curr - state.0 == 3;
            state.0 = *curr;

            if sequence_end {
                let count = state
                    .1
                    .filter(|count| count > &0)
                    .map(|count: usize| 1 + (count.pow(2) + count) / 2);
                state.1 = None;
                Some(count)
            } else {
                state.1 = state.1.map(|count| count + 1).or(Some(0));
                Some(None)
            }
        })
        .flat_map(|v| v)
        .product();

    println!("{:?}", combinations);
}
