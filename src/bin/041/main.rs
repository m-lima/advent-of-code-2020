const INPUT: &str = include_str!("input.txt");

const JIG: [&str; 7] = ["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"];

fn main() {
    let count = INPUT
        .split("\n\n")
        .map(|line| line.replace('\n', " "))
        .map(|line| {
            let mut fields = line
                .split_whitespace()
                .filter(|field| !field.starts_with("cid"))
                .map(|item| item.chars().take(3).collect::<String>())
                .collect::<Vec<_>>();
            fields.sort();
            fields
        })
        .filter(|fields| fields.as_slice() == JIG.as_ref())
        .count();

    println!("{}", count);
}
