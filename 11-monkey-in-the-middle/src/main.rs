use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Monkey {
    starting_items: Vec<u64>,
    divisible_by: u64,
    operation: Vec<String>,
    inspections: usize,
    on_true: usize,
    on_false: usize,
}

fn remove_non_digits(line: &str) -> String {
    line.chars()
        .filter(|c| c.is_digit(10) || c == &',')
        .collect::<String>()
}

fn parse_starting_items(line: &String) -> Vec<u64> {
    remove_non_digits(line)
        .split(',')
        .map(|item| item.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn parse_operation(line: &String) -> Vec<String> {
    line.split(" ")
        .map(|item| item.to_string())
        .collect::<Vec<String>>()
        .chunks(3)
        .last()
        .unwrap()
        .to_vec()
}

fn parse_monkey(monkey: &[String]) -> Monkey {
    Monkey {
        starting_items: parse_starting_items(&monkey[1]),
        operation: parse_operation(&monkey[2]),
        divisible_by: remove_non_digits(&monkey[3]).parse::<u64>().unwrap(),
        on_true: remove_non_digits(&monkey[4]).parse::<u32>().unwrap() as usize,
        on_false: remove_non_digits(&monkey[5]).parse::<u32>().unwrap() as usize,
        inspections: 0,
    }
}

fn file_parser(path: &str) -> Vec<Monkey> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
        .chunks(6)
        .map(|monkey| parse_monkey(monkey))
        .collect::<Vec<Monkey>>()
}

fn perform_operation(old: u64, value_one: &String, oper: &String, value_two: &String) -> u64 {
    let v1 = if value_one == "old" {
        old
    } else {
        value_one.parse::<u64>().unwrap()
    };

    let v2 = if value_two == "old" {
        old
    } else {
        value_two.parse::<u64>().unwrap()
    };

    match oper.as_str() {
        "+" => (v1 + v2),
        "-" => (v1 - v2),
        "*" => (v1 * v2),
        "/" => (v1 / v2),
        _ => 0,
    }
}

fn monkey_rounds(monkeys: &mut Vec<Monkey>, amount: i32, manage_worry: bool) {
    let divisor = monkeys.iter().fold(1, |acc, f| acc * f.divisible_by);

    for _ in 0..amount {
        for index in 0..monkeys.len() {
            let item = monkeys[index].starting_items.to_owned();
            monkeys[index].starting_items.clear();
            item.iter().for_each(|i2| {
                let mut value = perform_operation(
                    *i2,
                    &monkeys[index].operation[0],
                    &monkeys[index].operation[1],
                    &monkeys[index].operation[2],
                );

                value = if manage_worry {
                    value / 3
                } else {
                    value % divisor
                };

                let monkey_throw_index = if value % monkeys[index].divisible_by == 0 {
                    monkeys[index].on_true
                } else {
                    monkeys[index].on_false
                };

                monkeys[monkey_throw_index].starting_items.push(value);
                monkeys[index].inspections += 1;
            });
        }
    }
}

fn calculate_monkey_business(monkeys: &mut Vec<Monkey>) -> usize {
    let mut values = monkeys.clone();
    values.sort_by(|a, b| a.inspections.cmp(&b.inspections));
    values.reverse();
    values.iter().take(2).fold(1, |acc, f| acc * f.inspections)
}

fn main() {
    let mut monkeys = file_parser("input.txt");

    monkey_rounds(&mut monkeys, 10000, false);
    let result = calculate_monkey_business(&mut monkeys);

    println!("Result: {}", result);
}

#[cfg(test)]
#[test]
fn test_part_one() {
    let mut monkeys = file_parser("input.example.txt");
    monkey_rounds(&mut monkeys, 20, true);
    let result = calculate_monkey_business(&mut monkeys);

    assert_eq!(result, 10605);
}

#[test]
fn test_part_two() {
    let mut monkeys = file_parser("input.example.txt");
    monkey_rounds(&mut monkeys, 10000, false);
    let result = calculate_monkey_business(&mut monkeys);

    assert_eq!(result, 2713310158);
}
