use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

struct Universe {
    sky: Vec<String>,
    no_of_galaxies: i32,
    map: HashMap<u32, (i32, i32)>,
}

impl Universe {
    fn build(lines: &Vec<&str>) -> Self {
        let it = vec!["."; lines[0].len()].into_iter();
        let mut sky = lines
            .clone()
            .iter_mut()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let universe_expander = String::from_iter(it);
        let mut i = 0;
        while i < sky.len() {
            if !sky[i].contains("#") {
                sky.insert(i, universe_expander.clone());
                i += 1;
            }
            i += 1;
        }

        let mut galaxy_set = HashSet::new();
        for line in &sky {
            for (i, c) in line.char_indices() {
                if c == '#' {
                    galaxy_set.insert(i);
                }
            }
        }
        let mut columns_to_expand = Vec::new();
        for i in 0..sky[0].len() {
            if !galaxy_set.contains(&i) {
                columns_to_expand.push(i);
            }
        }
        let mut aux = 0;
        for i in 0..columns_to_expand.len() {
            columns_to_expand[i] = columns_to_expand[i] + aux;
            aux += 1;
        }
        for line in &mut sky {
            for pos in &columns_to_expand {
                String::insert(line, *pos, '.');
            }
        }

        Self {
            sky,
            map: HashMap::new(),
            no_of_galaxies: 0,
        }
    }

    fn index_galaxies(&mut self) {
        let mut cnt = 1;
        let mut map = HashMap::new();
        for i in 0..self.sky.len() {
            for (j, c) in self.sky[i].clone().char_indices() {
                if c == '#' {
                    map.insert(cnt, (i as i32, j as i32));
                    cnt += 1;
                }
            }
        }
        self.map = map;
        self.no_of_galaxies = cnt as i32;
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
    for i in 1..universe.no_of_galaxies as u32 {
        for j in i + 1..universe.no_of_galaxies as u32 {
            res += manhattan_distance(
                *universe.map.get(&i).unwrap(),
                *universe.map.get(&j).unwrap(),
            );
        }
    }
    println!("{res}");

    Ok(())
}

fn manhattan_distance(x: (i32, i32), y: (i32, i32)) -> i32 {
    (x.0.abs_diff(y.0) + x.1.abs_diff(y.1)) as i32
}
