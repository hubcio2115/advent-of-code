use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Wrong file provided!");
    let lines = input.lines();

    // 1.
    let res1: usize = lines
        .clone()
        .map(|line| {
            let pair = line.split(',').collect::<Vec<&str>>();
            let (left, right) = (
                pair[0]
                    .split("-")
                    .map(|number_as_str| number_as_str.parse::<usize>().unwrap())
                    .into_iter()
                    .collect::<Vec<usize>>(),
                pair[1]
                    .split("-")
                    .map(|number_as_str| number_as_str.parse::<usize>().unwrap())
                    .into_iter()
                    .collect::<Vec<usize>>(),
            );

            let left_range = left[0]..left[1];
            let right_range = right[0]..right[1];

            if left_range.start >= right_range.start && left_range.end <= right_range.end {
                return 1;
            }

            if right_range.start >= left_range.start && right_range.end <= left_range.end {
                return 1;
            }

            return 0;
        })
        .collect::<Vec<usize>>()
        .iter()
        .sum();

    println!("{:?}", res1);

    // 2.
    let res2: usize = lines
        .map(|line| {
            let pair = line.split(',').collect::<Vec<&str>>();
            let (left, right) = (
                pair[0]
                    .split("-")
                    .map(|number_as_str| number_as_str.parse::<usize>().unwrap())
                    .into_iter()
                    .collect::<Vec<usize>>(),
                pair[1]
                    .split("-")
                    .map(|number_as_str| number_as_str.parse::<usize>().unwrap())
                    .into_iter()
                    .collect::<Vec<usize>>(),
            );

            let left_range = left[0]..left[1];
            let right_range = right[0]..right[1];

            if (left_range.start >= right_range.start && left_range.start <= right_range.end)
                || (left_range.end >= right_range.start && left_range.end <= right_range.end)
            {
                return 1;
            }

            if (right_range.start >= left_range.start && right_range.start <= left_range.end)
                || (right_range.end >= left_range.start && right_range.end <= left_range.end)
            {
                return 1;
            }

            return 0;
        })
        .collect::<Vec<usize>>()
        .iter()
        .sum();

    println!("{:?}", res2);
}
