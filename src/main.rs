use std::{env};

mod days;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1);
    if day.is_none() {
        println!("Please enter a valid day");
        return;
    }

    let day = day.unwrap().parse().unwrap();

    println!("################ Day {} ################", day);
    days::select_day(day);
}
