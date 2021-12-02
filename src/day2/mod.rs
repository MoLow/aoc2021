use std::fs;
use std::env;

struct Coordinates {
    x: i32,
    y: i32,
    aim: i32,
}

enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}
impl std::str::FromStr for Command {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let command = iter.next().unwrap();
        let value = iter.next().unwrap().parse::<i32>().unwrap();
        match command {
            "up" => Ok(Command::Up(value)),
            "down" => Ok(Command::Down(value)),
            "forward" => Ok(Command::Forward(value)),
            _ => Err(format!("Unknown command {}", command)),
        }
    }
}

fn get_lines() -> Vec<Command> {
    let mut dir = env::current_dir().unwrap().to_str().unwrap().to_owned();
    dir.push_str("/src/day2/input.txt");
    let contents = fs::read_to_string(dir).expect("Something went wrong reading the file");

    return contents
        .split('\n')
        .filter(|&x| !x.is_empty())
        .map(|x| x.parse::<Command>().unwrap())
        .collect();
}

fn calc_coordinates() -> Coordinates {
    return get_lines()
        .iter()
        .fold(Coordinates { x: 0, y: 0, aim: 0 }, |acc, x| {
            match x {
                Command::Up(x) => Coordinates { x: acc.x, y: acc.y - x, aim: acc.aim },
                Command::Down(x) => Coordinates { x: acc.x, y: acc.y + x, aim: acc.aim },
                Command::Forward(x) => Coordinates { x: acc.x + x, y: acc.y, aim: acc.aim },
            }
        });
}

fn calc_coordinates_by_aim() -> Coordinates {
    return get_lines()
        .iter()
        .fold(Coordinates { x: 0, y: 0, aim: 0 }, |acc, x| {
            match x {
                Command::Up(x) => Coordinates { x: acc.x, y: acc.y, aim: acc.aim - x },
                Command::Down(x) => Coordinates { x: acc.x, y: acc.y, aim: acc.aim + x },
                Command::Forward(x) => Coordinates { x: acc.x + x, y: acc.y + acc.aim * x, aim: acc.aim },
            }
        });
}

pub fn run() {
    let coords = calc_coordinates();
    let coord_by_aim = calc_coordinates_by_aim();

    println!("part 1: {} X {} = {}", coords.x, coords.y, coords.x * coords.y);
    println!("part 2: {} X {} = {}", coord_by_aim.x, coord_by_aim.y, coord_by_aim.x * coord_by_aim.y);
}