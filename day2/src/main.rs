use std::collections::HashMap;

fn main() {
    let mut advantages = HashMap::new();
    advantages.insert('A', 'C');
    advantages.insert('B', 'A');
    advantages.insert('C', 'B');

    let a: u32 = include_str!("../input")
        .trim()
        .split('\n')
        .map(|line| {
            let mut iter = line.split(' ').map(|s| s.chars().next().unwrap());
            let (opponent_play, my_play) = (
                iter.next().unwrap(),
                ((iter.next().unwrap() as u8) - 23) as char,
            );

            if opponent_play == my_play {
                3 + my_play as u32 - 64
            } else if *advantages.get(&opponent_play).unwrap() != my_play {
                6 + my_play as u32 - 64
            } else {
                my_play as u32 - 64
            }
        })
        .sum();

    println!("{a}");
}
