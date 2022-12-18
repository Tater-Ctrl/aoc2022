mod snake;
use std::fs::read_to_string;

use snake::Rope;

#[derive(Debug)]
struct Command {
    direction: char,
    steps: usize,
}

fn file_parser(str: &str) -> Vec<Command> {
    read_to_string(str)
        .expect("Error reading file")
        .lines()
        .map(|line| line.split_once(" "))
        .map(|some| some.unwrap())
        .map(|(direction, steps)| Command {
            direction: direction.chars().next().unwrap(),
            steps: steps.parse().unwrap(),
        })
        .collect()
}

fn part_one(commands: &Vec<Command>) -> usize {
    let mut rope = Rope::new(2);

    for command in commands {
        rope.move_rope(command.direction, command.steps);
    }

    rope.parts.last().unwrap().moves
}

fn part_two(commands: &Vec<Command>) -> usize {
    let mut rope = Rope::new(10);

    for command in commands {
        rope.move_rope(command.direction, command.steps);
    }

    rope.parts.last().unwrap().locations.len()
}

fn main() {
    let time = std::time::Instant::now();
    let commands = file_parser("input.txt");
    let result = part_one(&commands);
    println!("Part one: {}", result);
    println!("Time: {}ms", time.elapsed().as_millis());

    let result = part_two(&commands);
    println!("Part two: {}", result);
    println!("Time: {}ms", time.elapsed().as_millis());
}

#[cfg(test)]
#[test]
fn test_part_one() {
    let commands = file_parser("input.example.txt");
    assert_eq!(part_one(&commands), 13);
}

#[test]
fn test_part_two() {
    let commands = file_parser("input.example.txt");
    assert_eq!(part_two(&commands), 1);
}
