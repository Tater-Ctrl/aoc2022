use std::{fs::read_to_string, rc::Weak};

mod types;

use types::{Directory, Explorer};

fn command_handler(explorer: &mut Explorer, input: &String) {
    if input.starts_with("$ cd") {
        let dir_name = input.split(" ").collect::<Vec<&str>>()[2];
        if input.contains("..") {
            explorer.move_up();
        } else {
            explorer.move_down(dir_name);
        }
    } else if input.starts_with("dir") {
        let dir_name = input.split(" ").collect::<Vec<&str>>()[1];
        explorer.create_dir(dir_name);
    } else if input.starts_with("$ ls") {
        return;
    } else {
        let file = input.split(" ").collect::<Vec<&str>>();
        explorer.create_file(file[1], file[0].parse::<u64>().unwrap());
    }
}

fn file_parser(str: &str) -> Vec<String> {
    read_to_string(str)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn part_one() {
    let mut explorer = Explorer::new("/");
    file_parser("input.txt").iter().for_each(|line| {
        command_handler(&mut explorer, line);
    });

    explorer.move_to_root();
    let sum: u64 = explorer.dir_below_size(100000).iter().sum();
    println!("{}", sum);
}

fn part_two() {
    let mut explorer = Explorer::new("/");
    file_parser("input.txt").iter().for_each(|line| {
        command_handler(&mut explorer, line);
    });

    explorer.move_to_root();

    const MAX_SIZE: u64 = 70000000;
    const MIN_SIZE: u64 = 30000000;
    let current_size = explorer.dir_size();
    let to_remove_size = current_size - (MAX_SIZE - MIN_SIZE);

    println!("CUR SIZE {}", explorer.dir_size());
    println!("TO REMOVE {}", to_remove_size);

    let mut list: Vec<Weak<Directory>> = Vec::new();
    explorer.dir_greater_than(to_remove_size, &mut list);

    list.sort_unstable_by(|item, other| {
        item.upgrade()
            .unwrap()
            .dir_size()
            .cmp(&&other.upgrade().unwrap().dir_size())
    });

    println!("{}", current_size);
    println!("{}", list[0].upgrade().unwrap().dir_size());

    println!("AFTER SIZE {}", explorer.dir_size());
}

fn main() {
    // part_one();
    part_two();
}

#[cfg(test)]
#[test]
fn part_one_example_test() {
    let mut explorer = Explorer::new("/");
    file_parser("example_one.txt").iter().for_each(|line| {
        command_handler(&mut explorer, line);
    });

    explorer.move_to_root();
    let sum: u64 = explorer.dir_below_size(100000).iter().sum();

    assert_eq!(sum, 95437);
}

#[test]
fn part_two_example_test() {
    let mut explorer = Explorer::new("/");
    file_parser("input.txt").iter().for_each(|line| {
        command_handler(&mut explorer, line);
    });

    explorer.move_to_root();

    const MAX_SIZE: u64 = 70000000;
    const MIN_SIZE: u64 = 30000000;
    let current_size = explorer.dir_size();
    let to_remove_size = current_size - MIN_SIZE;

    let mut list: Vec<Weak<Directory>> = Vec::new();
    explorer.dir_greater_than(to_remove_size, &mut list);

    list.sort_unstable_by(|item, other| {
        item.upgrade()
            .unwrap()
            .dir_size()
            .cmp(&&other.upgrade().unwrap().dir_size())
    });

    assert!(list[0].upgrade().unwrap().dir_size() == 100000);

    // println!("{}", current_size);
    // list[0].upgrade().unwrap().delete();

    // println!("{}", explorer.dir_size());
}
