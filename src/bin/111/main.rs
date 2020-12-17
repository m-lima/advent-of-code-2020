// const INPUT: &str = include_str!("input.txt");
// const WIDTH: usize = 92;
mod input;
use input::INPUT;

const WIDTH: usize = INPUT[0].len();
const HEIGHT: usize = INPUT.len();
type Layout = [[Seat; WIDTH]; HEIGHT];

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
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
        let occupied_neighbors = Adjecents::new(&self.0, row, col)
            .filter(|s| *s == Seat::Occupied)
            .count();
        if seat == Seat::Vacant && occupied_neighbors == 0 {
            Some((row, col, Seat::Occupied))
        } else if seat == Seat::Occupied && occupied_neighbors > 3 {
            Some((row, col, Seat::Vacant))
        } else {
            None
        }
    }
}

struct Adjecents<'a> {
    layout: &'a Layout,
    row: usize,
    col: usize,
    curr: usize,
}

impl<'a> Adjecents<'a> {
    const SEQ: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    fn new(layout: &'a Layout, row: usize, col: usize) -> Self {
        Self {
            layout,
            row,
            col,
            curr: 0,
        }
    }
}

impl std::iter::Iterator for Adjecents<'_> {
    type Item = Seat;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == Self::SEQ.len() {
            return None;
        }

        while self.curr < Self::SEQ.len() {
            let offset = Self::SEQ[self.curr];
            let (row, col) = (self.row as isize + offset.0, self.col as isize + offset.1);
            self.curr += 1;

            if row < 0 || row >= HEIGHT as isize {
                continue;
            }

            if col < 0 || col >= WIDTH as isize {
                continue;
            }

            return Some(self.layout[row as usize][col as usize]);
        }
        None
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
