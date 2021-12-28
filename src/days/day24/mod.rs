use std::{collections::HashMap};

static INPUT: &str = include_str!("./input.txt");


#[derive(Debug)]
struct ALU {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}
impl ALU {
    fn new(z: i64) -> Self {
        return ALU { w: 0, x: 0, y: 0, z, };
    }

    fn get_value(&self, reg: &str) -> i64 {
        match reg {
            "w" => self.w,
            "x" => self.x,
            "y" => self.y,
            "z" => self.z,
            number => {
                if let Ok(num) = number.parse::<i64>() {
                    return num;
                }
                panic!("Unknown register {}", number);
            },
        }
    }

    fn set_value(&mut self, reg: &str, value: i64) -> &Self {
        match reg {
            "w" => self.w = value,
            "x" => self.x = value,
            "y" => self.y = value,
            "z" => self.z = value,
            _ => panic!("Unknown register {}", reg),
        }
        return self;
    }

    fn inp(&mut self, reg: &str, input: i64) {
        self.set_value(reg, input);
    }

    fn add(&mut self, a: &str, b: &str) {
        let value = self.get_value(a) + self.get_value(b);
        self.set_value(a, value);
    }

    fn mul(&mut self, a: &str, b: &str) {
        let value = self.get_value(a) * self.get_value(b);
        self.set_value(a, value);
    }

    fn div(&mut self, a: &str, b: &str) {
        let value = self.get_value(a) / self.get_value(b);
        self.set_value(a, value);
    }

    fn modulo(&mut self, a: &str, b: &str) {
        let value = self.get_value(a) % self.get_value(b);
        self.set_value(a, value);
    }

    fn eql(&mut self, a: &str, b: &str) {
        let value = if self.get_value(a) == self.get_value(b) { 1 } else { 0 };
        self.set_value(a, value);
    }

    fn exec_cmd(&mut self, line: &str, input: &mut Vec<i64>) {
        let mut parts = line.split_whitespace();
        let cmd = parts.next().unwrap();
        let a = parts.next().unwrap();
        let b = match cmd {
            "inp" => "",
            _ => parts.next().unwrap(),
        };
        
        match cmd {
            "inp" => self.inp(a, input.remove(0)),
            "add" => self.add(a, b),
            "mul" => self.mul(a, b),
            "div" => self.div(a, b),
            "mod" => self.modulo(a, b),
            "eql" => self.eql(a, b),
            _ => panic!("Unknown command {}", cmd),
        };
    }

    fn exec(&mut self, program: &Vec<String>, input: &mut Vec<i64>) -> &Self {
        for line in program {
            self.exec_cmd(line, input);
        }
        return self;
    }
}

struct Monad {
    programs: Vec<Vec<String>>,
}
impl std::str::FromStr for Monad {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let recurring = 18;
        let mut counter = 0;
        let mut programs = Vec::new();
    
        while counter < s.lines().count() {
            programs.push(s.lines().skip(counter).take(recurring).map(|line| line.to_string()).collect());
            counter += recurring;
        }
        return Ok(Monad { programs });
    }
}

impl Monad {
    fn get_pairs(&self) -> HashMap<usize, usize>  {
        // each program either increments or decrements the z register
        // we want to pair each program that increments the z register with the one that decrements it
        let mut pairs = HashMap::new();
        let mut stack: Vec<usize> = Vec::new();
        for (i, program) in self.programs.iter().enumerate() {
            if program[4].split_whitespace().last().unwrap().parse::<i32>().unwrap() == 1 {
                stack.push(i);
            } else {
                if let Some(j) = stack.pop() {
                    pairs.insert(i, j);
                }
            }
            
        }
        return pairs;
    }

    fn run<F: Fn(i64) -> i64>(&mut self, transformer: F) -> u64 {
        let mut digits = [0; 14];

        for (x, y) in self.get_pairs() {
            let first = ALU::new(0).exec(&self.programs[y], &mut vec![0]).z;
            for delta in (-9..10).rev() {
                let second = ALU::new(first).exec(&self.programs[x], &mut vec![delta]).z;
                if second == 0 {
                    println!("{} {} {}", x, y, delta);
                    digits[y] = transformer(delta);
                    digits[x] = digits[y] + delta;
                    break;
                }
            }
        }

        //collect digits to a number
        let mut number = 0;
        for (i, digit) in digits.iter().rev().enumerate() {
            number += digit * 10_i64.pow(i as u32);
        }
        return number as u64;
    }
}


pub fn run() -> (u64, u64) {
    let mut monad = INPUT.parse::<Monad>().unwrap();


    let part1 = monad.run(|delta| 9.min(9 - delta));
    println!();
    let part2 = monad.run(|delta| 1.max(1 - delta));
    return (part1, part2);
}