use std::{env, fs};

fn main() {
    let args = env::args()
        .nth(1)
        .expect("Erm... we need a file as input... erm...");
    let input = fs::read_to_string(args).expect("Erm... Unable to read file... erm");
    println!("Part 1: {}", part1(&input));
    //println!("Part 2: {}", part2(&input));
}

fn part1(input: &String) -> usize {
    input
        .lines()
        .map(|line| {
            line.chars()
                .collect::<Vec<_>>()
                .windows(3)
                .filter(|window| window[0] == '^' || window[2] == '^')
                .count()
        })
        .sum()
}
