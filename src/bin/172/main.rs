use std::collections::HashSet;

const INPUT: &str = "...#...#
#######.
....###.
.#..#...
#.#.....
.##.....
#.####..
#....##.";

#[derive(Eq, PartialEq, Copy, Clone)]
struct Cell(i8, i8, i8, i8);

impl std::hash::Hash for Cell {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.0 as usize
            | (self.1 as usize) << 8
            | (self.2 as usize) << 16
            | (self.3 as usize) << 24)
            .hash(state)
    }
}

impl std::iter::IntoIterator for Cell {
    type Item = Self;
    type IntoIter = Neighbors;

    fn into_iter(self) -> Self::IntoIter {
        Neighbors(self, 0)
    }
}

struct Neighbors(Cell, u8);

impl std::iter::Iterator for Neighbors {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 == 81 {
            return None;
        }

        if self.1 == 40 {
            self.1 = 41
        }

        let next = Cell(
            self.0 .0 + (self.1 % 3) as i8 - 1,
            self.0 .1 + ((self.1 / 3) % 3) as i8 - 1,
            self.0 .2 + ((self.1 / 9) % 3) as i8 - 1,
            self.0 .3 + (self.1 / 27) as i8 - 1,
        );
        self.1 += 1;
        Some(next)
    }
}

fn main() {
    let mut neighbors = HashSet::<Cell>::new();

    let mut cells: HashSet<Cell> = INPUT
        .split('\n')
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(Cell(x as i8, y as i8, 0, 0))
                } else {
                    None
                }
            })
        })
        .collect();

    for _ in 0..6 {
        let mut next_round: HashSet<Cell> = cells
            .iter()
            .filter(|c| {
                let active_neighbors = c
                    .into_iter()
                    .filter(|n| {
                        neighbors.insert(*n);
                        cells.contains(n)
                    })
                    .count();
                active_neighbors == 2 || active_neighbors == 3
            })
            .map(Clone::clone)
            .collect();

        next_round.extend(
            neighbors
                .iter()
                .filter(|c| c.into_iter().filter(|n| cells.contains(n)).count() == 3),
        );

        cells = next_round;
        neighbors.clear();
    }

    println!("{}", cells.len())
}
