use std::{env, fs, usize};

fn main() {
    let args = env::args()
        .nth(1)
        .expect("Erm... we need a file as input... erm...");
    let input = fs::read_to_string(args).expect("Erm... Unable to read file... erm");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &String) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let operators = lines
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|op| {
            if op.trim() == "+" {
                usize::strict_add
            } else {
                usize::strict_mul
            }
        })
        .collect::<Vec<_>>();
    let numbers = lines
        .iter()
        .take(lines.len() - 1)
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.trim().parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (0..numbers[0].len())
        .map(|i| {
            (0..numbers.len())
                .map(|j| numbers[j][i])
                .reduce(operators[i])
                .unwrap()
        })
        .sum()
}

fn part2(input: &String) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let mut starts = lines
        .last()
        .unwrap()
        .chars()
        .enumerate()
        .filter_map(|c| {
            if c.1 == ' ' {
                return None;
            }
            Some(c.0)
        })
        .collect::<Vec<_>>();
    starts.push(lines.last().unwrap().len() + 1);
    println!("starts: {:?}", starts);

    let num_nums = starts
        .windows(2)
        .map(|range| {
            let hori_nums = lines
                .iter()
                .take(lines.len() - 1)
                .map(|line| {
                    line.chars()
                        .skip(range[0])
                        .take(range[1] - range[0] - 1)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            (0..hori_nums[0].len())
                .map(|i| {
                    let num = (0..hori_nums.len()).map(|j| hori_nums[j][i]).fold(
                        String::new(),
                        |mut acc, e| {
                            acc.push(e);
                            acc
                        },
                    );
                    println!("{num}");
                    num.trim().parse::<usize>().unwrap()
                })
                .reduce(
                    if lines.last().unwrap().chars().skip(range[0]).next().unwrap() == '+' {
                        usize::strict_add
                    } else {
                        usize::strict_mul
                    },
                ).unwrap()
        })
        .sum();
    return num_nums;
}
