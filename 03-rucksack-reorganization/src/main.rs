use std::fs;

fn check_rucksack(left: &str, right: Vec<&str>) -> u32 {
    left.chars()
        .find(|c| right.iter().all(|&e| e.contains(*c)))
        .map(|c| {
            if c as u32 & 1 << 5 == 0 {
                c as u32 - 38
            } else {
                c as u32 - 96
            }
        })
        .unwrap_or(0)
}

fn calculate_priority(lines: Vec<String>) -> u32 {
    lines
        .iter()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            let mut list: Vec<&str> = Vec::new();
            list.push(right);

            check_rucksack(left, list)
        })
        .sum()
}

fn calculate_group_badges(lines: Vec<String>) -> u32 {
    lines
        .chunks(3)
        .map(|chunk| {
            let mut list: Vec<&str> = Vec::new();
            list.push(&chunk[1]);
            list.push(&chunk[2]);

            check_rucksack(&chunk[0], list)
        })
        .sum()
}

fn file_parser(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|e| e.to_string())
        .collect()
}

fn main() {
    let result = file_parser("input.txt");

    println!("Result: {}", calculate_priority(result.clone()));
    println!("Result: {}", calculate_group_badges(result));
}
