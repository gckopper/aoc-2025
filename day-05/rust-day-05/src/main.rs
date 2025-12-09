use std::{cmp::max, env, fs};

fn main() {
    let args = env::args()
        .nth(1)
        .expect("Erm... we need a file as input... erm...");
    let input = fs::read_to_string(args).expect("Erm... Unable to read file... erm");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &String) -> usize {
    let (ranges_chunk, foods) = input.split_once("\n\n").unwrap();
    let ranges = ranges_chunk.lines().map(|range| {
        let (start, end) = range.split_once('-').unwrap();
        return (start.parse::<usize>().unwrap(), end.parse::<usize>().unwrap());
    }).collect::<Vec<_>>();
    let freshies = foods.lines().filter_map(|line| {
        let food = line.parse::<usize>().unwrap();
        for (start, end) in &ranges {
            if food >= *start && *end >= food {
                return Some(());
            }
        }
        return None;
    }).count();
    return freshies;
}

fn part2(input: &String) -> usize {
    let (ranges_chunk, _) = input.split_once("\n\n").unwrap();
    let mut ranges = ranges_chunk.lines().map(|range| {
        let (start, end) = range.split_once('-').unwrap();
        return (start.parse::<usize>().unwrap(), end.parse::<usize>().unwrap());
    }).collect::<Vec<_>>();
    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    println!("{:?}", &ranges);

    let mut total: Vec<(usize, usize)> = Vec::new();

    total.push(ranges[0]);

    for i in 1..ranges.len() {
        let (start, end) = ranges[i].clone();
        if start <= total.last().unwrap().1 {
            total.iter_mut().last().unwrap().1 = max(end, total.last().unwrap().1);
            continue;
        }
        total.push((start, end));
    }
    
    println!("{:?}", &total);
    println!("{:?}", &total.len());
    return total.iter().map(|(start, end)| end - start + 1).sum();
}
