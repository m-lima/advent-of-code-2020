const INPUT: &str = include_str!("input.txt");

enum Instruction {
    Nop,
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
            "nop" => Self::Nop,
            "acc" => Self::Acc(value),
            "jmp" => Self::Jmp(value),
            _ => unreachable!(),
        }
    }

    fn execute(&self, state: &mut StateMachine) {
        match self {
            Self::Nop => state.ir += 1,
            Self::Acc(value) => {
                state.acc += value;
                state.ir += 1;
            }
            Self::Jmp(value) => state.ir += value,
        }
    }
}

#[derive(Default)]
struct StateMachine {
    ir: isize,
    acc: isize,
}

struct Cpu {
    state: StateMachine,
    program: Vec<Instruction>,
    visited: Vec<bool>,
}

impl std::iter::Iterator for Cpu {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let ir = self.state.ir as usize;
        if self.visited[ir] {
            None
        } else {
            self.visited[ir] = true;
            self.program[ir].execute(&mut self.state);
            Some(self.state.acc)
        }
    }
}

fn main() {
    let cpu = {
        let program: Vec<_> = INPUT
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(Instruction::parse)
            .collect();
        let loc = program.len();

        Cpu {
            state: StateMachine::default(),
            program,
            visited: vec![false; loc],
        }
    };

    println!("{}", cpu.last().unwrap());
}
