use std::{collections::HashMap, fs::File, io::Read};

struct Platform {
    rocks: Vec<Vec<char>>,
}

impl Platform {
    fn build(input: String) -> Self {
        let mut rocks = Vec::new();
        for l in input.lines() {
            let mut aux = Vec::new();
            for c in l.chars() {
                aux.push(c);
            }
            rocks.push(aux);
        }
        Self { rocks }
    }

    fn roll_north(&mut self) {
        let rows = self.rocks.len();
        let cols = self.rocks[0].len();
        for i in 0..cols {
            let mut dot: Option<usize> = None;
            for j in 0..rows {
                match self.rocks[j][i] {
                    'O' => {
                        if let Some(mut x) = dot {
                            self.rocks[j][i] = '.';
                            self.rocks[x][i] = 'O';
                            if self.rocks[x + 1][i] == '.' {
                                dot = Some(x + 1);
                            } else {
                                while x < rows && self.rocks[x + 1][i] == '#' {
                                    x += 1;
                                }
                            }
                        }
                    }
                    '.' => {
                        if dot.is_none() {
                            dot = Some(j);
                        }
                    }
                    '#' => dot = None,
                    _ => unreachable!(),
                }
            }
        }
    }

    fn roll_east(&mut self) {
        let rows = self.rocks.len();
        let cols = self.rocks[0].len();
        for i in 0..rows {
            let mut dot: Option<usize> = None;
            for j in (0..cols).rev() {
                match self.rocks[i][j] {
                    'O' => {
                        if let Some(mut x) = dot {
                            self.rocks[i][j] = '.';
                            self.rocks[i][x] = 'O';
                            if self.rocks[i][x - 1] == '.' {
                                dot = Some(x - 1);
                            } else {
                                while self.rocks[i][x - 1] == '#' {
                                    x -= 1;
                                    if x == 0 {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    '.' => {
                        if dot.is_none() {
                            dot = Some(j);
                        }
                    }
                    '#' => dot = None,
                    _ => unreachable!(),
                }
            }
        }
    }

    fn roll_south(&mut self) {
        let rows = self.rocks.len();
        let cols = self.rocks[0].len();
        for i in 0..cols {
            let mut dot: Option<usize> = None;
            for j in (0..rows).rev() {
                match self.rocks[j][i] {
                    'O' => {
                        if let Some(mut x) = dot {
                            self.rocks[j][i] = '.';
                            self.rocks[x][i] = 'O';
                            if self.rocks[x - 1][i] == '.' {
                                dot = Some(x - 1);
                            } else {
                                while self.rocks[x + 1][i] == '#' {
                                    x -= 1;
                                    if x == 0 {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    '.' => {
                        if dot.is_none() {
                            dot = Some(j);
                        }
                    }
                    '#' => dot = None,
                    _ => unreachable!(),
                }
            }
        }
    }

    fn roll_west(&mut self) {
        let rows = self.rocks.len();
        let cols = self.rocks[0].len();
        for i in 0..rows {
            let mut dot: Option<usize> = None;
            for j in 0..cols {
                match self.rocks[i][j] {
                    'O' => {
                        if let Some(mut x) = dot {
                            self.rocks[i][j] = '.';
                            self.rocks[i][x] = 'O';
                            if self.rocks[i][x + 1] == '.' {
                                dot = Some(x + 1);
                            } else {
                                while x < cols && self.rocks[i][x + 1] == '#' {
                                    x += 1;
                                }
                            }
                        }
                    }
                    '.' => {
                        if dot.is_none() {
                            dot = Some(j);
                        }
                    }
                    '#' => dot = None,
                    _ => unreachable!(),
                }
            }
        }
    }

    fn calculate_load(&self) -> i32 {
        let rows = self.rocks.len();
        let cols = self.rocks[0].len();
        let mut s = 0;
        for i in 0..rows {
            for j in 0..cols {
                if self.rocks[i][j] == 'O' {
                    s += rows - i;
                }
            }
        }
        s as i32
    }

    fn stringify(&self) -> String {
        let mut res = String::new();
        for r in self.rocks.iter() {
            for c in r.iter() {
                res.push(*c);
            }
            res.push('\n');
        }
        res
    }
}
fn main() -> Result<(), std::io::Error> {
    let mut input = File::open("input.txt")?;
    let mut contents = String::new();

    input.read_to_string(&mut contents)?;
    let mut platform = Platform::build(contents);

    let mut dp: HashMap<String, (i32, String)> = HashMap::new();
    let mut cycle = None;
    for i in 1..1_000_000_001 {
        let x = platform.stringify();
        if dp.contains_key(&x) {
            cycle = Some((dp.get(&x).unwrap().0, i - 1));
            break;
        }
        platform.roll_north();
        platform.roll_west();
        platform.roll_south();
        platform.roll_east();
        dp.insert(x, (i, platform.stringify()));
    }

    let mut final_map = HashMap::new();
    if let Some(x) = cycle {
        println!("{}, {}", x.0, x.1);
        for v in dp.values() {
            if v.0 >= x.0 && v.0 <= x.1 {
                final_map.insert(v.0, v.1.clone());
            }
        }

        let traveled = 1_000_000_000 - x.0;
        platform = Platform::build(
            final_map
                .get(&(x.0 + (traveled) % (x.1 - x.0 + 1)))
                .unwrap()
                .clone(),
        );
    }

    println!("{}", platform.calculate_load());

    Ok(())
}
