const INPUT: [usize; 1000] = include!("input.txt");

fn invalid((index, slice): (usize, &'static [usize])) -> Option<(usize, usize)> {
    let target = slice[25];
    for i in 0..24 {
        for j in i..25 {
            if slice[i] + slice[j] == target {
                return None;
            }
        }
    }
    Some((index + 25, target))
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

fn weakness(target: usize, limit: usize) -> Option<usize> {
    let mut min = 0;
    let mut max = 0;
    let mut sum = INPUT[min];

    while max < limit {
        if sum == target {
            let slice = &INPUT[min..=max];
            return Some(slice.iter().min().unwrap() + slice.iter().max().unwrap());
        }

        while sum < target {
            max += 1;
            sum += INPUT[max];
        }

        while sum > target {
            sum -= INPUT[min];
            min += 1;
        }
    }
    None
}

fn main() {
    let (max_index, target) = GroupIterator::default()
        .enumerate()
        .filter_map(invalid)
        .next()
        .unwrap();

    let weakness = weakness(target, max_index).unwrap();
    println!("{}", weakness);
}
