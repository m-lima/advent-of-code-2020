const INPUT: &str = include_str!("input.txt");

#[derive(Copy, Clone)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    fn as_movement(self) -> (isize, isize) {
        match self {
            Self::N => (0, -1),
            Self::S => (0, 1),
            Self::E => (-1, 0),
            Self::W => (1, 0),
        }
    }
}

#[derive(Copy, Clone)]
enum Instruction {
    I(Direction, isize),
    L(isize),
    F(isize),
    R(isize),
}

impl Instruction {
    fn parse(line: &str) -> Self {
        let parts = line.split_at(1);
        let value = parts.1.parse().unwrap();

        match parts.0.chars().next().unwrap() {
            'N' => Self::I(Direction::N, value),
            'S' => Self::I(Direction::S, value),
            'E' => Self::I(Direction::E, value),
            'W' => Self::I(Direction::W, value),
            'L' => Self::L(value),
            'F' => Self::F(value),
            'R' => Self::R(value),
            _ => unreachable!(),
        }
    }
}

struct Ship {
    x: isize,
    y: isize,
    direction: Direction,
}

impl Ship {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            direction: Direction::E,
        }
    }

    fn navigate(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::I(direction, value) => {
                let axes = direction.as_movement();
                self.x += axes.0 * value;
                self.y += axes.1 * value;
            }
            Instruction::L(value) => {
                self.turn_left(value);
            }
            Instruction::F(value) => {
                let axes = self.direction.as_movement();
                self.x += axes.0 * value;
                self.y += axes.1 * value;
            }
            Instruction::R(value) => {
                self.turn_left(360 - value);
            }
        }
    }

    fn turn_left(&mut self, times: isize) {
        if times > 0 {
            self.direction = match self.direction {
                Direction::N => Direction::W,
                Direction::S => Direction::E,
                Direction::E => Direction::N,
                Direction::W => Direction::S,
            };
            self.turn_left(times - 90);
        }
    }
}

fn main() {
    let mut ship = Ship::new();

    INPUT
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(Instruction::parse)
        .for_each(|i| ship.navigate(i));

    println!("{}", ship.x.abs() + ship.y.abs());
}
