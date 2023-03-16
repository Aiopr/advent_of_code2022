use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("./input/day01.in").unwrap();
    let reader = BufReader::new(file);
    let mut elves = Vec::new();

    let mut elf = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            elves.push(elf);
            elf = 0;
        } else {
            let cnt = line.parse::<i32>().unwrap();
            elf += cnt;
        }
    }

    // Push the sum of the last group
    elves.push(elf);
    let top_one = elves.iter().max().unwrap();
    println!("Part 1: {}", top_one);

    let mut sorted_elves = elves.clone();
    sorted_elves.sort_by(|a, b| b.cmp(a));
    let top_three = &sorted_elves[..3]; // take the first three elements
    let sum: i32 = top_three.iter().sum();
    println!("The top three numbers are {:?}", sum);
}
