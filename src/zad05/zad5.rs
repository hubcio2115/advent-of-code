use std::fs;

fn main() {
    let operations: Vec<Vec<usize>> = fs::read_to_string("./src/zad05/input.txt")
        .expect("No input file!")
        .lines()
        .map(|line| {
            let words = line.split(" ").collect::<Vec<&str>>();

            return vec![
                words[1].parse::<usize>().unwrap(),
                words[3].parse::<usize>().unwrap(),
                words[5].parse::<usize>().unwrap(),
            ];
        })
        .collect::<Vec<Vec<usize>>>();

    let mut piles = vec![
        vec![],
        vec!['B', 'S', 'V', 'Z', 'G', 'P', 'W'],
        vec!['J', 'V', 'B', 'C', 'Z', 'F'],
        vec!['V', 'L', 'M', 'H', 'N', 'Z', 'D', 'C'],
        vec!['L', 'D', 'M', 'Z', 'P', 'F', 'J', 'B'],
        vec!['V', 'F', 'C', 'G', 'J', 'B', 'Q', 'H'],
        vec!['G', 'F', 'Q', 'T', 'S', 'L', 'B'],
        vec!['L', 'G', 'C', 'Z', 'V'],
        vec!['N', 'L', 'G'],
        vec!['J', 'F', 'H', 'C'],
    ];

    for operation in operations {
        let edited_vec = piles.get_mut(operation[1]).unwrap();

        let mut drained_items = edited_vec
            .drain((edited_vec.len() - operation[0])..)
            .collect::<Vec<char>>();

        // uncomment for answer for part 1
        // drained_items.reverse();

        piles[operation[2]].append(&mut drained_items);
    }

    for pile in piles {
        match pile.last() {
            Some(x) => print!("{x}"),
            None => print!(" "),
        }
    }
}
