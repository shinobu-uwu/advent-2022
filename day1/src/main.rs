fn main() {
    let result: i32 = std::fs::read_to_string("input")
        .unwrap()
        .trim()
        .split("

")
        .map(|chunk| {
            chunk
                .split(n)
                .map(|line| str::parse::<i32>(line).unwrap())
                .sum()
        })
        .max()
        .unwrap();

    println!("{}", result);
}
