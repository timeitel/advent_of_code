use std::fs;

// first 4 chars that are different
fn process(file: &str) -> usize {
    let mut start_of_packet: Vec<char> = vec![];

    for (i, ch) in file.chars().enumerate() {
        let contains = start_of_packet.contains(&ch);

        if contains {
            let position = start_of_packet.iter().position(|x| *x == ch).unwrap();
            start_of_packet.drain(..=position);
        }

        start_of_packet.push(ch);

        if start_of_packet.len() == 14 {
            return i + 1;
        }
    }

    0
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let result = process(&file);
    println!("{result}");
}

#[test]
fn passes() {
    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const INPUT_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    // const INPUT_3: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    // const INPUT_4: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    assert_eq!(process(INPUT), 19);
    assert_eq!(process(INPUT_2), 23);
    // assert_eq!(process(INPUT_3), 10);
    // assert_eq!(process(INPUT_4), 11);
}
