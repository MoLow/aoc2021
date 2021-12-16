use std::{fs};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;

fn get_input(day: u8) -> String {
    let mut file = String::from(""); //env::current_dir().unwrap().to_str().unwrap().to_owned();
    file.push_str(format!("src/days/day{}/input.txt", day).as_str());
    let contents = fs::read_to_string(file);
    if contents.is_err() {
        panic!("Could not read file, input.txt does not exist for day {}", day);
    }

    return contents.expect("Could not read file, input.txt does not exist for day {}");
}

pub fn select_day(day: u8) {
    let input = get_input(day);
    return match day {
        1 => day1::run(input),
        2 => day2::run(input),
        3 => day3::run(input),
        4 => day4::run(input),
        5 => day5::run(input),
        6 => day6::run(input),
        7 => day7::run(input),
        8 => day8::run(input),
        9 => day9::run(input),
        10 => day10::run(input),
        11 => day11::run(input),
        12 => day12::run(input),
        13 => day13::run(input),
        14 => day14::run(input),
        15 => day15::run(input),
        _ => panic!("Day {} not implemented", day),
    };
}

