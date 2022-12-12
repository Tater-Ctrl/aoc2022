use std::fs;

#[derive(Clone)]
pub struct Elf {
    pub calories: Vec<u32>,
    pub max_calories: u32,
}

impl Elf {
    pub fn new(calories: Vec<u32>) -> Self {
        Elf {
            calories,
            max_calories: 0,
        }
    }

    pub fn calculate_calories(&self) -> u32 {
        self.calories.iter().sum()
    }
}

fn parse_input(path: &str) -> Vec<Elf> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let mut elves: Vec<Elf> = Vec::new();
    let mut values: Vec<u32> = Vec::new();

    for line in contents.lines() {
        if line.is_empty() {
            elves.push(Elf::new(values.clone()));
            values.clear();
            continue;
        }

        match line.parse::<u32>() {
            Ok(n) => values.push(n),
            Err(_) => continue,
        }
    }

    return elves;
}

fn print_result(x: &u32) -> Option<u32> {
    println!("Max calories: {}", x);
    Some(*x)
}

fn main() {
    parse_input("input.txt")
        .iter()
        .map(|e| e.calculate_calories())
        .collect::<Vec<u32>>()
        .iter()
        .max()
        .and_then(print_result);
}
