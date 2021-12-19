
static INPUT: &str = include_str!("./input.txt");

#[derive(Debug, Clone)]
struct SnailNumber {
    value: i32,
    depth: i32,
}
#[derive(Debug, Clone)]
struct SnailNumbers {
    numbers: Vec<SnailNumber>,
}
impl std::str::FromStr for SnailNumbers {
    type Err = std::num::ParseIntError;
    
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut numbers = Vec::new();
        let mut depth = 0;
        for char in input.chars() {
            match char {
                '[' => depth += 1,
                ']' => depth -= 1,
                ',' => { }
                c => { numbers.push(SnailNumber { value: c.to_digit(10).unwrap() as i32, depth: depth - 1 }); },
            }
        }
        
        return Ok(SnailNumbers { numbers });
    }
}

impl std::ops::Add<> for SnailNumbers {
    type Output = SnailNumbers;
    
    fn add(self, other: SnailNumbers) -> Self::Output {
        let mut numbers = self.numbers.clone();
        numbers.extend(other.numbers.clone());
        numbers.iter_mut().for_each(|snail| snail.depth += 1);
        let mut result = SnailNumbers { numbers };
        result.reduce();
        return result;
    }
}

impl SnailNumbers {
    fn expload(&mut self) -> bool{
        for i in 0..self.numbers.len() {
            if self.numbers[i].depth != 4 {
                continue;
            }
    
            if i> 0 {
                self.numbers[i-1].value += self.numbers[i].value;
            }
    
            if i < self.numbers.len() - 2 {
                self.numbers[i+2].value += self.numbers[i+1].value;
            }
    
            self.numbers[i].value = 0;
            self.numbers[i].depth = 3;
            self.numbers.remove(i+1);
            return true;
        }
    
        return false
    }
    
    fn split(&mut self) -> bool {
        for i in 0..self.numbers.len() {
            if self.numbers[i].value < 10 {
                continue;
            }
    
            let half = self.numbers[i].value as f32 / 2.0;
            self.numbers[i].value = half.floor() as i32;
            self.numbers[i].depth += 1;
            self.numbers.insert(i+1, SnailNumber { value: half.ceil() as i32, depth: self.numbers[i].depth });
    
            return true;
        }
    
        return false;
    }

    fn reduce(&mut self) {
        while self.expload() || self.split() {
        }
    }
    
    fn count_magnitude(&self) -> i32 {
        let mut values = self.numbers.iter().map(|s| s.value).collect::<Vec<i32>>();
        while values.len() > 1 {
            let mut next_pairs: Vec<i32> = Vec::new();
            for pair in values.chunks(2) {
                if pair.len() ==1 {
                    next_pairs.push(pair[0]);
                } else {
                    next_pairs.push(pair[0] * 3 + pair[1] * 2);
                }
            }
            values = next_pairs;
        }
    
        return values[0];
    }    
}


fn part1() -> i32 {
    let mut lines = INPUT.lines();
    let mut snails = lines.next().unwrap().parse::<SnailNumbers>().unwrap();
    for line in lines {
        snails = snails + line.parse::<SnailNumbers>().unwrap();
    }
    
    return snails.count_magnitude();
}

fn part2() -> i32 {
    let snails = INPUT.lines().map(|line| line.parse::<SnailNumbers>().unwrap()).collect::<Vec<SnailNumbers>>();
    let mut max_magnitude = 0;
    for sanil1 in &snails {
        for sanil2 in &snails {
            let sum = sanil1.clone() + sanil2.clone();
            let magnitude = sum.count_magnitude();
            
            max_magnitude = max_magnitude.max(magnitude);
        }
    }
    return max_magnitude;
}


pub fn run() -> (u64, u64) { 
    let part1 = part1() as u64;
    let part2 = part2() as u64;
    return (part1, part2);
}