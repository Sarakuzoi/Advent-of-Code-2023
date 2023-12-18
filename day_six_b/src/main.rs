use std::{fs::File, io::Read};

fn main() -> Result<(), std::io::Error> {
    let mut input = File::open("input.txt")?;
    let mut contents = String::new();

    input.read_to_string(&mut contents)?;
    let mut p = 1;
    let (time, distance) = (
        extract_vals(contents.lines().next().unwrap()),
        extract_vals(contents.lines().skip(1).next().unwrap()),
    );
    println!("{time} {distance}");

    let high = (time as f64 + ((time * time - 4 * distance) as f64).sqrt()) / 2.0;
    let low = (time as f64 - ((time * time - 4 * distance) as f64).sqrt()) / 2.0;
    p *= (high - 0.00000001).floor() as i32 - (low + 0.00000001).ceil() as i32 + 1;

    println!("{}", p as i32);
    Ok(())
}

fn extract_vals(line: &str) -> i64 {
    let mut res = Vec::new();
    for n in line.split(" ").filter(|x| *x != "").skip(1) {
        res.push(n);
    }
    res.concat()
        .parse::<i64>()
        .expect("Couldn't parse number string.")
}
