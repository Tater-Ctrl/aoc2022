use std::fs::read_to_string;

#[derive(Debug)]
enum InstructionType {
    NOOP = 1,
    ADDX = 2,
}

#[derive(Debug)]
struct Instruction {
    instruction: InstructionType,
    value: i32,
}

struct CPU {
    cycle: i32,
    value: i32,
    signals: Vec<i32>,
}

impl CPU {
    fn new() -> Self {
        Self {
            cycle: 0,
            value: 1,
            signals: Vec::new(),
        }
    }

    fn cycle(&mut self) {
        self.cycle += 1;
        self.draw();

        if (self.cycle + 20) % 40 == 0 {
            self.signals.push((self.cycle) * self.value);
        }
    }

    fn draw(&self) {
        let c = (self.cycle - 1) % 40;
        let v = self.value;
        if self.cycle > 0 && c == 0 {
            println!();
        }

        if v == c - 1 || v == c || v == c + 1 {
            print!("#");
        } else {
            print!(".");
        }
    }
}

fn file_parser(str: &str) -> Vec<Instruction> {
    read_to_string(str)
        .expect("Error reading file")
        .lines()
        .map(|line| line.split_once(' ').unwrap_or(("noop", "0")))
        .map(|(instruction, value)| Instruction {
            instruction: match instruction {
                "noop" => InstructionType::NOOP,
                "addx" => InstructionType::ADDX,
                _ => panic!("Unknown instruction"),
            },
            value: value.parse().unwrap(),
        })
        .collect()
}

fn solution(path: &str) -> i32 {
    let mut cpu = CPU::new();
    let instructions = file_parser(path);

    for instruction in instructions.iter() {
        match instruction.instruction {
            InstructionType::NOOP => {
                cpu.cycle();
            }
            InstructionType::ADDX => {
                for _ in 0..2 {
                    cpu.cycle();
                }
            }
        }

        cpu.value += instruction.value;
    }

    cpu.signals.iter().sum()
}

fn main() {
    solution("input.txt");
}

#[cfg(test)]
#[test]
fn test_part_one() {
    assert_eq!(solution("input.example.txt"), 13140);
}
