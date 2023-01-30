use common_macros::hash_map;
use std::collections::HashMap;
use std::fs;

fn calculate(rounds: &Vec<&str>, helper_map: HashMap<&str, i32>) -> i32 {
    return rounds
        .iter()
        .map(|round| helper_map.get(round).unwrap())
        .sum::<i32>();
}

fn main() {
    let input = fs::read_to_string("./src/zad02/input.txt").expect("Wrong file provided!");

    let rounds = input.split("\n").map(|round| round).collect::<Vec<_>>();

    // 1.
    let helper_results_map_1 = hash_map! {
        "A X" => 4,
        "A Y" => 8,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 7,
        "C Y" => 2,
        "C Z" => 6,
    };

    let res_1 = calculate(&rounds, helper_results_map_1);
    println!("{}", res_1);

    // 2.
    let helper_results_map_2 = hash_map! {
        "A X" => 3,
        "A Y" => 4,
        "A Z" => 8,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 2,
        "C Y" => 6,
        "C Z" => 7,
    };

    let res_2 = calculate(&rounds, helper_results_map_2);
    println!("{}", res_2);
}
