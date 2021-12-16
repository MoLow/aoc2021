use std::{env};

mod days;

fn print(s: &str) {
    println!("{}", s);
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1);
    if day.is_none() {
        println!("Please enter a valid day");
        return;
    }

    let day = day.unwrap().parse().unwrap();
    let (part1, part2) = days::select_day(day, print);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
