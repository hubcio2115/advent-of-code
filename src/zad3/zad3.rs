use std::{collections::HashSet, fs};

fn clean_common_characters(characters: Vec<&str>) -> &str {
    return characters
        .iter()
        .filter(|character| *character != &"")
        .cloned()
        .collect::<Vec<_>>()[0];
}

fn main() {
    let input = fs::read_to_string("./src/zad3/input.txt").expect("Wrong file provided!");
    let alphanumeric = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .split("")
        .collect::<Vec<&str>>();

    // 1.
    let result: usize = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);

            let (left_set, right_set) = (
                left.split("").collect::<HashSet<&str>>(),
                right.split("").collect::<HashSet<&str>>(),
            );

            let common_character = clean_common_characters(
                left_set
                    .intersection(&right_set)
                    .cloned()
                    .collect::<Vec<&str>>(),
            );

            return alphanumeric
                .iter()
                .position(|letter| letter == &common_character)
                .unwrap();
        })
        .into_iter()
        .sum();

    println!("{result}");

    // 2.
    let result2: usize = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            let (first, second, third) = (
                chunk[0].split("").collect::<HashSet<_>>(),
                chunk[1].split("").collect::<HashSet<_>>(),
                chunk[2].split("").collect::<HashSet<_>>(),
            );

            let common_characters = first
                .intersection(&second)
                .cloned()
                .collect::<HashSet<&str>>();

            let common_character = clean_common_characters(
                common_characters
                    .intersection(&third)
                    .cloned()
                    .collect::<Vec<_>>(),
            );

            return alphanumeric
                .iter()
                .position(|letter| letter == &common_character)
                .unwrap();
        })
        .into_iter()
        .sum();

    println!("{result2}");
}
