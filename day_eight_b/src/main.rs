use std::{collections::HashMap, fs::File, io::Read};

fn main() -> Result<(), std::io::Error> {
    let mut input = File::open("input.txt")?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;

    let mut map = HashMap::new();

    let mut it = contents.lines();
    let directions = it.next().unwrap().chars().collect::<Vec<char>>();

    it.next();
    while let Some(x) = it.next() {
        let words: Vec<&str> = x.split(" ").collect();
        let key = words[0];
        let left = &words[2][1..words[2].len() - 1];
        let right = &words[3][..words[3].len() - 1];
        map.insert(key, (left, right));
    }

    let mut dir_ind = 0;
    let mut positions = map
        .keys()
        .filter(|x| x.chars().last().unwrap() == 'A')
        .map(|x| (x.to_string(), 0))
        .collect::<Vec<(String, i32)>>();

    let mut final_pos = false;
    while !final_pos {
        final_pos = true;
        for i in 0..positions.len() {
            if positions[i].0.ends_with('Z') {
                continue;
            }
            positions[i] = match directions[dir_ind] {
                'L' => (
                    map.get(&positions[i].0.as_ref()).unwrap().0.to_string(),
                    positions[i].1 + 1,
                ),
                'R' => (
                    map.get(&positions[i].0.as_ref()).unwrap().1.to_string(),
                    positions[i].1 + 1,
                ),
                _ => panic!("Invalid direction."),
            };
            if !positions[i].0.ends_with('Z') {
                final_pos = false;
            }
        }
        dir_ind += 1;
        if dir_ind == directions.len() {
            dir_ind = 0;
        }
    }

    let mut res = positions[0].1 as u64;
    for i in 1..positions.len() {
        res = lcm(res as u64, positions[i].1 as u64);
    }

    println!("{res}");

    Ok(())
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
