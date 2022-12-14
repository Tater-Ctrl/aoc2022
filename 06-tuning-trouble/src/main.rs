use std::time::Instant;
use std::{fs, str::Chars};

fn check_chars(chars: Chars, len: usize) -> Option<usize> {
    for (i, _c) in chars.clone().enumerate() {
        let clone: Vec<char> = chars.clone().collect();
        let mut list: Vec<char> = Vec::new();
        let mut duplicate = false;

        for j in i..i + len {
            if list.contains(&clone[j]) {
                duplicate = true;
                break;
            } else {
                list.push(clone[j]);
            }
        }

        if !duplicate {
            return Some(i + len);
        }
    }

    None
}

fn file_parser(str: &str) -> Vec<String> {
    fs::read_to_string(str)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn calculate_start(name: &str, len: usize) -> Vec<usize> {
    file_parser(name)
        .iter()
        .map(|line| line.chars())
        .map(|c| check_chars(c, len))
        .filter(|c| c.is_some())
        .map(|c| c.unwrap())
        .collect()
}

fn main() {
    let now = Instant::now();
    let part_one_result = calculate_start("input.txt", 4);
    let first = now.elapsed().as_millis();
    let part_two_result = calculate_start("input.txt", 14);
    let second = now.elapsed().as_millis() - first;

    println!("Part One Result: {:?} Time: {}ms", part_one_result, first);
    println!("Part Two Result: {:?} Time: {}ms", part_two_result, second);
}

#[cfg(test)]
#[test]
fn test_part_one() {
    let result: Vec<usize> = calculate_start("part_one.test.txt", 4);
    assert_eq!(result, vec![5, 6, 10, 11]);
}

#[test]
fn test_part_two() {
    let result: Vec<usize> = calculate_start("part_two.test.txt", 14);
    assert_eq!(result, vec![19, 23, 23, 29, 26]);
}
