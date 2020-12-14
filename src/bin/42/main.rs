const INPUT: &str = include_str!("input.txt");

const JIG: [&str; 7] = ["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"];

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Field {
    field_name: String,
    field_value: String,
}

impl Field {
    fn from((name, value): (&str, &str)) -> Self {
        Self {
            field_name: String::from(name),
            field_value: String::from(&value[1..]),
        }
    }

    fn valid(&self) -> bool {
        match self.field_name.as_str() {
            "byr" => self.validate_year(1920, 2002),
            "iyr" => self.validate_year(2010, 2020),
            "eyr" => self.validate_year(2020, 2030),
            "hgt" => self.validate_hgt(),
            "hcl" => self.validate_hcl(),
            "ecl" => self.validate_ecl(),
            "pid" => self.validate_pid(),
            _ => false,
        }
    }

    fn to_field_name(self) -> String {
        self.field_name
    }

    fn validate_year(&self, min: isize, max: isize) -> bool {
        if self.field_value.len() != 4 {
            return false;
        }

        let year = match self.field_value.parse::<isize>() {
            Ok(year) => year,
            Err(_) => return false,
        };

        year >= min && year <= max
    }

    fn validate_hgt(&self) -> bool {
        if self.field_value.len() < 3 {
            return false;
        }

        let height = match self.field_value[..self.field_value.len() - 2].parse::<isize>() {
            Ok(height) => height,
            Err(_) => return false,
        };

        match &self.field_value[self.field_value.len() - 2..] {
            "cm" => height >= 150 && height <= 193,
            "in" => height >= 59 && height <= 76,
            _ => return false,
        }
    }

    fn validate_hcl(&self) -> bool {
        self.field_value.len() == 7
            && self
                .field_value
                .chars()
                .skip(1)
                .find(|c| !c.is_digit(16))
                .is_none()
    }

    fn validate_ecl(&self) -> bool {
        match self.field_value.as_str() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        }
    }

    fn validate_pid(&self) -> bool {
        self.field_value.len() == 9
            && self
                .field_value
                .chars()
                .find(|c| !c.is_ascii_digit())
                .is_none()
    }
}

fn main() {
    let count = INPUT
        .split("\n\n")
        .map(|line| line.replace('\n', " "))
        .map(|line| {
            let mut fields = line
                .split_whitespace()
                .filter(|field| !field.starts_with("cid"))
                .map(|item| item.split_at(3))
                .map(Field::from)
                .filter(Field::valid)
                .map(Field::to_field_name)
                .collect::<Vec<_>>();
            fields.sort();
            fields
        })
        .filter(|fields| fields.as_slice() == JIG.as_ref())
        .count();

    println!("{}", count);
}
