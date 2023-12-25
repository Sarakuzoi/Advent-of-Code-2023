use std::{fs::File, io::Read, iter::zip};

fn main() -> Result<(), std::io::Error> {
    let mut input = File::open("input.txt")?;
    let mut contents = String::new();

    input.read_to_string(&mut contents)?;
    let mut p = 1;
    let (times, distances) = (
        extract_vals(contents.lines().next().unwrap()),
        extract_vals(contents.lines().skip(1).next().unwrap()),
    );

    let iter = zip(times, distances);
    for (t, d) in iter {
        let high = (t as f64 + ((t * t - 4 * d) as f64).sqrt()) / 2.0;
        let low = (t as f64 - ((t * t - 4 * d) as f64).sqrt()) / 2.0;
        p *= (high - 0.00000001).floor() as i32 - (low + 0.00000001).ceil() as i32 + 1;
    }

    println!("{}", p as i32);
    Ok(())
}

fn extract_vals(line: &str) -> Vec<i32> {
    let mut res = Vec::new();
    for n in line.split(" ").filter(|x| *x != "").skip(1) {
        res.push(n.parse::<i32>().unwrap());
    }
    res
}
