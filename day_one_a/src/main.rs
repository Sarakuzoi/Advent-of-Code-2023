use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read the file");
    let mut sum = 0;
    for line in input.lines() {
        sum += extract_number(line);
    }
    println!("The result is {sum}");
}

fn extract_number(line: &str) -> i32 {
    let mut num = 0;
    for c in line.chars() {
        if c.is_digit(10) {
            num += c.to_digit(10).unwrap();
            num *= 10;
            break;
        }
    }
    for c in line.chars().rev() {
        if c.is_digit(10) {
            num += c.to_digit(10).unwrap();
            break;
        }
    }
    num.try_into().unwrap()
}
