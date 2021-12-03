
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

fn get_commands(input: &String) -> Vec<Command> {
    return input
        .split('\n')
        .filter(|&x| !x.is_empty())
        .map(|x| x.parse::<Command>().unwrap())
        .collect();
}

fn calc_coordinates(input: &String) -> Coordinates {
    return get_commands(input)
        .iter()
        .fold(Coordinates { x: 0, y: 0, aim: 0 }, |acc, x| {
            match x {
                Command::Up(x) => Coordinates { x: acc.x, y: acc.y - x, aim: acc.aim },
                Command::Down(x) => Coordinates { x: acc.x, y: acc.y + x, aim: acc.aim },
                Command::Forward(x) => Coordinates { x: acc.x + x, y: acc.y, aim: acc.aim },
            }
        });
}

fn calc_coordinates_by_aim(input: &String) -> Coordinates {
    return get_commands(input)
        .iter()
        .fold(Coordinates { x: 0, y: 0, aim: 0 }, |acc, x| {
            match x {
                Command::Up(x) => Coordinates { x: acc.x, y: acc.y, aim: acc.aim - x },
                Command::Down(x) => Coordinates { x: acc.x, y: acc.y, aim: acc.aim + x },
                Command::Forward(x) => Coordinates { x: acc.x + x, y: acc.y + acc.aim * x, aim: acc.aim },
            }
        });
}


pub fn run(input: String) {
    let input_ref = &input;
    let coords = calc_coordinates(input_ref);
    let coord_by_aim = calc_coordinates_by_aim(input_ref);

    println!("part 1: {} X {} = {}", coords.x, coords.y, coords.x * coords.y);
    println!("part 2: {} X {} = {}", coord_by_aim.x, coord_by_aim.y, coord_by_aim.x * coord_by_aim.y);
}