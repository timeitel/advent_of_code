use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut counter = 0;
    let mut top_three: Vec<u32> = vec![0; 3];

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            if counter > top_three.last().copied().unwrap() {
                top_three.pop();
                top_three.push(counter);
                top_three.sort_by(|a, b| b.cmp(a));
            }
            counter = 0;
        } else {
            counter += line.parse::<u32>().unwrap();
        }
    }

    println!("Top 3 elves: {:?}", top_three);
    println!("Sum of: {}", top_three.iter().sum::<u32>());
    Ok(())
}
