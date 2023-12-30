use std::{fs::File, io::Read};

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
}
fn main() -> Result<(), std::io::Error> {
    let mut input = File::open("input.txt")?;
    let mut contents = String::new();

    input.read_to_string(&mut contents)?;
    let mut platform = Platform::build(contents);
    platform.roll_north();
    println!("{}", platform.calculate_load());

    Ok(())
}
