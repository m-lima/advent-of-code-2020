// const INPUT: &str = include_str!("input.txt");
// const WIDTH: usize = 92;
mod input;
use input::INPUT;

const WIDTH: usize = INPUT[0].len();
const HEIGHT: usize = INPUT.len();
type Layout = [[Seat; WIDTH]; HEIGHT];

#[derive(Eq, PartialEq, Copy, Clone)]
enum Seat {
    None,
    Vacant,
    Occupied,
}

impl Seat {
    fn parse(c: &char) -> Self {
        match c {
            '.' => Self::None,
            'L' => Self::Vacant,
            '#' => Self::Occupied,
            _ => unreachable!(),
        }
    }

    fn occupied(&self) -> bool {
        *self == Self::Occupied
    }
}

struct FloorPlan(Layout);

impl FloorPlan {
    fn people_moved(&mut self) -> bool {
        let new_layout = self
            .0
            .iter()
            .enumerate()
            .flat_map(|(row, seats)| {
                let borrow = &self;
                seats
                    .iter()
                    .enumerate()
                    .filter_map(move |(col, seat)| borrow.try_progress_seat(row, col, *seat))
            })
            .collect::<Vec<_>>();

        if new_layout.is_empty() {
            false
        } else {
            for (row, col, seat) in new_layout {
                self.0[row][col] = seat;
            }
            true
        }
    }

    fn try_progress_seat(
        &self,
        row: usize,
        col: usize,
        seat: Seat,
    ) -> Option<(usize, usize, Seat)> {
        let occupied_neighbors = 0
            + self.occupied_line_of_sight(row, col, (-1, -1))
            + self.occupied_line_of_sight(row, col, (-1, 0))
            + self.occupied_line_of_sight(row, col, (-1, 1))
            + self.occupied_line_of_sight(row, col, (0, -1))
            + self.occupied_line_of_sight(row, col, (0, 1))
            + self.occupied_line_of_sight(row, col, (1, -1))
            + self.occupied_line_of_sight(row, col, (1, 0))
            + self.occupied_line_of_sight(row, col, (1, 1));

        if seat == Seat::Vacant && occupied_neighbors == 0 {
            Some((row, col, Seat::Occupied))
        } else if seat == Seat::Occupied && occupied_neighbors > 4 {
            Some((row, col, Seat::Vacant))
        } else {
            None
        }
    }

    fn occupied_line_of_sight(&self, row: usize, col: usize, direction: (isize, isize)) -> u8 {
        Adjecents::new(&self.0, row, col, direction)
            .find(Seat::occupied)
            .map_or(0, |_| 1)
    }
}

struct Adjecents<'a> {
    layout: &'a Layout,
    direction: (isize, isize),
    row: isize,
    col: isize,
}

impl<'a> Adjecents<'a> {
    fn new(layout: &'a Layout, row: usize, col: usize, direction: (isize, isize)) -> Self {
        Self {
            layout,
            direction,
            row: row as isize,
            col: col as isize,
        }
    }
}

impl std::iter::Iterator for Adjecents<'_> {
    type Item = Seat;

    fn next(&mut self) -> Option<Self::Item> {
        self.row += self.direction.0;
        self.col += self.direction.1;

        if self.row < 0 || self.row >= HEIGHT as isize {
            return None;
        }

        if self.col < 0 || self.col >= WIDTH as isize {
            return None;
        }

        let output = self.layout[self.row as usize][self.col as usize];
        if output != Seat::None {
            self.row = isize::MIN + 1;
        }
        Some(output)
    }
}

fn main() {
    let mut floor_plan = {
        let mut floor_plan = [[Seat::None; WIDTH]; HEIGHT];
        INPUT.iter().enumerate().for_each(|(row, seats)| {
            seats
                .iter()
                .map(Seat::parse)
                .enumerate()
                .for_each(|(col, seat)| floor_plan[row][col] = seat)
        });
        FloorPlan(floor_plan)
    };

    while floor_plan.people_moved() {}
    let count = floor_plan
        .0
        .iter()
        .flat_map(|seats| seats.iter().filter(|s| **s == Seat::Occupied))
        .count();
    println!("{}", count);
}
