use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Input not found.");

    if let Some(first_packet) = find_first_packet(&input) {
        let number_of_characters = count_characters_before_packet(&input, first_packet);
        println!("Result: {}", number_of_characters);
    } else {
        println!("Could not find.");
    };
}

fn find_first_packet(input: &String) -> Option<usize> {
    if input.len() < 14 {
        return None;
    }

    for i in 0..=input.len() - 14 {
        let substring = &input[i..i + 14];
        let unique_chars: Vec<_> = substring.chars().collect();
        if unique_chars
            .iter()
            .all(|&c| unique_chars.iter().filter(|&&x| x == c).count() == 1)
        {
            return Some(i + 14);
        }
    }

    None
}

fn count_characters_before_packet(input: &String, first_packet: usize) -> usize {
    input.chars().collect::<Vec<char>>()[..first_packet].len()
}
