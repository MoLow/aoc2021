use std::fs;
use std::env;

fn get_lines() -> Vec<i32> {
    let mut dir = env::current_dir().unwrap().to_str().unwrap().to_owned();
    dir.push_str("/src/day1/input.txt");
    let contents = fs::read_to_string(dir).expect("Something went wrong reading the file");

    return contents.split_whitespace().map(|x| x.parse::<i32>().expect("Not a number")).collect();
}

pub fn count_increasing(window_size: usize) -> i32 {
    return get_lines()
        .windows(window_size)
        .map(|x| x.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|x| x[0] < x[1])
        .count()
        .try_into()
        .unwrap()
}

pub fn run() {
    println!("total increasing: {}", count_increasing(1));
    println!("total increasing - part 2: {}", count_increasing(3));
}