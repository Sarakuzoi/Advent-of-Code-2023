use std::{
    cmp::max as maxi,
    cmp::min as mini,
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

struct Universe {
    sky: Vec<String>,
    no_of_galaxies: i64,
    map: HashMap<u64, (i64, i64)>,
    dup_rows: Vec<i64>,
    dup_cols: Vec<i64>,
}

impl Universe {
    fn build(lines: &Vec<&str>) -> Self {
        let sky = lines
            .clone()
            .iter_mut()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let mut dup_rows = Vec::new();
        let mut dup_cols = Vec::new();
        for i in 0..sky.len() {
            if !sky[i].contains("#") {
                dup_rows.push(i as i64);
            }
        }

        let mut galaxy_set = HashSet::new();
        for line in &sky {
            for (i, c) in line.char_indices() {
                if c == '#' {
                    galaxy_set.insert(i);
                }
            }
        }
        for i in 0..sky[0].len() {
            if !galaxy_set.contains(&i) {
                dup_cols.push(i as i64);
            }
        }

        Self {
            sky,
            map: HashMap::new(),
            no_of_galaxies: 0,
            dup_rows,
            dup_cols,
        }
    }

    fn index_galaxies(&mut self) {
        let mut cnt = 1;
        let mut map = HashMap::new();
        for i in 0..self.sky.len() {
            for (j, c) in self.sky[i].clone().char_indices() {
                if c == '#' {
                    map.insert(cnt, (i as i64, j as i64));
                    cnt += 1;
                }
            }
        }
        self.map = map;
        self.no_of_galaxies = cnt as i64;
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut input = File::open("input.txt")?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;

    let lines = contents.lines().collect::<Vec<&str>>();
    let mut universe = Universe::build(&lines);
    universe.index_galaxies();
    let mut res = 0;
    for i in 1..universe.no_of_galaxies as u64 {
        for j in i + 1..universe.no_of_galaxies as u64 {
            res += manhattan_distance(
                *universe.map.get(&i).unwrap(),
                *universe.map.get(&j).unwrap(),
                &universe.dup_rows,
                &universe.dup_cols,
            );
        }
    }
    println!("{res}");

    Ok(())
}

fn manhattan_distance(
    x: (i64, i64),
    y: (i64, i64),
    dup_rows: &Vec<i64>,
    dup_cols: &Vec<i64>,
) -> i64 {
    let mut cnt = 0;
    let (startr, endr, startc, endc) = (
        mini(x.0, y.0),
        maxi(x.0, y.0),
        mini(x.1, y.1),
        maxi(x.1, y.1),
    );
    for i in startr..endr {
        if dup_rows.contains(&i) {
            cnt += 1;
        }
    }
    for i in startc..endc {
        if dup_cols.contains(&i) {
            cnt += 1;
        }
    }
    (x.0.abs_diff(y.0) + x.1.abs_diff(y.1) + cnt * 999_999) as i64
}
