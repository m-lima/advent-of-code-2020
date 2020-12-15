const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Seat {
    row: u8,
    col: u8,
}

impl Seat {
    fn from(string: &str) -> Self {
        let (row, col) = string.split_at(7);
        Self {
            row: Self::string_to_u8(row),
            col: Self::string_to_u8(col),
        }
    }

    fn string_to_u8(string: &str) -> u8 {
        string.chars().fold(0, |acc, curr| {
            if curr == 'B' || curr == 'R' {
                (acc << 1) + 1
            } else {
                acc << 1
            }
        })
    }

    fn id(self) -> usize {
        self.row as usize * 8 + self.col as usize
    }
}

fn main() {
    let mut seat_ids = INPUT
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(Seat::from)
        .map(Seat::id)
        .collect::<Vec<_>>();
    seat_ids.sort();

    let id = seat_ids
        .iter()
        .enumerate()
        .find(|(index, value)| index + 48 != **value)
        .unwrap();

    println!("{}", id.0 + 48);
}
