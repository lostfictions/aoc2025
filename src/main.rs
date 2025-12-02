use std::fs::read_to_string;

fn _part1(input: &str) -> u32 {
    let mut value = 50i32;

    input
        .trim()
        .split('\n')
        .map(|l| l.trim())
        .fold(0, |acc, line| {
            let (dir, numstr) = line.split_at(1);
            let num = numstr.parse::<i32>().expect("couldn't parse");

            match dir {
                "L" => value = (value - num).rem_euclid(100),
                "R" => value = (value + num).rem_euclid(100),
                _ => unreachable!(),
            }

            if value == 0 { acc + 1 } else { acc }
        })
}

fn _part2(input: &str) -> u32 {
    let mut value = 50i32;
    let mut zero_count = 0u32;

    for line in input.trim().split('\n').map(|l| l.trim()) {
        let (dir, numstr) = line.split_at(1);
        let num = numstr.parse::<i32>().expect("couldn't parse");

        // quadratic but easy
        for _ in 0..num {
            match dir {
                "L" => value -= 1,
                "R" => value += 1,
                _ => unreachable!("weird dir"),
            }

            match value {
                0 | 100 => {
                    value = 0;
                    zero_count += 1;
                }
                -1 => value = 99,
                _ => (),
            }
        }
    }

    zero_count
}

fn main() {
    let input = read_to_string("./inputs/1").expect("couldn't read file");

    // let res = _part1(&input);
    let res = _part2(&input);
    println!("{res}");
}
