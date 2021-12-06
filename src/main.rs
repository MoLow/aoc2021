use std::{fs, env};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn get_input(day: u8) -> String {
    let mut file = String::from(""); //env::current_dir().unwrap().to_str().unwrap().to_owned();
    file.push_str(format!("src/day{}/input.txt", day).as_str());
    let contents = fs::read_to_string(file);
    if contents.is_err() {
        panic!("Could not read file, input.txt does not exist for day {}", day);
    }

    return contents.expect("Could not read file, input.txt does not exist for day {}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1);
    if day.is_none() {
        println!("Please enter a valid day");
        return;
    }

    let day = day.unwrap().parse().unwrap();
    let input = get_input(day);

    println!("################ Day {} ################", day);
    match day {
        1 => day1::run(input),
        2 => day2::run(input),
        3 => day3::run(input),
        4 => day4::run(input),
        5 => day5::run(input),
        6 => day6::run(input),
        _ => println!("Please enter a valid day"),
    }
}
