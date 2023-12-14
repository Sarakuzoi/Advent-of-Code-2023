use std::{cmp, fs::File, io::Read};

fn main() -> Result<(), std::io::Error> {
    let mut input = File::open("input.txt")?;
    let mut contents = String::new();

    input.read_to_string(&mut contents)?;

    let mut sum = 0;
    for line in contents.lines() {
        let game = Game::build(line, (12, 13, 14));
        if let Some(x) = game {
            sum += x.id;
        }
    }

    println!("{sum}");

    Ok(())
}

// This struct with both attributs is not neccessary, but I wanted some practice
#[allow(dead_code)]
pub struct Game {
    id: u32,
    max_colors: (u32, u32, u32),
}

impl Game {
    fn build(line: &str, filter: (u32, u32, u32)) -> Option<Self> {
        let words: Vec<&str> = line.split(" ").collect();
        // We get the ID as the second 'word' and parse it into a u32
        let id = words[1].replace(":", "").parse::<u32>().unwrap();

        let (mut r_max, mut g_max, mut b_max) = (0, 0, 0);
        let (mut r, mut g, mut b) = (0, 0, 0);
        for ind in 2..words.len() {
            // We check for valid numbers and increment color counters accordingly
            if words[ind].parse::<u32>().is_ok() {
                match &words[ind + 1].chars().next().unwrap() {
                    'r' => r += words[ind].parse::<u32>().unwrap(),
                    'g' => g += words[ind].parse::<u32>().unwrap(),
                    'b' => b += words[ind].parse::<u32>().unwrap(),
                    _ => panic!("Incorrect reading"),
                }
            // We reset our color counters and update the max colors found
            } else if words[ind].chars().last() != Some(',') {
                r_max = cmp::max(r_max, r);
                g_max = cmp::max(g_max, g);
                b_max = cmp::max(b_max, b);
                (r, g, b) = (0, 0, 0);
            }
        }

        // We handle our filtering in the build method, as the game could not exist if more balls
        // of some color were extracted than the total for that color from the bag
        if r_max > filter.0 || g_max > filter.1 || b_max > filter.2 {
            return None;
        }
        Some(Game {
            id: id,
            max_colors: (r_max, g_max, b_max),
        })
    }
}
