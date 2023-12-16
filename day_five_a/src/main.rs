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

fn main() -> Result<(), std::io::Error> {
    let mut input = File::open("input.txt")?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;

    let mut seeds = extract_seeds(&contents);
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
        let mut aux = *seed;
        for i in 0..7 {
            for (k, v) in converters.get(i).unwrap().conversions.iter() {
                if aux >= k.0 && aux <= k.1 {
                    aux += v;
                    break;
                }
            }
        }
        mini = std::cmp::min(mini, aux);
    }

    println!("{mini}");

    Ok(())
}

fn extract_seeds(contents: &str) -> Vec<i64> {
    let mut seeds = Vec::new();
    for word in contents.lines().next().unwrap().split(" ").skip(1) {
        seeds.push(word.parse::<i64>().expect("Failed to parse seed number"));
    }
    seeds
}
