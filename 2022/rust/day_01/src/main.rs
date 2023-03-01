use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut max: u32 = 0;
    let mut counter: u32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            if counter > max {
                max = counter;
            }
            counter = 0;
        } else {
            counter += line.parse::<u32>().unwrap();
        }
    }

    println!("Max is: {}", max);
    Ok(())
}
