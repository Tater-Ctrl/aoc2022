pub struct RopePart {
    pub x: i32,
    pub y: i32,
    pub moves: usize,
    pub locations: Vec<(i32, i32)>,
}

impl RopePart {
    pub fn new() -> RopePart {
        RopePart {
            x: 0,
            y: 0,
            moves: 0,
            locations: vec![(0, 0)],
        }
    }

    pub fn add_location(&mut self) {
        for i in 0..self.locations.len() {
            if self.locations[i] == (self.x, self.y) {
                return;
            }
        }

        self.locations.push((self.x, self.y));
    }
}

pub struct Rope {
    pub parts: Vec<RopePart>,
}

impl Rope {
    pub fn new(len: i32) -> Rope {
        let mut parts = Vec::new();
        for _ in 0..len {
            parts.push(RopePart::new());
        }

        Rope { parts }
    }

    fn get_length(&self, x: f64, y: f64) -> f64 {
        f64::sqrt(x.powi(2) + y.powi(2))
    }

    fn get_distance(&self, index: usize) -> f64 {
        f64::sqrt(
            f64::powi(
                self.parts[index - 1].x as f64 - self.parts[index].x as f64,
                2,
            ) + f64::powi(
                self.parts[index - 1].y as f64 - self.parts[index].y as f64,
                2,
            ),
        )
    }

    fn get_direction(&self, index: usize) -> (f64, f64) {
        let x: f64 = self.parts[index - 1].x as f64 - self.parts[index].x as f64;
        let y: f64 = self.parts[index - 1].y as f64 - self.parts[index].y as f64;
        let len = self.get_length(x, y);

        (x / len, y / len)
    }

    fn round_number(&self, num: f64) -> i32 {
        if num > 0.0 {
            return num.ceil() as i32;
        } else if num < 0.0 {
            return num.floor() as i32;
        }

        0
    }

    fn move_parts(&mut self) {
        for i in 1..self.parts.len() {
            let dist = self.get_distance(i);

            if dist >= 2.0 {
                let (x, y) = self.get_direction(i);

                self.parts[i].x += self.round_number(x);
                self.parts[i].y += self.round_number(y);
                self.parts[i].moves += 1;
                self.parts[i].add_location();
            }
        }
    }

    pub fn move_rope(&mut self, direction: char, steps: usize) {
        for _ in 0..steps {
            match direction {
                'U' => self.parts[0].y += 1,
                'D' => self.parts[0].y -= 1,
                'L' => self.parts[0].x -= 1,
                'R' => self.parts[0].x += 1,
                _ => panic!("Invalid direction"),
            }

            self.parts[0].moves += 1;
            self.move_parts();
        }
    }
}
