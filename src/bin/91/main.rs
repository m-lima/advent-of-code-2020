const INPUT: [usize; 1000] = include!("input.txt");

fn invalid(slice: &'static [usize]) -> Option<usize> {
    let target = slice[25];
    for i in 0..24 {
        for j in i..25 {
            if slice[i] + slice[j] == target {
                return None;
            }
        }
    }
    Some(target)
}

#[derive(Default)]
struct GroupIterator(usize);

impl std::iter::Iterator for GroupIterator {
    type Item = &'static [usize];

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 >= INPUT.len() - 26 {
            return None;
        }

        let output = &INPUT[self.0..self.0 + 26];

        self.0 += 1;
        Some(output)
    }
}

fn main() {
    let invalid = GroupIterator::default().filter_map(invalid).next().unwrap();
    println!("{}", invalid);
}
