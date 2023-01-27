use std::fs;

fn main() {
    // 1.
    let input = fs::read_to_string("input.txt").expect("Wrong file provided!");

    let counts = input
        .split("\n\n")
        .map(|cal| -> usize { cal.split("\n").map(|row| row.parse().unwrap_or(0)).sum() });

    let mut v = counts.collect::<Vec<_>>();
    v.sort();

    println!("{}", v[v.len() - 1]);

    // 2.

    let sum_of_best_three = v[v.len() - 1] + v[v.len() - 2] + v[v.len() - 3];
    println!("{}", sum_of_best_three)
}
