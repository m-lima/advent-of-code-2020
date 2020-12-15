const INPUT: &str = include_str!("input.txt");

enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl Instruction {
    fn parse(line: &str) -> Self {
        let (operator, value) = {
            let parts = line.split_at(3);
            (parts.0, parts.1.replace('+', "").trim().parse().unwrap())
        };
        match operator {
            "nop" => Self::Nop(value),
            "acc" => Self::Acc(value),
            "jmp" => Self::Jmp(value),
            _ => unreachable!(),
        }
    }

    fn execute(&self, state: &State, program: &[Instruction], swapped: bool) -> Option<State> {
        if state.visited[state.ir] {
            return None;
        }

        let mut new_state = state.clone();
        new_state.visited[state.ir] = true;

        match self {
            Self::Nop(_) => new_state.ir += 1,
            Self::Acc(value) => {
                new_state.acc += value;
                new_state.ir += 1;
            }
            Self::Jmp(value) => new_state.ir = ((new_state.ir as isize) + value) as usize,
        };

        if new_state.ir < program.len() {
            if let Some(next_state) = program[new_state.ir].execute(&new_state, &program, swapped) {
                Some(next_state)
            } else if !swapped {
                self.swap().and_then(|s| s.execute(&state, &program, true))
            } else {
                None
            }
        } else {
            Some(new_state)
        }
    }

    fn swap(&self) -> Option<Instruction> {
        match self {
            Self::Nop(value) => Some(Self::Jmp(*value)),
            Self::Jmp(value) => Some(Self::Nop(*value)),
            Self::Acc(_) => None,
        }
    }
}

#[derive(Default, Clone)]
struct State {
    ir: usize,
    acc: isize,
    visited: Vec<bool>,
}

impl State {
    fn new(size: usize) -> Self {
        Self {
            ir: 0,
            acc: 0,
            visited: vec![false; size],
        }
    }
}

fn main() {
    let program: Vec<_> = INPUT
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(Instruction::parse)
        .collect();

    let output = program[0]
        .execute(&State::new(program.len()), &program, false)
        .unwrap()
        .acc;

    println!("{}", output);
}
