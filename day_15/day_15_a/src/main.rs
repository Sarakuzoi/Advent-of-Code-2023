#![feature(ascii_char)]
use std::{fs::File, io::Read};
fn main() -> Result<(), std::io::Error> {
    let mut input = File::open("input.txt")?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;

    let steps = contents.split(",").collect::<Vec<&str>>();
    let mut res = 0;
    for s in steps {
        res += hash(s);
    }
    println!("{}", res);
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
