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

fn part_one(str: &str) -> usize {
    let commands = file_parser(str);
    let mut snake = Rope::new(2);

    for command in commands {
        snake.move_rope(command.direction, command.steps);
    }

    snake.parts[1].locations.len()
}

fn part_two(str: &str) -> usize {
    let commands = file_parser(str);
    let mut snake = Rope::new(10);

    for command in commands {
        snake.move_rope(command.direction, command.steps);
    }

    snake.parts.last().unwrap().locations.len()
}

fn main() {
    let time = std::time::Instant::now();
    let result = part_one("input.txt");
    println!("Part one: {}", result);
    println!("Time: {}ms", time.elapsed().as_millis());

    let result = part_two("input.txt");
    println!("Part two: {}", result);
    println!("Time: {}ms", time.elapsed().as_millis());
}

#[cfg(test)]
#[test]
fn test_part_one() {
    assert_eq!(part_one("input.example.txt"), 13);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two("input.example_two.txt"), 36);
}
