use std::fs::read_to_string;

struct Array2D {
    width: i32,
    height: i32,
    data: Vec<u32>,
}

impl Array2D {
    fn bounds_check(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn get(&self, x: i32, y: i32) -> i32 {
        if !self.bounds_check(x, y) {
            return -1;
        }

        let x = x % self.width;
        let y = y % self.height;
        self.data[(y * self.width + x) as usize] as i32
    }

    pub fn iter(&self) -> std::slice::Iter<u32> {
        self.data.iter()
    }

    pub fn convert(&self, index: i32) -> (i32, i32) {
        let x = index % self.width;
        let y = index / self.width;
        (x, y)
    }

    pub fn scenic(&self, x: i32, y: i32) -> u32 {
        let mut dir = vec![true, true, true, true];
        let mut offset = 1;
        let mut left = 0;
        let mut right = 0;
        let mut up = 0;
        let mut down = 0;
        let value = self.get(x, y);

        while dir[0] || dir[1] || dir[2] || dir[3] {
            if dir[0] {
                // left
                let left_value = self.get(x - offset, y);

                if left_value == -1 {
                    // left += 1;
                    dir[0] = false;
                } else {
                    left += 1;
                    if left_value >= value {
                        dir[0] = false;
                    }
                }
            }

            if dir[1] {
                // right
                let right_value = self.get(x + offset, y);
                if right_value == -1 {
                    // right += 1;
                    dir[1] = false;
                } else {
                    right += 1;
                    if right_value >= value {
                        dir[1] = false;
                    }
                }
            }

            if dir[2] {
                // up
                let up_value = self.get(x, y - offset);
                if up_value == -1 {
                    // up += 1;
                    dir[2] = false;
                } else {
                    up += 1;
                    if up_value >= value {
                        dir[2] = false;
                    }
                }
            }

            if dir[3] {
                // down
                let down_value = self.get(x, y + offset);
                if down_value == -1 {
                    // down += 1;
                    dir[3] = false;
                } else {
                    down += 1;
                    if down_value >= value {
                        dir[3] = false;
                    }
                }
            }

            offset += 1;
        }

        // println!("{} {} {} {}", up, left, down, right);
        up * left * down * right
    }

    pub fn visible(&self, x: i32, y: i32) -> bool {
        let value = self.get(x, y);
        let mut right = Vec::new();
        let mut left = Vec::new();
        let mut up = Vec::new();
        let mut down = Vec::new();

        for hor in 0..self.width {
            if hor < x {
                // left
                left.push(self.get(hor, y));
            } else if hor > x {
                // right
                right.push(self.get(hor, y));
            }
        }

        for ver in 0..self.height {
            if ver < y {
                // left
                up.push(self.get(x, ver));
            } else if ver > y {
                // right
                down.push(self.get(x, ver));
            }
        }

        left.iter().all(|&x| x < value)
            || right.iter().all(|&x| x < value)
            || up.iter().all(|&x| x < value)
            || down.iter().all(|&x| x < value)
    }
}

fn file_parser(name: &str) -> Array2D {
    read_to_string(name)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.chars())
        .map(|c| c.map(|c| c.to_digit(10).unwrap()).collect())
        .fold(
            Array2D {
                data: Vec::new(),
                width: 0,
                height: 0,
            },
            |mut data, line: Vec<u32>| {
                data.width = line.len() as i32;
                data.height += 1;
                data.data.extend(line);
                data
            },
        )
}

fn part_one(input: &str) -> Vec<(i32, i32)> {
    let data = file_parser(input);

    data.iter()
        .enumerate()
        .map(|(index, _)| data.convert(index as i32))
        .filter(|(x, y)| data.visible(*x, *y))
        .collect()
}

fn part_two(input: &str) -> u32 {
    let data = file_parser(input);
    data.iter()
        .enumerate()
        .map(|(index, _)| data.convert(index as i32))
        .map(|(x, y)| data.scenic(x, y))
        .max()
        .unwrap()
}

fn main() {
    let res = part_one("input.txt");
    println!("Part One: {:?}", res.len());
    let res = part_two("input.txt");
    println!("Part Two: {:?}", res);
}

#[cfg(test)]
#[test]
fn test_part_one() {
    let res = part_one("input.example.txt");
    assert_eq!(res.len(), 21);
}

#[test]
fn test_part_two() {
    let res = part_two("input.example.txt");
    assert_eq!(res, 8);
}
