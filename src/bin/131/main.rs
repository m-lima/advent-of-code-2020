const TIME: usize = 1014511;
const IDS: &[usize] = &[17, 41, 643, 23, 13, 29, 433, 37, 19];

fn main() {
    let answer = IDS
        .iter()
        .map(|id| (*id, ((TIME + id) / id) * id - TIME))
        .fold(
            (0, usize::MAX),
            |acc, curr| if curr.1 < acc.1 { curr } else { acc },
        );
    println!("{}", answer.0 * answer.1);
}
