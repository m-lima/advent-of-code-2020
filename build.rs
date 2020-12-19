fn prepare_111() {
    use std::io::Write;

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
    use std::io::Write;

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

fn prepare_191() {
    use std::io::Write;

    fn number_to_string(number: &str) -> String {
        if number.starts_with('"') {
            number.to_owned()
        } else {
            number
                .chars()
                .map(|c| ('A' as u8 + c as u8 - '0' as u8) as char)
                .collect()
        }
    }

    const INPUT: &str = include_str!("src/bin/191/input.txt");
    println!("cargo:rerun-if-changed={}", INPUT);

    {
        const OUTPUT: &str = "src/bin/191/rules.peg";

        let rules: String = INPUT
            .split('\n')
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let (name, rule_parts) = {
                    let mut parts = line.split(':');
                    (
                        number_to_string(parts.next().unwrap()),
                        parts.next().unwrap(),
                    )
                };

                let rules = {
                    fn join(expr: &str) -> String {
                        let mut parts = expr.split_whitespace();

                        let first = number_to_string(parts.next().unwrap());

                        parts.fold(String::from(first), |acc, curr| {
                            format!("{} ~ {}", acc, number_to_string(curr))
                        })
                    }

                    let mut parts = rule_parts.split('|');

                    let first = join(parts.next().unwrap());
                    parts.map(join).fold(String::from(first), |acc, curr| {
                        format!("{} | {}", acc, curr)
                    })
                };

                format!("{} = _{{ {} }}\n", name, rules)
            })
            .collect();

        let mut output = std::fs::File::create(OUTPUT).unwrap();
        output.write_all(rules.as_bytes()).unwrap();
        output.write_all(b"main = _{ SOI ~ A ~ EOI }").unwrap();
    }

    {
        const OUTPUT: &str = "src/bin/191/messages.txt";

        let mut output = std::fs::File::create(OUTPUT).unwrap();
        INPUT
            .split('\n')
            .skip_while(|line| !line.is_empty())
            .filter(|line| !line.is_empty())
            .for_each(|line| {
                output.write_all(line.as_bytes()).unwrap();
                output.write(b"\n").unwrap();
            });
    }
}

fn main() {
    prepare_111();
    prepare_112();
    prepare_191();
}
