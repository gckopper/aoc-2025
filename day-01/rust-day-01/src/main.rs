fn main() {
    let input = include_str!("../../input-mini.txt");

    let mut zeros = 0;

    input.lines().map(|line| {
        let (direction, number) = line.split_at(1);
        let number = number.parse::<i64>().expect("Failed to parse a number");

        if direction == "L" {
            return -number;
        }
        return number;
    }).fold(50, |acc, e| {
        let partial = acc + e;

        println!("partial: {partial} = {acc} + {e}");

        if partial < 0 && acc != 0 {
            println!("Ding!");
            zeros += 1;
        }

        let rounds = ((partial / 100) - (acc / 100)).abs();

        println!("rounds: {rounds}");

        zeros += rounds;

        if partial == 0 && rounds == 0 {
            println!("Ding!");
            zeros += 1;
        }

        return (100 + (partial % 100)) % 100;
    });

    println!("Answer: {zeros}");
}

/* part 1 i think
    let mut zeros = 0;

    input.lines().map(|line| {
        let (direction, number) = line.split_at(1);
        let number = number.parse::<i32>().expect("Failed to parse a number");

        if direction == "L" {
            return -number;
        }
        return number;
    }).fold(50, |acc, e| {
        let partial = acc + e;

        println!("{partial} = {acc} + {e}");

        if partial == 0 {
            println!("Ding!");
            zeros += 1;
        }

        return partial;
    });
*/
