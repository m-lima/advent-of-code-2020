const INPUT: [usize; 107] = include!("input.txt");

fn main() {
    let mut sorted = INPUT.iter().map(Clone::clone).collect::<Vec<_>>();
    sorted.sort();

    let differences = sorted
        .iter()
        .scan(0, |state, curr| {
            let diff = curr - *state;
            *state = *curr;
            Some(diff)
        })
        .collect::<Vec<_>>();

    let one_jolt = differences.iter().filter(|v| v == &&1).count();
    let three_jolts = differences.iter().filter(|v| v == &&3).count() + 1;
    println!("{}", one_jolt * three_jolts);
}
