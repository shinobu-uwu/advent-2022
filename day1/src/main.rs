use itertools::Itertools;

fn main() {
    let vec: Vec<i32> = std::fs::read_to_string("input")
        .unwrap()
        .trim()
        .split("\n\n")
        .map(|chunk| -> i32 {
            chunk
                .split('\n')
                .map(|line| str::parse::<i32>(line).unwrap())
                .sum()
        })
        .sorted()
        .rev()
        .collect();

    println!(
        "Part 1: {}\nPart 2: {}",
        vec[0],
        vec.as_slice()[..3].iter().sum::<i32>()
    );
}
