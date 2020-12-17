fn prepare_111() {
    use std::io::{BufWriter, Write};

    const OUTPUT: &str = "src/bin/111/input.rs";
    const INPUT: &str = include_str!("src/bin/111/input.txt");
    println!("cargo:rerun-if-changed={}", INPUT);

    let input: Vec<_> = INPUT
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let mut output = std::fs::File::create(OUTPUT).unwrap();
    output
        .write_all(
            format!(
                "pub const INPUT: [[char; {}]; {}] = [",
                input[0].len(),
                input.len()
            )
            .as_bytes(),
        )
        .unwrap();

    for line in input {
        output.write_all(format!("{:?},", line).as_bytes()).unwrap();
    }

    output.write_all(b"];").unwrap();
}

fn prepare_112() {
    use std::io::{BufWriter, Write};

    const OUTPUT: &str = "src/bin/112/input.rs";
    const INPUT: &str = include_str!("src/bin/112/input.txt");
    println!("cargo:rerun-if-changed={}", INPUT);

    let input: Vec<_> = INPUT
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let mut output = std::fs::File::create(OUTPUT).unwrap();
    output
        .write_all(
            format!(
                "pub const INPUT: [[char; {}]; {}] = [",
                input[0].len(),
                input.len()
            )
            .as_bytes(),
        )
        .unwrap();

    for line in input {
        output.write_all(format!("{:?},", line).as_bytes()).unwrap();
    }

    output.write_all(b"];").unwrap();
}

fn main() {
    prepare_111();
    prepare_112();
}
