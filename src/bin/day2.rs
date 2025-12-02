use advent2025::read_input;
use fancy_regex::Regex;

fn _part1(input: &str) -> u64 {
    let re = Regex::new(r"^(\d+)\1$").unwrap();

    let input = input.trim().split(',').map(|seg| {
        let (start, end) = seg.split_once('-').expect("expected hyphen");
        (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
    });

    let mut sum = 0u64;

    for (start, end) in input {
        for i in start..=end {
            if re.is_match(&i.to_string()).expect("couldn't run regex") {
                // println!("{i}");
                sum += i
            }
        }
    }

    sum
}

// same as above but regex is \1+ instead of \1
fn _part2(input: &str) -> u64 {
    let re = Regex::new(r"^(\d+)\1+$").unwrap();

    let input = input.trim().split(',').map(|seg| {
        let (start, end) = seg.split_once('-').expect("expected hyphen");
        (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
    });

    let mut sum = 0u64;

    for (start, end) in input {
        for i in start..=end {
            if re.is_match(&i.to_string()).expect("couldn't run regex") {
                // println!("{i}");
                sum += i
            }
        }
    }

    sum
}

fn main() {
    let input = read_input(file!());
    // let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    // let res = _part1(&input);
    let res = _part2(&input);
    println!("result: {res}");
}
