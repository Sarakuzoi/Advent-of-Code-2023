use std::{fs::File, io::Read};

struct Pattern {
    row_mirror: Vec<Vec<char>>,
    col_mirror: Vec<Vec<char>>,
}

impl Pattern {
    fn build(pattern: &str, row_size: usize) -> Self {
        let (mut row_mirror, mut col_mirror) = (Vec::new(), Vec::new());
        let mut aux = Vec::new();
        for (i, c) in pattern.char_indices() {
            aux.push(c);
            if i % row_size == row_size - 1 {
                row_mirror.push(aux.clone());
                aux.clear();
            }
        }
        aux.clear();
        for i in 0..row_size {
            for j in 0..row_mirror.len() {
                aux.push(row_mirror[j][i]);
            }
            col_mirror.push(aux.clone());
            aux.clear();
        }
        Self {
            row_mirror,
            col_mirror,
        }
    }

    fn compute_val(&self) -> i32 {
        let mut res = 0;
        for r_ind in 1..self.row_mirror.len() {
            if self.row_mirror[r_ind] == self.row_mirror[r_ind - 1] {
                if Self::valid_split(&self.row_mirror, r_ind) {
                    res += r_ind * 100;
                }
            }
        }
        for c_ind in 1..self.col_mirror.len() {
            if self.col_mirror[c_ind] == self.col_mirror[c_ind - 1] {
                if Self::valid_split(&self.col_mirror, c_ind) {
                    res += c_ind;
                }
            }
        }
        res as i32
    }

    fn valid_split(matrix: &Vec<Vec<char>>, i: usize) -> bool {
        let (mut up, mut down) = ((i - 1) as i32, i as i32);
        while up >= 0 && down < matrix.len() as i32 {
            if matrix[up as usize] != matrix[down as usize] {
                return false;
            }
            up -= 1;
            down += 1;
        }
        true
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut input = File::open("input.txt")?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;

    let mut patterns = Vec::new();
    let mut pattern_extractor = String::new();
    let mut prev = 0;
    for line in contents.lines() {
        if line.is_empty() {
            patterns.push(Pattern::build(&pattern_extractor, prev));
            pattern_extractor.clear();
        } else {
            pattern_extractor.push_str(line);
        }
        prev = line.len();
    }
    patterns.push(Pattern::build(&pattern_extractor, prev));

    let mut res = 0;
    for p in patterns {
        res += p.compute_val();
    }

    println!("{res}");
    Ok(())
}
