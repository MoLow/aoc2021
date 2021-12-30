use std::collections::HashMap;

static INPUT: &str = include_str!("./input.txt");


#[derive(Debug, PartialEq)]
enum Cucumber {
    East,
    South
}
impl Cucumber {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '>' => Some(Cucumber::East),
            'v' => Some(Cucumber::South),
            _ => None
        }
    }
}

struct Ocean {
    width: i32,
    height: i32,
    map: HashMap<(i32, i32), Cucumber>,
}

impl std::str::FromStr for Ocean {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if let Some(cucumber) = Cucumber::from_char(c) {
                    map.insert((x as i32, y as i32), cucumber);
                }
            }
        }
        let width = input.lines().next().unwrap().len() as i32;
        let height = input.lines().count() as i32;

        Ok(Ocean { map, width, height })
    }
}
impl Ocean {
    fn print (&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(cucumber) = self.map.get(&(x, y)) {
                    match cucumber {
                        Cucumber::East => print!(">"),
                        Cucumber::South => print!("v"),
                    }
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn progress(&mut self, moves: & Vec<((i32, i32), (i32, i32))>) {
        for (from, to) in moves {
            let x = self.map.remove(&from).unwrap();
            self.map.insert(*to, x);
        }
    }

    fn step(&mut self) -> i32 {
        let mut moves: Vec<((i32, i32), (i32, i32))> = Vec::new();
        let mut total_moves = 0;
        for (&(x, y), _) in self.map.iter().filter(|(_, c)| **c == Cucumber::East).collect::<Vec<_>>() {
            let dest = ((x + 1) % self.width, y);
            if self.map.get(&dest).is_none() {
                moves.push(((x, y), dest));
                total_moves += 1;
            }
        }
        self.progress(&moves);
        moves.clear();

        for (&(x, y), _) in self.map.iter().filter(|(_, c)| **c == Cucumber::South).collect::<Vec<_>>() {
            let dest = (x, (y + 1) % self.height);
            if self.map.get(&dest).is_none() {
                moves.push(((x, y), dest));
                total_moves += 1;
            }
        }
        self.progress(&moves);

        return total_moves;
    }

    fn part1(&mut self) -> u64 {
        let mut count = 1;
        while self.step() > 0 {
            count += 1;
        }
        return count;
    }
}


pub fn run() -> (u64, u64) {
    let mut ocean = INPUT.parse::<Ocean>().unwrap();
    let part1 = ocean.part1();
    ocean.print();

    return (part1, 0);
}