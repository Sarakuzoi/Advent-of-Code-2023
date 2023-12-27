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

    fn compute_val(&mut self) -> i32 {
        let mut res = 0;
        let mut smudge_cnt = 0;
        for r_ind in 1..self.row_mirror.len() {
            if let Some(_) =
                Self::contains_smudge(&self.row_mirror[r_ind], &self.row_mirror[r_ind - 1])
            {
                if let Some(x) = Self::valid_split(&self.row_mirror, r_ind) {
                    if smudge_cnt == 0 {
                        res = 0;
                    }
                    res += r_ind * 100;
                    if x.0 != -1 {
                        smudge_cnt += 1;
                        self.col_mirror[x.1 as usize][x.0 as usize] =
                            match self.col_mirror[x.1 as usize][x.0 as usize] {
                                '.' => '#',
                                '#' => '.',
                                _ => unreachable!(),
                            };
                        break;
                    }
                }
            }
        }
        if res != 0 && smudge_cnt != 0 {
            return res as i32;
        }
        for c_ind in 1..self.col_mirror.len() {
            if let Some(k) =
                Self::contains_smudge(&self.col_mirror[c_ind], &self.col_mirror[c_ind - 1])
            {
                if k != -1 && smudge_cnt > 0 {
                    break;
                }
                if let Some(x) = Self::valid_split(&self.col_mirror, c_ind) {
                    if smudge_cnt == 0 && x.0 != -1 {
                        res = c_ind;
                        break;
                    }
                    if smudge_cnt > 0 && x.0 == -1 {
                        res += c_ind;
                        break;
                    }
                }
            }
        }
        res as i32
    }

    fn valid_split(matrix: &Vec<Vec<char>>, i: usize) -> Option<(i32, i32)> {
        let (mut up, mut down) = ((i - 1) as i32, (i) as i32);
        let mut smudge_cnt = 0;
        let mut res = Some((-1, -1));
        while up >= 0 && down < matrix.len() as i32 {
            if matrix[up as usize] != matrix[down as usize] {
                if let Some(x) = Self::contains_smudge(&matrix[up as usize], &matrix[down as usize])
                {
                    smudge_cnt += 1;
                    res = Some((up, x as i32));
                    if smudge_cnt > 1 {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            up -= 1;
            down += 1;
        }
        res
    }

    fn contains_smudge(a: &Vec<char>, b: &Vec<char>) -> Option<i32> {
        let mut diff_cnt = 0;
        let mut res = -1;
        for i in 0..a.len() {
            if a[i] != b[i] {
                diff_cnt += 1;
                if diff_cnt == 1 {
                    res = i as i32;
                }
            }
            if diff_cnt > 1 {
                return None;
            }
        }
        Some(res)
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
    for p in patterns.iter_mut() {
        res += p.compute_val();
    }

    println!("{res}");
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::Pattern;

    const PATTERN: &str = ".##.#...##.#..#..#.##.#...####.#..##..##.##..##.###.#..#.##..##";

    #[test]
    fn test_contains_smudge() {
        let mut a = vec!['.', '#', '.'];
        let mut b = vec!['.', '.', '.'];
        assert_eq!(Some(1), Pattern::contains_smudge(&a, &b));
        a[0] = '#';
        b[1] = '#';
        assert_eq!(Some(0), Pattern::contains_smudge(&a, &b));
    }

    #[test]
    fn test_does_not_contains_smudge() {
        let a = vec!['.', '#', '#'];
        let b = vec!['.', '.', '.'];
        assert_eq!(None, Pattern::contains_smudge(&a, &b));
    }

    #[test]
    fn compute_val() {
        let mut x = Pattern::build(PATTERN, 7);
        assert_eq!(100, x.compute_val());
    }
}
