use std::{fs::File, io::Read};
fn main() -> std::io::Result<()> {
    let mut input = File::open("input.txt")?;
    let mut contents = String::new();

    input.read_to_string(&mut contents)?;

    // Summing up the numbers extracted from each line
    let mut sum = 0;
    for line in contents.lines() {
        sum += find_digits(line).expect("Failed to find digits");
    }

    println!("{sum}");

    Ok(())
}

fn find_digits(line: &str) -> Result<i32, ()> {
    let digits = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero",
    ];

    // Extracting spelled-out digits
    let mut first_spelled_digit = None;
    let mut second_spelled_digit = None;
    for digit in digits {
        for (idx, matched) in line.match_indices(digit) {
            if first_spelled_digit.is_none() {
                first_spelled_digit = Some((
                    idx,
                    convert(matched).expect("Failed to parse word to a number."),
                ));
                second_spelled_digit = Some((
                    idx,
                    convert(matched).expect("Failed to parse word to a number."),
                ));
            }
            if idx < first_spelled_digit.map(|v: (usize, u32)| v.0).unwrap() {
                first_spelled_digit = Some((
                    idx,
                    convert(matched).expect("Failed to parse word to number"),
                ));
            }
            if idx > second_spelled_digit.map(|v: (usize, u32)| v.0).unwrap() {
                second_spelled_digit = Some((
                    idx,
                    convert(matched).expect("Failed to parse word to number"),
                ));
            }
        }
    }

    // We extract the digits and their positions using an iterator with filter and map
    let mut it = line
        .char_indices()
        .filter(|character| character.1.is_digit(10))
        .map(|character| (character.0, character.1.to_digit(10).unwrap()));

    // We can directly unwrap it, since AoC guarantees at least one digit
    let first_digit = it.next().unwrap();
    let second_digit = it.last();

    // We add all of the digits in a vector, together with their positions
    let mut total_digits = Vec::new();
    if let Some(x) = first_spelled_digit {
        total_digits.push(x);
    }
    if let Some(x) = second_spelled_digit {
        if x != first_spelled_digit.unwrap() {
            total_digits.push(x);
        }
    }
    total_digits.push(first_digit);
    if let Some(x) = second_digit {
        total_digits.push(x);
    }

    // We will then sort the vector, so we have access to the first and last digits
    // that compose our desired number
    total_digits.sort();

    // We return the desired number, handling the edge case in which there's only one digit
    if total_digits.len() == 1 {
        return Ok((total_digits[0].1 * 11) as i32);
    }

    Ok((total_digits[0].1 * 10 + total_digits.last().unwrap().1) as i32)
}

// Converts specific &str to u32
fn convert(word: &str) -> Result<u32, ()> {
    match word {
        "one" => Ok(1),
        "two" => Ok(2),
        "three" => Ok(3),
        "four" => Ok(4),
        "five" => Ok(5),
        "six" => Ok(6),
        "seven" => Ok(7),
        "eight" => Ok(8),
        "nine" => Ok(9),
        "zero" => Ok(0),
        _ => Err(()),
    }
}
