pub fn prepare() {
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

    const INPUT: &str = include_str!("input.txt");
    println!("cargo:rerun-if-changed=src/bin/192/{}", INPUT);

    {
        const OUTPUT: &str = "src/bin/192/rules.peg";

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

                let rules = if name == "I" {
                    String::from("EC | EC ~ I")
                } else if name == "BB" {
                    String::from("EC ~  DB | EC ~ BB ~ DB")
                } else {
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
}
