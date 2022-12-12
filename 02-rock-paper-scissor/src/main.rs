use std::collections::HashMap;
use std::fs;

fn init_map() -> HashMap<String, i32> {
    let mut map = HashMap::new();
    map.insert("A".to_string(), 1);
    map.insert("B".to_string(), 2);
    map.insert("C".to_string(), 3);
    map.insert("X".to_string(), 1);
    map.insert("Y".to_string(), 2);
    map.insert("Z".to_string(), 3);
    map
}

fn calculate_result(user: &i32, computer: &i32) -> i32 {
    let result = user - computer;
    match result {
        -2 | 1 => 6 + user,
        0 => 3 + user,
        _ => 0 + user,
    }
}

fn parse_input(input: &str) -> Vec<String> {
    fs::read_to_string(input)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|s| s.split_whitespace())
        .into_iter()
        .flatten()
        .map(|s| s.to_string())
        .collect()
}

fn get_user_choice(computer: i32, action: i32) -> i32 {
    let result: i32 = match action {
        1 => computer - 1,
        2 => computer,
        3 => computer + 1,
        _ => 1,
    };

    match result {
        0 => 3,
        4 => 1,
        _ => result,
    }
}

fn main() {
    let map = init_map();
    let input = parse_input("input.txt");

    let mut total = 0;
    for games in input.chunks(2) {
        let computer = map.get(&games[0]).unwrap();
        let cheat_action = map.get(&games[1]).unwrap();
        let user = &get_user_choice(*computer, *cheat_action);
        let result = calculate_result(user, computer);

        total += result;
    }

    println!("{}", total);
}
