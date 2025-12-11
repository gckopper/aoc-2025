use std::{env, fs};

fn main() {
    let args = env::args()
        .nth(1)
        .expect("Erm... we need a file as input... erm...");
    let input = fs::read_to_string(args).expect("Erm... Unable to read file... erm");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &String) -> usize {
    let line_size = input.lines().next().unwrap().len();
    let mut rays = vec![vec![false; line_size], vec![false; line_size]];
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let total = lines.windows(2).map(|liners| {
        rays.swap(0, 1);
        rays[1].fill_with(|| false);
        let rays_casted = liners[0].windows(3).enumerate().zip(liners[1].windows(3)).map(|((i, top), down)| {
            if rays[0][i+1] || top[1] == 'S' {
                if down[1] == '^' {
                    rays[1][i+2] = true;
                    rays[1][i] = true;
                    return 1;
                }
                rays[1][i+1] = true;
            }
            return 0;
        }).sum::<usize>();
        return rays_casted;
    }).sum::<usize>();
    return total;
}

fn part2(input: &String) -> usize {
    let line_size = input.lines().next().unwrap().len();
    let mut rays = vec![vec![0; line_size], vec![0; line_size]];
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    lines.windows(2).for_each(|liners| {
        rays.swap(0, 1);
        rays[1].fill_with(|| 0);
        liners[0].windows(3).enumerate().zip(liners[1].windows(3)).for_each(|((i, top), down)| {
            if  top[1] == 'S' {
                rays[1][i+1] += 1;
            }
            if rays[0][i+1] > 0 {
                if down[1] == '^' {
                    rays[1][i+2] += rays[0][i+1];
                    rays[1][i] += rays[0][i+1];
                    return;
                }
                rays[1][i+1] += rays[0][i+1];
            }
        });
    });
    return rays.first().unwrap().iter().sum();
}
