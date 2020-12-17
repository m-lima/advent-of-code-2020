const IDS: &str = "17,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,643,x,x,x,x,x,x,x,23,x,x,x,x,13,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,29,x,433,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,x,x,19";

fn main() {
    let mut answer: Vec<_> = IDS
        .split(',')
        .enumerate()
        .filter_map(|(index, string)| string.parse::<usize>().ok().map(|v| (v, (index % v))))
        .collect();
    answer.sort();
    answer.reverse();

    let out = answer
        .iter()
        .scan((0, 0), |state, curr| {
            if *state == (0, 0) {
                *state = *curr;
            } else {
                while state.1 % curr.0 != curr.1 {
                    state.1 += state.0;
                }
                state.0 *= curr.0;
            }

            Some(*state)
        })
        .last()
        .unwrap();

    println!("{}", out.0 - out.1);
}
