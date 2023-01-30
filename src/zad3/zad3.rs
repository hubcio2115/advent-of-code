use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("./src/zad3/input.txt").expect("Wrong file provided!");
    let alphanumeric = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .split("")
        .collect::<Vec<&str>>();

    // 3.

    let result: usize = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);

            let left_set = left.split("").collect::<HashSet<&str>>();
            let right_set = right.split("").collect::<HashSet<&str>>();

            let mut common_characters = left_set
                .intersection(&right_set)
                .cloned()
                .collect::<Vec<&str>>();

            if let Some(i) = common_characters
                .iter()
                .position(|character| *character == "")
            {
                common_characters.remove(i);
            }

            return alphanumeric
                .iter()
                .position(|letter| *letter == common_characters[0])
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
            let first = chunk[0].split("").collect::<HashSet<_>>();
            let second = chunk[1].split("").collect::<HashSet<_>>();
            let third = chunk[2].split("").collect::<HashSet<_>>();

            let temp = first.intersection(&second).cloned().collect::<HashSet<_>>();
            let mut common = temp.intersection(&third).collect::<Vec<_>>();

            if let Some(i) = common.iter().position(|character| *character == &"") {
                common.remove(i);
            }

            return alphanumeric
                .iter()
                .position(|letter| letter.eq(common[0]))
                .unwrap();
        })
        .into_iter()
        .sum();

    println!("{result2}");
}
