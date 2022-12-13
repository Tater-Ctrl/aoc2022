use std::{fs, time::Instant};

struct Section {
    first: (u32, u32),
    second: (u32, u32),
}

fn overlap(s: &Section, partial: bool) -> bool {
    if partial {
        return (s.first.0 <= s.second.0 && s.first.1 >= s.second.0)
            || (s.first.0 <= s.second.1 && s.first.1 >= s.second.1)
            || (s.first.0 >= s.second.0 && s.first.1 <= s.second.1)
            || (s.first.0 <= s.second.0 && s.first.1 >= s.second.1);
    } else {
        (s.first.0 >= s.second.0 && s.first.1 <= s.second.1)
            || (s.first.0 <= s.second.0 && s.first.1 >= s.second.1)
    }
}

fn file_parser(str: &str) -> Vec<Section> {
    fs::read_to_string(str)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.split_once(','))
        .map(|obj| {
            if let Some((first, second)) = obj {
                Ok(Section {
                    first: (
                        first.split_once('-').unwrap().0.parse().unwrap(),
                        first.split_once('-').unwrap().1.parse().unwrap(),
                    ),
                    second: (
                        second.split_once('-').unwrap().0.parse().unwrap(),
                        second.split_once('-').unwrap().1.parse().unwrap(),
                    ),
                })
            } else {
                Err("Failed to parse section".to_string())
            }
            .unwrap()
        })
        .collect::<Vec<Section>>()
}

fn main() {
    let time = Instant::now();
    let count = file_parser("input.txt")
        .iter()
        .filter(|s| overlap(*s, true))
        .count();

    println!("Overlap: {}", count);
    println!("Time: {}μs", time.elapsed().as_micros());
}

#[cfg(test)]
#[test]
fn test_parser() {
    let test = file_parser("input.test.txt");
    assert_eq!(test.len(), 6);
    assert_eq!(test[0].first.0, 2);
    assert_eq!(test[0].first.1, 4);
    assert_eq!(test[0].second.0, 6);
    assert_eq!(test[0].second.1, 8);
    assert_eq!(test[1].first.0, 2);
    assert_eq!(test[1].first.1, 3);
    assert_eq!(test[1].second.0, 4);
    assert_eq!(test[1].second.1, 5);
}

#[test]
fn test_full_overlap() {
    let count = file_parser("input.test.txt")
        .iter()
        .filter(|s| overlap(*s, false))
        .count();

    assert_eq!(count, 2);
}

#[test]
fn test_partial_overlap() {
    let count = file_parser("input.test.txt")
        .iter()
        .filter(|s| overlap(*s, true))
        .count();

    assert_eq!(count, 4);
}
