use rayon::prelude::*;
use std::{collections::HashMap, fs::File, io::Read};

struct Converter {
    _id: i64,
    conversions: HashMap<(i64, i64), i64>,
}

impl Converter {
    fn new(id: i64) -> Self {
        Self {
            _id: id,
            conversions: HashMap::new(),
        }
    }

    fn add_interval(&mut self, line: &str) {
        let mut line = line.split(" ").map(|x| {
            x.parse::<i64>()
                .expect("Conversion line element was not a number")
        });

        let (dest, src, range) = (
            line.next().unwrap(),
            line.next().unwrap(),
            line.next().unwrap(),
        );

        self.conversions.insert((src, src + range - 1), dest - src);
    }
}

// This solution uses a kind of bruteforce approach, but I wanted to play around
// with parallel iterators (using Rayon), so if you try to run it,
// you'll have to wait through the ~40s execution time :)
fn main() -> Result<(), std::io::Error> {
    let mut input = File::open("input.txt")?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;

    let mut seeds = extract_seeds(&contents);
    println!("{:?}", seeds);
    let mut converters: Vec<Converter> = Vec::new();

    let mut id = 0;
    for line in contents.lines().skip(2) {
        if line.is_empty() {
            continue;
        }
        if line.contains(|x: char| x.is_alphabetic()) {
            id += 1;
            converters.push(Converter::new(id));
            continue;
        }
        converters.last_mut().unwrap().add_interval(line);
    }

    let mut mini = std::i64::MAX;
    for seed in seeds.iter_mut() {
        let x = (seed.0..(seed.0 + seed.1))
            .into_par_iter()
            .map(|x| {
                let mut aux = x;
                for i in 0..7 {
                    for (k, v) in converters.get(i).unwrap().conversions.iter() {
                        if aux >= k.0 && aux <= k.1 {
                            aux += v;
                            break;
                        }
                    }
                }
                aux
            })
            .min()
            .unwrap();
        mini = std::cmp::min(mini, x);
    }

    println!("{mini}");

    Ok(())
}

fn extract_seeds(contents: &str) -> Vec<(i64, i64)> {
    let mut seeds = Vec::new();
    let mut it = contents.lines().next().unwrap().split(" ").into_iter();
    it.next();
    while let Some(x) = it.next() {
        println!("{x}");
        seeds.push((
            x.parse::<i64>().expect("Input was not a number"),
            it.next()
                .unwrap()
                .parse::<i64>()
                .expect("Input was not a number"),
        ));
    }
    seeds
}
