use std::fs;

mod day1;
mod day2;

fn get_input(day: u8) -> String {
    let mut file = String::from(""); //env::current_dir().unwrap().to_str().unwrap().to_owned();
    file.push_str(format!("src/day{}/input.txt", day).as_str());
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");

    return contents;
}

fn main() {
    println!("################ Day 1 ################");
    day1::run(get_input(1));
    println!("################ Day 2 ################");
    day2::run(get_input(1));
}
