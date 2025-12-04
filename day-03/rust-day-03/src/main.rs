use std::{env, fs};

fn main() {
    let args = env::args()
        .nth(1)
        .expect("Erm... we need a file as input... erm...");
    let input = fs::read_to_string(args).expect("Erm... Unable to read file... erm");
    println!("Part 1: {}", part2(&input, 2));
    println!("Part 2: {}", part2(&input, 12));
}

fn part1(input: &str) -> u32 {
    return input
        .lines()
        .map(|bank| {
            let mut high_algarism = 0;
            for (i, battery) in bank.chars().enumerate().take(bank.len() - 1) {
                if battery > bank.chars().nth(high_algarism).unwrap() {
                    high_algarism = i;
                }
            }
            let mut joltage = 10
                * bank
                    .chars()
                    .nth(high_algarism)
                    .unwrap()
                    .to_digit(10)
                    .unwrap();
            let mut low_algarism = high_algarism + 1;
            for (i, battery) in bank.chars().enumerate().skip(low_algarism) {
                if battery > bank.chars().nth(low_algarism).unwrap() && i != high_algarism {
                    low_algarism = i;
                }
            }
            joltage += bank
                .chars()
                .nth(low_algarism)
                .unwrap()
                .to_digit(10)
                .unwrap();
            return joltage;
        })
        .sum::<u32>();
}

fn part2(input: &str, size: u32) -> u64 {
    return input
        .lines()
        .map(|bank| {
            let mut joltage = 0;
            let mut algarism = 0;
            (0..size).for_each(|j: u32| {
                let skip = algarism;
                let take = bank.len() - algarism - ((size - 1 - j) as usize);
                for (i, battery) in bank.chars().enumerate().skip(skip).take(take) {
                    if battery > bank.chars().nth(algarism).unwrap() {
                        algarism = i;
                    }
                }
                joltage += 10_u64.pow(size - 1 - j) as u64
                    * bank.chars().nth(algarism).unwrap().to_digit(10).unwrap() as u64;
                algarism += 1;
            });
            return joltage;
        })
        .sum::<u64>();
}
