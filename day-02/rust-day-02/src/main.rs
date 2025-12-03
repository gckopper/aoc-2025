use std::{env, fs};

fn main() {
    let args = env::args()
        .nth(1)
        .expect("Erm... we need a file as input... erm...");
    let input = fs::read_to_string(args).expect("Erm... Unable to read file... erm");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u64 {
    return input.split(',').map(|range| {
        let (lhs, rhs) = range.trim().split_once('-').expect(&format!("No - in input! ({range})"));
        let start = lhs.parse::<u64>().expect(&format!("Failed to parse lhs ({lhs}) as u64"));
        let end = rhs.parse::<u64>().expect(&format!("Failed to parse rhs ({rhs}) as u64"));

        return (start..=end).map(|num| {
            let size = num.ilog10();
            if size % 2 == 0 {
                return 0;
            }
            let half = 10_u64.pow(size / 2 + 1);
            if num / half != num % half {
                return 0;
            };
            println!("Selected: {num}");
            return num;
        }).sum::<u64>();

    }).sum::<u64>();
}

fn part2(input: &str) -> u64 {
    return input.split(',').map(|range| {
        let (lhs, rhs) = range.trim().split_once('-').expect(&format!("No - in input! ({range})"));
        let start = lhs.parse::<u64>().expect(&format!("Failed to parse lhs ({lhs}) as u64"));
        let end = rhs.parse::<u64>().expect(&format!("Failed to parse rhs ({rhs}) as u64"));

        return (start..=end).map(|num| {
            let num_size = num.ilog10() + 1;
            if (1..10).filter(|size| num_size != *size && num_size % size == 0).map(|exp| {
                let window = 10_u64.pow(exp);
                let mut scratch = num;
                let refecence = num % window;
                let mut decision = true;
                while scratch > window {
                    decision &= scratch % window == refecence;
                    scratch /= window;
                }
                decision &= scratch == refecence;
                return decision;
            }).all(|e| !e) {
                return 0;
            }
            return num;
        }).sum::<u64>();

    }).sum::<u64>();
}
