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

    let mut cnt = 0;
    let mut dir_ind = 0;
    let mut current = "AAA";
    while current != "ZZZ" {
        match directions[dir_ind] {
            'L' => {
                current = map.get(current).unwrap().0;
            }
            'R' => {
                current = map.get(current).unwrap().1;
            }
            _ => panic!("Invalid direction"),
        }
        cnt += 1;
        dir_ind += 1;
        if dir_ind == directions.len() {
            dir_ind = 0;
        }
    }

    println!("{cnt}");

    Ok(())
}
