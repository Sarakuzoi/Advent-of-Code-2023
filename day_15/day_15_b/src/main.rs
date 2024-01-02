#![feature(ascii_char)]
use std::{collections::HashMap, fs::File, io::Read};
fn main() -> Result<(), std::io::Error> {
    let mut input = File::open("input.txt")?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;

    let steps = contents.split(",").collect::<Vec<&str>>();
    let mut res = 0;
    let mut label_map = HashMap::new();
    let mut boxes_map = HashMap::new();
    let mut cnt = 0;
    for s in steps {
        cnt += 1;
        match s.chars().last().unwrap() {
            '-' => {
                if let Some(_) = label_map.get(&s[0..s.len() - 1]) {
                    label_map.remove_entry(&s[0..s.len() - 1]);
                }
            }
            _ => {
                let aux = s.split('=').collect::<Vec<&str>>();
                if let Some((x, _)) = label_map.get(aux[0]) {
                    label_map.insert(aux[0], (*x, aux[1].parse::<i32>().unwrap()));
                    continue;
                }
                label_map.insert(aux[0], (cnt, aux[1].parse::<i32>().unwrap()));
            }
        }
    }
    for (k, v) in label_map.iter() {
        let mut aux: Vec<(i32, i32)> = boxes_map.entry(hash(&k)).or_insert(Vec::new()).to_vec();
        aux.push(*v);
        boxes_map.insert(hash(&k), aux);
    }
    for (k, v) in boxes_map.iter_mut() {
        v.sort();
        let mut cnt = 0;
        for (_, x) in v.iter() {
            cnt += 1;
            res += (k + 1) * cnt * *x;
        }
    }
    println!("{res}");
    Ok(())
}

fn hash(s: &str) -> i32 {
    let mut res = 0;
    for c in s.chars() {
        res += c.as_ascii().unwrap().as_u8() as i32;
        res *= 17;
        res %= 256;
    }
    res
}
