use std::fs;

enum CrateMover {
    M9000,
    M9001,
}

fn container_parser(containers: &[String]) -> Vec<Vec<char>> {
    let unaligned = containers
        .iter()
        .map(|x| x.chars().skip(1).step_by(4).collect::<String>())
        .rev()
        .collect::<Vec<String>>();

    let mut aligned: Vec<Vec<char>> = Vec::new();
    unaligned.iter().for_each(|str| {
        str.chars().enumerate().for_each(|(i, c)| {
            if c.ne(&' ') {
                if aligned.len() <= i {
                    aligned.push(Vec::new());
                }

                aligned[i].push(c);
            }
        });
    });

    aligned
}

fn instruction_parser(instructions: &[String]) -> Vec<i32> {
    instructions
        .iter()
        .map(|line| line.split_ascii_whitespace().collect::<Vec<&str>>())
        .flatten()
        .filter(|word| word.chars().all(char::is_numeric))
        .map(|digit| digit.parse::<i32>().unwrap())
        .collect()
}

fn file_parser(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("Unable to read file")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn rearrange(instructions: &Vec<i32>, containers: &mut Vec<Vec<char>>, model: CrateMover) {
    instructions
        .chunks(3)
        .map(|x| x.to_vec())
        .for_each(|chunk| {
            let (amount, from, to) = (
                chunk[0] as usize,
                chunk[1] as usize - 1,
                chunk[2] as usize - 1,
            );

            let len = containers[from].len();
            let mut crates = match model {
                CrateMover::M9000 => containers[from].drain(len - amount..len).rev().collect(),
                CrateMover::M9001 => containers[from].drain(len - amount..len).collect(),
            };

            containers[to].append(&mut crates);
        });
}

fn main() {
    let file = file_parser("input.txt");
    let position = file.iter().position(|x| x.is_empty()).unwrap();
    let containers = &file[0..position - 1];
    let instructions = &file[containers.len() + 2..file.len()];
    let mut aligned_containers = container_parser(containers);
    let instruction_list = instruction_parser(instructions);
    rearrange(
        &instruction_list,
        &mut aligned_containers,
        CrateMover::M9001,
    );

    print!("Answer: ");
    aligned_containers
        .iter()
        .map(|container| container.last().unwrap())
        .for_each(|c| print!("{}", c));

    println!();
}

#[cfg(test)]
#[test]
fn test_part_one() {
    let file = file_parser("input.test.txt");
    let position = file.iter().position(|x| x.is_empty()).unwrap();
    let containers = &file[0..position - 1];
    let instructions = &file[containers.len() + 2..file.len()];
    let mut aligned_containers = container_parser(containers);
    let instruction_list = instruction_parser(instructions);
    rearrange(
        &instruction_list,
        &mut aligned_containers,
        CrateMover::M9000,
    );

    assert_eq!(
        aligned_containers,
        vec![vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z']]
    );
}

#[test]
fn test_part_two() {
    let file = file_parser("input.test.txt");
    let position = file.iter().position(|x| x.is_empty()).unwrap();
    let containers = &file[0..position - 1];
    let instructions = &file[containers.len() + 2..file.len()];
    let mut aligned_containers = container_parser(containers);
    let instruction_list = instruction_parser(instructions);
    rearrange(
        &instruction_list,
        &mut aligned_containers,
        CrateMover::M9001,
    );

    assert_eq!(
        aligned_containers,
        vec![vec!['M'], vec!['C'], vec!['P', 'Z', 'N', 'D']]
    )
}
