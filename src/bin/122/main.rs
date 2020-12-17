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
            Self::E => (1, 0),
            Self::W => (-1, 0),
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

#[derive(Default)]
struct Coordinates {
    x: isize,
    y: isize,
}

impl Coordinates {
    fn translate(&mut self, direction: Direction, value: isize) {
        let axes = direction.as_movement();
        self.x += axes.0 * value;
        self.y += axes.1 * value;
    }
}

struct Ship {
    position: Coordinates,
    waypoint: Coordinates,
}

impl Ship {
    fn new() -> Self {
        Self {
            position: Coordinates::default(),
            waypoint: Coordinates { x: 10, y: -1 },
        }
    }

    fn navigate(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::I(direction, value) => self.waypoint.translate(direction, value),
            Instruction::L(value) => {
                self.turn_left(value % 360);
            }
            Instruction::F(value) => {
                self.position.x += self.waypoint.x * value;
                self.position.y += self.waypoint.y * value;
            }
            Instruction::R(value) => {
                self.turn_left(360 - (value % 360));
            }
        }
    }

    fn turn_left(&mut self, angle: isize) {
        match angle {
            0 => {}
            90 => {
                std::mem::swap(&mut self.waypoint.x, &mut self.waypoint.y);
                self.waypoint.y *= -1;
            }
            180 => {
                self.waypoint.x *= -1;
                self.waypoint.y *= -1;
            }
            270 => {
                std::mem::swap(&mut self.waypoint.x, &mut self.waypoint.y);
                self.waypoint.x *= -1;
            }
            _ => unreachable!(),
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

    println!("{}", ship.position.x.abs() + ship.position.y.abs());
}
